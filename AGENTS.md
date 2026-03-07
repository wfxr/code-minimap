# Repository Guidelines

## Project Structure & Module Organization
`code-minimap` is a small Rust workspace with one library crate and one CLI binary. Core rendering logic lives in `src/core.rs`; public exports are re-exported from `src/lib.rs`; lossy UTF-8 input handling lives in `src/lossy_reader.rs`. The executable entrypoint is `src/bin/code-minimap/main.rs`, and CLI argument definitions live in `src/bin/code-minimap/cli.rs`. Example programs are under `examples/`, and generated shell completion scripts are checked in under `completions/`.

## Build, Test, and Development Commands
- `cargo build` — build the library and default CLI feature set.
- `cargo run -- src/core.rs -H 0.6 -V 0.5` — run the CLI against a sample file.
- `cargo test` — run unit tests, including `rstest` cases in `src/core.rs`.
- `cargo fmt --all` — format the codebase with Rustfmt.
- `cargo clippy --all-targets --all-features` — catch lint issues before opening a PR.
- `SHELL_COMPLETIONS_DIR=completions cargo build` — regenerate completion files when CLI flags change.

## Coding Style & Naming Conventions
Follow idiomatic Rust and keep changes focused. Use Rustfmt defaults (4-space indentation, trailing commas where appropriate) and keep module APIs small and explicit. Prefer `snake_case` for functions, variables, and modules; `PascalCase` for types and enums; and descriptive flag names for CLI options. Match the existing pattern of colocating tests with the code they cover.

## Testing Guidelines
This repository uses Rust’s built-in test harness with `rstest` for table-driven cases. Add new tests next to the affected module under `#[cfg(test)]`, and name them after observable behavior, for example `test_write_to_string`. Cover both library behavior and CLI-facing edge cases when changing parsing, scaling, encoding, or padding logic.

## Commit & Pull Request Guidelines
Recent history follows Conventional Commit style: `feat:`, `fix:`, `docs:`, `ci:`, and `chore(deps):`. Keep commit subjects imperative and under ~72 characters. PRs should explain the user-visible change, list validation commands run, and link related issues when applicable. Include before/after terminal output or `--help` screenshots when changing CLI behavior or rendered minimap output.

## Release & Contributor Notes
Avoid broad refactors unless they directly support the change. If you touch CLI options, update completion scripts and any affected `README.md` examples in the same PR.
