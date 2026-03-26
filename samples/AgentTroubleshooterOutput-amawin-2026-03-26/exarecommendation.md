# Log Summary UX Recommendations for AMA Agent Troubleshooter Output

> Based on Exa research across UX design for log monitoring, observability dashboards, structured logging best practices, and log formatting guidelines (sources: Elastic Observability Labs, UXmatters, Sematext, OneUptime, Zebrium/ScienceLogic, IN-COM).

---

## Problem Statement

The current AMA Agent Troubleshooter output is a sequential wall of text (~1,482 lines) mixing raw command output, verbose collection steps, and diagnostic results. A support engineer must scroll to the very bottom to find the 34-line Diagnostic Summary table, then mentally correlate abbreviated check names with the 7 warning details buried further below. This format violates multiple core UX principles for log readability.

---

## Recommended Report Structure

### 1. Executive Summary at the Top (Inverted Pyramid)

**Why:** Research consistently shows that log dashboards and triage tools surfacing the "so what?" upfront reduce mean-time-to-understand by 60-80% (Elastic Observability Labs, OneUptime). Humans scan top-down; the most critical information should never require scrolling.

**Recommendation:** Generate a short summary block at the very top of the output:

```
╔══════════════════════════════════════════════════════════════╗
║  AMA AGENT DIAGNOSTIC SUMMARY — amawin (westus2)           ║
║  Run: 2026-03-26 13:48 UTC    Agent: 1.39.0.0 (v47.4.1)   ║
╠══════════════════════════════════════════════════════════════╣
║  PASSED: 27    WARNINGS: 7    FAILED: 0                    ║
╚══════════════════════════════════════════════════════════════╝

⚠ WARNINGS REQUIRING ATTENTION:
  1. IPv6 connectivity failed for MCS, ODS, IMDS, Token endpoints
  2. No RuntimeSettings file found — VM extension settings skipped
  3. AAD log collection PowerShell script error (exit code 1)
```

---

### 2. Severity-Based Visual Hierarchy with Symbols

**Why:** Color coding is the #1 recommended pattern for log severity differentiation (Sematext "8 Best Practices for Log Readability", IN-COM "Log Levels Explained"). In terminal/text output where ANSI colors may not render, Unicode symbols provide equivalent at-a-glance scanning.

**Recommendation:** Use a consistent symbol vocabulary throughout:

| Symbol | Meaning | When to Use |
|--------|---------|-------------|
| `✅ PASS` | Check succeeded | All healthy checks |
| `⚠️ WARNING` | Non-blocking issue | IPv6 failures, missing optional files |
| `❌ FAIL` | Blocking issue | Connectivity failures on required endpoints |
| `ℹ️ INFO` | Informational | Collection steps, skipped optional items |
| `⏭️ SKIP` | Intentionally skipped | Admin check skipped, optional collection skipped |

For HTML output (already in the `Html/` folder), use CSS classes:
- `.severity-pass { color: #22c55e; font-weight: bold; }`
- `.severity-warn { color: #f59e0b; font-weight: bold; }`
- `.severity-fail { color: #ef4444; font-weight: bold; background: #fef2f2; }`

---

### 3. Grouped Sections with Collapsible Detail

**Why:** "Logs are one of the richest signals in observability — but also one of the messiest" (Elastic Streams UX). Grouping related checks into logical categories reduces cognitive load. The current output interleaves collection steps with diagnostic checks with no visual separation.

**Recommendation:** Organize output into logical sections:

