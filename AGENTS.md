# Agent Notes for the Repository

- Always run `cargo +nightly fmt` (and install the nightly `rustfmt` component if needed) before submitting changes so the workspace follows the settings in `rustfmt.toml`.
- Do not hand-tune formatting or reorder imports manually—rustfmt will handle this consistently across crates.
- Check for nested `AGENTS.md` files (e.g., within `libs/clockwork_tuples/`) and follow their additional instructions when touching files in those directories.
