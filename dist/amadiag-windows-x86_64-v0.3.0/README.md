<div align="center">

# 🔍 AMADiag

**Automated root-cause analysis for Azure Monitor Agent troubleshooter output**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.3.0-green.svg)](https://github.com/johnsirmon/AMADiag/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey.svg)]()

*Drop in an AMA troubleshooter bundle. Get a diagnostic report in seconds — no AMA expertise required.*

</div>

---

<img width="1263" height="663" alt="image" src="https://github.com/user-attachments/assets/96bcab00-8aff-4bfc-9d57-2aa13182a5d2" />

## 💡 Why AMADiag?

When Azure Monitor Agent breaks — logs stop, perf counters vanish, syslog goes silent — diagnosing the cause usually means manually sifting through extension logs, DCR configs, IMDS responses, and MetricsExtension traces across multiple Microsoft Learn guides.

**AMADiag does that work for you.** Feed it a troubleshooter bundle and it returns a structured report with findings, severity, evidence, remediation guidance, and links to the relevant docs.

```
┌─────────────────────────────────────┐
│  AMA Troubleshooter Output Bundle   │
│  (.tgz / .tar.gz / .zip / folder)   │
└──────────────┬──────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│           AMADiag CLI                │
│                                      │
│  ┌────────────┐  ┌────────────────┐  │
│  │  Parsers   │─▶│  Rules Engine  │  │
│  │  Win / Lin │  │  (YAML rules + │  │
│  └────────────┘  │   log scanners)│  │
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
│  • Severity-ranked findings          │
│  • Evidence and remediation          │
│  • Environment summary               │
│  • Microsoft Learn links             │
└──────────────────────────────────────┘
```

---

## ✨ Highlights

- 🔎 **37 built-in YAML detection rules** across installation, connectivity, DCR, identity, performance, syslog, Linux collection, and sizing scenarios
- 🧠 **Regex-based log scanners** for common AMA failures such as IMDS, auth, service crashes, extension issues, throttling, disk pressure, and upload errors
- 🐧 **Linux-specific analysis** for Fluent Bit, rsyslog forwarding, QoS upload health, proxy signals, and OS details
- 🪟🐧 **Automatic Windows/Linux bundle detection** from the extracted content
- 📦 **Archive and directory input support** for `.tgz`, `.tar.gz`, `.zip`, and extracted folders
- 📊 **Dual output formats** with Markdown for humans and JSON for automation
- 🖥️ **Interactive TUI** with a file browser, severity and time filtering, threaded analysis, and in-app export
- ⚡ **Single binary, zero runtime setup** with rules embedded at compile time
- 🧩 **YAML-extensible rule catalog** for adding new detections without changing the CLI surface

---

## 🚀 Quick Start

If you want the easiest path, especially for someone new to command-line tools, start with [`quickstart.md`](quickstart.md). It explains how to download a release build and run AMADiag without installing Rust.

```bash
# 1. Clone and build
git clone https://github.com/johnsirmon/AMADiag.git
cd AMADiag
cargo build --release

# 2. Analyze a troubleshooter bundle
./target/release/amadiag analyze /path/to/bundle.tgz

# 3. Or launch the interactive TUI
./target/release/amadiag tui
```

The `analyze` command prints a full Markdown diagnostic report to stdout by default. Use `--format json` when you want structured output for scripts or CI.

You can also install it directly:

```bash
cargo install --path .
amadiag analyze /path/to/bundle.tgz
```

For non-build users, download the Windows or Linux release package from **Releases** and run the included binary.

---

## 📖 Usage

### Analyze a troubleshooter bundle

```bash
# Linux archive
amadiag analyze /tmp/ama-troubleshooter-output.tgz

# Extracted folder
amadiag analyze /tmp/ama-logs

# JSON output for automation
amadiag analyze ./bundle.tgz --format json --output report.json

# Verbose logging (global flag)
amadiag --verbose analyze ./bundle.tgz
```

```powershell
# Windows zip or extracted directory
amadiag analyze C:\AMA-Diag-Logs
amadiag analyze C:\temp\ama-troubleshooter-output.zip
```

### Launch the interactive TUI

```bash
# Start in the file browser
amadiag tui

# Start immediately with a known bundle path
amadiag tui /path/to/bundle.tgz

# Alias
amadiag interactive
```

The TUI starts in the **file browser** unless you pass a path. Analysis runs on a worker thread so the UI stays responsive while a bundle is being processed.

#### File browser

- `Enter` selects the highlighted file or folder
- `Backspace` goes to the parent directory
- `Up` / `Down` or `j` / `k` moves through the current directory
- `t` switches from the browser to manual path entry
- `h` shows or hides dotfiles
- `Esc` or `q` quits

#### Manual path entry

- type or paste a `.zip`, `.tgz`, `.tar.gz`, or extracted folder path
- `Enter` starts analysis
- `Ctrl+T` switches back to the file browser
- `Esc` quits

#### Dashboard and export

- `Tab`, `Left`, `Right`, or `Shift+Tab` cycles focus through **Navigator → Findings → Details → Evidence**
- `Up` / `Down` moves inside the focused list pane
- `Page Up` / `Page Down` scrolls the active detail or evidence pane
- `Home` / `End` jumps to the first or last item in the active list
- `1`, `2`, `3` set severity filters: **critical**, **critical + warning**, or **all**
- `t` cycles the time filter used by the dashboard timeline and grouped findings
- `m` opens Markdown export and `j` opens JSON export
- on the export screen, edit the output path, use `Tab` to switch format, and press `Enter` to write the file
- if the export target already exists, press `Enter` again to confirm overwrite
- `r` reruns the last analysis
- `n` or `Esc` returns to the file browser
- `q` quits

### Validate a bundle without full analysis

```bash
amadiag validate /path/to/bundle
```

Validation prints a quick summary that includes bundle format, file counts, total size, and XML/log/CSV counts.

### List built-in YAML rules

```bash
amadiag rules list
```

Use this command as the authoritative source for the current built-in rule catalog.

### Exit codes

| Code | Meaning |
|:----:|---------|
| `0`  | Analysis completed with no critical findings |
| `1`  | Analysis completed and at least one critical finding was detected |
| `2`  | Input or execution error |

---

## 🛡️ Detection coverage

AMADiag combines three layers of analysis:

- **Embedded YAML rules** in `src/rules/` for file presence, file absence, content matches, and XML checks
- **Programmatic analyzers** for Linux-specific scenarios such as Fluent Bit config health, rsyslog forwarding, and MDSD QoS upload behavior
- **Regex-based pattern scanning** for recurring log signatures that indicate auth, connectivity, provisioning, throttling, or resource-pressure failures

Current rule families include:

- Installation
- Connectivity
- DCR
- Identity
- Performance counters
- Linux collection
- Syslog / CEF
- Sizing

Severity levels are `critical`, `warning`, and `info`.

If you want the exact current built-in YAML rules, run `amadiag rules list`.

<details>
<summary><strong>🧩 Adding custom rules</strong></summary>

Rules are defined in YAML and embedded at compile time from `src/rules/`.

```yaml
- id: CUSTOM-001
  name: My Custom Detection
  severity: warning
  category: connectivity
  platforms: [windows, linux]
  description: >
    Description of what this rule detects.
  detection:
    file_pattern: ".log"
    condition: content_match
    content_regex: "(?i)my-error-pattern"
  remediation: >
    Steps to fix the issue.
  doc_link: https://learn.microsoft.com/en-us/...
```

Supported condition values are:

- `file_missing`
- `file_present`
- `content_match`
- `xml_element_missing`

After adding or updating rules under `src/rules/`, rebuild the project to embed the changes.

</details>

<details>
<summary><strong>📁 Project structure</strong></summary>

```
src/
├── main.rs                 # CLI entry point (clap)
├── lib.rs                  # Library root
├── detect.rs               # Orchestrator: extract -> parse -> analyze -> report
├── input.rs                # Bundle extraction and validation
├── parsers/
│   ├── mod.rs              # File walker and platform detection
│   ├── common.rs           # Log line classification and pattern scanning
│   ├── xml_config.rs       # mcsconfig XML parser
│   ├── event_table.rs      # MetricsExtension ETW CSV parser
│   ├── fluentbit_config.rs # Fluent Bit config parser
│   ├── mdsd_qos.rs         # MDSD QoS parser
│   ├── windows.rs          # Windows-specific enrichment
│   └── linux.rs            # Linux-specific enrichment
├── analyzers/
│   ├── mod.rs              # Analysis orchestration
│   ├── finding.rs          # Diagnostic report data model
│   ├── linux_analysis.rs   # Linux programmatic analysis
│   ├── rules.rs            # YAML rules engine
│   └── sizing.rs           # Environment sizing checks
├── reporters/
│   ├── mod.rs              # OutputFormat enum
│   ├── markdown.rs         # Markdown report renderer
│   └── json.rs             # JSON report renderer
├── rules/                  # Embedded YAML rule definitions
└── tui/                    # Interactive terminal UI
```

</details>

---

## 🎯 Who is this for?

| Persona | Use case |
|---------|----------|
| **Azure Admin / IT Pro** | Identify AMA root causes without reading multiple troubleshooting guides |
| **SOC / Sentinel Analyst** | Investigate why Syslog or CEF data stopped flowing |
| **Microsoft Support Engineer** | Turn customer bundles into a structured diagnostic summary quickly |
| **DevOps / SRE** | Integrate AMA validation into scripts and CI/CD with JSON output |

---

## 📋 Requirements

| | |
|---|---|
| **Build** | Rust 1.80+ |
| **Runtime** | Single binary, no additional runtime dependencies |
| **Input** | AMA troubleshooter output as `.tgz`, `.tar.gz`, `.zip`, or directory |

---

## 🤝 Contributing

Contributions are welcome. A good first contribution is adding or refining a detection rule in `src/rules/`.

1. Fork the repo
2. Create a branch such as `git checkout -b feat/my-rule`
3. Update the relevant rule file or analyzer
4. Run `cargo test`
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
- [AMA Troubleshooter - Windows](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-windows)
- [AMA Troubleshooter - Linux](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/troubleshooter-ama-linux)
- [Troubleshoot CEF/Syslog via AMA](https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting)
- [Data Collection Rules](https://learn.microsoft.com/en-us/azure/azure-monitor/vm/data-collection)

---

<div align="center">

*Built with 🦀 Rust — fast, safe diagnostics for Azure Monitor Agent bundles.*

</div>
