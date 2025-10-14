# Crate-level config

There's a `.cargo/config.toml` file inside the `test-pkg` crate directory
setting `--cfg tokio_unstable` in both `RUSTFLAGS` and `RUSTDOCFLAGS`,
which are required to `cargo check` and `cargo doc` the crate respectively.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level *does not work*.
- `cargo check --manifest-path <path>` only works from inside the `test-pkg` directory,
  regardless of whether it points to the workspace manfiest
  or the package's own manifest `Cargo.toml` file.
- `cargo doc` behaves the same way.
