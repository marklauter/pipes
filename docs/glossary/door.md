---
title: door
type: definition
summary: "A delivery mechanism for passing digital artifacts between pipes and an external system."
status: evolving
---

A delivery mechanism for passing digital artifacts between pipes and an external system.

## Examples

- The `scans` folder.
- The `invoices` mailbox.
- A partner's SFTP server.

## Rationale

The term comes from a distribution warehouse, where trucks load and unload at doors. "Dock" lost. "Connection" lost, because the folder or mailbox is itself the door. A folder counts because pipes can watch it, as in the outbox pattern. Doors are implemented with hexagonal architecture, as [[pipes-is-a-virtual-warehouse-with-in-side-and-out-side-doors]] describes.
