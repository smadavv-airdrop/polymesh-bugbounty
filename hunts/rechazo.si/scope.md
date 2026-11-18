# rechazo.si - Bug Bounty Hunt Scope

## Program Overview
- **Target**: rechazo.si (Rechazo - AI Platform, Slovenia)
- **Type**: Self-hosted Bug Bounty / VDP
- **Discovery Date**: 2026-11-12 (6 days old in index)
- **Source**: security.txt (RFC 9116)
- **Policy URL**: https://rechazo.si/legal/security
- **Security Contact**: security@rechazo.si
- **PGP/Encryption**: Not specified
- **Status**: ACTIVE (confirmed live, policy accessible)
- **Reward Type**: MONETARY (confirmed in dataset)
- **Safe Harbor**: STATED
- **Competition Level**: LOW

## Scope

### In-Scope Assets
| Asset | Type | Notes |
|-------|------|-------|
| `*.rechazo.si` | Wildcard subdomain | All subdomains |
| `rechazo.si` | Web application | Main site |

### Out-of-Scope / Excluded Techniques
- None explicitly listed

## Technical Profile
- **Frontend**: Custom/Next.js
- **Service**: AI platform (Slovenia-based)
- **Domain**: rechazo.si (.si = Slovenia)
- **Language**: Likely Slovenian/English

## Hunt Plan Playbooks
1. `skills/09-security/recon/subdomain.md`
2. `skills/09-security/recon/js-mining.md`
3. `skills/09-security/web/idor.md`
4. `skills/09-security/web/xss.md`
5. `skills/09-security/auth/session.md`
6. `skills/09-security/api/rest-authz.md`

## Authorization Gate
- **Gate Mode**: STRICT → HUMAN_CONFIRM
- **All 5 gate checks**: CLEAR

## Priority: HIGH (Tier 1) - AI platform, EU jurisdiction (GDPR), fresh

## Reporting
- **Channel**: Email to security@rechazo.si
- **Submission**: Human-gated