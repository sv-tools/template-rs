# template-rs

[![Rust](https://github.com/sv-tools/template-rs/actions/workflows/checks.yaml/badge.svg)](https://github.com/sv-tools/template-rs/actions/workflows/checks.yaml)
[![codecov](https://codecov.io/gh/sv-tools/template-rs/graph/badge.svg)](https://codecov.io/gh/sv-tools/template-rs)

A template for Rust workspaces: crates live under `crates/`, CI runs builds, tests, lints, docs, dependency and
vulnerability checks, and releases publish to crates.io with trusted publishing.

Replace it with real readme content.

## Releasing

1. Bump the crate's `version` in its `Cargo.toml` and merge the PR. The `Test publishing bumped crates` job lists the
   tags to push, in dependency order.
2. Push the tag, e.g. `git tag project/v0.1.0 && git push origin project/v0.1.0`. `release-stage.yaml` runs all checks
   and creates a draft GitHub Release.
3. Review and publish the draft. `publish.yaml` publishes the crate to crates.io. It can be re-run for a tag by hand.

Trusted publishing cannot create a crate, so publish a crate's first version by hand with `cargo publish`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
