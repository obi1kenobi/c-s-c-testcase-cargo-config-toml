# Crate-level config

There's a `.cargo/config.toml` file inside the `test-pkg` crate directory
setting `--cfg tokio_unstable` in both `RUSTFLAGS` and `RUSTDOCFLAGS`,
which are required to `cargo check` and `cargo doc` the crate respectively.

- Both `cargo check` and `cargo doc` from inside `test-pkg` works.
- `cargo check` and `cargo doc` at workspace level *do not work*.
- `cargo check --manifest-path <path>` and `cargo doc --manifest-path <path>` only work
  from inside the `test-pkg` directory, regardless of whether they point to the workspace manfiest
  or the package's own manifest `Cargo.toml` file.
