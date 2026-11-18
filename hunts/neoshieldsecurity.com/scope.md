# neoshieldsecurity.com - Bug Bounty Hunt Scope

## Program Overview
- **Target**: neoshieldsecurity.com (NeoShield Security - Security Platform)
- **Type**: Self-hosted Bug Bounty / VDP
- **Discovery Date**: 2026-11-12 (6 days old in index)
- **Source**: security.txt (RFC 9116)
- **Policy URL**: https://neoshieldsecurity.com/responsible-disclosure
- **Security Contact**: security@neoshieldsecurity.com
- **PGP/Encryption**: Not specified
- **Status**: ACTIVE (confirmed live, policy accessible)
- **Reward Type**: MONETARY (confirmed in dataset)
- **Safe Harbor**: STATED
- **Competition Level**: LOW

## Scope

### In-Scope Assets
| Asset | Type | Notes |
|-------|------|-------|
| `*.neoshieldsecurity.com` | Wildcard subdomain | All subdomains |
| `neoshieldsecurity.com` | Web application | Main site |

### Out-of-Scope / Excluded Techniques
- None explicitly listed

## Technical Profile
- **Frontend**: Custom/Next.js
- **Service**: Security platform
- **Domain**: neoshieldsecurity.com

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

## Priority: HIGH (Tier 1) - Security platform, fresh

## Reporting
- **Channel**: Email to security@neoshieldsecurity.com
- **Submission**: Human-gated