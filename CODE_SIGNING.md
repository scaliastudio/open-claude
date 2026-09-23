# Code signing

Open Claude releases are not code-signed yet. Windows therefore shows a SmartScreen warning ("Windows protected your PC") before the installer runs, and on PCs with Smart App Control turned on it does not run at all. For those PCs, the web app at https://scaliastudio.dev/products/openclaude/app does the same job with nothing to install.

Until releases are signed, this is what stands behind a download:

- Every release is built by this repository's `Release` workflow on GitHub Actions, from a version tag, after a maintainer approves the protected `release` environment. Nothing is built or uploaded from a developer's machine.
- Each release publishes SHA-256 checksums (`SHA256SUMS.txt`) and a GitHub build-provenance attestation. Check one with `gh attestation verify OpenClaude-Setup.exe --repo scaliastudio/open-claude`, or compare `Get-FileHash` against the checksum file.
- The program is about 70 lines in [`src/main.rs`](src/main.rs), with one dependency, Microsoft's `windows-sys`.

## Maintainers

[Matteomio16](https://github.com/Matteomio16) commits, reviews pull requests and approves releases.

## Privacy

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

It asks the operating system to open `claude://` or `https://claude.ai`; whatever happens after that is between you and Anthropic.
