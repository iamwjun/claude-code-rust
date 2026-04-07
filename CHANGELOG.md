# Changelog

## v1.0.0 - 2026-04-07
- Built from commit 2632544


## Initial Implementation

### Core Features

- **Interactive REPL** — persistent multi-turn conversation with Claude via `rustyline`
- **Single-shot query mode** — pass a prompt as CLI argument for one-off queries
- **Streaming responses** — real-time output via Server-Sent Events (SSE) parsing
- **Conversation history** — full message history sent with each request for context retention

### REPL Commands

| Command    | Description                        |
|------------|------------------------------------|
| `/help`    | Show available commands            |
| `/clear`   | Clear conversation history         |
| `/history` | Print conversation history as JSON |
| `/count`   | Show number of messages in history |
| `/exit`    | Exit the program                   |

### API Client (`src/api.rs`)

- `ClaudeClient::new` — initializes client with configurable base URL, model, and max tokens
- `query_streaming` — streams response chunks to stdout in real time
- `query` — non-streaming variant, returns full `ApiResponse`
- `set_model` / `set_max_tokens` — runtime configuration

### Unit Tests (`cargo test`)

26 tests across three modules:

| Module       | Tests | What's covered |
|--------------|-------|----------------|
| `types`      | 12    | `Message` constructors, `ConversationHistory` all methods, JSON roundtrip |
| `api`        | 4     | Default values on `ClaudeClient::new`, `set_model`, `set_max_tokens` |
| `repl`       | 10    | All `handle_command` branches, case-insensitivity |

### Examples (`examples/`)

| File              | Demonstrates |
|-------------------|-------------|
| `basic.rs`        | Simple non-streaming API call |
| `streaming.rs`    | SSE streaming response |
| `conversation.rs` | Multi-turn conversation with context |
| `custom_model.rs` | Different models and temperature settings |
| `error_handling.rs` | Invalid API key, empty message, token limit validation |
