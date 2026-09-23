<p align="center"><img src="assets/icon-512.png" width="128" alt="Open Claude icon"></p>

<h1 align="center">Open Claude</h1>
<p align="center">It opens Claude.</p>

---

Open Claude is a parody product by [Scalia Studio](https://scaliastudio.dev/products/openclaude). You click it, and it opens Claude: the Claude desktop app if you have it, otherwise [claude.ai](https://claude.ai) in your browser. That is the whole product.

**Not affiliated with, endorsed by or sponsored by Anthropic.** Claude is a trademark of Anthropic, PBC.

## Download

- **Windows 10 and 11:** download `OpenClaude-Setup.exe` from the [latest release](https://github.com/scaliastudio/open-claude/releases/latest).
- **Mac:** there is nothing to download. Open [scaliastudio.dev/products/openclaude/app](https://scaliastudio.dev/products/openclaude/app) in Safari and choose **File → Add to Dock**.

### Windows will warn you first

Open Claude is not code-signed yet, so Windows cannot tell who made it:

- Your browser may say the file isn't commonly downloaded. Choose **Keep**.
- When you run the installer, Windows shows **Windows protected your PC**. Click **More info**, then **Run anyway**.
- If your PC has **Smart App Control** turned on, Windows won't run unsigned apps at all, and there is no button to allow this one. Use the web app instead: open [scaliastudio.dev/products/openclaude/app](https://scaliastudio.dev/products/openclaude/app) in Edge or Chrome and choose **Install**.

Before you click **Run anyway**, you can check that the file is the one GitHub built: see [Verify a download](#verify-a-download).

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

Every release is built by GitHub Actions from a tagged commit in this repository, never on a developer's machine, and lists SHA-256 checksums in `SHA256SUMS.txt`.

In PowerShell, in the folder you downloaded to:

```
Get-FileHash .\OpenClaude-Setup.exe -Algorithm SHA256
```

The hash should match the line for `OpenClaude-Setup.exe` in the release's `SHA256SUMS.txt`. With the GitHub CLI you can also check the signed build record, which proves the file came from this repository's workflow:

```
gh attestation verify OpenClaude-Setup.exe --repo scaliastudio/open-claude
```

## Build it yourself

Requires Rust (the version is pinned in `rust-toolchain.toml`) and, for the installer, [Inno Setup 6](https://jrsoftware.org/isinfo.php).

```
cargo build --release --locked
iscc /DAppVersion=1.0.0 installer/open-claude.iss
```

## Security

See [SECURITY.md](SECURITY.md) to report a problem, and [CODE_SIGNING.md](CODE_SIGNING.md) for why releases are not signed yet and how they are protected instead.

## License

[MIT](LICENSE)
