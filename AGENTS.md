# Repository Guidelines

## Project Structure & Module Organization

This repository is a Cargo workspace with one or more crates under `crates/`. Each crate carries its own `README.md`,
`Cargo.toml`, and `src/` tree. Shared metadata (`edition`, `rust-version`, `authors`, `license`, `repository`,
`keywords`), lints, and shared third-party dependencies live in the root `Cargo.toml` under `[workspace.package]`,
`[workspace.lints]`, and `[workspace.dependencies]`; per-crate manifests inherit via `.workspace = true` and add only
what is crate-specific (description, features, dependency selection). New crates go in `crates/<name>/`, are picked up
automatically by the `crates/*` workspace glob, and need `[lints] workspace = true`.

LICENSE files live at the repository root; each crate symlinks them so they are packaged with the crate.

Integration tests for a crate live under that crate's `tests/` directory, with fixtures next to the tests that load
them.

## Build, Test, and Development Commands

- `cargo install-tools`: installs the CI toolchain from `.cargo/config.toml` (`cargo-deny`, `cargo-llvm-cov`,
  `cargo-nextest`, `cargo-action-fmt`).
- `cargo build --workspace` builds every crate with default features; pass `-p <crate>` to build one.
- `cargo nextest run --workspace --all-features` runs all tests; `cargo test --doc --workspace` runs doctests, which
  nextest skips.
- `cargo fmt --all` applies standard Rust formatting across the whole workspace.
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`: lints every crate with warnings as errors.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps` checks the docs.
- `cargo deny check` audits licenses, advisories, bans, and sources using `deny.toml`.
- Unused dependencies fail the build through cargo's `unused_dependencies` lint (CI denies warnings through
  `build.warnings`). It only checks the features being built, so build each feature alone
  (`--no-default-features --features <f>`) after touching a crate's dependencies.

## Coding Style & Naming Conventions

Use Rust 2024 idioms and standard `rustfmt` formatting. Prefer descriptive snake_case for modules, functions, and
tests; `UpperCamelCase` for public types. Prefer direct sibling modules (e.g. `host_impl.rs`) over `mod.rs`.

Keep crate boundaries clean: each crate's public API lives at its crate root. Prefer `pub(crate)` for new items unless
external callers need them. Cross-crate code reuse goes through a dependency, not path-only links into another crate's
internals.

When adding a new dependency, declare it once in the root `[workspace.dependencies]` and reference it via
`.workspace = true` in the consuming crate so versions stay in lockstep.

## Testing Guidelines

Tests should be runnable with `cargo nextest run -p <name>` for fast iteration, and pass under
`cargo nextest run --workspace --all-features` before merge.

Aim for ~95% line coverage on changed code; treat ~90% as the floor. Codecov statuses on PRs are informational; the
`All checks` job is the merge gate.

## Commit & Pull Request Guidelines

**Never commit to `main` directly.** Always create a branch, push it, and open a pull request. `main` is updated only
by merging PRs.

**Branch names** mirror the commit-prefix vocabulary: `feat/<slug>`, `refactor/<slug>`, `fix/<slug>`, `docs/<slug>`,
`test/<slug>`, `chore/<slug>`, `ci/<slug>`.

**Commits** use concise, scoped prefixes such as `<scope>:` where `<scope>` names the affected area (`docs`, `ci`,
`test`, a crate name, etc.), followed by an imperative summary. **Always pass `-s`** so each commit carries a
`Signed-off-by` trailer. Do not use `Co-authored-by` for agent assistance; use `Assisted-by` instead.

**Pull request descriptions** cover what changed and why, call out breaking changes for downstream consumers, and link
related issues. Do not include `## Test plan` or similar verification sections; CI is the source of truth.

## Releasing

Bump the crate's `version`, merge, then push a `<crate>/v<x.y.z>` tag. `release-stage.yaml` runs all checks and creates
a draft GitHub Release; publishing the draft triggers `publish.yaml`, which publishes the crate with crates.io trusted
publishing. Multiple crates are released one tag at a time, in dependency order. A crate's first version must be
published by hand.

## Security & Configuration Tips

Do not commit credentials or local machine paths. Keep dependency and license changes intentional, and run
`cargo deny check` when adding, upgrading, or removing dependencies.
