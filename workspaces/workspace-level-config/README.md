# Workspace-level config

There's a `.cargo/config.toml` file at workspace level,
setting `--cfg tokio_unstable` which is required to compile the test crate here.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level works.
- `cargo check --manifest-path <path>` works so long as we're inside the workspace directory.
