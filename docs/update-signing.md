# Update Signing Configuration

## Current State

**No GPG signing key is provisioned.** The release signing key must be stored in the GitHub secret `RELEASE_SIGNING_KEY`. Signature verification is not functional until this secret is configured.

## Required Secret

| Secret Name | Purpose |
|-------------|---------|
| `RELEASE_SIGNING_KEY` | GPG private key for signing release artifacts |

## How Signing Works

1. When a release tag is created, the release workflow:
   - Downloads the artifact archive (`.tar.gz`)
   - Generates `SHA256SUMS.txt` with bare filenames
   - Generates a CycloneDX SBOM
   - Signs `SHA256SUMS.txt` with a detached GPG signature (`SHA256SUMS.txt.asc`)
   - Attaches `.msi`, `.dmg`, `.deb` (x86_64 and arm64), `SHA256SUMS.txt`, `SHA256SUMS.txt.asc`, and the SBOM to the GitHub Release

2. The public key fingerprint is compiled into the `crates/update-check` crate at build time.

3. The in-app updater verifies signatures before installing.

## Key Fingerprint (provisioned)

No key is currently provisioned. Once a key is added to the repository secrets, the fingerprint will be documented here.

## Provisioned Key Rotation

Any rotation of the signing key requires:
1. Publishing a new app release with the updated public key
2. A security advisory if the key was compromised
3. Updating this document with the old and new fingerprints

## Development Builds

Development/integration builds (e.g., from CI branches) are **not signed**. Only release tags trigger signing.