```
═══ AGENT HEALTH ═══════════════════════════════════════════
  ✅ MonAgentHost (PID 7120) — Running, v47.4.1.0
  ✅ MonAgentLauncher (PID 5740) — Running, v47.4.1.0
  ✅ MonAgentManager (PID 6032) — Running, v47.4.1.0
  ✅ MonAgentCore (PID 704) — Running, v47.4.1.0

═══ CONNECTIVITY ════════════════════════════════════════════
  ✅ IMDS (169.254.169.254) — IPv4: PASS | ⚠️ IPv6: FAIL
  ✅ MCS (westus2-5w48...monitor.azure.com) — IPv4: PASS | ⚠️ IPv6: FAIL
  ✅ ODS (d1521c3b...opinsights.azure.com) — IPv4: PASS | ⚠️ IPv6: FAIL
  ✅ Token Endpoint — IPv4: PASS | ⚠️ IPv6: FAIL
  ℹ️ GigLa — No hosts configured (skipped)

═══ CONFIGURATION ═══════════════════════════════════════════
  ✅ MCS Config — Present and parsed
  ⚠️ RuntimeSettings — No settings file found
  ⚠️ VM Extension Settings — Skipped (no settings file)
  ✅ AMPLS — Not configured (direct connectivity)

═══ DATA COLLECTION ═════════════════════════════════════════
  ✅ 11 tables collected successfully
  ✅ MDM Data Collection — Completed (4m 3s)
  ⚠️ AAD Logs — Collection failed (no events / script error)
  ✅ dsregcmd — Executed successfully
```

---

### 4. Actionable Warning/Error Details with Remediation

**Why:** "If a log message only makes sense to the person who wrote the code, it is not a useful log" (Buka.sh — Designing Logs That Actually Help). The current warnings like `curl.exe failed to connect to the endpoint` tell the engineer *what happened* but not *what to do about it*.

**Recommendation:** Each warning/error should include:
1. **What** — the symptom
2. **Why it matters** — impact on monitoring
3. **What to do** — remediation step or link

```
⚠️ WARNING: IPv6 connectivity failed across all endpoints (MCS, ODS, IMDS, Token)
   Impact:  None if IPv4 is working. Agent uses IPv4 by default.
   Action:  No action required unless IPv6-only networking is intended.
            If IPv6 is required, verify NSG rules allow outbound 443 for IPv6.
   Ref:     https://learn.microsoft.com/azure/azure-monitor/agents/troubleshoot

⚠️ WARNING: No RuntimeSettings file found
   Impact:  VM extension settings could not be parsed for validation.
   Action:  Verify the extension was enabled via Azure Portal or CLI.
            Check: az vm extension list --resource-group <rg> --vm-name <vm>

⚠️ WARNING: AAD log collection failed (exit code 1)
   Impact:  Cannot verify Entra ID authentication state from event logs.
   Action:  Non-critical on non-AAD-joined VMs. This VM shows AzureAdJoined: NO.
            If AAD join is expected, investigate dsregcmd output.
```

---

### 5. Structured Data for Machine Consumption

**Why:** "Plain text logs are human-readable but machine-useless. Structured logs are queryable, filterable, and aggregatable" (Buka.sh). The current `.txt` output cannot be programmatically processed or integrated into dashboards.

**Recommendation:** In addition to the human-readable text, emit a companion JSON summary:

```json
{
  "diagnosticRun": {
    "timestamp": "2026-03-26T13:48:38Z",
    "vmName": "amawin",
    "region": "westus2",
    "agentVersion": "1.39.0.0",
    "troubleshooterVersion": "47.4.1.0"
  },
  "results": {
    "totalPassed": 27,
    "totalWarnings": 7,
    "totalFailed": 0
  },
  "checks": [
    {
      "category": "connectivity",
      "name": "MCS IPv4",
      "result": "PASS",
      "host": "westus2-5w48.westus2-1.handler.control.monitor.azure.com",
      "latencyMs": 112
    },
    {
      "category": "connectivity",
      "name": "MCS IPv6",
      "result": "WARNING",
      "host": "westus2-5w48.westus2-1.handler.control.monitor.azure.com",
      "detail": "curl.exe failed to connect to the endpoint",
      "remediation": "No action needed if IPv4 connectivity is healthy"
    }
  ]
}
```

---

### 6. Suppress Noise, Surface Signal

**Why:** "Without proper visualization, this data stream becomes noise" (OneUptime). "The goal is not to replace log search — dashboards give you the high-level view that tells you where to look" (OneUptime). The current output dedicates ~700 lines to raw command output, countdown timers (`189188187...`), and repeated environment variable dumps across 4 processes.

**Recommendations:**

