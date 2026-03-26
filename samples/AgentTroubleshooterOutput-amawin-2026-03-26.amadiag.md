# AMADiag Ops Cockpit Export

## Scope

- **Bundle**: `.\samples\AgentTroubleshooterOutput-amawin-2026-03-26.zip`
- **Time filter**: Last 24h
- **Category filter**: All categories
- **Visible grouped findings**: 5

## Selected finding

- **Title**: AMA service crash or unexpected termination
- **Category**: Service
- **Severity**: Critical
- **Status**: Fail
- **Event count**: 29

Error (2026-03-26T13:02:26Z): Tasks - MAEventTable: Level:2 ActivityId:00-e804afdb0b4f4f65bd2b40c6892f71ff-6b5ec8c7fb802c02-00 MDRESULT:0x80080005 ErrorCode(...

### Likely causes

- AMA service or extension installation is unhealthy.

### Suggested actions

- Review extension/service logs and restart or reinstall if necessary.

## Grouped findings

| Severity | Category | Title | Events |
|---|---|---|---:|
| Critical | Service | AMA service crash or unexpected termination | 29 |
| High | Connectivity | Connectivity failures to ingestion or control endpoint | 48 |
| High | Install | Extension provisioning or installation failure | 124 |
| High | Other | Error log event | 23 |
| Medium | Other | Warning log event | 36 |
