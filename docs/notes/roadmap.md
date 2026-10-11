---
title: Roadmap
type: note
summary: "Build order: the core (ports, sorter, sort plan, attention-queue fallback, events), folder doors and the service, mailbox doors, taps, optional native FTP/SFTP, then the plugin pipeline."
tags: [roadmap]
created: 2026-10-10
status: evolving
---

# Roadmap

Every door plugs into the same core, so the core comes first and the doors follow in order of complexity. The model is in [[pipes-is-a-virtual-warehouse-with-in-side-and-out-side-doors]], and the terms are in `docs/glossary/`.

## Order

1. **Core.** Everything a door plugs into, as pure logic with no I/O, so the coverage and mutation gates test it fully:
   - The in-side port and the out-side port.
   - The in-flight digital artifact, which records its in-side door.
   - The config: the in-side doors, the out-side doors and the sort plan, in three TOML files, as [[config]] describes. A parser validates it and reports where each bad entry is.
   - The sorter, which diverts each in-flight digital artifact by the sort plan:
     - Fan-out: one digital artifact departs through every out-side door its routes name.
     - The fallback: a digital artifact with no route, or whose departure failed, departs to the [[attention-queue]].
   - The events: arrived, sorted, departed and departure failed, emitted through `tracing`, as [[events]] describes.
2. **Folder doors and the first runnable service.** A folder-watch in-side door, built on the `notify` crate, and a folder-write out-side door. The attention-queue starts as a folder-write door. A service binary loads the config, runs the doors, and runs as a Windows service or a Linux service. It writes events to a JSON-lines file sink and a stdout sink. FTP/SFTP and web upload use the folder-watch door at first: a standard server (vsftpd, OpenSSH SFTP) or the upload handler writes into the watched folder. A file arrives only after its writer has finished with it.
3. **Mailbox doors.** The email adapter reads each mailbox over IMAP, and each attachment arrives as its own digital artifact. The design is in [[email-adapter-reads-imap-behind-a-trait-and-tests-against-greenmail]].
4. **Taps.** A `tracing` layer forwards events to a broadcast channel, and an SSE endpoint serves them to a UI.
5. **Native FTP/SFTP doors (optional).** Pipes runs its own FTP or SFTP server, so uploads arrive with no intermediate folder. Build these only if an external server proves inadequate.
6. **Plugin pipeline.** Plugin steps that transform an in-flight digital artifact or wrap it in an envelope. Each step has two exits: success, and failure to the attention-queue on a permanent fault or a transient fault that exceeds its retry limit.

## Open questions

1. What a tag is: metadata carried with the digital artifact, a change to its name, or a field in an envelope. The answer may fold tagging into the plugin pipeline.
2. Fan-out with a partial failure: whether the attention-queue receives only the failed departure or the whole digital artifact.
3. How a person releases a digital artifact from the attention-queue, and through which in-side door it arrives again.
4. Which out-side doors are needed beyond folders and the attention-queue.
5. How the web upload endpoint and the tap SSE endpoint are hosted, whether they share a host, and whether they belong in this repo.
