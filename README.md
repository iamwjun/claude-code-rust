# Claude Code Rust

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/iamwjun/claude-code-rust/main/install.sh | bash
```

Supports macOS (arm64 / x86_64) and Linux (x86_64 / aarch64). The script detects your platform automatically, downloads the latest release from GitHub, and installs the binary to `/usr/local/bin`.

## REPL Commands

| Command    | Description                           |
|------------|---------------------------------------|
| `/help`    | Show available commands               |
| `/clear`   | Clear conversation history            |
| `/history` | View conversation history (JSON)      |
| `/count`   | Show number of messages in history    |
| `/exit`    | Exit the program                      |
