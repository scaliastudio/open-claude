<p align="center"><img src="assets/icon-512.png" width="128" alt="Open Claude icon"></p>

<h1 align="center">Open Claude</h1>
<p align="center">It opens Claude.</p>

---

Open Claude is a parody product by [Scalia Studio](https://scaliastudio.dev/products/openclaude). You click it, and it opens Claude: the Claude desktop app if you have it, otherwise [claude.ai](https://claude.ai) in your browser. That is the whole product.

**Not affiliated with, endorsed by or sponsored by Anthropic.** Claude is a trademark of Anthropic, PBC.

## Download

- **Windows:** the installer is on the [Releases](https://github.com/scalia-studio/open-claude/releases) page.
- **Mac:** no download. Open [scaliastudio.dev/products/openclaude](https://scaliastudio.dev/products/openclaude) in Safari and choose **File → Add to Dock**.

## What it does, exactly

The whole program is [`src/main.rs`](src/main.rs). It:

1. checks whether the `claude:` link type is registered on this PC (Claude Desktop registers it),
2. asks Windows to open `claude://` if it is, or `https://claude.ai` if it is not,
3. exits.

It does not:

- read arguments, files or settings, so a shortcut cannot point it anywhere else
- connect to the internet itself, collect data, or phone home
- run in the background, start with Windows, or update itself
- need administrator rights. The installer puts it in your user profile and removes it cleanly from **Settings → Apps**.

## Verify a download

Every release is built by GitHub Actions from a tagged commit in this repository, never on a developer's machine. Each release lists SHA-256 checksums and carries a signed build attestation:

```
gh attestation verify OpenClaude-Setup-1.0.0.exe --repo scalia-studio/open-claude
```

## Build it yourself

Requires Rust (the version is pinned in `rust-toolchain.toml`) and, for the installer, [Inno Setup 6](https://jrsoftware.org/isinfo.php).

```
cargo build --release --locked
iscc /DAppVersion=1.0.0 installer/open-claude.iss
```

## Security

See [SECURITY.md](SECURITY.md) to report a problem, and [CODE_SIGNING.md](CODE_SIGNING.md) for how releases are signed.

## License

[MIT](LICENSE)
