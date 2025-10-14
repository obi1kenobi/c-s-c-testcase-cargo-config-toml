# Crate-level config

There's a `.cargo/config.toml` file inside the `test-pkg` crate directory
setting `--cfg tokio_unstable` which is required to compile the test crate here.

However, `cargo doc` uses `RUSTDOCFLAGS`, and `--cfg tokio_unstable` is not set there.
Despite this, `cargo doc` passes when run from any path because rustdoc largely ignores type errors
in method bodies, and `--cfg tokio_unstable` in this crate is only required inside a method body.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level *does not work*.
- `cargo check --manifest-path <path>` only works from inside the `test-pkg` directory,
  even if pointing to the workspace-level `Cargo.toml` file.
- `cargo doc` behaves the same way.
