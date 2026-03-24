# AMADiag — Azure Monitor Agent Diagnostic Analyzer

> Open-source tool for automated analysis of Azure Monitor Agent (AMA) troubleshooter output on Linux and Windows. Identifies common failures, environment sizing issues, and actionable remediation steps — so customers can self-resolve without opening a support case.

---

## 1. Problem

Azure Monitor Agent (AMA) is deployed across millions of Azure VMs, VMSS instances, and Arc-enabled servers. When data collection breaks — logs stop flowing, performance counters go missing, syslog/CEF ingestion fails — customers must:

1. **Run the AMA Troubleshooter** ([Windows](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-windows?tabs=WindowsPowerShell) | [Linux](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-linux?tabs=redhat%2CGenerateLogs)) to generate diagnostic logs.
2. **Manually interpret** log files spanning extension logs, agent core logs, DCR config XML, IMDS responses, MetricsExtension ETW traces, and syslog forwarder state.
3. **Cross-reference** multiple Microsoft Learn troubleshooting guides ([Windows VM](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm), [Windows Arc](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc), [Linux VM](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm), [Syslog/CEF](https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting)) to map symptoms to root causes.

**This process is time-consuming, error-prone, and requires deep product knowledge.** Common issues — missing managed identity, DCR misconfiguration, IMDS unreachable, undersized VMs dropping events, extension provisioning failures — are well-documented but hard to spot in raw logs. The result: longer time-to-resolution, unnecessary support tickets, and customer frustration.

### Evidence

- A significant percentage of AMA support cases involve issues identifiable from troubleshooter output alone (installation failures, DCR association, connectivity).
- The Microsoft Q&A forums show recurring patterns: heartbeat present but no perf data on a subset of VMs, syslog not forwarding, MetricsExtension errors — all diagnosable from logs.
- The existing AMA Troubleshooter performs health checks but does **not** provide root-cause analysis or remediation guidance from the collected logs.

---

## 2. Non-Goals

Define boundaries early — what AMADiag will **not** do:

| Non-Goal | Rationale |
|---|---|
| Replace the AMA Troubleshooter itself | AMADiag consumes troubleshooter output; it does not replicate the agent health checks or log collection |
| Modify agent configuration or DCRs | Read-only analysis only — no writes to the customer environment |
| Real-time monitoring or alerting | This is a post-hoc diagnostic tool, not an ongoing monitoring agent |
| Support for legacy MMA/OMS agents | Scope is limited to AMA (Azure Monitor Agent) only |
| Cloud-hosted SaaS service | Runs locally or in CI; no data leaves the customer's machine unless they choose to share |
| Automated remediation / fix-it scripts | V1 provides diagnosis and recommendations; automated fixes are a future consideration |

---

## 3. Goals

| # | Goal | Success Signal |
|---|---|---|
| G1 | Automatically identify the root cause of AMA data collection failures from troubleshooter log bundles | ≥ 80% of known issue categories detected in test corpus |
| G2 | Detect undersized environments (CPU, memory, disk) that cause event drops or agent instability | Correct sizing alerts on all test VMs with known resource constraints |
| G3 | Produce human-readable reports with specific remediation steps and linked documentation | Every finding includes a severity, explanation, and link to relevant Microsoft Learn article |
| G4 | Support both AMA Windows and AMA Linux troubleshooter output formats | Full parser coverage for both OS troubleshooter bundles |
| G5 | Be usable by customers with no AMA internals expertise | A customer can run the tool and understand the output without reading the troubleshooting guides first |

---

## 4. Target Users

