# Release signing & update verification (Dust Trail)

How release artefacts are signed, which GitHub secrets the release pipeline
requires, how the signing key's fingerprint is pinned and verified, and how a
key is rotated. The release workflow (`.github/workflows/release.yml`) enforces
everything described here; this document is the human-readable record.

## Current status (honest)

- **No GPG signing key is provisioned.** No fingerprint is listed below
  because none exists yet — a fingerprint is only recorded here after a real
  key has been generated and its public half published. Do not invent or
  pre-fill one.
- Because signing is a hard gate, **every release run currently fails at the
  signing step by design** (`Sign SHA256SUMS.txt`) until the two required
  secrets below are configured. This is intentional: the pipeline must never
  silently publish an unsigned release.
- **In-app signature verification is not implemented yet.** The
  `crates/update-check` crate currently performs version checks only; it does
  not verify GPG signatures. The `.asc` signature shipped with each release is
  verifiable manually (below) and exists so that in-app enforcement can be
  wired in later without changing the release format. Until then, users should
  verify checksums and the signature themselves.

## Required GitHub secrets

| Secret name | Required | Purpose |
|-------------|----------|---------|
| `GPG_PRIVATE_KEY` | Yes | The **ASCII-armored private** signing key — the complete `-----BEGIN PGP PRIVATE KEY BLOCK-----` … `-----END PGP PRIVATE KEY BLOCK-----` text. The key must be passphrase-protected; the workflow refuses an empty passphrase. |
| `GPG_PASSPHRASE` | Yes | The passphrase protecting `GPG_PRIVATE_KEY`. |
| `GPG_KEY_FINGERPRINT` | Before the next release after provisioning | The 40-character primary-key fingerprint of the signing key (spaces stripped is fine). When set, the workflow refuses to sign with any other key. |

Note: this replaces the secret name `RELEASE_SIGNING_KEY` recorded in an
earlier revision of `DECISIONS.md`; the recorded decision to use **GPG
detached signatures (`SHA256SUMS.txt.asc`)** is unchanged.

## Key hygiene requirements

- Generate the key **offline**, on a trusted machine, with a strong
  passphrase. RSA 4096 or Ed25519, sign-only capability is sufficient.
- The key lives **only** in the GitHub secret. Never commit it, never paste it
  into an issue, PR, log, or chat, never store it in the repository or a build
  artefact.
- The workflow imports the key into a temporary `GNUPGHOME` created with
  `mktemp -d` **outside** the workspace, passes the passphrase on file
  descriptor 0 (never as a command-line argument, never echoed), and deletes
  the temporary home when the step ends. GitHub additionally masks secret
  values in workflow logs. Key material therefore never appears in logs,
  artefacts, or the repository.
- Publishing the **public** key and its fingerprint is expected and safe; they
  belong in this document and the release notes.

## How the release workflow signs

1. The four installers are downloaded and checked against the exact names
   (`dust-trail-windows-x86_64.msi`, `dust-trail-macos-arm64.dmg`,
   `dust-trail-linux-x86_64.deb`, `dust-trail-linux-arm64.deb`); any missing
   or unexpected installer fails the release.
2. `SHA256SUMS.txt` is generated with checksums for **the four installers
   only** (bare filenames, so `sha256sum -c` works beside the downloads).
3. `GPG_PRIVATE_KEY` is imported into the temporary `GNUPGHOME`.
4. If `GPG_KEY_FINGERPRINT` is set, the imported key's fingerprint must match
   it exactly or the workflow refuses to sign.
5. The key signs `SHA256SUMS.txt` with a **detached, armored** signature to
   `SHA256SUMS.txt.asc` (passphrase supplied via fd 0).
6. The signature is verified with the imported public key; a bad signature
   fails the release.
7. Every asset (installers, sums, signature, SBOM) is scanned for personal
   `/home` paths (CI's own `/home/runner` allowed) and secret-assignment
   patterns before publication; hits fail the release and only the file name
   is reported.
8. The release is published or updated **non-draft**, attaching: the four
   installers, `SHA256SUMS.txt`, `SHA256SUMS.txt.asc`, and
   `dust-trail-sbom.cdx.json`.

`dust-trail-sbom.cdx.json` is a CycloneDX SBOM (JSON) generated with
`cargo-cyclonedx` **0.5.10** (`cargo install cargo-cyclonedx --locked
--version 0.5.10`; `cargo cyclonedx --all-features --target all --format json
--override-filename dust-trail-sbom.cdx` — the override sets the explicit
output filename). It describes the application package and embeds the
`crash-report` and `update-check` workspace crates as components.

## Verifying a release (users and reviewers)

```bash
# 1. Checksums (run in the directory holding the downloads)
sha256sum -c SHA256SUMS.txt

# 2. Signature: import the public key published with the release
#    (or from docs/update-signing.md once provisioned), then:
gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt
#    Expect: "Good signature from ..." and no fingerprint mismatch.
```

## Pinned public key / fingerprint verification process

1. Generate the signing key and export the armored public key:
   `gpg --armor --export <FINGERPRINT> > dust-trail-release.pub`.
2. Publish the public key and its fingerprint: store the public key in the
   release notes of the release that first carries signatures **and** record
   the fingerprint in this document (section below).
3. Pin the fingerprint in the `GPG_KEY_FINGERPRINT` repository secret. From
   then on the workflow asserts the imported key's fingerprint equals the
   pinned value before every signature — a wrong or swapped key fails the
   release with the mismatch printed.
4. Reviewers verifying a release compare the signature's key fingerprint
   (`gpg --verify` output) against the fingerprint recorded here. They must
   match; a mismatch means the artefacts were not signed by this project.

## Signed-key fingerprint record

No key is currently provisioned — no fingerprint is recorded yet.

After provisioning, this section records: the fingerprint, the date it was
first used, and where the armored public key is published. This document and
the release notes are the authoritative pin.

## Key rotation procedure

Routine rotation:

1. Generate a new passphrase-protected key offline.
2. Update `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` secrets to the new key, and
   update the `GPG_KEY_FINGERPRINT` pin to the new fingerprint.
3. Publish the new armored public key with the next release's notes, and
   update this document (old fingerprint → new fingerprint, with the date and
   reason).
4. Cut a normal release: every artefact after that point is signed with the
   new key, and the release notes state that the signing key rotated.

Compromised (or suspected compromised) key:

1. Stop using the key immediately — update the secrets as above so no further
   artefacts are signed with the old key.
2. Publish a new app release whose release notes carry the **new public key
   and fingerprint before any further artefacts are signed with the old key**,
   with a clear "Security" banner, per the workspace key-compromise procedure.
3. Record the incident in `CHANGELOG.md`/`SECURITY.md` (describe the event,
   never reproduce key material) and update this document with the old and new
   fingerprints plus the reason.
4. Never silently re-sign existing artefacts with the new key — re-signed
   files must be published as a new release so users can tell the difference.
   Artefacts already published remain verifiable against the old public key.

## Open items (remaining release blockers)

1. `GPG_PRIVATE_KEY` / `GPG_PASSPHRASE` secrets are not provisioned, so the
   release pipeline fails at the signing step by design. Provision them (and
   pin `GPG_KEY_FINGERPRINT`) before the next tag.
2. In-app updater verification of `SHA256SUMS.txt.asc` is not implemented in
   `crates/update-check` (tracked as future work; the release-side signature
   format above is the contract it will consume).
3. `DECISIONS.md` still names the retired secret `RELEASE_SIGNING_KEY`;
   update the record to the secret names above.