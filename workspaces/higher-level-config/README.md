# Higher-level config

There's a `.cargo/config.toml` file one directory above the workspace level,
setting `--cfg tokio_unstable` which is required to compile the test crate inside.

- `cargo check` from inside `test-pkg` works.
- `cargo check` at workspace level works.
- `cargo check --manifest-path <path>` works so long as we're inside the parent directory
  of the workspace.
