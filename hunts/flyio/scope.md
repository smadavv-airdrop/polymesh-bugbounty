# Fly.io Bug Bounty Scope Document

## Program
- **Platform**: HackerOne (https://hackerone.com/flyio)
- **Policy URL**: https://fly.io/security/
- **Rewards**: Yes (bug bounty program)

---

## In-Scope Assets

### Domains
| Asset | Status | Notes |
|-------|--------|-------|
| fly.io | ✅ Live | Main marketing site, Phoenix/Elixir |
| *.fly.io | ✅ Wildcard | Subdomains resolve |
| api.fly.io | ✅ Live | GraphQL endpoint, introspection enabled |
| dashboard.fly.io | ❌ DNS Fail | Listed in scope but DNS does not resolve |
| portal.fly.io | ✅ Live | Redirects to fly.io/app/sign-in/ |
| docs.fly.io | ✅ Live | Mintlify docs, OpenAPI spec at /api/machines/openapi.json |
| community.fly.io | ✅ Live | Discourse forum (3rd party) |
| status.fly.io | ✅ Live | Status page |
| api.machines.dev | ✅ Live | REST Machines API (discovered via docs) |

### APIs
| API | Base URL | Auth | Notes |
|-----|----------|------|-------|
| GraphQL | https://api.fly.io/graphql | FlyV1 Token / Session | Full introspection enabled, Playground accessible |
| REST Machines | https://api.machines.dev/v1 | Bearer FlyV1 Token | OpenAPI spec public at docs.fly.io/api/machines/openapi.json |
| Tokens | https://api.machines.dev/v1/tokens/* | Client ID/Secret | OAuth2-style token exchange |

---

## Out-of-Scope

### Domains / Services
- Third-party services (GitHub OAuth, Google OAuth, Cloudflare, PostHog, Google Analytics)
- community.fly.io (Discourse - third party, but test for SSO issues)
- status.fly.io (Status page)

### Vulnerability Classes
- Social engineering (phishing, vishing, smishing)
- Physical attacks
- DoS/DDoS attacks
- Spam
- Issues in third-party services
- Self-XSS requiring unusual user interaction
- Missing security headers (unless exploitable)
- Cookie flags (HttpOnly, Secure) unless exploitable
- TLS version/configuration issues (unless exploitable)
- Content spoofing / clickjacking (unless exploitable)
- Rate limiting (unless bypassable for significant impact)
- Information disclosure of non-sensitive data
- User enumeration (unless leads to account takeover)

---

## Submission Channel
- **Primary**: HackerOne (https://hackerone.com/flyio)
- **Direct**: security@fly.io

---

## Testing Restrictions
- No automated scanning without approval
- No testing on production customer apps without explicit permission
- Respect rate limits
- No destructive testing
- Report findings immediately, do not exploit further
- **Authorization Gate**: Must have valid Fly.io account before testing authenticated endpoints

---

## Reward Tiers (Estimated from HackerOne)
| Severity | Min Bounty | Max Bounty |
|----------|------------|------------|
| Critical | $3,000 | $10,000+ |
| High | $1,000 | $3,000 |
| Medium | $300 | $1,000 |
| Low | $100 | $300 |
| Informational | $0 | $100 |

---

## Discovered Attack Surface (For Reference)

### GraphQL API (api.fly.io/graphql)
- Introspection: **ENABLED** (no auth)
- Playground: **ACCESSIBLE** (GET with Accept: text/html)
- Sensitive fields: `Secret.value`, `AddOn.password`, `App.secrets`, `App.config`, `App.certificates`, `App.machines`, `App.volumes`
- Mutations: App/machine/volume/secret/certificate/AddOn management
- Internal field: `flapsAuthzCheck` (inter-service auth)

### REST Machines API (api.machines.dev/v1)
- `/v1/apps` - List/create apps (requires org_slug)
- `/v1/apps/{app_name}/machines` - Machine CRUD
- `/v1/apps/{app_name}/machines/{id}/exec` - **Command execution on VMs**
- `/v1/apps/{app_name}/secrets` - Secret management
- `/v1/apps/{app_name}/certificates` - TLS cert management
- `/v1/apps/{app_name}/volumes` - Volume management
- `/v1/apps/{app_name}/deploy_token` - Create deploy tokens
- `/v1/tokens/authenticate` - Token exchange
- `/v1/tokens/authorize` - OAuth code exchange
- `/v1/tokens/current` - Current token info

### Web Application (fly.io)
- Framework: Phoenix (Elixir) + LiveView
- Auth: Session + OAuth (GitHub, Google)
- CSRF: Token in meta tag (`csrf-token`)
- Analytics: PostHog (posthog.fly.io), Google Analytics
- Debug endpoint: `/debug` exposes internal headers (api.fly.io only)

---

## 7-Question Gate Results (Pre-Submission)

| # | Question | /debug | Introspection | Playground |
|---|----------|--------|---------------|------------|
| Q1 | Real HTTP request works now? | ✅ YES | ✅ YES | ✅ YES |
| Q2 | Impact on accepted list? | ✅ YES | ✅ YES | ✅ YES |
| Q3 | Root cause in-scope? | ✅ YES | ✅ YES | ✅ YES |
| Q4 | Works without privileged access? | ✅ YES | ✅ YES | ✅ YES |
| Q5 | Not already known? | ✅ LIKELY | ❓ MAYBE | ❓ MAYBE |
| Q6 | Impact provable? | ✅ YES | ✅ YES | ✅ YES |
| Q7 | Not on never-submit list? | ✅ YES | ✅ YES | ✅ YES |

---

## Ready for Submission
1. **HIGH** - Information Disclosure via `/debug` endpoint (api.fly.io)
2. **MEDIUM** - GraphQL Introspection Enabled (api.fly.io/graphql)
3. **LOW/MEDIUM** - GraphQL Playground Publicly Accessible

---

## Files
- `/home/ubuntu/bugbounty-agents/hunts/flyio/scope.md` - This file
- `/home/ubuntu/bugbounty-agents/hunts/flyio/findings.md` - Detailed findings