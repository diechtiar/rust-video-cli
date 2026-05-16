# rust-video-cli
Production-quality Rust CLI for website login flows, video player detection, stream interception, and ffmpeg downloads. Built iteratively as a modern Rust learning project following best practices (clap, thiserror, tracing, tests, clippy).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 Wojciech Diechtiar

## Development Setup (Windows + WSL + VS Code)

This project is developed inside **WSL2** (Ubuntu) for Linux compatibility.

### 1. Prerequisites

- Windows 11
- [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) with Ubuntu
- [Visual Studio Code](https://code.visualstudio.com/)
- [Remote - WSL](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) extension

### 2. Setup Steps

```bash
# 1. Install WSL2 (run in PowerShell as Administrator)
wsl --install

# 2. Open Ubuntu and install build tools
sudo apt update && sudo apt install build-essential -y

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# 4. Clone the project
git clone https://github.com/diechtiar/rust-video-cli.git
cd rust-video-cli

# 5. Open in VS Code from WSL
code .
```

### 3. Recommended VS Code Settings

This goes into `.vscode/settings.json` in the project directory:

```JSON
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.checkOnSave": true
}
```

### 4. Useful commands
```bash
cargo check
cargo fmt
cargo clippy
cargo test
cargo run -- --help

```