<div align="center">

# 🔍 AMADiag

**Automated root-cause analysis for Azure Monitor Agent troubleshooter output**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/johnsirmon/AMADiag/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey.svg)]()

*Drop in an AMA troubleshooter bundle. Get a diagnostic report in seconds — no AMA expertise required.*

</div>

---

## 💡 Why AMADiag?

When Azure Monitor Agent breaks — logs stop, perf counters vanish, syslog goes silent — diagnosing the cause means manually sifting through extension logs, DCR configs, IMDS responses, and MetricsExtension traces across multiple Microsoft Learn guides.

**AMADiag does that for you.** Feed it a troubleshooter bundle and it returns a structured report with root causes, severity ratings, and step-by-step remediation — linked directly to the relevant docs.

```
┌─────────────────────────────────────┐
│  AMA Troubleshooter Output Bundle   │
│  (.tgz / .zip / log directory)      │
└──────────────┬──────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│           AMADiag CLI                │
│                                      │
│  ┌────────────┐  ┌────────────────┐  │
│  │  Parsers   │─▶│  Rules Engine  │  │
│  │  Win / Lin │  │  (17 YAML +    │  │
│  └────────────┘  │   9 regex)     │  │
│                  └───────┬────────┘  │
│                          │           │
│  ┌───────────────────────▼────────┐  │
│  │  Report Generator              │  │
│  │  Markdown  ·  JSON             │  │
│  └────────────────────────────────┘  │
└──────────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│  Diagnostic Report                   │
│  • Findings with severity & evidence │
│  • Remediation steps                 │
│  • Environment sizing assessment     │
│  • Links to Microsoft Learn docs     │
└──────────────────────────────────────┘
```

---

## ✨ Highlights

- 🔎 **17 built-in detection rules** across 7 categories — installation, connectivity, DCR, identity, performance counters, syslog/CEF, and environment sizing
- 🧠 **9 regex pattern scanners** for IMDS, auth, connectivity, service crashes, DCR errors, extension failures, syslog, MetricsExtension, and Arc agent issues
- 🪟🐧 **Auto-detect Windows & Linux** bundles from file structure alone
- 📦 **Archive support** — `.tgz`, `.zip`, or plain directories
- 📊 **Dual output** — human-readable Markdown or machine-readable JSON for CI/CD
- 🖥️ **Interactive TUI** — keyboard-first dashboard for analyzing bundles and exporting reports
- ⚡ **Single binary, zero runtime deps** — all rules embedded at compile time
- 🧩 **YAML-extensible** — add custom detection rules without writing Rust
- 🏥 **XML config parsing** — streaming parser for `mcsconfig.lkg.xml` / `mcsconfig.latest.xml`
- 📈 **ETW trace analysis** — MetricsExtension CSV event table parsing

---

## 🚀 Quick Start

If you want the easiest path, especially for someone new to command-line tools, start with [`quickstart.md`](quickstart.md).

```bash
# 1. Clone and build
git clone https://github.com/johnsirmon/AMADiag.git
cd AMADiag
cargo build --release

# 2. Analyze a troubleshooter bundle
./target/release/amadiag analyze /path/to/bundle.tgz

# 2b. Or launch the interactive TUI
./target/release/amadiag tui

# 3. Review the report
#    Findings are printed to stdout as a Markdown table
```

Or install directly:

```bash
cargo install --path .
amadiag analyze /path/to/bundle.tgz
```

For non-build users, download prebuilt Windows and Linux binaries from the repository's **Releases** page.

---

## 📖 Usage

### Analyze a troubleshooter bundle

```bash
# Linux .tgz bundle
amadiag analyze /tmp/ama-troubleshooter-output.tgz

# Windows zip or directory
amadiag analyze C:\AMA-Diag-Logs\

# JSON output for CI/CD integration
amadiag analyze ./bundle.tgz --format json --output report.json

# Verbose logging
amadiag analyze ./bundle.tgz --verbose

# Interactive TUI
amadiag tui

# Start the TUI with a preselected path
amadiag tui ./bundle.tgz
```

The TUI is keyboard-first and path-driven in v1:

- type or paste a path to a `.zip`, `.tgz`, `.tar.gz`, or extracted log folder
- press `Enter` to analyze
- use `Up` / `Down` to navigate findings
- use `Tab` to switch between the findings list and the details pane
- press `m` to export Markdown or `j` to export JSON
- press `n` to return to path entry, `r` to rerun, and `q` to quit

### Validate a bundle (no analysis)

```bash
amadiag validate /path/to/bundle
```

Prints file counts, sizes, and format summary without running the full analysis pipeline.

### List all detection rules

```bash
amadiag rules list
```

### Exit Codes

| Code | Meaning |
|:----:|---------|
| `0`  | Analysis complete — no critical findings |
| `1`  | Analysis complete — **critical findings detected** |
| `2`  | Input error (invalid path, unrecognized format) |

---

## 🛡️ Detection Rules

AMADiag ships with **17 YAML-defined rules** covering the most common AMA failure modes:

