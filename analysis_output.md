    Finished `release` profile [optimized] target(s) in 0.64s
     Running `target\release\amadiag.exe analyze .\samples\AgentTroubleshooterOutput-amawin-2026-03-26 --format markdown`
[2m2026-03-26T16:22:46.340500Z[0m [32m INFO[0m [2mamadiag::detect[0m[2m:[0m Analyzing bundle: .\samples\AgentTroubleshooterOutput-amawin-2026-03-26
[2m2026-03-26T16:22:46.340621Z[0m [32m INFO[0m [2mamadiag::detect[0m[2m:[0m Parsing bundle contents...
[2m2026-03-26T16:22:47.103585Z[0m [32m INFO[0m [2mamadiag::detect[0m[2m:[0m Running analysis rules...
[2m2026-03-26T16:22:47.293235Z[0m [32m INFO[0m [2mamadiag::detect[0m[2m:[0m Analysis complete: 11 findings (6 critical)
# AMADiag Diagnostic Report

## Summary

- **Bundle**: `.\samples\AgentTroubleshooterOutput-amawin-2026-03-26`
- **Files analyzed**: 183
- **Total findings**: 11
- **Critical**: 6
- **Warning**: 5
- **Info**: 0

## Environment

- **OS**: Windows
- **Platform**: Windows
- **AMA Version**: 47.4.1.0
- **VM SKU**: Standard_D4s_v3
- **DCR Count**: 2
- **Hostname**: amawin

## Findings Overview

| # | Severity | Category | Rule | Name |
|---|----------|----------|------|------|
| 1 | ≡ƒö┤ Critical | Installation | INSTALL-002 | Extension Provisioning Error Log |
| 2 | ≡ƒö┤ Critical | Identity | IDENTITY-001 | Missing Managed Identity |
| 3 | ≡ƒö┤ Critical | Connectivity | IMDS-001 | IMDS Connectivity Failure |
| 4 | ≡ƒö┤ Critical | Connectivity | CONN-001 | Network Connectivity Failure |
| 5 | ≡ƒö┤ Critical | Agent Not Running | AGENT-001 | Agent Service Crash or Unexpected Termination |
| 6 | ≡ƒö┤ Critical | Installation | INSTALL-001 | Extension Provisioning Failure |
| 7 | ≡ƒƒí Warning | Connectivity | CONN-004 | Proxy Configuration Error |
| 8 | ≡ƒƒí Warning | Performance Counters | PERF-001 | No Performance Counter Configuration |
| 9 | ≡ƒƒí Warning | Environment Sizing | SZ-003 | Disk Space Low |
| 10 | ≡ƒƒí Warning | Metrics Extension | METRICS-001 | MetricsExtension Error |
| 11 | ≡ƒƒí Warning | Environment Sizing | SZ-003 | Disk Space Constraint |

## Detailed Findings

### 1. ≡ƒö┤ Critical INSTALL-002 ΓÇö Extension Provisioning Error Log

**Severity**: Critical  
**Category**: Installation  

Extension provisioning logs contain error entries indicating the AMA extension failed to install or enable properly.


**Evidence:**

```
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.62.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.27.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.25.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.6.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.29.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.8.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.13.log
File: AgentDataStore\Tables\DefaultExtensionEtw.csv
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.20.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.1.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.26.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.21.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.41.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.15.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.4.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.22.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.60.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.31.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.37.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.57.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.23.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.2.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.19.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.61.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.28.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.3.log
File: AgentDataStore\Tables\MaMetricsExtensionEtw.csv
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.18.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.42.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.40.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.17.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.32.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.14.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.16.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.24.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.58.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.7.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.59.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.33.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.5.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.38.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.30.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.39.log
```

**Remediation:**

Check the extension status in the Azure portal. Common causes include blocked network endpoints, insufficient permissions, or OS compatibility issues. Try removing and reinstalling the extension.


**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly)

---

### 2. ≡ƒö┤ Critical IDENTITY-001 ΓÇö Missing Managed Identity

**Severity**: Critical  
**Category**: Identity  

No system-assigned or user-assigned managed identity token was found. AMA requires a managed identity to authenticate with Azure Monitor services. The AuthToken-MSI.json file is absent from the bundle.


**Evidence:**

```
Detected by rule: IDENTITY-001
```

**Remediation:**

Enable a system-assigned managed identity on the VM: az vm identity assign --resource-group <rg> --name <vm> For Arc servers, the managed identity is created automatically with the Connected Machine Agent registration.


**Documentation**: [https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm](https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm)

---

### 3. ≡ƒö┤ Critical IMDS-001 ΓÇö IMDS Connectivity Failure

**Severity**: Critical  
**Category**: Connectivity  

Detected 5 pattern match(es) in log files.

**Evidence:**

```
agentprocesses.txt:924: 169.254.169.254/metadata/instance IPv6     FAIL
agentprocesses.txt:1423: [IMDS Connectivity Test] Connection to 169.254.169.254/metadata/instance [IPv6]: curl.exe failed to connect to the endpoint
allprocesses.txt:748: 169.254.169.254/metadata/instance IPv6     FAIL
allprocesses.txt:1247: [IMDS Connectivity Test] Connection to 169.254.169.254/metadata/instance [IPv6]: curl.exe failed to connect to the endpoint
Html\TroubleshooterLogs.html:81: <br/>Starting external process: C:\Windows\system32\curl.exe -6 --noproxy '*' -v -s -S -k -H "Metadata: true" "http://169.254.169.254/metadata/instance?api-version=2019-11-01&format=json"<br/>* closing connection #0<br/>curl: (7) Could not connect to server<br/>Execution completed in 00:00:00.0523492<br/>Force IPv6 test: FAIL
```

**Remediation:**

See documentation: https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity

**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity)

---

### 4. ≡ƒö┤ Critical CONN-001 ΓÇö Network Connectivity Failure

**Severity**: Critical  
**Category**: Connectivity  

Detected 5 pattern match(es) in log files.

**Evidence:**

```
agentprocesses.txt:685: WARNING: TCP connect to (2603:1030:7:5::24 : 80) failed
agentprocesses.txt:687: WARNING: TCP connect to (2603:1030:7:5::24 : 443) failed
agentprocesses.txt:697: WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed
agentprocesses.txt:698: WARNING: TCP connect to (10.1.0.4 : 80) failed
agentprocesses.txt:699: WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed
```

**Remediation:**

See documentation: https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration

**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration)

---

### 5. ≡ƒö┤ Critical AGENT-001 ΓÇö Agent Service Crash or Unexpected Termination

**Severity**: Critical  
**Category**: Agent Not Running  

Detected 5 pattern match(es) in log files.

**Evidence:**

```
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.54.log:32: [2026-01-29 19:28:05.000]  [IsHealthMonitorRunning] ErrorCode:1301 INFO: Health monitor is not running. Id:4512
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.54.log:40: [2026-01-29 19:28:05.000]  [IsHealthMonitorRunning] ErrorCode:1301 INFO: Health monitor is not running. Id:4512
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log:23: [2024-10-17 11:14:15.000]  [CheckIfMAServiceIsRunByExtension] ErrorCode:1205 INFO: Unable to read the m_registry for MA process. Assume it is not running.
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log:24: [2024-10-17 11:14:15.000]  [AggregateHeartbeat] ErrorCode:0 INFO: Configured MA Service is not running on the node.
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log:87: [2024-10-17 11:19:15.000]  [CheckIfMAServiceIsRunByExtension] ErrorCode:1205 INFO: Unable to read the m_registry for MA process. Assume it is not running.
```

**Remediation:**

See documentation: https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm

**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm)

---

### 6. ≡ƒö┤ Critical INSTALL-001 ΓÇö Extension Provisioning Failure

**Severity**: Critical  
**Category**: Installation  

Detected 5 pattern match(es) in log files.

**Evidence:**

```
Html\TroubleshooterLogs.html:55: <br/>This activity could take up to 15 minutes to complete.<br/>Starting external process: C:\Windows\system32\WindowsPowerShell\v1.0\powershell.exe C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection.ps1 -OutputDirectory C:\Windows\SystemTemp\AgentTroubleshooter -SkipAdminCheck -TimeOutSecondsSearchFiles 600 -SkipCollectionTsfLogs -SkipCollectionCertificates<br/>WARNING: Administrative permissions check skipped due to SkipAdminCheck being set.<br/>Begin - MDM Data Collection Script - 03/26/2026 13:48:38<br/>Script version 1.14<br/>Collecting general information...<br/>Collecting information about logical disks<br/>Searching the process: MetricsExtension.Native.exe<br/>Testing several MetricsExtension global endpoints<br/>Collecting time from ME client and server<br/>WARNING: SkipCollectionCertificates has been set, skipping the collection of certificates.<br/>Listing open sockets and owning processes<br/>WARNING: TCP connect to (2603:1030:7:5::24 : 80) failed<br/>WARNING: Ping to 2603:1030:7:5::24 failed<br/>WARNING: TCP connect to (2603:1030:7:5::24 : 443) failed<br/>WARNING: Ping to 2603:1030:7:5::24 failed<br/>Resolve-DnsName : global.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (global.metrics.nsatc.net:String) [Resolve-DnsName], Win32Exception<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>Resolve-DnsName : global.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (global.metrics.nsatc.net:String) [Resolve-DnsName], Win32Exception<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>Resolve-DnsName : azglobal.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (azglobal.metrics.nsatc.net:String) [Resolve-DnsName], Win32Excepti <br/>   on<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>Resolve-DnsName : azglobal.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (azglobal.metrics.nsatc.net:String) [Resolve-DnsName], Win32Excepti <br/>   on<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>Collecting application event logs for Level Error and EventID 1000 happened within last one hour...<br/>Collecting raw and aggregated metrics, please DO NOT press CTRL+C<br/>Waiting for 190 seconds, press CTRL+C to quit ...189188187186185184183182181180179178177176175174173172171170169168167166165164163162161160159158157156155154153152151150149148147146145144143142141140139138137136135134133132131130129128127126125124123122121120119118117116115114113112111110109108107106105104103102101100 99 98 97 96 95 94 93 92 91 90 89 88 87 86 85 84 83 82 81 80 79 78 77 76 75 74 73 72 71 70 69 68 67 66 65 64 63 62 61 60 59 58 57 56 55 54 53 52 51 50 49 48 47 46 45 44 43 42 41 40 39 38 37 36 35 34 33 32 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0<br/>WARNING: No autopilot logs were found. If this machine is not an Autopilot machine, this is not an issue.<br/>WARNING: D:\app\autopilot.ini was not found. If this machine is not an Autopilot machine, this is not an issue.<br/>Searching the process: MonAgentHost.exe<br/>WARNING: SkipCollectionTsfLogs has been set, skipping the collection of tsf logs.<br/>Compressing MDM Data Collection Output ...<br/>Done -^> MDM Data ready at C:\Windows\SystemTemp\AgentTroubleshooter\MdmDataCollectionOutput2026_03_26_01_48_38_random_2025558213.zip<br/>End - MDM Data Collection Script - 03/26/2026 13:52:40<br/>Execution completed in 00:04:03.5779817<br/>Successfully executed log collection ps script.<br/>
Html\TroubleshooterLogs.html:146: <br/>Found 7 warnings<br/><br/>[MCS Connectivity Test] Connection to westus2-5w48.westus2-1.handler.control.monitor.azure.com [IPv6]: curl.exe failed to connect to the endpoint<br/>[ODS Connectivity Test] Connection to d1521c3b-0bd3-4d94-a6ed-22c3170a6b74.ods.opinsights.azure.com [IPv6]: curl.exe failed to connect to the endpoint<br/>[IMDS Connectivity Test] Connection to 169.254.169.254/metadata/instance [IPv6]: curl.exe failed to connect to the endpoint<br/>[Collecting Directory: RuntimeSettings] Collecting Directory: C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\RuntimeSettings: <br/>[Parse VM Extension Settings] Settings File Found: No settings file was found in the expected format. Skipping the VM Extension settings parsing.<br/>[Token Endpoint Connectivity Test] Connection to westus2-5w48.westus2-1.handler.control.monitor.azure.com [IPv6]: curl.exe failed to connect to the endpoint<br/>[Collect Active Directory Status] Collect Active Directory Status: Failed to execute log collection PowerShell script: 1<br/>
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log:1: [2024-10-22 23:05:17.000]  [ExtensionConfiguration] ErrorCode:0 INFO: Initializing ExtensionConfiguration
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log:6: [2024-10-22 23:05:17.000]  [ParseExtensionSettingsJson] ErrorCode:0 WARN: extension settings file C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\RuntimeSettings\0.settings does not exist
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log:9: [2024-10-22 23:05:17.000]  [Extension] ErrorCode:0 INFO: LogFolder: C:\WindowsAzure\Logs\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0, ConfigFolder: C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\RuntimeSettings, StatusFolder: C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Status, HeartbeatFile: C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Status\HeartBeat.Json, DeploymentId: 7ece9028-6c4b-452a-a9a1-f6e07d35e94b, RoleName: _amawin, Instance: _amawin, ExtensionVersion: 46.19.3.0
```

**Remediation:**

See documentation: https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly

**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly)

---

### 7. ≡ƒƒí Warning CONN-004 ΓÇö Proxy Configuration Error

**Severity**: Warning  
**Category**: Connectivity  

Log entries indicate errors related to proxy configuration. The agent may be unable to reach Azure endpoints through the configured proxy.


**Evidence:**

```
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.62.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.27.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134700818.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.25.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.6.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023230822943.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025050526582.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233251542.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.29.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.8.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129192806420.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223232946226.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.20.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025230526183.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.1.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.26.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026230529869.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.21.log
File: AgentDataStore\Configuration\MonAgentHost.15.log
File: AgentDataStore\Configuration\MonAgentLauncher.11.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.41.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024170523777.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134409665.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025170524771.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.15.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.4.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202406390.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.22.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129193145178.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024110524389.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.60.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.31.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.37.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.57.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023170521339.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129193147376.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233133677.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.23.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023050520569.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.2.log
File: AgentDataStore\Configuration\MonAgentHost.14.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326125617983.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.19.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202319199.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution.log
File: AgentDataStore\Configuration\MonAgentHost.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260224023848671.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.61.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024050524827.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260224000527297.log
File: AgentDataStore\Configuration\MonAgentLauncher.15.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134542879.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.28.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.3.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223190136588.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026050527654.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023110521689.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.18.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.42.log
File: AgentDataStore\Configuration\MonAgentLauncher.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.40.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326132554207.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134456294.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241027050559758.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134629769.log
File: AgentDataStore\Configuration\MonAgentLauncher.14.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.17.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.32.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026110529491.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.14.log
File: AgentDataStore\Configuration\MonAgentLauncher.12.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.16.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.24.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024230526005.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025110524038.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.58.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.7.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223191424359.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.59.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.33.log
File: AgentDataStore\Configuration\MonAgentHost.11.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134745452.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.5.log
File: AgentDataStore\Configuration\MonAgentHost.12.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233220559.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.38.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.30.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233049078.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.39.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202506886.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026170528406.log
```

**Remediation:**

Verify the proxy configuration in /etc/opt/microsoft/azuremonitoragent/proxy.conf (Linux) or the agent proxy settings (Windows). Test connectivity through the proxy to *.handler.control.monitor.azure.com and *.ingest.monitor.azure.com.


**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration)

---

### 8. ≡ƒƒí Warning PERF-001 ΓÇö No Performance Counter Configuration

**Severity**: Warning  
**Category**: Performance Counters  

The mcsconfig XML does not contain any CounterSet definitions. If the DCR is expected to collect performance counters, they are not configured.


**Evidence:**

```
Detected by rule: PERF-001
```

**Remediation:**

Add performance counter data sources to the DCR in the Azure portal. Navigate to Monitor > Data Collection Rules > [your DCR] > Data sources, and add a Performance Counters source.


**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/data-collection-performance-counters](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/data-collection-performance-counters)

---

### 9. ≡ƒƒí Warning SZ-003 ΓÇö Disk Space Low

**Severity**: Warning  
**Category**: Environment Sizing  

Log entries indicate disk space issues on the machine. AMA requires disk space for its data store, log buffering, and configuration cache.


**Evidence:**

```
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.62.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.27.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134700818.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.25.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.6.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023230822943.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025050526582.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.53.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233251542.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.29.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.8.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129192806420.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223232946226.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.20.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025230526183.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.1.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.26.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026230529869.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.21.log
File: AgentDataStore\Configuration\MonAgentHost.15.log
File: AgentDataStore\Configuration\MonAgentLauncher.11.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.41.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.55.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024170523777.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.56.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134409665.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025170524771.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.15.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.4.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202406390.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.22.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129193145178.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024110524389.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.60.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.31.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.37.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.57.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023170521339.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20260129193147376.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233133677.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.52.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.23.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023050520569.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.2.log
File: AgentDataStore\Configuration\MonAgentHost.14.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326125617983.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.19.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202319199.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution.log
File: AgentDataStore\Configuration\MonAgentHost.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260224023848671.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.46.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.54.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.61.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024050524827.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.44.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260224000527297.log
File: AgentDataStore\Configuration\MonAgentLauncher.15.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134542879.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.28.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.3.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223190136588.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026050527654.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241023110521689.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.18.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.42.log
File: AgentDataStore\Configuration\MonAgentLauncher.13.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.40.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326132554207.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134456294.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241027050559758.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134629769.log
File: AgentDataStore\Configuration\MonAgentLauncher.14.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.50.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.17.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.51.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.32.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026110529491.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.14.log
File: AgentDataStore\Configuration\MonAgentLauncher.12.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.48.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.16.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.24.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.49.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.45.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241024230526005.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241025110524038.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.58.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.7.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223191424359.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.59.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.33.log
File: AgentDataStore\Configuration\MonAgentHost.11.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260326134745452.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\ExtensionHealth.5.log
File: AgentDataStore\Configuration\MonAgentHost.12.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233220559.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.38.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.42.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\ExtensionHealth.43.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.47.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Extension.30.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223233049078.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.39.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\CommandExecution_20260223202506886.log
File: VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\CommandExecution_20241026170528406.log
```

**Remediation:**

Free disk space on the partition where AMA stores its data. On Windows this is typically C:\\WindowsAzure\\Resources\\AMADataStore. On Linux, check /var/opt/microsoft/azuremonitoragent/.


**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance)

---

### 10. ≡ƒƒí Warning METRICS-001 ΓÇö MetricsExtension Error

**Severity**: Warning  
**Category**: Metrics Extension  

Detected 5 pattern match(es) in log files.

**Evidence:**

```
Html\TroubleshooterLogs.html:55: <br/>This activity could take up to 15 minutes to complete.<br/>Starting external process: C:\Windows\system32\WindowsPowerShell\v1.0\powershell.exe C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection.ps1 -OutputDirectory C:\Windows\SystemTemp\AgentTroubleshooter -SkipAdminCheck -TimeOutSecondsSearchFiles 600 -SkipCollectionTsfLogs -SkipCollectionCertificates<br/>WARNING: Administrative permissions check skipped due to SkipAdminCheck being set.<br/>Begin - MDM Data Collection Script - 03/26/2026 13:48:38<br/>Script version 1.14<br/>Collecting general information...<br/>Collecting information about logical disks<br/>Searching the process: MetricsExtension.Native.exe<br/>Testing several MetricsExtension global endpoints<br/>Collecting time from ME client and server<br/>WARNING: SkipCollectionCertificates has been set, skipping the collection of certificates.<br/>Listing open sockets and owning processes<br/>WARNING: TCP connect to (2603:1030:7:5::24 : 80) failed<br/>WARNING: Ping to 2603:1030:7:5::24 failed<br/>WARNING: TCP connect to (2603:1030:7:5::24 : 443) failed<br/>WARNING: Ping to 2603:1030:7:5::24 failed<br/>Resolve-DnsName : global.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (global.metrics.nsatc.net:String) [Resolve-DnsName], Win32Exception<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>Resolve-DnsName : global.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (global.metrics.nsatc.net:String) [Resolve-DnsName], Win32Exception<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>Resolve-DnsName : azglobal.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (azglobal.metrics.nsatc.net:String) [Resolve-DnsName], Win32Excepti <br/>   on<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 80) failed<br/>WARNING: TCP connect to (10.1.0.4 : 80) failed<br/>Resolve-DnsName : azglobal.metrics.nsatc.net : DNS name does not exist<br/>At C:\Packages\Plugins\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.39.0.0\Troubleshooter\tools\mdmDataCollection<br/>.ps1:440 char:26<br/>+             $solvedDNS = Resolve-DnsName -Name $meFrontendUrl<br/>+                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ResourceUnavailable: (azglobal.metrics.nsatc.net:String) [Resolve-DnsName], Win32Excepti <br/>   on<br/>    + FullyQualifiedErrorId : DNS_ERROR_RCODE_NAME_ERROR,Microsoft.DnsClient.Commands.ResolveDnsName<br/> <br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>WARNING: TCP connect to (fe80::2add:a8ea:1599:2c4b%4 : 443) failed<br/>WARNING: TCP connect to (10.1.0.4 : 443) failed<br/>Collecting application event logs for Level Error and EventID 1000 happened within last one hour...<br/>Collecting raw and aggregated metrics, please DO NOT press CTRL+C<br/>Waiting for 190 seconds, press CTRL+C to quit ...189188187186185184183182181180179178177176175174173172171170169168167166165164163162161160159158157156155154153152151150149148147146145144143142141140139138137136135134133132131130129128127126125124123122121120119118117116115114113112111110109108107106105104103102101100 99 98 97 96 95 94 93 92 91 90 89 88 87 86 85 84 83 82 81 80 79 78 77 76 75 74 73 72 71 70 69 68 67 66 65 64 63 62 61 60 59 58 57 56 55 54 53 52 51 50 49 48 47 46 45 44 43 42 41 40 39 38 37 36 35 34 33 32 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0<br/>WARNING: No autopilot logs were found. If this machine is not an Autopilot machine, this is not an issue.<br/>WARNING: D:\app\autopilot.ini was not found. If this machine is not an Autopilot machine, this is not an issue.<br/>Searching the process: MonAgentHost.exe<br/>WARNING: SkipCollectionTsfLogs has been set, skipping the collection of tsf logs.<br/>Compressing MDM Data Collection Output ...<br/>Done -^> MDM Data ready at C:\Windows\SystemTemp\AgentTroubleshooter\MdmDataCollectionOutput2026_03_26_01_48_38_random_2025558213.zip<br/>End - MDM Data Collection Script - 03/26/2026 13:52:40<br/>Execution completed in 00:04:03.5779817<br/>Successfully executed log collection ps script.<br/>
Html\TroubleshooterLogs.html:64:                 } -SkipCollectionCertificates<br/>Get-WinEvent : No events were found that match the specified selection criteria.<br/>At line:3 char:13<br/>+  $aadLogs = Get-WinEvent -LogName Microsoft-Windows-AAD/Operational<br/>+             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ObjectNotFound: (:) [Get-WinEvent], Exception<br/>    + FullyQualifiedErrorId : NoMatchingEventsFound,Microsoft.PowerShell.Commands.GetWinEventCommand<br/> <br/>-SkipCollectionCertificates : The term '-SkipCollectionCertificates' is not recognized as the name of a cmdlet, <br/>function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the <br/>path is correct and try again.<br/>At line:5 char:4<br/>+    ~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ObjectNotFound: (-SkipCollectionCertificates:String) [], CommandNotFoundException<br/>    + FullyQualifiedErrorId : CommandNotFoundException<br/> <br/>Execution completed in 00:00:01.1734188<br/>Get-WinEvent : No events were found that match the specified selection criteria.<br/>At line:3 char:13<br/>+  $aadLogs = Get-WinEvent -LogName Microsoft-Windows-AAD/Operational<br/>+             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ObjectNotFound: (:) [Get-WinEvent], Exception<br/>    + FullyQualifiedErrorId : NoMatchingEventsFound,Microsoft.PowerShell.Commands.GetWinEventCommand<br/> <br/>-SkipCollectionCertificates : The term '-SkipCollectionCertificates' is not recognized as the name of a cmdlet, <br/>function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the <br/>path is correct and try again.<br/>At line:5 char:4<br/>+  } -SkipCollectionCertificates<br/>+    ~~~~~~~~~~~~~~~~~~~~~~~~~~~<br/>    + CategoryInfo          : ObjectNotFound: (-SkipCollectionCertificates:String) [], CommandNotFoundException<br/>    + FullyQualifiedErrorId : CommandNotFoundException<br/> <br/>Starting external process: C:\Windows\system32\dsregcmd.exe /status<br/>+----------------------------------------------------------------------+<br/>| Device State                                                         |<br/>+----------------------------------------------------------------------+<br/>             AzureAdJoined : NO<br/>          EnterpriseJoined : NO<br/>              DomainJoined : NO<br/>+----------------------------------------------------------------------+<br/>| User State                                                           |<br/>+----------------------------------------------------------------------+<br/>                    NgcSet : NO<br/>           WorkplaceJoined : NO<br/>             WamDefaultSet : ERROR<br/>+----------------------------------------------------------------------+<br/>| SSO State                                                            |<br/>+----------------------------------------------------------------------+<br/>                AzureAdPrt : NO<br/>       AzureAdPrtAuthority : NO<br/>             EnterprisePrt : NO<br/>    EnterprisePrtAuthority : NO<br/>+----------------------------------------------------------------------+<br/>| Ngc Prerequisite Check                                               |<br/>+----------------------------------------------------------------------+<br/>            IsDeviceJoined : NO<br/>             IsUserAzureAD : NO<br/>             PolicyEnabled : NO<br/>          PostLogonEnabled : YES<br/>            DeviceEligible : YES<br/>        SessionIsNotRemote : YES<br/>            CertEnrollment : none<br/>              PreReqResult : WillNotProvision<br/>Execution completed in 00:00:00.5980589<br/>
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.36.log:38: [2024-10-22 23:05:17.000]  [DeleteRegValue] ErrorCode:1207 ERROR: Failed to delete s_registry value, name=ExtensionIsInUpdate, key=SOFTWARE\Microsoft\Windows Azure\CurrentVersion\AzureMonitorAgentExtension\46.19.3.0, error=2
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.37.log:38: [2024-10-23 05:05:17.000]  [DeleteRegValue] ErrorCode:1207 ERROR: Failed to delete s_registry value, name=ExtensionIsInUpdate, key=SOFTWARE\Microsoft\Windows Azure\CurrentVersion\AzureMonitorAgentExtension\46.19.3.0, error=2
VmExtLogs\Microsoft.Azure.Monitor.AzureMonitorWindowsAgent\1.28.2.0\Extension.38.log:38: [2024-10-23 11:05:18.000]  [DeleteRegValue] ErrorCode:1207 ERROR: Failed to delete s_registry value, name=ExtensionIsInUpdate, key=SOFTWARE\Microsoft\Windows Azure\CurrentVersion\AzureMonitorAgentExtension\46.19.3.0, error=2
```

**Remediation:**

See documentation: https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm

**Documentation**: [https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm)

---

### 11. ≡ƒƒí Warning SZ-003 ΓÇö Disk Space Constraint

**Severity**: Warning  
**Category**: Environment Sizing  

Log entries indicate disk space issues. AMA requires adequate disk space for log buffering and data store.

**Evidence:**

```
AgentDataStore\Configuration\MonAgentHost.11.log:39: Info  (2026-01-30T00:33:12Z): TableManager - Setting the disk quota for table storage to 10000MB; the available disk space is 129481MB
AgentDataStore\Configuration\MonAgentHost.12.log:39: Info  (2026-02-20T20:55:19Z): TableManager - Setting the disk quota for table storage to 10000MB; the available disk space is 129481MB
AgentDataStore\Configuration\MonAgentHost.13.log:37: Info  (2026-02-21T00:31:12Z): TableManager - Setting the disk quota for table storage to 10000MB; the available disk space is 129481MB
AgentDataStore\Configuration\MonAgentHost.14.log:39: Info  (2026-02-23T19:02:42Z): TableManager - Setting the disk quota for table storage to 10000MB; the available disk space is 129481MB
AgentDataStore\Configuration\MonAgentHost.15.log:39: Info  (2026-03-26T12:57:25Z): TableManager - Setting the disk quota for table storage to 10000MB; the available disk space is 129481MB
```

**Remediation:**

Free disk space on the data store partition or expand the disk.

---


error: process didn't exit successfully: `target\release\amadiag.exe analyze .\samples\AgentTroubleshooterOutput-amawin-2026-03-26 --format markdown` (exit code: 1)
