# AMADiag

Diagnostic analyzer for [Azure Monitor Agent (AMA)](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-overview) troubleshooter output. Identifies common failures, environment sizing issues, and actionable remediation steps from troubleshooter log bundles — so problems can be resolved without manually reading through dozens of log files.

## Features

- **Automatic platform detection** — recognizes Windows and Linux AMA bundles
- **Archive extraction** — handles `.tgz`, `.zip`, and plain directories
- **17 built-in detection rules** across 7 categories: installation, connectivity, DCR, identity, performance counters, syslog/CEF, and environment sizing
- **Pattern-based log scanning** — regex engine detects IMDS failures, auth errors, service crashes, and more
- **XML config parsing** — streaming parser for `mcsconfig.lkg.xml` / `mcsconfig.latest.xml` (CounterSet, Subscription nodes)
- **CSV event table parsing** — MetricsExtension ETW trace analysis
- **Markdown and JSON reports** — human-readable or machine-readable output
- **Single binary** — no runtime dependencies, all rules embedded at compile time

## Installation

### From source

```bash
cargo install --path .
```

### Build from source

```bash
git clone https://github.com/johnsirmon/AMADiag.git
cd AMADiag
cargo build --release
# Binary at target/release/amadiag (.exe on Windows)
```

## Usage

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
```

### Validate a bundle

```bash
amadiag validate /path/to/bundle
```

Prints file counts, sizes, and format summary without running full analysis.

### List detection rules

```bash
amadiag rules list
```

### Exit codes

| Code | Meaning |
|------|---------|
| 0 | Analysis complete — no critical findings |
| 1 | Analysis complete — critical findings detected |
| 2 | Input error (invalid path, unrecognized format) |

## Detection Rules

| ID | Name | Severity | Category | Platforms |
|----|------|----------|----------|-----------|
| INSTALL-001 | Extension Not Installed | Critical | Installation | Windows, Linux |
| INSTALL-002 | Extension Provisioning Error Log | Critical | Installation | Windows, Linux |
| CONN-001 | AMCS Endpoint Unreachable | Critical | Connectivity | Windows, Linux |
| CONN-002 | Log Ingestion Endpoint Unreachable | Critical | Connectivity | Windows, Linux |
| CONN-003 | IMDS Endpoint Unreachable | Critical | Connectivity | Windows, Linux |
| DCR-001 | No DCR Configuration Found | Critical | DCR | Windows, Linux |
| DCR-002 | Empty DCR Configuration | Warning | DCR | Windows, Linux |
| IDENTITY-001 | Missing Managed Identity | Critical | Identity | Windows, Linux |
| IDENTITY-002 | Authentication Token Expired or Invalid | Critical | Identity | Windows, Linux |
| PERF-001 | No Performance Counter Configuration | Warning | Performance Counters | Windows |
| PERF-002 | Invalid Performance Counter Path | Warning | Performance Counters | Windows |
| SYSLOG-001 | Syslog Forwarder Not Running | Critical | Syslog | Linux |
| SYSLOG-002 | Syslog Port Not Listening | Warning | Syslog | Linux |
| SYSLOG-003 | Syslog Configuration Missing AMA Forwarding | Warning | Syslog | Linux |
| SZ-001 | Memory Pressure Detected | Warning | Sizing | Windows, Linux |
| SZ-002 | High CPU Usage by AMA | Warning | Sizing | Windows, Linux |
| SZ-003 | Disk Space Low | Warning | Sizing | Windows, Linux |

In addition to YAML-defined rules, the log scanner runs 9 regex-based pattern checks for IMDS errors, authentication failures, connectivity issues, service crashes, DCR errors, extension failures, syslog issues, MetricsExtension errors, and Arc agent problems.

## Project Structure

```
src/
├── main.rs                 # CLI entry point (clap)
├── lib.rs                  # Library root
├── detect.rs               # Orchestrator: extract → parse → analyze → report
├── input.rs                # Bundle extraction (.tgz, .zip, directory) and validation
├── parsers/
│   ├── mod.rs              # File walker, platform detection
│   ├── common.rs           # Log line classification, regex patterns
│   ├── xml_config.rs       # mcsconfig XML parser (quick-xml)
│   ├── event_table.rs      # MetricsExtension ETW CSV parser
│   ├── windows.rs          # Windows-specific enrichment
│   └── linux.rs            # Linux-specific enrichment
├── analyzers/
│   ├── mod.rs              # Analysis orchestration, pattern scanning
│   ├── finding.rs          # Data model (Finding, Severity, Category, DiagnosticReport)
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

## Adding Custom Rules

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

## Requirements

- Rust 1.70+ (build)
- No runtime dependencies

## License

MIT
