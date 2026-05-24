# rust-video-cli

A production-quality Rust CLI for handling website login flows, detecting video players, intercepting streams, and downloading videos using ffmpeg.

This project is being built **iteratively** as a learning exercise, following modern Rust best practices (clap, thiserror, tracing, clean architecture, testing, and clippy).

## Current Features

- Clean command-line interface powered by `clap`
- Reusable `HttpClient` module featuring:
  - Automatic cookie/session handling
  - Custom default headers support
  - `GET` and `POST` methods
  - Proxy support (for debugging with mitmproxy)
- Structured logging using `tracing`
- Proper error handling with `thiserror`
- Modular and testable project structure

## Usage

```bash
# Show help
cargo run -- --help

# Run with debug output
cargo run -- --debug
```

## Development Setup (Windows + WSL + VS Code)

This project is developed inside **WSL2** (Ubuntu) for Linux compatibility.

### Prerequisites

- Windows 11
- [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) with Ubuntu installed
- [Visual Studio Code](https://code.visualstudio.com/)
- [Remote - WSL](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) extension

### Setup Steps

```bash
# 1. Install WSL2 (run in PowerShell as Administrator)
wsl --install

# 2. Open Ubuntu and install required build tools
sudo apt update && sudo apt install build-essential -y

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# 4. Clone the project
git clone https://github.com/diechtiar/rust-video-cli.git
cd rust-video-cli

# 5. Open the project in VS Code (from WSL)
code .
```

### Recommended VS Code Settings

Create `.vscode/settings.json`:

```json
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.checkOnSave": true
}
```

### Useful Commands

```bash
cargo check
cargo fmt
cargo clippy
cargo test
cargo run -- --help
cargo run -- --debug
```

### Notes

**Note on Browser Automation**: When using browser-based features, Chrome will be automatically downloaded into the `.chrome` folder if not present on the system. You can run with a visible browser window during development for easier debugging.

## Debugging & Traffic Inspection

You can route all HTTP/HTTPS traffic through **mitmproxy** for inspection:

```bash
HTTP_PROXY=http://127.0.0.1:8080 HTTPS_PROXY=http://127.0.0.1:8080 cargo run -- --debug
```

> Note: You may need to run mitmproxy with `.danger_accept_invalid_certs(true)` temporarily during development.

## Browser Automation

For websites protected by Cloudflare or that rely heavily on JavaScript (such as login flows), this project uses **browser automation** powered by `chromiumoxide`.

- Automatically downloads and manages a Chrome instance if none is found
- Enables realistic login flows that can handle CSRF tokens and JavaScript-rendered pages
- Supports both headed mode (visible browser window) for debugging and headless mode
- Currently used as the primary method for authentication flows

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 Wojciech Diechtiar
