# keelOS fork of Kanidm

Upstream base: Kanidm v1.11.0 (commit ed10baa), branch `keel-1.11`.

## Why

keelOS / keelAuth run Kanidm with an IP-literal origin (for example
`https://192.168.12.198:8443`) while `domain` stays a DNS name, so customers need
no DNS records or hosts-file entries. In this mode passkeys (WebAuthn) are
unavailable; password/TOTP login, OIDC and LDAP work.

## Changes

1. `server/lib/src/idm/server.rs`: an IP-literal origin passes the origin/domain
   consistency check (a warning is logged). webauthn-rs refuses IP origins, so it is
   given a substitute origin `https://<domain>`; passkeys are therefore unusable.
2. `server/core/src/https/{mod.rs,v1.rs,views/cookies.rs}`: new
   `ServerState.cookie_domain`, set only when the origin host is a DNS name. With an
   IP origin cookies are host-only (a `Domain=` cookie would be rejected by browsers).
3. `server/core/src/lib.rs`: `cert-generate` with an IP origin uses `domain` as the
   certificate name. The generated cert has no IP SAN; supply a real certificate
   for IP use.

## Licence

Kanidm is under the MPL 2.0. All modified files remain under the same licence.
