# Security

Open Claude has a deliberately tiny attack surface: two hard-coded destinations, no input, no network access, no updater. If you still find a problem, please report it privately through GitHub's **Report a vulnerability** button on the Security tab of this repository, not in a public issue.

We aim to acknowledge reports within 3 working days.

## Supported versions

Only the latest release is supported. There is no auto-updater, so a fix means a new release and a new download.

## How releases are protected

- Releases are built only by the `Release` workflow, from a version tag, after a maintainer approves the protected `release` environment.
- Third-party GitHub Actions are pinned to full commit SHAs; the Rust toolchain is pinned in `rust-toolchain.toml` and dependencies in `Cargo.lock`.
- Every release publishes SHA-256 checksums and a signed build-provenance attestation.
- Maintainer accounts use two-factor authentication.
