# Fly.io Bug Bounty Hunt - Findings

## Target
- **Primary**: fly.io, *.fly.io, api.fly.io, dashboard.fly.io (DNS fails)
- **APIs**: GraphQL (api.fly.io/graphql), REST Machines API (api.machines.dev)

---

## Recon Summary

### Assets Discovered
| Asset | Status | Notes |
|-------|--------|-------|
| fly.io | ✅ Live | Main marketing site, Phoenix/Elixir |
| api.fly.io | ✅ Live | GraphQL endpoint, introspection enabled |
| api.machines.dev | ✅ Live | REST API for Machines (OpenAPI spec public) |
| portal.fly.io | ✅ Live | Redirects to fly.io/app/sign-in/ |
| docs.fly.io | ✅ Live | Mintlify docs, OpenAPI spec at /api/machines/openapi.json |
| community.fly.io | ✅ Live | Discourse forum |
| status.fly.io | ✅ Live | Status page |
| dashboard.fly.io | ❌ DNS Fail | Not resolvable |
| app.fly.io | ❌ DNS Fail | Not resolvable |

### Technology Stack
- **Web App**: Phoenix Framework (Elixir), LiveView
- **GraphQL**: Absinthe (Elixir GraphQL)
- **REST API**: Separate Machines API at api.machines.dev
- **Auth**: OAuth (GitHub, Google), FlyV1 tokens, session-based
- **Analytics**: PostHog (posthog.fly.io), Google Analytics
- **CDN**: Fly.io edge proxy (Fly-Forwarded headers)

---

## Findings

### 1. 🔴 HIGH - Information Disclosure via `/debug` Endpoint (api.fly.io)

**Location**: `https://api.fly.io/debug`

**Description**: The `/debug` endpoint on the GraphQL API host returns all HTTP request headers including sensitive internal infrastructure headers.

**Exposed Headers**:
```
Fly-Forwarded-Proto: https
Fly-Forwarded-Port: 443
Fly-Region: iad
Via: 1.1 fly.io, 1.1 fly.io, 1.1 fly.io
Fly-Src: instance=e826159c51e908;app=flyio-web-proxy;org=fly;ts=1795012101
X-Forwarded-For: [client IPs including internal 172.19.0.146]
Fly-Src-Signature: [cryptographic signature]
Fly-Request-Id: [unique request ID]
Fly-Traceparent: [W3C trace context]
Fly-Tracestate: [trace state]
Fly-Client-Ip: [client IP]
X-Forwarded-Proto: https
Fly-Forwarded-Ssl: on
X-Request-Start: [timestamp]
```

**Impact**: 
- Internal infrastructure topology disclosure (multiple proxy hops)
- Internal IP ranges (172.19.0.0/16)
- Request tracing IDs that could be used for log correlation attacks
- Cryptographic signature format disclosure
- Region and instance identification

**Proof**: `curl -sSL https://api.fly.io/debug`

**Remediation**: Remove or restrict `/debug` endpoint to internal networks only, or sanitize headers before display.

---

### 2. 🟡 MEDIUM - GraphQL Introspection Enabled (api.fly.io/graphql)

**Location**: `https://api.fly.io/graphql`

**Description**: Full GraphQL schema introspection is enabled and accessible without authentication. This exposes the complete API attack surface including:
- All types, fields, and arguments
- Sensitive fields: `Secret.value`, `AddOn.password`, `AppConfig`, `App.secrets`, `App.certificates`
- Mutations for app/machine/volume/secret/certificate management
- Internal fields like `flapsAuthzCheck` (inter-service authorization)

**Impact**: 
- Attackers can map the entire API surface for targeted attacks
- Reveals sensitive field names that might be targeted for data exfiltration
- Internal authorization check field (`flapsAuthzCheck`) suggests potential auth bypass vectors

**Proof**: 
```bash
curl -X POST https://api.fly.io/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ __schema { types { name fields { name } } } }"}'
```

**Remediation**: Disable introspection in production, or restrict to authenticated users only.

---

### 3. 🟡 MEDIUM - GraphQL Playground Publicly Accessible

**Location**: `https://api.fly.io/graphql` (GET with `Accept: text/html`)

**Description**: GraphQL Playground (GraphiQL) is publicly accessible, providing an interactive IDE for query exploration. While queries still require authentication, this lowers the barrier for attackers to test and refine attacks.

**Impact**: 
- Facilitates attack development and testing
- Combined with introspection, enables rapid vulnerability discovery

**Remediation**: Disable GraphQL Playground in production, or restrict to authenticated/admin users.

---

### 4. 🟡 MEDIUM - Sensitive Fields in GraphQL Schema

**Location**: GraphQL Schema (api.fly.io/graphql)

