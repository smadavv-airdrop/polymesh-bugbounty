# eve.security - Bug Bounty Hunt Scope

## Program Overview
- **Target**: eve.security (Eve Security - Security Platform)
- **Type**: Self-hosted Bug Bounty / VDP
- **Discovery Date**: 2026-11-12 (6 days old in index)
- **Source**: security.txt (RFC 9116)
- **Policy URL**: https://eve.security/vulnerability-disclosure
- **Security Contact**: security@eve.security
- **PGP/Encryption**: Not specified in security.txt
- **Status**: ACTIVE (confirmed live, policy accessible)
- **Reward Type**: MONETARY (confirmed in dataset)
- **Safe Harbor**: STATED
- **Competition Level**: LOW (no acknowledgments page, very new)

## Scope

### In-Scope Assets
| Asset | Type | Notes |
|-------|------|-------|
| `*.eve.security` | Wildcard subdomain | All subdomains |
| `api.eve.security` | API endpoint | Primary API surface |
| `eve.security` | Web application | Main site |
| `app.eve.security` | Web application | Likely dashboard (wildcard covers) |

### Out-of-Scope
- None explicitly listed

### Excluded Testing Techniques
- None explicitly listed
- Automated/agentic testing: **ALLOWED**

## Technical Profile
- **Frontend**: Next.js (React) - Vercel deployment
- **Service**: Security platform (vulnerability management, compliance)
- **Domain**: eve.security (.security TLD = security company)

## Attack Surface Keywords
- `security` / `vulnerability` / `compliance` → Platform logic, report access, tenant isolation
- `api` / `rest` → REST API authorization testing
- `web` / `dashboard` → Web application baseline (IDOR, XSS, Session)

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

## Priority: HIGH (Tier 1) - Security platform, fresh, no competition

## Reporting
- **Channel**: Email to security@eve.security
- **Submission**: Human-gated