| Persona | Description | Primary Need |
|---|---|---|
| **Azure Admin / IT Pro** | Manages AMA deployments across VMs and Arc servers. Comfortable with PowerShell/Bash but not AMA internals. | Fast root-cause identification without reading multiple docs |
| **SOC / Sentinel Analyst** | Depends on syslog/CEF data flowing into Sentinel. Needs to know *why* logs stopped and *when* they'll resume. | Clear status: is AMA healthy, is the DCR correct, is syslog forwarding working |
| **Microsoft Support Engineer (CSS)** | Triages AMA support cases. Needs to quickly assess troubleshooter bundles from customers. | Structured analysis output to accelerate case resolution |
| **DevOps / SRE** | Manages AMA at scale via IaC. Wants to validate AMA health as part of deployment pipelines. | CLI-friendly tool with machine-readable output (JSON) for CI/CD integration |

---

## 5. Solution Overview

**AMADiag** is a command-line tool that:

1. **Ingests** an AMA troubleshooter output bundle (`.tgz` on Linux, log directory on Windows).
2. **Parses** all relevant log files, configuration files, and diagnostic artifacts.
3. **Analyzes** the parsed data against a library of known issue signatures (rules engine).
4. **Reports** findings as a structured diagnostic report (human-readable Markdown + machine-readable JSON).

```
┌─────────────────────────────────────┐
│  AMA Troubleshooter Output Bundle   │
│  (.tgz / log directory)             │
└──────────────┬──────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│           AMADiag CLI                │
│  ┌────────────┐  ┌────────────────┐  │
│  │  Parsers   │  │  Rules Engine  │  │
│  │  (Win/Lin) │→ │  (Signatures)  │  │
│  └────────────┘  └───────┬────────┘  │
│                          │           │
│  ┌───────────────────────▼────────┐  │
│  │  Report Generator              │  │
│  │  (Markdown + JSON)             │  │
│  └────────────────────────────────┘  │
└──────────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│  Diagnostic Report                   │
│  • Findings (severity, description)  │
│  • Remediation steps                 │
│  • Environment sizing assessment     │
│  • Links to Microsoft Learn docs     │
└──────────────────────────────────────┘
```

---

## 6. Functional Requirements

### 6.1 Input Handling

| ID | Requirement |
|---|---|
| F-IN-1 | Accept a path to a Linux AMA troubleshooter `.tgz` bundle and automatically extract it |
| F-IN-2 | Accept a path to a Windows AMA troubleshooter log directory or zipped output |
| F-IN-3 | Auto-detect whether the input is from a Windows or Linux agent based on file structure |
| F-IN-4 | Validate input completeness — warn if expected files are missing from the bundle |

### 6.2 Log Parsing — Windows

