# pipes

Pipes is a service for routing digital artifacts between external systems. It works like a cross-dock warehouse. Digital artifacts arrive through in-side doors such as FTP/SFTP, email, web upload and folder watch. A route map sends each one to depart through one or more out-side doors. Digital artifacts that need a person go to an attention-queue.

The build order is in [docs/notes/roadmap.md](docs/notes/roadmap.md), and the terms are defined in [docs/glossary](docs/glossary).

## Mail testing

`scripts/greenmail.sh up` starts a GreenMail test mail server in Podman, with SMTP on port 3025 and IMAP on port 3143. `cargo run -p pipes --example mail_poc` then sends an email with an attachment and reads it back.

## Development

`rust-toolchain.toml` installs the stable toolchain with clippy, rustfmt and rust-analyzer. Enable the hooks once per clone. Before each commit they check formatting, lints, docs and tests; before each push, coverage and the dependency audit, as CI does. The push checks need `cargo-llvm-cov` and `cargo-audit`. Mutation testing of a branch's changes runs in CI only.

```sh
git config core.hooksPath .githooks
```
