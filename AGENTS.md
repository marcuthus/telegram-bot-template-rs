# AGENTS.md - Telegram Bot Template (Rust)

## Build/Lint/Test Commands

### Development

- `cargo run` - Run bot in development mode
- `cargo build` - Compile in debug mode
- `cargo build --release` - Compile optimized binary to `./target/release/`
- `RUST_LOG=debug cargo run` - Run with verbose console logs

### Linting & Formatting

- `cargo fmt` - Format code (config in `rustfmt.toml`)
- `cargo fmt --check` - Check formatting without changing files
- `cargo clippy --all-targets -- -D warnings` - Lint (warnings fail CI)

### Testing

- `cargo test` - Run all tests
- `cargo test <name>` - Run a single test by name

Unit tests live next to the code in `#[cfg(test)] mod tests`.

## Code Style Guidelines

### Imports

Order (separated by blank lines, `rustfmt` keeps each group sorted):

1. `std`
2. External crates (`teloxide`, `tracing`, ...)
3. Internal modules (`crate::...`)

Use `teloxide::prelude::*` for the common teloxide traits and types.

### Naming Conventions

- **Files/Modules/Functions/Variables**: `snake_case` (e.g., `uptime.rs`, `require_bot_owner`)
- **Types/Enums/Traits**: `PascalCase` (e.g., `Config`, `Data`, `Command`)
- **Constants**: `UPPER_SNAKE_CASE`
- Telegram command names must be 1-32 lowercase letters, digits or underscores (`rename_rule = "lowercase"`)

### Comments

- **Never write comments in code**. Code must be 100% self-explanatory through clean structure and descriptive names.

### Error Handling

- Command and event functions return `Result<(), crate::data::Error>`; use `?` to propagate.
- Startup code returns `anyhow::Result`; config errors use `thiserror` (`ConfigError`).
- Avoid `unwrap()`/`expect()` outside tests. Never `panic!` in handlers.
- The global handler in `src/error.rs` logs the error and sends a friendly message to the user; `CommandError::PermissionDenied` produces the permission message.

## Telegram Bot Patterns

**Commands:**

- Declare each command as a variant of the `Command` enum in `src/commands/mod.rs` (`#[derive(BotCommands)]`) with its `description`; the enum also feeds the Telegram command menu.
- Arguments are the variant fields and are parsed by teloxide (`parse_with = "split"` for more than one field); parse errors are reported to the user automatically.
- Place the implementation in `src/commands/<category>/<command_name>.rs` exposing `pub async fn run(ctx: &Context, ...)`, and dispatch it in the `match` of `commands::run`.
- Restrict to the bot owner with `require_bot_owner(ctx)?` (`src/utils/checks.rs`).
- Replies use HTML parse mode; escape user input with `teloxide::utils::html::escape`.

**Events:**

- Place events in `src/events/<event_name>.rs` exposing `pub async fn run(...)`.
- Register update branches in the `dptree` handler built in `src/client.rs`.

**Client:**

- `client::start(config)` creates the bot, synchronizes the commands with Telegram (`set_my_commands`) and runs the teloxide `Dispatcher` (Ctrl+C shuts it down).
- Shared state goes in `data::Data` (available as `ctx.data`); use `Arc`/atomics/`Mutex` for mutable state.
- `data::Context` bundles the `Bot`, the incoming `Message` and the `Data`, and provides `reply`/`edit` helpers.

## Configuration

- Environment variables loaded via `dotenvy` and validated by `Config::from_env()` (`src/helpers/config.rs`).
- Required env vars: `TELEGRAM_BOT_TOKEN`, `TELEGRAM_ADMIN_ID`.

## Logging

- Use `tracing` macros with a scope as target: `info!(target: "client", "...")`, `error!`, `warn!`, `debug!`.
- Logs go to console and to `logs/combined.log` / `logs/error.log` (see `src/helpers/logger.rs`).
- Keep the `WorkerGuard`s returned by `logger::init()` alive in `main`.

## Docker

- `Dockerfile` is multi-stage (`cargo build --release` with a cached dependency layer, runtime on `debian:bookworm-slim`) and runs as an unprivileged user.
- `docker-compose.yml` loads `.env` through `env_file` and keeps `logs/` in named volumes.
- `.dockerignore` keeps `.env`, `logs/`, `auth/`, `.git` and build output out of the image. Never copy secrets into the image.
- Build and run: `docker compose up --build`.