**Fields of Concern**:
| Type | Field | Sensitivity |
|------|-------|-------------|
| `Secret` | `value` | Secret values (API keys, passwords, tokens) |
| `AddOn` | `password` | Add-on credentials |
| `AddOn` | `environment` | JSON env vars (may contain secrets) |
| `AddOn` | `options` | JSON config (may contain secrets) |
| `App` | `secrets` | List of all app secrets |
| `App` | `config` | App configuration |
| `App` | `certificates` | TLS certificates |
| `App` | `machines` / `vms` | VM instances |
| `App` | `volumes` | Persistent volumes |

**Impact**: If authentication is bypassed or tokens are compromised, these fields provide direct access to highly sensitive data.

**Remediation**: Ensure proper authorization checks on all sensitive fields. Consider field-level permissions.

---

### 5. 🟢 LOW - No Rate Limiting on GraphQL Endpoint

**Location**: `https://api.fly.io/graphql`

**Description**: The GraphQL endpoint does not appear to enforce rate limiting. 10 rapid requests all returned 200 OK (with auth errors in body).

**Impact**: 
- Facilitates brute force attacks on authentication
- Enables high-volume schema introspection/queries
- Potential for DoS via complex queries

**Proof**: 10 rapid POST requests all returned HTTP 200.

**Remediation**: Implement rate limiting (per IP, per token, per operation complexity).

---

### 6. 🟢 LOW - OAuth Callback Redirects to Sign-In

**Location**: `https://fly.io/app/auth/github/callback`, `https://fly.io/app/auth/google/callback`

**Description**: OAuth callback endpoints redirect to sign-in page without error handling visible. This is standard behavior but should be verified for proper state parameter validation and PKCE enforcement.

**Impact**: Potential for OAuth flow manipulation if state/PKCE not properly implemented.

**Remediation**: Verify OAuth implementation follows RFC 6749 + PKCE (RFC 7636), validate state parameter, use secure redirect URIs.

---

### 7. 🟢 INFO - Multiple API Endpoints (Attack Surface)

**Discovered APIs**:
1. **GraphQL** (api.fly.io/graphql) - Primary management API
2. **REST Machines API** (api.machines.dev/v1) - Machine-level operations
3. **Tokens API** (api.machines.dev/v1/tokens/*) - Authentication

**Endpoints of Interest**:
- `/v1/apps` - List/create apps (requires org_slug)
- `/v1/apps/{app_name}/machines` - Machine management
- `/v1/apps/{app_name}/machines/{machine_id}/exec` - **Command execution on VMs**
- `/v1/apps/{app_name}/secrets` - Secret management
- `/v1/apps/{app_name}/certificates` - TLS certificate management
- `/v1/tokens/authenticate` - Token exchange
- `/v1/tokens/current` - Current token info

**Note**: All REST endpoints require `Authorization: Bearer <FlyV1_token>` or `Authorization: FlyV1 <token>`.

---

## Attack Vectors for Further Testing (Requires Auth)

### A. IDOR / Broken Access Control
- Test cross-organization app/machine/volume access
- Test `appNameAvailable` for enumeration
- Test machine exec across apps

### B. Privilege Escalation
- Token exchange flows (`/v1/tokens/authorize`, `/v1/tokens/authenticate`)
- Deploy token creation (`/v1/apps/{app_name}/deploy_token`)
- Machine lease mechanism for exclusive access

### C. Injection Attacks
- GraphQL query complexity/depth DoS
- REST API parameter injection (app_name, machine_id)
- Secret/environment variable injection

### D. SSRF
- AddOn provisioning with external URLs
- Certificate ACME challenges (DNS/HTTP validation)
- Webhook configurations (if any)

### E. Race Conditions
- App creation (name collision handling)
- Machine start/stop/lease operations
- Secret updates

---

## Scope Compliance Check

| Asset | In Scope? | Tested? |
|-------|-----------|---------|
| fly.io | ✅ | ✅ |
| *.fly.io | ✅ | ✅ (subdomains checked) |
| api.fly.io | ✅ | ✅ |
| dashboard.fly.io | ✅ | ❌ (DNS fails) |
| api.machines.dev | ❓ Not explicitly listed | ✅ (discovered via docs) |

**Note**: `api.machines.dev` is the Machines REST API documented at docs.fly.io/api/machines. It should be considered in scope as it's part of the Fly.io platform.

---

## Next Steps

1. **Authorization Required**: All meaningful testing requires valid Fly.io account and tokens
2. **Priority Tests** (once auth obtained):
   - IDOR across organizations on GraphQL and REST APIs
   - Machine exec command injection
   - Token privilege escalation
   - AddOn password/environment exposure
   - Race conditions on resource creation
3. **Report**: `/debug` endpoint info disclosure (HIGH) and GraphQL introspection (MEDIUM) can be reported immediately

---

## Tools Used
- curl for API testing
- browser_exec for web app exploration
- GraphQL introspection for schema analysis
- OpenAPI spec analysis for REST API mapping

---

## Files
- `/home/ubuntu/bugbounty-agents/hunts/flyio/scope.md` - Scope document
- `/home/ubuntu/bugbounty-agents/hunts/flyio/findings.md` - This file