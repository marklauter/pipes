---
title: Pipes builds the route map first, then folder and email intake
type: note
summary: "Build order: route map, folder watcher, email watcher, then optional native FTP/SFTP, then the plugin pipeline. Each intake route feeds the same map."
tags: [roadmap]
created: 2026-10-10
status: evolving
---

# Pipes builds the route map first, then folder and email intake

Pipes watches for incoming files, tags them, and moves each one to a destination chosen by a route map. Files arrive through in-side doors: FTP/SFTP, email, web upload, and folder watch. The route map steers every in-flight file, whichever door received it, so the route map comes first and the doors follow in order of complexity. The model behind this is in [[pipes-is-a-virtual-warehouse-with-in-side-and-out-side-doors]].

## Order

1. **Route map.** Config that defines the doors, each with its adapter type and settings, and the routes that connect an in-side door to an out-side door. The work is the core types, a parser, and validation that rejects a bad entry and reports where it is. The code is pure logic with no I/O, so the coverage and mutation gates test it fully.
2. **Folder watcher.** An in-side door that watches the folder in its settings with the `notify` crate and receives each finished file. Its out-side counterpart writes a file to the folder in its settings. FTP/SFTP and web upload use this route at first: a standard server (vsftpd, OpenSSH SFTP) or the upload handler writes into an inbound folder. The watcher must not move a file until the writer has finished with it.
3. **Email watcher.** Reads mailboxes over IMAP and receives each attachment as a file. The design is in [[email-intake-reads-imap-behind-a-trait-and-tests-against-greenmail]].
4. **Native FTP/SFTP (optional).** Pipes runs its own FTP or SFTP server and routes uploads directly, without a folder in between. Build this only if an external server proves inadequate.
5. **Plugin pipeline.** Plugin-style steps between intake and delivery that transform a file or wrap it in an envelope.

## Open

- What a tag is: metadata stored beside the file, a change to the file name, or a field in an envelope. The answer may fold tagging into the plugin pipeline.
- The config format. CSV fits routes, but door settings differ by adapter type and may need a nested format such as TOML.
- How the web upload endpoint is hosted, and whether it belongs in this repo.
