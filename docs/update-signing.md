# Update Signing — Dust Trail

Release artefacts are sealed with `SHA256SUMS.txt` plus a GPG detached,
ASCII-armored signature (`SHA256SUMS.txt.asc`) over that file. The in-app
updater verifies the checksum **and** the signature before any install
(guide §7.2); a missing or bad signature aborts the update and leaves the
running version untouched.

## Current signing key

| Field | Value |
|---|---|
| Identity | `Dust Trail Dev <dev@example.com>` |
| Algorithm | RSA 3072 (sign-only) |
| Fingerprint | `E3C7 0D79 A8A8 A209 60B4 5FD4 2A2E 09D1 B7BC D1A1` |
| Created | 2026-11-22 |
| Expires | 2028-11-21 (2 years) |
| Private key | GitHub Actions secret `GPG_PRIVATE_KEY` only — never on a developer machine (guide §7.2) |
| Passphrase | GitHub Actions secret `GPG_PASSPHRASE` only |
| Pin | `GPG_KEY_FINGERPRINT` secret pins the key; a mismatched import fails the release |

## Public key

Verify a release by importing this key, then
`gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt` inside the download
directory:

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

mQGNBGsCwkMBDADMB8Ytg8QiFKPwR1m7C+yki2qFRQff5hqrZscxL2T8FLryEA/K
9eCJFTmxGLq2qqJNua7cGznAs7mE4DzTmq2Ob52s9OIQ18DZLkEFmVcXVPspW3t6
60ts1u+zFiVpKfBh9jsO8QaZ5EjukXH5bO+spW1Zo56zmHdb7sO1SUV93+Zb170O
eu3O44YPuTqTJ65dWu5H7pRd9CChTpzmeKi2lrPO1+8ULlFjT3soJo8VKDSvJNaL
WC3LMMx/ypCIq3Y7ioHktzZ1V7ibrG4SYrsKjpalJXwmx8rfHEafcxqdApgq5h8M
Fgwk3k/7+4AtNiwHPc5Ay/GxdZxTQa8oCq1Oq0pTaNNhXs5PINp7WB9CmJDOxKKH
knK51DPY7X8kVhdX3j2QDd/dVIzdEPhH0fSJyVTV3hWo1wRzpUPjLqK+kouPtpmO
r5t2175jaPIwzafQ60ndK/uSQSF7K+94M17bdxf4nKMpG1dLqOZK1aXmWU7VajMp
Udtw15tMYLauhq0AEQEAAbQgRHVzdCBUcmFpbCBEZXYgPGRldkBleGFtcGxlLmNv
bT6JAdQEEwEKAD4WIQTjxw15qKiiCWC0X9QqLgnRt7zRoQUCawLCQwIbAwUJA8Jn
AAULCQgHAgYVCgkICwIEFgIDAQIeAQIXgAAKCRAqLgnRt7zRoWK5C/wMygSt3Jf+
JJKBuFniUSEC8B0XXRRtJuLh349cvxy/3+Gxp3A9egFptFaCYn8qV3L53/i0PYjR
USv++/PTEaraMnW3hMtHjhr7WI1cn0/aw6bTTUDWAdmAvCqfeXu1Z8SMWP37BV2r
EdoCSLaPyPUe6CLNxK64Yhs4HVmR1BvXWfQBvr86bhVDI0R1UxCoZzMSDwk6cRwS
LncgU/oXphbDfsioQ3LBtv8yZcExdRNU5LwFmPPdu0sFNYM7xTTB3yJhgCkKbstr
MT/tTT/QNeGFC/3OKiDNdbPcBYW/y9VTgYGv9Ewdjl7hAf78BJo0t2INbX2C8oZn
2W8W366cFv+J3zUWxnOEzTQ+7LOS8Kzp7t0BhD0n/OHebfzLw3ZE/LZEJOGZXP8y
VdwtnrJk7R3DAEl/+/mlXSbkTdHvGh1H6ybVfnp6b8rOwQ49MTkDqfiJ0mLHr68i
TSYsxZKBlTspDyfgCZ3J9VTcDH3TM0EDygBwOcfw4sO5Yt9u8VPIllI=
=qgs8
-----END PGP PUBLIC KEY BLOCK-----
```

## Rotation history

| Date | Event |
|---|---|
| 2026-11-20 | Release v1.5.1 published with `SHA256SUMS.txt.asc` signed with key `9D07 07AF 32A5 F249 F7F8 2EA2 9829 4016 C6C9 D993` — key was provisioned as a GitHub secret before this session; its public key was **not captured locally** and is not recoverable from the repo or public keyservers. Verification of v1.5.1's signature requires that public key. |
| 2026-11-22 | Key rotated at owner direction ("create the gpg yourself"). New key `E3C7 0D79 A8A8 A209 60B4 5FD4 2A2E 09D1 B7BC D1A1` generated, UID-verified in an isolated `GNUPGHOME`, secrets provisioned (`GPG_PRIVATE_KEY`/`GPG_PASSPHRASE`/`GPG_KEY_FINGERPRINT`), local private material destroyed after isolated import→sign→verify test. Public key published below. |

## Compromise procedure (guide §7.2)

1. Generate a new key pair; publish a new app release whose bundled public
   key is the new key **before** anything further is signed with the old key.
2. Mark affected releases with a "Security" banner in the release notes and
   the in-app release-notes popup.
3. Update this file: old fingerprint, new fingerprint, reason.
4. Rotate the `GPG_PRIVATE_KEY` / `GPG_PASSPHRASE` / `GPG_KEY_FINGERPRINT`
   secrets. Never silently re-sign existing artefacts with a new key.