| Current Noise | Recommended Change |
|---------------|-------------------|
| Full env vars dumped 4x (one per process) | Show once in a collapsible "Environment" section; diff only unique vars across processes |
| Countdown timer `189188187186...` | Replace with single line: `Waiting 190s for metric collection...` then `Done (190s)` |
| Full curl verbose output for every test | Show only in verbose/debug mode; default shows `PASS`/`FAIL` + latency |
| File collection "Successfully collected X.tsf" x11 | Summarize: `✅ 11/11 tables collected` |
| Repeated `Enabling activity X to satisfy dependency of Y` | Move to debug log; omit from user-facing output |

---

### 7. HTML Report Enhancements

**Why:** The troubleshooter already generates HTML files in the `Html/` folder. This is the ideal format for rich visual presentation with color, collapsible sections, and interactive elements.

**Recommendations for the HTML report:**

1. **Sticky header** with pass/warn/fail counts always visible
2. **Accordion sections** — collapsed by default, click to expand raw details
3. **Color-coded rows** — green/amber/red backgrounds in the summary table
4. **Copy-to-clipboard buttons** on key values (subscription ID, resource ID, endpoint URLs)
5. **Search/filter box** — let users type to filter checks by name or result
6. **Timestamp tooltips** — hover to see exact time each check ran
7. **Deep links** — anchor IDs on each section so URLs like `TroubleshooterLogs.html#connectivity` work

---

### 8. Progressive Disclosure Pattern

**Why:** "Automated error triage shifts the focus from 'what happened?' to 'is this fix correct?'" (Elastic Observability Labs). Different audiences need different detail levels.

**Recommendation:** Three tiers of output:

| Tier | Audience | Content | Format |
|------|----------|---------|--------|
| **Summary** | Support engineer triage | Pass/Warn/Fail counts + top issues | 10-20 lines, top of output |
| **Diagnostic** | L2 troubleshooting | Grouped checks with remediation | ~100 lines, categorized |
| **Verbose** | Engineering escalation | Full raw command output, env vars, curl traces | Current ~1,500 lines (appendix or separate file) |

---

## Implementation Priority

| Priority | Recommendation | Effort | Impact |
|----------|---------------|--------|--------|
| **P0** | Executive summary at top of output | Low | High — immediate time savings |
| **P0** | Severity symbols (✅/⚠️/❌) in diagnostic summary | Low | High — instant visual scanning |
| **P1** | Grouped sections with category headers | Medium | High — reduces cognitive load |
| **P1** | Actionable remediation text per warning/error | Medium | High — reduces escalations |
| **P2** | Structured JSON companion output | Medium | Medium — enables automation |
| **P2** | Noise suppression (env vars, countdown, verbose curl) | Medium | Medium — reduces scroll fatigue |
| **P3** | Enhanced HTML with accordion/color/filter | High | Medium — best visual experience |
| **P3** | Three-tier progressive disclosure | High | Medium — serves all audiences |

---

## Sources

1. **Elastic Observability Labs** — "Log Processing UX Design in Elastic Streams" (2026-03-03) — Design decisions for making log data accessible, consistent, and actionable
2. **OneUptime** — "How to Create Log Dashboards" (2026-01-30) — Log dashboards transform noise into actionable insights; pattern recognition at scale
3. **Buka.sh** — "Designing Logs That Actually Help When Production Is on Fire" (2026-02-22) — Structured logging, operational context, and actionable messages
4. **Sematext** — "Log Formatting: 8 Best Practices for Better Readability" (2025-01-24) — Formatting standards for human and machine readability
5. **IN-COM** — "Log Levels Explained: Hierarchy, Severity Mapping, and Operational Risk" (2026-02-18) — Severity level design as architectural control signals
6. **OneUptime** — "How to Create Log Analysis Patterns" (2026-01-30) — Pattern matching to transform raw text into structured, queryable data
7. **Elastic Observability Labs** — "Automated Error Triage: From Reactive to Autonomous" (2026-03-18) — Closing the gap between error occurrence and root cause understanding
8. **UXmatters** — "Crafting Seamless User Experiences: A UX-Driven Approach to Log Monitoring and Observability" (2024-05-20) — UX-first approach to log monitoring design