| ID | Requirement | Key Files |
|---|---|---|
| F-PW-1 | Parse extension provisioning logs | `C:\WindowsAzure\Logs\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\` |
| F-PW-2 | Parse agent core logs | `C:\WindowsAzure\Resources\AMADataStore.<vm>\Configuration\` |
| F-PW-3 | Parse DCR configuration | `mcsconfig.latest.xml`, `mcsconfig.lkg.xml`, `configchunks\` |
| F-PW-4 | Parse IMDS / managed identity state | `AuthToken-MSI.json`, MAEventTable entries |
| F-PW-5 | Parse MetricsExtension ETW traces | `MaMetricsExtensionEtw.tsf` (via `table2csv.exe` output) |
| F-PW-6 | Parse performance counter configuration | `CounterSet` nodes in `mcsconfig.lkg.xml` |
| F-PW-7 | Parse Windows Event Log subscription config | `Subscription` nodes in `mcsconfig.lkg.xml` |
| F-PW-8 | Parse AgentTroubleshooter output log | `AgentTroubleshooter.exe` output file |

### 6.3 Log Parsing — Linux

| ID | Requirement | Key Files |
|---|---|---|
| F-PL-1 | Parse AMA extension logs | `/var/lib/waagent/Microsoft.Azure.Monitor.AzureMonitorLinuxAgent-{version}/` |
| F-PL-2 | Parse agent core logs (mdsd) | `/var/opt/microsoft/azuremonitoragent/log/mdsd.*` |
| F-PL-3 | Parse DCR configuration | Downloaded DCR JSON and config chunks |
| F-PL-4 | Parse syslog forwarder state (rsyslog/syslog-ng) | `/etc/rsyslog.d/`, `/etc/syslog-ng/`, rsyslog service status |
| F-PL-5 | Parse AMA Troubleshooter interactive/log mode output | `ama_troubleshooter.sh` output |
| F-PL-6 | Parse Python dependency state | Python version, required packages presence |

### 6.4 Analysis Rules — Issue Detection

| ID | Category | Detection |
|---|---|---|
| F-AN-1 | **Installation Failure** | Extension not provisioned, missing binaries, version mismatch |
| F-AN-2 | **Agent Not Running** | `MonAgentCore.exe` / `mdsd` process not running, service crash loops |
| F-AN-3 | **Managed Identity Missing** | No system-assigned or user-assigned identity, `AuthToken-MSI.json` absent |
| F-AN-4 | **IMDS Unreachable** | IMDS/HIMDS connectivity failures in MAEventTable or agent logs |
| F-AN-5 | **DCR Not Associated** | No `mcsconfig.latest.xml`, empty configchunks directory |
| F-AN-6 | **DCR Misconfiguration** | Missing `performanceCounters`, `windowsEventLogs`, or `syslog` sections when expected |
| F-AN-7 | **Connectivity Failure** | Cannot reach AMCS, log ingestion endpoints, or metrics endpoints |
| F-AN-8 | **Performance Counter Issues** | Missing `CounterSet` nodes, invalid counter paths, custom metrics token errors |
| F-AN-9 | **Event Log Collection Issues** | Missing `Subscription` nodes, invalid XPath queries |
| F-AN-10 | **Syslog/CEF Forwarding Failure** | rsyslog/syslog-ng misconfiguration, port 514 not listening, AMA not receiving forwarded logs |
| F-AN-11 | **MetricsExtension Errors** | Level 2 errors in `MaMetricsExtensionEtw.csv`, `-TokenSource MSI` missing |
| F-AN-12 | **Heartbeat Present / Data Missing** | Agent heartbeat exists but perf/event data not flowing — partial collection failure |
| F-AN-13 | **Arc Agent Issues** | Connected Machine Agent not running, extension service stopped, HIMDS errors |
| F-AN-14 | **Version Mismatch** | Agent version below minimum supported, troubleshooter/agent version skew |

### 6.5 Analysis Rules — Environment Sizing

| ID | Detection |
|---|---|
| F-SZ-1 | Detect low available memory (< 256 MB free) that causes event buffer drops |
| F-SZ-2 | Detect high CPU utilization by AMA processes indicating undersized VM |
| F-SZ-3 | Detect disk space constraints on log/data store partitions |
| F-SZ-4 | Detect high event volume relative to VM SKU (ingestion rate exceeding agent capacity) |
| F-SZ-5 | Flag VM SKU / size in report when identifiable from IMDS data |

### 6.6 Reporting

| ID | Requirement |
|---|---|
| F-RP-1 | Generate a Markdown report with: summary, findings table, detailed findings, and environment info |
| F-RP-2 | Generate a JSON report with structured findings for programmatic consumption |
| F-RP-3 | Each finding includes: severity (`Critical` / `Warning` / `Info`), category, description, evidence (log excerpts), remediation steps, and documentation link |
| F-RP-4 | Include an environment summary section: OS, AMA version, VM type, Arc vs native, DCR count |
| F-RP-5 | Support output to stdout, file, or piped to another tool |

---

## 7. Technical Approach

### 7.1 Technology

| Decision | Choice | Rationale |
|---|---|---|
| Language | **Python 3.9+** | Cross-platform, rich parsing libraries, accessible to contributors, aligns with Linux AMA troubleshooter (Python-based) |
| Packaging | PyPI (`pip install amadiag`) + standalone binary (PyInstaller) | Maximize reach — pip for DevOps, binary for IT Pros |
| Rules format | YAML rule definitions | Easy for community to contribute new detection rules without writing code |
| Output | Markdown + JSON | Human-readable + machine-readable |

### 7.2 Project Structure

```
amadiag/
├── amadiag/
│   ├── __init__.py
│   ├── cli.py                  # CLI entry point (argparse)
│   ├── detector.py             # Orchestrates parsing → analysis → reporting
│   ├── parsers/
│   │   ├── __init__.py
│   │   ├── windows.py          # Windows log/config parsers
│   │   ├── linux.py            # Linux log/config parsers
│   │   └── common.py           # Shared parsing utilities
│   ├── analyzers/
│   │   ├── __init__.py
│   │   ├── rules_engine.py     # Loads and evaluates YAML rules
│   │   ├── sizing.py           # Environment sizing analysis
│   │   └── findings.py         # Finding data model
│   ├── reporters/
│   │   ├── __init__.py
│   │   ├── markdown.py         # Markdown report generator
│   │   └── json_report.py      # JSON report generator
│   └── rules/
│       ├── installation.yaml
│       ├── connectivity.yaml
│       ├── dcr.yaml
│       ├── identity.yaml
│       ├── performance.yaml
│       ├── syslog.yaml
│       └── sizing.yaml
├── tests/
│   ├── fixtures/               # Anonymized sample troubleshooter bundles
│   │   ├── windows/
│   │   └── linux/
│   └── test_*.py
├── docs/
│   ├── contributing.md
│   ├── adding-rules.md         # Guide for community rule contributions
│   └── sample-reports/
├── pyproject.toml
├── LICENSE                     # MIT
└── README.md
```

### 7.3 Rule Definition Format

```yaml
# rules/identity.yaml
- id: IDENTITY-001
  name: Missing Managed Identity
  severity: critical
  category: identity
  platforms: [windows, linux]
  description: >
    The VM does not have a system-assigned or user-assigned managed identity enabled.
    AMA requires a managed identity to authenticate with Azure Monitor services.
  detection:
    file_pattern: "AuthToken-MSI.json"
    condition: file_missing
  remediation: >
    Enable a system-assigned managed identity on the VM:
    az vm identity assign --resource-group <rg> --name <vm>
  doc_link: https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm
