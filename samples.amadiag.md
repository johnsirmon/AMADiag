# AMADiag Ops Cockpit Export

## Scope

- **Bundle**: `\\?\C:\source\AMADiag\samples`
- **Time filter**: Last 24h
- **Category filter**: All categories
- **Visible grouped findings**: 3

## Selected finding

- **Title**: AMA service crash or unexpected termination
- **Category**: Service
- **Severity**: Critical
- **Status**: Fail
- **Event count**: 2

Error (2026-03-26T13:02:26Z): Tasks - MAEventTable: Level:2 ActivityId:00-e804afdb0b4f4f65bd2b40c6892f71ff-6b5ec8c7fb802c02-00 MDRESULT:0x80080005 ErrorCode(...

### Likely causes

- AMA service or extension installation is unhealthy.

### Suggested actions

- Review extension/service logs and restart or reinstall if necessary.

## Grouped findings

| Severity | Category | Title | Events |
|---|---|---|---:|
| Critical | Service | AMA service crash or unexpected termination | 2 |
| High | Other | Error log event | 98 |
| Medium | Other | Warning log event | 45 |
