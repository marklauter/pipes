---
title: Roadmap
type: note
summary: "Build order: the core (ports, sorter, sort plan, attention-queue fallback), folder doors, mailbox doors, optional native FTP/SFTP, then the plugin pipeline."
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
   - The config: the doors, each with its adapter type and the location that identifies it, and the sort plan between them. A parser validates it and reports where each bad entry is.
   - The sorter, which diverts each in-flight digital artifact by the sort plan:
     - Fan-out: one digital artifact departs through every out-side door its routes name.
     - The fallback: a digital artifact with no route, or whose departure failed, departs to the [[attention-queue]].
2. **Folder doors and the first runnable service.** A folder-watch in-side door, built on the `notify` crate, and a folder-write out-side door. The attention-queue starts as a folder-write door. A service binary loads the config and runs the doors. FTP/SFTP and web upload use the folder-watch door at first: a standard server (vsftpd, OpenSSH SFTP) or the upload handler writes into the watched folder. A file arrives only after its writer has finished with it.
3. **Mailbox doors.** The email adapter reads each mailbox over IMAP, and each attachment arrives as its own digital artifact. The design is in [[email-adapter-reads-imap-behind-a-trait-and-tests-against-greenmail]].
4. **Native FTP/SFTP doors (optional).** Pipes runs its own FTP or SFTP server, so uploads arrive with no intermediate folder. Build these only if an external server proves inadequate.
5. **Plugin pipeline.** Plugin steps that transform an in-flight digital artifact or wrap it in an envelope.

## Open

- What a tag is: metadata carried with the digital artifact, a change to its name, or a field in an envelope. The answer may fold tagging into the plugin pipeline.
- The config format, and a term for the config. CSV fits routes, but door settings differ by adapter type and may need a nested format such as TOML.
- Fan-out with a partial failure: whether the attention-queue receives only the failed departure or the whole digital artifact.
- How a person releases a digital artifact from the attention-queue, and through which in-side door it arrives again.
- Which out-side doors are needed beyond folders and the attention-queue.
- How the web upload endpoint is hosted, and whether it belongs in this repo.
