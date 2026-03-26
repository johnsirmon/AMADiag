# How to Review AMA Windows Troubleshooter Output — Detailed Guide

> **Audience**: CSS support engineers, Azure Monitor SMEs, and anyone analyzing `AgentTroubleshooter.exe --ama` output for AMA Windows.
>
> **Sources**: [AMA Windows Troubleshooter (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/troubleshooter-ama-windows), [AMA Windows VM Troubleshooting (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm), and internal CSS Supportability wiki (AzureMonitor.wiki).
>
> **Last updated**: 2026-03-26

---

## Table of Contents

- [1. Overview: What the Troubleshooter Produces](#1-overview-what-the-troubleshooter-produces)
- [2. Start with the Diagnostic Summary](#2-start-with-the-diagnostic-summary)
- [3. Review Each Output File in Detail](#3-review-each-output-file-in-detail)
  - [3.1 TroubleshooterLogs.txt — Master Log](#31-troubleshooterlogstxt--master-log)
  - [3.2 systeminfo.output.txt — OS and VM Basics](#32-systeminfooutputtxt--os-and-vm-basics)
  - [3.3 agentprocesses.txt — Agent Process Tree](#33-agentprocessestxt--agent-process-tree)
  - [3.4 allprocesses.txt — Full Process List](#34-allprocessestxt--full-process-list)
  - [3.5 ImdsMetadataResponse.json — IMDS Health](#35-imdsmetadataresponsejson--imds-health)
  - [3.6 curl.output.txt — Endpoint Connectivity](#36-curloutputtxt--endpoint-connectivity)
  - [3.7 nslookup.output.txt — DNS Resolution](#37-nslookupoutputtxt--dns-resolution)
  - [3.8 ipconfig.output.txt — Network Configuration](#38-ipconfigoutputtxt--network-configuration)
  - [3.9 NetworkDiagnostics.txt — TLS and Named Pipes](#39-networkdiagnosticstxt--tls-and-named-pipes)
  - [3.10 dsregcmd.output.txt — AAD / Entra ID Join Status](#310-dsregcmdoutputtxt--aad--entra-id-join-status)
  - [3.11 environment — Agent Environment Variables](#311-environment--agent-environment-variables)
  - [3.12 AADLogs.csv — AAD Diagnostic Logs](#312-aadlogscsv--aad-diagnostic-logs)
  - [3.13 powershell.output.txt — MDM Data Collection Script](#313-powershelloutputtxt--mdm-data-collection-script)
- [4. AgentDataStore Analysis](#4-agentdatastore-analysis)
  - [4.1 Mcs/mcsconfig.latest.json — DCR Configuration](#41-mcsmcsconfiglatestjson--dcr-configuration)
  - [4.2 Mcs/mcsconfig.latest.xml — Compiled Agent Instructions](#42-mcsmcsconfiglatestxml--compiled-agent-instructions)
  - [4.3 Mcs/configchunks/*.json — Downloaded DCR Chunks](#43-mcsconfigchunksjson--downloaded-dcr-chunks)
  - [4.4 Mcs/monagentcore.hr.json — Core Health Report](#44-mcsmonagentcorehrjson--core-health-report)
  - [4.5 Mcs/monagentmanager.hr.json — Manager Health Report](#45-mcsmonagentmanagerhrjson--manager-health-report)
  - [4.6 Mcs/agentsettings.txt — Agent Settings](#46-mcsagentsettingstxt--agent-settings)
  - [4.7 Configuration/AgentIdentity.manager.cache.txt — Identity Cache](#47-configurationagentidentitymanagercachetxt--identity-cache)
- [5. Agent Tables — Data Pipeline Health](#5-agent-tables--data-pipeline-health)
  - [5.1 MAEventTable.csv — Lifecycle Events and Errors](#51-maeventtablecsv--lifecycle-events-and-errors)
  - [5.2 MaQosEvent.csv — Upload Success/Failure](#52-maqoseventcsv--upload-successfailure)
  - [5.3 LogAnalyticsHeartbeats.csv — Heartbeat Cache](#53-loganalyticsheartbeatscsv--heartbeat-cache)
  - [5.4 MaHeartBeats.csv — Internal Agent Heartbeats](#54-maheartbeatscsv--internal-agent-heartbeats)
  - [5.5 MaCounterEvent.csv — Performance Counter Data](#55-macountereventcsv--performance-counter-data)
  - [5.6 MaMetricsExtensionEtw.csv — Metrics Extension Logs](#56-mametricsextensionetwcsv--metrics-extension-logs)
  - [5.7 MaTelemetryEvents.csv — Agent Telemetry](#57-matelemetryeventscsv--agent-telemetry)
  - [5.8 Customer Data Tables (c*.csv)](#58-customer-data-tables-ccsv)
- [6. AggregateStatus — Extension Health Snapshots](#6-aggregatestatus--extension-health-snapshots)
- [7. VmExtLogs — Extension Install/Enable Logs](#7-vmextlogs--extension-installenable-logs)
- [8. Connectivity Check Deep Dive](#8-connectivity-check-deep-dive)
  - [8.1 IMDS Connectivity](#81-imds-connectivity)
  - [8.2 MCS (AMCS) Connectivity](#82-mcs-amcs-connectivity)
  - [8.3 ODS Connectivity](#83-ods-connectivity)
  - [8.4 Token Endpoint Connectivity](#84-token-endpoint-connectivity)
  - [8.5 GigLa Connectivity](#85-gigla-connectivity)
  - [8.6 AMPLS (Private Link) Check](#86-ampls-private-link-check)
- [9. Decision Tree: Mapping Symptoms to Files](#9-decision-tree-mapping-symptoms-to-files)
- [10. Common Patterns and What They Mean](#10-common-patterns-and-what-they-mean)
- [11. Quick Reference: Checks to Run First](#11-quick-reference-checks-to-run-first)

---

## 1. Overview: What the Troubleshooter Produces

The AMA Windows Troubleshooter (`AgentTroubleshooter.exe --ama`) is a C# command-line tool shipped with AMA Windows (versions ≥ 1.12.0.0). It collects diagnostic data and runs automated connectivity tests, producing a zip file containing all the artifacts described below.

**How to run it** (from an admin command prompt on the VM):

```cmd
cd C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\{version}\Troubleshooter\
AgentTroubleshooter.exe --ama
```

Or via PowerShell:

```powershell
$currentVersion = ((Get-ChildItem -Path "Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows Azure\HandlerState\" `
    | where Name -like "*AzureMonitorWindowsAgent*" `
    | ForEach-Object {$_ | Get-ItemProperty} `
    | where InstallState -eq "Enabled").PSChildName -split('_'))[1]

$troubleshooterPath = "C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\$currentVersion\Troubleshooter"
Set-Location -Path $troubleshooterPath
Start-Process -FilePath $troubleshooterPath\AgentTroubleshooter.exe -ArgumentList "--ama"
```

The output takes up to **15 minutes** to complete and produces a `troubleshooterOutput.zip` containing:

| Category | Files |
|----------|-------|
| **Master log** | `TroubleshooterLogs.txt` |
| **System info** | `systeminfo.output.txt`, `ipconfig.output.txt`, `environment` |
| **Processes** | `agentprocesses.txt`, `allprocesses.txt` |
| **Network** | `curl.output.txt`, `nslookup.output.txt`, `NetworkDiagnostics.txt` |
| **Identity** | `dsregcmd.output.txt`, `AADLogs.csv`, `ImdsMetadataResponse.json` |
| **Agent data store** | `AgentDataStore/Mcs/*`, `AgentDataStore/Configuration/*`, `AgentDataStore/Tables/*` |
| **Extension logs** | `VmExtLogs/.../*.log` |
| **Aggregate status** | `AggregateStatus/aggregatestatus*.json` |
| **MDM/Metrics** | `powershell.output.txt`, `table2csv.output.txt` |
| **Misc** | `TroubleshooterId.txt`, `tablestatgenerationverboselogs.txt` |

> **Important**: The Troubleshooter cannot be copied from a newer agent version to an older one. The agent version must match.

> **Known bug (v1.27.0.0)**: Some files (`AgentDataStore\Configuration\*`, `nslookup.output.txt`) may not be collected automatically and must be gathered manually.

---

## 2. Start with the Diagnostic Summary

**Always start your review at the bottom of `TroubleshooterLogs.txt`.** The troubleshooter prints a `Diagnostic Summary` table at the end showing PASS/WARNING/FAIL for each check.

```
Diagnostic Summary
Check                                                                    Result
------------------------------------------------------------------------ -------
Connection to westus2-5w48.westus2-1.handler.control.monitor.azure.co... PASS
Connection to westus2-5w48.westus2-1.handler.control.monitor.azure.co... WARNING
...
Found 7 warnings
Found 0 fails
```

**Interpretation**:

| Result | Meaning |
|--------|---------|
| **PASS** | Check succeeded — no action needed |
| **WARNING** | Non-critical issue found — review the detail line below the summary for context |
| **FAIL** | Critical issue — this is likely blocking agent functionality and must be investigated |

After the summary, the troubleshooter prints a **detailed explanation** for each WARNING and FAIL. Example:

```
[MCS Connectivity Test] Connection to westus2-5w48...azure.com [IPv6]: curl.exe failed to connect to the endpoint
[Parse VM Extension Settings] Settings File Found: No settings file was found in the expected format.
[Collect Active Directory Status] Collect Active Directory Status: Failed to execute log collection PowerShell script: 1
```

**Action**: Count FAILs. If there are 0 FAILs, the agent is fundamentally operational. Focus on WARNINGs that relate to the customer's reported problem.

---

## 3. Review Each Output File in Detail

### 3.1 TroubleshooterLogs.txt — Master Log

This is the **single most important file**. It contains the full output of every diagnostic activity the troubleshooter runs, in order of execution. Every other file is a subset of what's collected here.

**What to look for**:

| Section | Key signals |
|---------|-------------|
| Activity list at top | Confirms which checks were enabled |
| `Agent Process Details` | All running agent processes with PIDs, image versions, and command lines |
| `Agent Environment Vars` | Every environment variable for each agent process |
| `All Process Details` | Full process list — look for resource contention, competing agents |
| `Collect Agent Tables` | Table conversion results — record counts, time ranges |
| `Connectivity Test` sections | IMDS, MCS, ODS, Token Endpoint, GigLa results |
| `AMPLS` check | Whether Private Link is enabled |
| `Diagnostic Summary` | Final PASS/WARNING/FAIL summary |

**Tip**: Search for the string `FAIL` (case-sensitive) to quickly jump to failures.

### 3.2 systeminfo.output.txt — OS and VM Basics

Review this file to establish the baseline:

| Field | What to verify |
|-------|----------------|
| **OS Name / Version** | Must be on the [AMA supported OS list](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-supported-operating-systems). Server 2012 R2 requires special handling. |
| **System Boot Time** | If the VM was recently rebooted, that may explain transient data gaps |
| **Processor(s)** | Single-processor VMs under heavy load can starve the agent |
| **Total / Available Physical Memory** | If available memory is very low, the agent may OOM or be killed |
| **Hotfix(s) Installed** | Missing patches can cause TLS issues on older OS versions |
| **Time Zone** | UTC is expected for Azure VMs — a non-UTC timezone can cause timestamp drift in queries |
| **Domain** | WORKGROUP = standalone; if domain-joined, verify it doesn't block IMDS/AMCS |

### 3.3 agentprocesses.txt — Agent Process Tree

This file is **critical** for confirming agent health. AMA Windows requires these processes:

| Process | Must be running? | Role |
|---------|:-:|------|
| `MonAgentLauncher` | Yes | Initial launcher — parent of the process tree |
| `MonAgentHost` | Yes | Core data collection host |
| `MonAgentManager` | Yes | Process lifecycle manager |
| `MonAgentCore` | Conditional | Data processing core — **starts only after configuration is acquired** |
| `AMAExtHealthMonitor` | Yes | Extension health reporter (visible in `allprocesses.txt`) |

**What to check**:

1. **Are all 4 agent processes present?**
   - If `MonAgentCore` is missing but the other 3 are running, the agent has not acquired its DCR configuration. Follow the [Configuration TSG](#41-mcsmcsconfiglatestjson--dcr-configuration).
   - If `MonAgentLauncher` or `MonAgentHost` is missing, the agent failed to start. Check `VmExtLogs` for install/enable errors.

2. **Image version**: Confirm all processes have the same version (e.g., `47.4.1.0`). Mismatched versions indicate a failed upgrade.

3. **Command line arguments**: Verify `-mcsmode` is present (required for MCS-based configuration). Verify `-ConfigFile` points to `mcsconfig.latest.xml`.

4. **Environment variables per process**: Check for:
   - `MONITORING_MCS_MODE=1` — Confirms MCS mode
   - `MONITORING_IDENTITY=use_ip_address` — System-assigned managed identity via IMDS
   - `MA_RoleEnvironment_ResourceId` — Must match the expected VM resource ID
   - `MA_RoleEnvironment_Location` — Must match the expected Azure region
   - `MONITORING_TOKENBASED_ENV=true` — Token-based ingestion enabled (visible on `MonAgentCore`)

### 3.4 allprocesses.txt — Full Process List

Same as `agentprocesses.txt` but includes **all processes** on the VM. Use this to:

- Confirm `AMAExtHealthMonitor` is running (PID/process name in the list)
- Check for `MetricsExtension.Native.exe` if custom metrics are expected
- Look for competing monitoring agents (e.g., `HealthService.exe` for legacy MMA, `omsagent`)
- Check for resource hogs — sort by CPU or WorkingSet to find processes stealing resources from AMA
- Confirm `WindowsAzureGuestAgent` (`WaAppAgent`) is running (required for extension management)

### 3.5 ImdsMetadataResponse.json — IMDS Health

This file contains the Azure Instance Metadata Service response. A valid response confirms:

1. **IMDS is reachable** from the VM
2. **VM identity** is correct (subscription, resource group, VM name, VM ID)
3. **Region** (`location`) matches what AMA expects
4. **OS type** is correctly reported

**What to check**:

| Field | Expected |
|-------|----------|
| `compute.location` | Must match the AMA regional endpoint (e.g., `westus2`) |
| `compute.resourceId` | Must match the `MA_RoleEnvironment_ResourceId` env var |
| `compute.vmId` | Must match `MA_RoleEnvironment_VmId` |
| `compute.osType` | Must be `Windows` |
| `network.interface[].ipv4.ipAddress[].privateIpAddress` | Must match `ipconfig` output |

**If this file is empty or contains an error**: The VM cannot reach IMDS at `169.254.169.254`. This blocks identity token acquisition and DCR download. Check:
- Firewall rules blocking IMDS wireserver IP
- Proxy configuration redirecting IMDS traffic
- Network Virtual Appliance (NVA) intercepting traffic

> **For Azure Arc machines**: Check `HimdsMetadataResponse.json` instead (HIMDS at `localhost:40342`).

### 3.6 curl.output.txt — Endpoint Connectivity

The troubleshooter tests connectivity to critical endpoints using `curl.exe`. Each endpoint is tested with **Auto**, **IPv4**, and **IPv6** modes.

**Review pattern**: Look at the HTTP response code for each test.

| Endpoint | Expected Response | What it validates |
|----------|:-:|---|
| `169.254.169.254` (IMDS) | `200 OK` | Azure Instance Metadata Service reachable |
| `<region>-*.handler.control.monitor.azure.com/ping` | `200 OK` with body `Healthy` | MCS/AMCS handler endpoint reachable |
| `<workspaceId>.ods.opinsights.azure.com` | `200` or `403` (without valid token) | ODS (Log Analytics) endpoint reachable |
| Token endpoint | `200` series | Token issuance endpoint reachable |

**Interpreting failures**:

| Failure | Likely cause |
|---------|-------------|
| `curl: (7) Could not connect to server` | Network-level block — firewall, NSG, or route issue |
| `curl: (6) Could not resolve host` | DNS issue — check `nslookup.output.txt` |
| `curl: (28) Connection timed out` | Firewall or NVA silently dropping packets |
| `curl: (35) SSL connect error` | TLS version or certificate issue — check `NetworkDiagnostics.txt` |
| IPv6 tests fail but IPv4 passes | Normal on most Azure VMs — IPv6 is not required for AMA |

**Key detail**: The MCS endpoint may include a specific shard ID (e.g., `westus2-5w48.westus2-1.handler.control.monitor.azure.com`). This shard is derived from the MCS config. A `Healthy` ping response confirms the AMCS service is reachable and operational.

### 3.7 nslookup.output.txt — DNS Resolution

Confirms DNS resolution for the same endpoints tested by curl.

**What to check**:

1. **Server address**: Should be `168.63.129.16` (Azure DNS) for Azure VMs. A different DNS server may indicate custom DNS that could filter or redirect queries.
2. **Resolution succeeds**: Each endpoint should resolve to a public IP (or private IP if AMPLS is in use).
3. **CNAME chain**: The aliases show traffic manager routing (e.g., `amcs-prod-wus2-handler.trafficmanager.net`). This is normal.

**If an endpoint fails to resolve**:
- Check if a custom DNS server is configured that doesn't forward to Azure DNS
- If AMPLS/Private Link is in use, verify private DNS zones are configured
- NSG may be blocking UDP 53 / TCP 53 outbound

### 3.8 ipconfig.output.txt — Network Configuration

Validate basic network plumbing:

| Check | Why |
|-------|-----|
| IPv4 address present and preferred | Agent needs a valid IP to reach IMDS and endpoints |
| DHCP enabled with Azure DHCP server (`168.63.129.16`) | Standard Azure VM DHCP |
| DNS server is `168.63.129.16` | Azure-provided DNS; custom DNS may cause resolution issues |
| Default gateway present | Required for outbound connectivity |
| Multiple NICs | If accelerated networking is enabled, a Mellanox adapter will be present (normal) |

### 3.9 NetworkDiagnostics.txt — TLS and Named Pipes

This file contains two important sections:

#### TLS Registry Values

The troubleshooter checks TLS-related registry keys. Important items:

| Registry Value | Expected | Problem if |
|-------|----------|----------|
| SSL 3.0 Client/Server Enabled | `0` (disabled) | If `1`, security risk but not an AMA blocker |
| TLS 1.2 keys | Should NOT be explicitly disabled | If TLS 1.2 is disabled, AMA cannot connect to Azure endpoints |
| `SystemDefaultTlsVersions` | Not present or `1` | If `0`, .NET may not use TLS 1.2 |
| `SchUseStrongCrypto` | Not present or `1` | If `0`, weak crypto may be selected |

**Critical issue**: If TLS 1.0/1.1 are the only enabled protocols (TLS 1.2 explicitly disabled), AMA will fail to connect to all Azure endpoints. Look for `HKLM\SYSTEM\CurrentControlSet\Control\SecurityProviders\SCHANNEL\Protocols\TLS 1.2\Client: Enabled = 0`.

#### Named Pipes

Look for: `\\.\\pipe\\CAgentStream_CloudAgentInfo_AzureMonitorAgent`

This pipe confirms the agent is running and listening for inter-process communication. If it's absent, the agent processes may not be fully initialized.

### 3.10 dsregcmd.output.txt — AAD / Entra ID Join Status

Shows the device's Azure AD (Entra ID) join state.

| Field | Normal for Azure VMs | Concern |
|-------|---------------------|---------|
| `AzureAdJoined: NO` | Normal for standalone WORKGROUP VMs | Only a concern if customer expects AAD-based features |
| `DomainJoined: NO` | Normal for non-domain VMs | Domain-joined VMs may have Group Policy blocking IMDS or endpoints |
| `WamDefaultSet: ERROR` | Normal for server OS without interactive logon | Not an AMA issue |

**When this matters**: If the VM is AAD-joined or domain-joined and managed identity token acquisition fails, dsregcmd output helps diagnose identity configuration issues.

### 3.11 environment — Agent Environment Variables

A minimal file containing key agent configuration values:

```
MONITORING_IDENTITY=use_ip_address
MONITORING_MCS_MODE=1
MONITORING_VERSION=2.0
```

| Variable | Expected | Concern |
|----------|----------|---------|
| `MONITORING_IDENTITY` | `use_ip_address` (system-assigned MI) or specific GUID (user-assigned MI) | If missing, identity not configured |
| `MONITORING_MCS_MODE` | `1` | If `0`, agent is not in MCS mode |
| `MONITORING_VERSION` | `2.0` | Older versions may lack features |

### 3.12 AADLogs.csv — AAD Diagnostic Logs

Contains Azure AD authentication logs. **Often empty** on standalone VMs, which is normal.

**When to investigate**: If the AAD logs contain error entries related to token acquisition, managed identity failures, or 401/403 responses — these indicate authentication problems preventing DCR download.

### 3.13 powershell.output.txt — MDM Data Collection Script

This file captures output from the MetricsExtension (MDM) data collection script. It collects metrics-related diagnostics.

**Common warnings to look for**:

| Warning | Severity | Meaning |
|---------|----------|---------|
| `DNS name does not exist` for `*.nsatc.net` | Medium | MetricsExtension cannot reach its frontend endpoints — affects custom metrics/guest metrics collection |
| `TCP connect to (...) failed` | Medium | Network-level block to metrics endpoints |
| `No autopilot logs were found` | Informational | Normal unless on an Autopilot-managed machine |
| `SkipCollectionCertificates` / `SkipCollectionTsfLogs` | Informational | These are intentional skips by the troubleshooter |

**If customer is missing guest metrics/custom metrics**: These DNS and TCP failures are the primary cause.

---

## 4. AgentDataStore Analysis

The `AgentDataStore/` directory contains the agent's runtime data, configuration, and local caches.

### 4.1 Mcs/mcsconfig.latest.json — DCR Configuration

This JSON file lists the DCR configurations downloaded from AMCS.

**What to verify**:

```json
{
  "configurations": [{
    "configurationId": "dcr-7291dc57ac3c4d45aa35942270c31f4a",
    "eTag": "...",
    "tokenBasedChannels": [{
      "globallyUniqueChannelId": "dcr-...",
      "channelId": "ods-<workspaceId>",
      "tokenEndpointUri": "https://..."
    }],
    "contentFileName": "7836023744250932452.json"
  }]
}
```

| Check | Why |
|-------|-----|
| `configurations` array is not empty | At least one DCR is associated |
| `configurationId` matches the expected DCR | Correct DCR is assigned to this VM |
| `tokenBasedChannels` exist | Ingestion channels are configured |
| `channelId` contains the expected workspace ID | Data goes to the right workspace |
| `contentFileName` matches a file in `configchunks/` | DCR content was downloaded |

**If this file is missing or empty**: The agent cannot download DCR configuration from AMCS. Check:
1. Is a DCR associated with this VM in the portal?
2. Is managed identity enabled?
3. Can the agent reach AMCS endpoints? (check `curl.output.txt`)

### 4.2 Mcs/mcsconfig.latest.xml — Compiled Agent Instructions

This is the **compiled instruction set** that the agent actually executes. It is generated from the downloaded DCR chunks and contains:

- XPath queries for Windows Event Log subscriptions
- Performance counter definitions
- Heartbeat (HEALTH_ASSESSMENT_BLOB) configuration
- Destination workspace IDs and ingestion endpoints
- IIS log configuration (if applicable)

**What to check**:

1. **Event log subscriptions**: Look for `<Subscription>` nodes with the expected XPath queries
2. **Performance counters**: Look for `<CounterSet>` nodes with the expected counters
3. **Destinations**: Look for ODS workspace IDs matching the expected Log Analytics workspace
4. **Heartbeat config**: Look for `HEALTH_ASSESSMENT_BLOB` to confirm heartbeat pipeline is configured

### 4.3 Mcs/configchunks/*.json — Downloaded DCR Chunks

Each file corresponds to one DCR's compiled content. Per [Microsoft Learn](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm):

> Check if you see the latest DCR downloaded at this location `C:\WindowsAzure\Resources\AMADataStore.<vm-name>\mcs\configchunks`

- **File present**: DCR was successfully downloaded
- **File missing**: DCR download failed — check AMCS connectivity
- **File stale** (old timestamp): Agent cannot refresh — check token and connectivity

### 4.4 Mcs/monagentcore.hr.json — Core Health Report

```json
{
  "OSVer": "Microsoft Windows Server 2019 Datacenter",
  "AgentVer": "1.39.0.0",
  "Context": "monagentcore",
  "Id": "QT8I",
  "Connectivity": {
    "Ods": {
      "d1521c3b-0bd3-4d94-a6ed-22c3170a6b74.ods.opinsights.azure.com": true
    }
  }
}
```

**Key checks**:
- `AgentVer`: Confirm version matches the installed extension version
- `Connectivity.Ods.<endpoint>`: `true` = ODS reachable, `false` = ODS unreachable (data upload will fail)

### 4.5 Mcs/monagentmanager.hr.json — Manager Health Report

```json
{
  "OSVer": "Microsoft Windows Server 2019 Datacenter",
  "AgentVer": "1.39.0.0",
  "Context": "monagentmanager",
  "Id": "HP5B",
  "Connectivity": {
    "Amcs": {
      "https://westus2-5w48.westus2-1.handler.control.monitor.azure.com": true
    }
  },
  "Features": {
    "ConfigCheck": true
  }
}
```

**Key checks**:
- `Connectivity.Amcs.<endpoint>`: `true` = Can reach AMCS to download configuration
- `Features.ConfigCheck`: `true` = Configuration validation passed

**If either `.hr.json` file shows `false` for connectivity**: This is a definitive signal of a network issue between the agent and Azure endpoints. Cross-reference with `curl.output.txt` and `nslookup.output.txt`.

### 4.6 Mcs/agentsettings.txt — Agent Settings

Contains agent runtime settings. Typically:

```
BlobPathFormat=0
```

Small file — usually not actionable unless PG asks for it.

### 4.7 Configuration/AgentIdentity.manager.cache.txt — Identity Cache

```
Tenant=<subscriptionId>/Role=<resourceGroup>/RoleInstance=<vmId>
```

Confirms the agent's resolved identity triple. Verify:
- Tenant = subscription ID
- Role = resource group name
- RoleInstance = VM ID (matches IMDS `vmId`)

**If this file is missing**: The agent has not successfully authenticated. Check managed identity and IMDS connectivity.

---

## 5. Agent Tables — Data Pipeline Health

The `AgentDataStore/Tables/` directory contains TSF tables converted to CSV. These tables are the **local data pipeline** — data flows from collection → local tables → upload to Azure.

### 5.1 MAEventTable.csv — Lifecycle Events and Errors

**This is the most important diagnostic table.** It records agent lifecycle events, event log subscriptions, errors, and status messages.

**What to look for**:

| Pattern | Significance |
|---------|-------------|
| `SystemEventsListener::SubscribeEvents` | Agent subscribed to an event log channel — healthy |
| `SystemEventsListener::ResetSubscription` | Agent resubscribed — can indicate transient issues |
| `Subscribed to event eventName=...` | Successful subscription with XPath query |
| `Successfully resubscribed to Subscription` | Recovery after a subscription drop |
| `Ending scheduled task` | Agent completed a scheduled data collection task |
| Level 1 / Level 2 / Level 3 events | **Errors** — investigate these |
| PDH-related events | Performance counter collection errors |
| `IMDS` errors | Cannot reach Instance Metadata Service |
| `Could not open directory` | IIS log path issues |

**Interpreting record counts**: A healthy agent should have a steadily growing MAEventTable. If the table has 0 records or very few records, the agent is not collecting data.

### 5.2 MaQosEvent.csv — Upload Success/Failure

Each row represents an **upload attempt** to Azure. This is where you confirm data is actually leaving the VM.

| Column | What to check |
|--------|---------------|
| `Success` | `TRUE` = uploaded, `FALSE` = failed |
| `InputType` | Data type: `HEALTH_STATUS_BLOB`, `GENERIC_EVENT_BLOB`, `GENERIC_PERF_BLOB`, `ONPREM_IIS_BLOB_V2` |
| `Endpoint` | Where data was sent |
| HTTP error codes | `403` = auth failure, `429` = throttled, `500` = server error |

**Decision logic**:
- If `Success=TRUE` for `HEALTH_STATUS_BLOB` → heartbeats are uploading
- If `Success=TRUE` for `GENERIC_EVENT_BLOB` → event logs are uploading
- If `Success=FALSE` with `403` → token expired or managed identity issue
- If `Success=FALSE` with connection errors → network issue (check curl/nslookup)

### 5.3 LogAnalyticsHeartbeats.csv — Heartbeat Cache

Contains **cached heartbeat records** waiting to be uploaded. The presence of records here confirms:
1. The agent is generating heartbeats
2. The heartbeat pipeline is configured

**What to check**:
- Records exist with recent timestamps → heartbeat generation is healthy
- No records → heartbeat pipeline not configured or agent not running
- Records are very old → upload is failing (check MaQosEvent.csv)

### 5.4 MaHeartBeats.csv — Internal Agent Heartbeats

Similar to LogAnalyticsHeartbeats but for internal agent health monitoring. Compare timestamps between the two to ensure they're in sync.

### 5.5 MaCounterEvent.csv — Performance Counter Data

Contains **cached performance counter data**. Check:
- Record count: Should be growing if perf counters are configured in the DCR
- Time range: Gap in timestamps indicates collection interruption
- Counter names: Verify expected counters are present

**If counters are missing**: Use `typeperf -qx` on the VM to verify the counter exists with the exact name specified in the DCR. Counter names are **case-sensitive** and **locale-dependent**.

### 5.6 MaMetricsExtensionEtw.csv — Metrics Extension Logs

ETW traces from MetricsExtension.Native.exe. Per [Microsoft Learn](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm):

> Open MaMetricsExtensionEtw.csv and look for any **Level 2 errors** and try to fix them.

Level 2 entries indicate errors in the metrics pipeline.

### 5.7 MaTelemetryEvents.csv — Agent Telemetry

Internal telemetry data. Multiple schemas (typically 10+). Useful for PG escalations but rarely needed for CSS troubleshooting.

### 5.8 Customer Data Tables (c*.csv)

Files named with long numeric IDs (e.g., `c7836023744250932452_546419284751770569.csv`) contain the actual collected data mapped to DCR streams.

**What to check**:
- Record count: Should match expectations based on the DCR collection frequency
- Time range: Recent timestamps confirm active collection
- No records: Collection pipeline is broken — check MAEventTable for subscription errors

---

## 6. AggregateStatus — Extension Health Snapshots

The `AggregateStatus/` directory contains periodic health snapshots (every ~15 seconds).

**Read `aggregatestatus.json`** (the latest snapshot):

| Field | What to check |
|-------|---------------|
| `guestAgentStatus.status` | Should be `Ready` |
| `guestAgentStatus.version` | Azure VM Guest Agent version |
| Extension handler status | Look for `AzureMonitorWindowsAgent` with `status: success` |
| Extension handler `formattedMessage` | May contain error messages if status is not `success` |

**If the guest agent status is NOT `Ready`**: The VM Guest Agent itself has problems. This must be fixed first (SAP: `Azure/Virtual Machine running Windows/VM Extensions not operating correctly`).

---

## 7. VmExtLogs — Extension Install/Enable Logs

The `VmExtLogs/Microsoft.Azure.Monitor.AzureMonitorWindowsAgent/` directory contains:

| Log | Purpose |
|-----|---------|
| `CommandExecution*.log` | Extension install/enable output — the first place to look for install failures |
| `Extension*.log` | Extension lifecycle logs — handshake with Guest Agent |
| `ExtensionHealth.*.log` | Health monitoring logs — ongoing health checks |
| `Events/` | Extension event logs |

**For install failures**: Start with `CommandExecution*.log`. Common error patterns:
- `Failed to install` — binary download or unpack failure
- `Timeout` — Guest Agent took too long
- `Access denied` — antivirus or policy blocking

**For ongoing health issues**: Check `ExtensionHealth.*.log` for repeated error entries.

---

## 8. Connectivity Check Deep Dive

The troubleshooter runs connectivity tests against all endpoints the agent needs. Each test is run in Auto (default), IPv4-only, and IPv6-only modes.

### 8.1 IMDS Connectivity

| Test | Expected |
|------|----------|
| `169.254.169.254` Auto | PASS |
| `169.254.169.254` IPv4 | PASS |
| `169.254.169.254` IPv6 | WARNING (expected — IMDS is IPv4 only) |

**If IMDS fails on Auto/IPv4**: Critical issue. The agent cannot:
- Acquire managed identity tokens
- Read VM metadata
- Download DCR configuration

Causes: Firewall blocking wireserver, proxy intercepting local traffic, route table override.

### 8.2 MCS (AMCS) Connectivity

Tests connectivity to the Azure Monitor Configuration Service handler endpoint.

**Endpoint format**: `<region>-<shard>.<region>-<instance>.handler.control.monitor.azure.com`

- Auto/IPv4 PASS + IPv6 FAIL = Normal (IPv6 not required)
- Auto FAIL = Agent cannot download DCR configuration
- Ping response body should be `Healthy`

### 8.3 ODS Connectivity

Tests connectivity to the ODS (Operational Data Store) endpoint for the Log Analytics workspace.

**Endpoint format**: `<workspaceId>.ods.opinsights.azure.com`

- PASS = Data can be uploaded to Log Analytics
- FAIL = Data uploads will fail — check NSG, firewall, AMPLS

### 8.4 Token Endpoint Connectivity

Tests the token issuance endpoint (same host as MCS typically).

- PASS = Tokens can be acquired for authenticated data upload
- FAIL = Agent cannot authenticate — check managed identity, IMDS

### 8.5 GigLa Connectivity

Tests connectivity to the GigLa (Gigamon Log Analytics) endpoint if applicable. This test may not produce results for all configurations.

### 8.6 AMPLS (Private Link) Check

The troubleshooter resolves DNS for each endpoint and determines if AMPLS is enabled.

```
AMPLS check complete for d1521c3b-....ods.opinsights.azure.com
    Host: ipv4-wus2-oi-ods-cses-b.westus2.cloudapp.azure.com
            40.78.253.213
    AMPLS Enabled: False
```

| AMPLS Enabled | Meaning |
|:---:|---------|
| `False` | Endpoints resolve to public IPs — standard configuration |
| `True` | Endpoints resolve to private IPs — customer has Azure Monitor Private Link Scope configured |

**If AMPLS is True**: Verify private DNS zones are properly configured, NSG rules allow traffic to private endpoints, and the DCE (Data Collection Endpoint) is correctly set for private link.

---

## 9. Decision Tree: Mapping Symptoms to Files

| Customer symptom | Primary files to check | Secondary files |
|-----------------|----------------------|-----------------|
| **No data at all** | `agentprocesses.txt` → `mcsconfig.latest.json` → `curl.output.txt` | `VmExtLogs/CommandExecution*.log` |
| **Agent not installed** | `VmExtLogs/CommandExecution*.log` → `AggregateStatus/aggregatestatus.json` | `systeminfo.output.txt` |
| **No heartbeat** | `LogAnalyticsHeartbeats.csv` → `MaQosEvent.csv` → `monagentcore.hr.json` | `curl.output.txt` (ODS test) |
| **Missing event logs** | `MAEventTable.csv` (subscription events) → `mcsconfig.latest.xml` (XPath) → `MaQosEvent.csv` | `configchunks/*.json` |
| **Missing perf counters** | `MaCounterEvent.csv` → `MAEventTable.csv` (PDH errors) → `mcsconfig.latest.xml` (CounterSet) | `MaQosEvent.csv` |
| **Missing custom metrics** | `powershell.output.txt` → `MaMetricsExtensionEtw.csv` → `nslookup.output.txt` (nsatc.net) | `curl.output.txt` |
| **Missing IIS logs** | `MAEventTable.csv` (IIS errors) → `mcsconfig.latest.xml` → `MaQosEvent.csv` | N/A |
| **Connectivity failure** | `curl.output.txt` → `nslookup.output.txt` → `NetworkDiagnostics.txt` | `monagentcore.hr.json`, `monagentmanager.hr.json` |
| **Identity / auth failure** | `ImdsMetadataResponse.json` → `AgentIdentity.manager.cache.txt` → `dsregcmd.output.txt` | `AADLogs.csv`, `VmExtLogs` |
| **Config not downloading** | `mcsconfig.latest.json` → `configchunks/` → `monagentmanager.hr.json` | `curl.output.txt` (AMCS test) |
| **Data delayed** | `MaQosEvent.csv` (upload timestamps) → `MAEventTable.csv` | `systeminfo.output.txt` (boot time) |

---

## 10. Common Patterns and What They Mean

### Pattern: IPv6 Failures Across All Endpoints
**Meaning**: Normal. Most Azure VMs do not have IPv6 configured. AMA uses IPv4. These are informational WARNINGs only.

### Pattern: `global.metrics.nsatc.net` DNS Failure
**Meaning**: The MetricsExtension cannot resolve its frontend endpoints. Affects **guest metrics** sent to Azure Monitor Metrics. Does NOT affect Log Analytics data. If customer uses custom metrics, they need to allowlist `*.nsatc.net` in their firewall/DNS.

### Pattern: `WamDefaultSet: ERROR` in dsregcmd
**Meaning**: Normal for standalone server OS VMs without interactive user logon. Not an AMA issue.

### Pattern: No RuntimeSettings File Found
**Meaning**: The extension is using MCS-based configuration delivery rather than VM extension settings files. This is the normal modern behavior. Only a concern if `mcsconfig.latest.json` is also empty.

### Pattern: MonAgentCore Not Running
**Meaning**: The agent has not yet acquired its DCR configuration. Either no DCR is associated, or configuration download failed. Check `mcsconfig.latest.json` and AMCS connectivity.

### Pattern: MaQosEvent Shows Repeated `Success=FALSE`
**Meaning**: Data is being collected locally but cannot be uploaded. Check the HTTP error code:
- `401/403` → Token expired or invalid — check managed identity and IMDS
- `429` → Throttling — high ingestion volume or workspace-level limits
- `500/503` → Service-side issue — check Azure status page
- Connection error → Network issue (check curl and nslookup)

### Pattern: MAEventTable Shows Level 2/3 PDH Errors
**Meaning**: Performance counter collection is failing. Common cause: counter name mismatch between DCR and what the OS reports. Test with:
```powershell
typeperf "\Processor(_Total)\% Processor Time" -sc 5
typeperf -qx > available_counters.txt
```

### Pattern: MCS Health Report Shows Connectivity `false`
**Meaning**: Agent definitively cannot reach the endpoint. This is authoritative — network trace is the next step.

### Pattern: Large MAEventTable (>300 MB) with High Record Count
**Meaning**: Normal for VMs with many event log subscriptions over extended periods. The table has built-in retention (365 days by default). Very large tables may slow troubleshooter collection.

---

## 11. Quick Reference: Checks to Run First

When you receive troubleshooter output, follow this sequence for fastest triage:

1. **Open `TroubleshooterLogs.txt`, scroll to bottom** → Read `Diagnostic Summary` → Count FAILs and WARNINGs
2. **Check agent processes** in `agentprocesses.txt` → Are all 4 processes running?
3. **Check health reports** → `monagentcore.hr.json` (ODS connectivity) and `monagentmanager.hr.json` (AMCS connectivity) — are they `true`?
4. **Check DCR config** → Does `mcsconfig.latest.json` have configurations? Do files exist under `configchunks/`?
5. **Check data flow** → Does `MaQosEvent.csv` show `Success=TRUE`? Does `LogAnalyticsHeartbeats.csv` have recent records?
6. **Check connectivity** → Any FAILs in curl.output.txt on Auto/IPv4?
7. **Check MAEventTable.csv** → Any Level 2/3 errors? Are subscriptions successful?

If all of steps 2-7 look healthy, the issue is likely:
- Downstream (ingestion pipeline, workspace-level issue, query-level issue)
- Transient (resolved by the time troubleshooter ran)
- In the DCR definition (wrong XPath, wrong counter names, wrong workspace)

---

## References

- [How to use the Windows OS Azure Monitor Agent Troubleshooter (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/troubleshooter-ama-windows)
- [Troubleshooting guidance for AMA on Windows VMs (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm)
- [Troubleshooting guidance for AMA on Windows Arc-enabled servers (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc)
- [Monitor DCR data collection (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/data-collection/data-collection-monitor)
- [AMA Supported operating systems (Microsoft Learn)](https://learn.microsoft.com/azure/azure-monitor/agents/azure-monitor-agent-supported-operating-systems)
- CSS Supportability ADO wiki: AzureMonitor.wiki — AMA Windows troubleshooting articles
