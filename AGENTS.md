# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust 2024 binary crate for a GTK4 layer-shell UI named `niri-island`.

- `src/main.rs` contains the application entry point, GTK setup, embedded CSS, and layer-shell window behavior.
- `Cargo.toml` and `Cargo.lock` define Rust package metadata and locked dependencies.
- `flake.nix` and `flake.lock` define the Nix development shell and reproducible package build.
- Build output lives in `target/`; Nix output may appear as `result`. Do not commit either.

There is no separate test or assets directory yet. Add unit tests near the code they exercise, or introduce `tests/` for integration tests.

## Build, Test, and Development Commands

- `nix develop` enters the shell with Rust, rust-analyzer, pkg-config, GTK4, and gtk4-layer-shell.
- `cargo build` compiles the local debug binary.
- `cargo run` launches the GTK application; use a graphical Wayland session with layer-shell support.
- `cargo test` runs Rust unit and integration tests.
- `cargo fmt` formats Rust code with rustfmt.
- `cargo clippy -- -D warnings` checks for lints and treats warnings as failures.
- `nix build` builds the package through the flake/crane pipeline.

## Coding Style & Naming Conventions

Use standard rustfmt output: 4-space indentation, idiomatic imports, and concise functions. Keep constants in `SCREAMING_SNAKE_CASE` (`APP_ID`, `NAMESPACE`) and functions/modules in `snake_case`. Prefer GTK object names that match CSS selectors, for example `icon` for `#icon`.

Avoid broad refactors while changing UI behavior. Keep CSS selectors, GTK construction, and layer-shell configuration easy to trace from `activate()`.

## Testing Guidelines

Run `cargo test` before submitting changes. For pure logic, add unit tests in the same file under `#[cfg(test)] mod tests`. For executable-level behavior, use integration tests under `tests/`. Smoke-test GUI changes with `cargo run` in the target compositor environment.

## Commit & Pull Request Guidelines

The current history uses short, imperative commit messages with a scope-like prefix, for example `init:repo init create`. Keep messages concise, such as `ui: adjust island hover state`.

Pull requests should describe the visible or behavioral change, list commands run (`cargo test`, `cargo clippy`, `nix build`), and include screenshots or recordings for UI changes. Link issues when applicable and mention compositor assumptions.

## Security & Configuration Tips

Do not commit local shell state such as `.direnv/`, generated build outputs, or machine-specific symlinks. Keep dependency updates intentional and include matching `Cargo.lock` or `flake.lock` changes.
