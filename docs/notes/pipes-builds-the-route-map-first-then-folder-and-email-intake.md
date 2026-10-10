---
title: Pipes builds the route map first, then folder and email intake
type: note
summary: "Build order: route map, folder watcher, email watcher, then optional native FTP/SFTP, then the plugin pipeline. Each intake route feeds the same map."
tags: [roadmap]
created: 2026-10-10
status: evolving
---

# Pipes builds the route map first, then folder and email intake

Pipes watches for incoming files, tags them, and moves each one to a destination chosen by a route map. Files arrive by three routes: FTP/SFTP, email, and web upload. Every route hands files to the same router, so the router comes first and the intake routes follow in order of complexity.

## Order

1. **Route map.** A config file, CSV for now. Each row maps a transport and an inbound location to an outbound location. The work is the core types, a parser, and validation that rejects a bad row and reports its line number. The code is pure logic with no I/O, so the coverage and mutation gates test it fully.
2. **Folder watcher.** Watches the inbound folders named in the map with the `notify` crate and moves each finished file to its outbound location. FTP/SFTP and web upload use this route at first: a standard server (vsftpd, OpenSSH SFTP) or the upload handler writes into an inbound folder. The watcher must not move a file until the writer has finished with it.
3. **Email watcher.** Reads mailboxes over IMAP and sends each attachment into the same router. The design is in [[email-intake-reads-imap-behind-a-trait-and-tests-against-greenmail]].
4. **Native FTP/SFTP (optional).** Pipes runs its own FTP or SFTP server and routes uploads directly, without a folder in between. Build this only if an external server proves inadequate.
5. **Plugin pipeline.** Plugin-style steps between intake and delivery that transform a file or wrap it in an envelope.

## Open

- What a tag is: metadata stored beside the file, a change to the file name, or a field in an envelope. The answer may fold tagging into the plugin pipeline.
- Whether the map stays CSV once plugin steps need per-route settings.
- How the web upload endpoint is hosted, and whether it belongs in this repo.