| ID | Name | Severity | Category | Platforms |
|----|------|:--------:|----------|:---------:|
| `INSTALL-001` | Extension Not Installed | 🔴 Critical | Installation | Win · Lin |
| `INSTALL-002` | Extension Provisioning Error Log | 🔴 Critical | Installation | Win · Lin |
| `CONN-001` | AMCS Endpoint Unreachable | 🔴 Critical | Connectivity | Win · Lin |
| `CONN-002` | Log Ingestion Endpoint Unreachable | 🔴 Critical | Connectivity | Win · Lin |
| `CONN-003` | IMDS Endpoint Unreachable | 🔴 Critical | Connectivity | Win · Lin |
| `DCR-001` | No DCR Configuration Found | 🔴 Critical | DCR | Win · Lin |
| `DCR-002` | Empty DCR Configuration | 🟡 Warning | DCR | Win · Lin |
| `IDENTITY-001` | Missing Managed Identity | 🔴 Critical | Identity | Win · Lin |
| `IDENTITY-002` | Auth Token Expired or Invalid | 🔴 Critical | Identity | Win · Lin |
| `PERF-001` | No Performance Counter Config | 🟡 Warning | Perf Counters | Win |
| `PERF-002` | Invalid Performance Counter Path | 🟡 Warning | Perf Counters | Win |
| `SYSLOG-001` | Syslog Forwarder Not Running | 🔴 Critical | Syslog | Lin |
| `SYSLOG-002` | Syslog Port Not Listening | 🟡 Warning | Syslog | Lin |
| `SYSLOG-003` | Syslog Config Missing AMA Forwarding | 🟡 Warning | Syslog | Lin |
| `SZ-001` | Memory Pressure Detected | 🟡 Warning | Sizing | Win · Lin |
| `SZ-002` | High CPU Usage by AMA | 🟡 Warning | Sizing | Win · Lin |
| `SZ-003` | Disk Space Low | 🟡 Warning | Sizing | Win · Lin |

> **Plus 9 regex-based pattern scanners** that run alongside YAML rules — catching IMDS errors, authentication failures, connectivity issues, service crashes, DCR errors, extension failures, syslog issues, MetricsExtension errors, and Arc agent problems.

---

<details>
<summary><strong>🧩 Adding Custom Rules</strong></summary>

Rules are defined in YAML. Each rule specifies a detection condition and remediation guidance:

```yaml
- id: CUSTOM-001
  name: My Custom Detection
  severity: warning          # critical | warning | info
  category: connectivity     # installation | identity | connectivity | dcr | ...
  platforms: [windows, linux]
  description: >
    Description of what this rule detects.
  detection:
    file_pattern: ".log"           # file name substring to match
    condition: content_match       # file_missing | file_present | content_match | xml_element_missing
    content_regex: "(?i)my-error-pattern"
  remediation: >
    Steps to fix the issue.
  doc_link: https://learn.microsoft.com/en-us/...
```

Built-in rules are embedded at compile time from `src/rules/`. To add new rules, create or modify YAML files in that directory and rebuild.

</details>

<details>
<summary><strong>📁 Project Structure</strong></summary>

```
src/
├── main.rs                 # CLI entry point (clap)
├── lib.rs                  # Library root
├── detect.rs               # Orchestrator: extract → parse → analyze → report
├── input.rs                # Bundle extraction (.tgz, .zip, directory) + validation
├── parsers/
│   ├── mod.rs              # File walker, platform detection
│   ├── common.rs           # Log line classification, regex patterns
│   ├── xml_config.rs       # mcsconfig XML parser (quick-xml)
│   ├── event_table.rs      # MetricsExtension ETW CSV parser
│   ├── windows.rs          # Windows-specific enrichment
│   └── linux.rs            # Linux-specific enrichment
├── analyzers/
│   ├── mod.rs              # Analysis orchestration, pattern scanning
│   ├── finding.rs          # Finding / Severity / Category / DiagnosticReport
│   ├── rules.rs            # YAML rules engine (load, evaluate, display)
│   └── sizing.rs           # Environment sizing checks (memory, CPU, disk)
├── reporters/
│   ├── mod.rs              # OutputFormat enum
│   ├── markdown.rs         # Markdown report renderer
│   └── json.rs             # JSON report renderer
└── rules/                  # Embedded YAML rule definitions
    ├── installation.yaml
    ├── connectivity.yaml
    ├── dcr.yaml
    ├── identity.yaml
    ├── performance.yaml
    ├── syslog.yaml
    └── sizing.yaml
```

</details>

---

## 🎯 Who Is This For?

| Persona | Use Case |
|---------|----------|
| **Azure Admin / IT Pro** | Fast root-cause identification without reading multiple troubleshooting docs |
| **SOC / Sentinel Analyst** | Determine why syslog/CEF data stopped flowing into Sentinel |
| **Microsoft Support Engineer** | Structured analysis to accelerate case resolution from customer bundles |
| **DevOps / SRE** | CLI + JSON output for validating AMA health in deployment pipelines |

---

## 📋 Requirements

| | |
|---|---|
| **Build** | Rust 1.70+ |
| **Runtime** | No dependencies — single statically-linked binary |
| **Input** | AMA troubleshooter output (`.tgz`, `.zip`, or directory) |

---

## 🤝 Contributing

Contributions are welcome! The easiest way to get started is by adding a new detection rule — all it takes is a YAML file (see **Adding Custom Rules** above).

1. Fork the repo
2. Create a feature branch (`git checkout -b feat/my-rule`)
3. Add or modify rules in `src/rules/`
4. Run `cargo test` to validate
5. Open a pull request

---

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

---

## 📚 Acknowledgements

AMADiag builds on the diagnostic guidance from these Microsoft Learn resources:

- [Azure Monitor Agent Overview](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-overview)
- [Troubleshoot AMA on Windows VMs](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm)
- [Troubleshoot AMA on Linux VMs](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm)
- [AMA Troubleshooter — Windows](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-windows)
- [AMA Troubleshooter — Linux](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-linux)
- [Troubleshoot CEF/Syslog via AMA](https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting)
- [Data Collection Rules](https://learn.microsoft.com/en-us/azure/azure-monitor/vm/data-collection)

---

<div align="center">

*Built with 🦀 Rust — fast, safe, zero-dependency diagnostics for Azure Monitor Agent.*

</div>
