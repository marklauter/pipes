---
title: Email intake reads IMAP behind a trait and tests against GreenMail
type: note
summary: The email watcher reads mailboxes over IMAP through a `MailSource` trait; unit tests use `.eml` fixtures and integration tests run against a GreenMail container.
tags: [email, intake, testing]
created: 2026-10-10
status: evolving
---

# Email intake reads IMAP behind a trait and tests against GreenMail

The email watcher reads mail and never sends it. POP3 and IMAP read from a mailbox, and SMTP sends. Pipes reads over IMAP for three reasons:

- IDLE pushes new mail to the watcher, so it doesn't have to poll.
- Each message has a stable UID, which records what has been processed.
- IMAP has folders, so the watcher can move a handled message to a `processed` folder rather than delete it.

## Shape

A `MailSource` trait hides the mailbox. It has two operations:

- `fetch_unseen()` returns each new message as raw RFC 822 bytes.
- `mark_processed(uid)` marks a message as handled.

Attachment extraction takes those bytes and returns the attachments. It doesn't depend on a server, and most of the email logic lives there. The email watcher is an in-side door. Each extracted attachment is in flight like a file from the folder watcher, and the route map sends it to the out-side doors routed from that email door. The mailbox address, host, and credentials are settings of the door.

Mailbox authentication also sits behind the trait. Gmail and Microsoft 365 accept only OAuth2 (XOAUTH2) for IMAP, not passwords, so OAuth2 can be added later without changing callers.

## Testing

- **Unit tests** run attachment extraction against `.eml` fixture files, with an in-memory `MailSource` fake for the watcher logic.
- **Integration tests** run against [GreenMail](https://greenmail-mail-test.github.io/greenmail/) (`greenmail/standalone`, Apache-2.0). It is a test server that speaks SMTP (3025), IMAP (3143) and POP3 (3110), and it creates a mailbox the first time mail is sent to an address. A test sends a message with an attachment over SMTP and checks that the watcher picks it up over IMAP.
  - Locally: `podman run -p 3025:3025 -p 3143:3143 -p 3110:3110 greenmail/standalone`, or start it from a test with `testcontainers`, with `DOCKER_HOST` pointed at the Podman socket.
  - In CI: GreenMail runs as a GitHub Actions service container on the Ubuntu runner.

Other servers that were ruled out:

- **Mailpit** only accepts SMTP and has no IMAP or POP3 to read from.
- **Stalwart** is a full Rust mail server that is open source, but it takes more setup than a test needs.
- **Dovecot with Postfix** takes more setup still.

## Crates

- `async-imap`: the IMAP client.
- `mail-parser`: MIME parsing and attachment extraction.
- `lettre`: sends test mail over SMTP in integration tests.
- `testcontainers`: starts GreenMail from inside tests.

## Open

- Whether to support POP3 at all. IMAP covers the known cases.
- Where mailbox credentials and OAuth2 tokens are stored.
