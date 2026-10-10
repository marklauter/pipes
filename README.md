# pipes
watch n route

## Development

`rust-toolchain.toml` installs the stable toolchain with clippy, rustfmt and rust-analyzer. Enable the hooks once per clone. Before each commit they check formatting, lints, docs and tests; before each push, coverage and the dependency audit, as CI does. The push checks need `cargo-llvm-cov` and `cargo-audit`. Mutation testing of a branch's changes runs in CI only.

```sh
git config core.hooksPath .githooks
```