```

---

## 8. CLI Interface

```bash
# Analyze a Linux troubleshooter bundle
amadiag analyze /tmp/ama-troubleshooter-output.tgz

# Analyze a Windows troubleshooter log directory
amadiag analyze C:\AMA-Diag-Logs\ --format json --output report.json

# Analyze with verbose output
amadiag analyze ./bundle.tgz --verbose

# List all detection rules
amadiag rules list

# Validate a bundle without full analysis
amadiag validate /path/to/bundle
```

### Exit Codes

| Code | Meaning |
|---|---|
| 0 | Analysis complete — no critical findings |
| 1 | Analysis complete — critical findings detected |
| 2 | Input error (invalid path, unrecognized format) |
| 3 | Internal error |

---

## 9. Training & Validation Strategy

The tool's detection rules will be developed and validated against a curated corpus of AMA troubleshooter output from intentionally broken environments:

| Scenario | How Reproduced |
|---|---|
| Missing managed identity | Remove system identity from VM, collect troubleshooter output |
| DCR not associated | Delete DCR association, collect logs after data stops |
| IMDS unreachable | Block IMDS endpoint (169.254.169.254) via firewall, collect output |
| Extension install failure | Install on unsupported OS version or with blocked endpoints |
| Undersized VM | Run AMA on B1s with high-volume DCR, collect resource exhaustion evidence |
| Syslog forwarding broken | Misconfigure rsyslog to wrong port, stop rsyslog service |
| Performance counter misconfiguration | Reference invalid counter paths in DCR |
| Connectivity failure | Block AMCS/ingestion endpoints via NSG |
| Partial data collection | Configure DCR with both working and broken data sources |
| Arc agent issues | Stop extension service on Arc machine |

Each scenario produces a labeled test fixture stored in `tests/fixtures/` with expected findings documented alongside.

---

## 10. Success Metrics

| Metric | Target | Measurement |
|---|---|---|
| **Detection accuracy** | ≥ 80% of known issue categories correctly identified | Test suite pass rate against labeled fixtures |
| **False positive rate** | < 5% of findings are incorrect | Manual review of reports from healthy environments |
| **Time to diagnosis** | < 30 seconds for a typical bundle | CLI execution time benchmark |
| **Community adoption** | 100+ GitHub stars, 5+ external contributors within 6 months | GitHub metrics |
| **Rule coverage** | All issues documented in Microsoft Learn troubleshooting guides have corresponding rules | Coverage audit against docs |

---

## 11. Milestones

| Phase | Scope | Deliverable |
|---|---|---|
| **v0.1 — Foundation** | CLI skeleton, Windows + Linux parsers, 5 core detection rules, Markdown reporter | Usable MVP analyzing basic installation/connectivity issues |
| **v0.2 — Core Coverage** | All F-AN rules implemented, JSON reporter, sizing analysis | Covers the full spectrum of documented AMA issues |
| **v0.3 — Community Ready** | PyPI package, test fixtures, contributing guide, rule authoring docs, CI pipeline | Open-source release on GitHub |
| **v1.0 — Stable Release** | Validated against 50+ real-world troubleshooter bundles, comprehensive test suite | Production-quality tool ready for customer use |

---

## 12. Risks & Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| AMA troubleshooter output format changes between versions | Parsers break on new versions | Version-aware parsing; test against multiple AMA versions; monitor AMA release notes |
| Log files contain PII (computer names, IPs, subscription IDs) | Privacy concerns in test fixtures | All test fixtures are anonymized; document data handling policy |
| Rule library becomes stale as AMA evolves | False negatives on new issue types | Community contribution model; rule versioning; regular audit against Microsoft Learn docs |
| Windows `.tsf` binary log format is hard to parse without `table2csv.exe` | Incomplete Windows analysis | Accept pre-converted CSV or bundle `table2csv.exe` output instructions in docs |
| Scope creep toward automated remediation | Increases complexity and risk of unintended changes | Firm non-goal boundary; V1 is read-only analysis only |

---

## 13. Open Questions

| # | Question | Owner | Status |
|---|---|---|---|
| 1 | Should AMADiag support analyzing multiple bundles at once (fleet-wide analysis)? | PM | Open |
| 2 | Should we integrate with Azure Resource Graph to enrich findings with DCR / VM metadata? | Engineering | Open |
| 3 | What license is most appropriate for Microsoft-adjacent open-source tooling? (MIT recommended) | Legal | Open |
| 4 | Should the JSON output conform to SARIF or a custom schema? | Engineering | Open |
| 5 | Is there appetite for a VS Code extension that renders reports inline? | PM | Open |

---

## 14. References

- [Troubleshoot AMA on Windows VMs](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm)
- [Troubleshoot AMA on Windows Arc-enabled servers](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc)
- [AMA Troubleshooter — Windows](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-windows?tabs=WindowsPowerShell)
- [AMA Troubleshooter — Linux](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-linux?tabs=redhat%2CGenerateLogs)
- [Troubleshoot AMA on Linux VMs](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm)
- [Troubleshoot CEF/Syslog via AMA](https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting)
- [AMA Installation Issues — Detailed Steps](https://learn.microsoft.com/en-us/troubleshoot/azure/azure-monitor/azure-monitor-agent/ama-windows-installation-issues-detailed-troubleshooting-steps)
- [Azure Monitor Agent Overview](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-overview)
- [Data Collection Rules](https://learn.microsoft.com/en-us/azure/azure-monitor/vm/data-collection)
