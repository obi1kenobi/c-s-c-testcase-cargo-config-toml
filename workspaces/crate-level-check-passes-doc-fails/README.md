# Crate-level config

There's a `.cargo/config.toml` file inside the `test-pkg` crate directory
setting `--cfg tokio_unstable` in `RUSTFLAGS` which is required to compile the test crate here.

However, `cargo doc` uses `RUSTDOCFLAGS`, and `--cfg tokio_unstable` is not set in the config.
`--cfg tokio_unstable` is required at top level, not only inside a method body, so rustdoc
cannot ignore this and will fail when `--cfg tokio_unstable` is not in `RUSTDOCFLAGS`.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level *does not work*.
- `cargo check --manifest-path <path>` only works from inside the `test-pkg` directory,
  even if pointing to the package or workspace-level `Cargo.toml` file.
- `cargo doc` fails from any path, since the config doesn't set `RUSTDOCFLAGS`.
