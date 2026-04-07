# Changelog

<!-- last-commit: 55672a2 -->

## v1.0.1 - 2026-04-07

| Commit | Description |
|--------|-------------|
| `55672a2` | update CHANGELOG.md |
| `26ad6bf` | feat: version command |
| `43d71ad` | update CHANGELOG.md |
| `44dd90e` | chore: optimize gitflow |
| `56c9274` | chore: optimize CHANGELOG.md update |


## v1.0.0 - 2026-04-07

| Commit | Description |
|--------|-------------|
| `5fe9ae8` | chore: update CHANGELOG for v1.0.0 [skip ci] |
| `2632544` | feat: support bash install (#1) |
| `2c6dd48` | ci: add GitHub Actions release workflow |
| `75df915` | docs: add CHANGELOG.md |
| `3720a04` | test: unit tests |
| `888c7a9` | feat: first version |
| `ccb2483` | Initial commit |

### Features

- **Interactive REPL** — persistent multi-turn conversation with Claude via `rustyline`
- **Single-shot query mode** — pass a prompt as CLI argument for one-off queries
- **Streaming responses** — real-time output via Server-Sent Events (SSE) parsing
- **Conversation history** — full message history sent with each request for context retention
- **Bash install script** — one-line install from GitHub Releases with platform auto-detection
- **GitHub Actions release workflow** — auto tag, build, package, and publish on push to main

### Unit Tests

26 tests across three modules:

| Module  | Tests | What's covered |
|---------|-------|----------------|
| `types` | 12    | `Message` constructors, `ConversationHistory` all methods, JSON roundtrip |
| `api`   | 4     | Default values on `ClaudeClient::new`, `set_model`, `set_max_tokens` |
| `repl`  | 10    | All `handle_command` branches, case-insensitivity |
