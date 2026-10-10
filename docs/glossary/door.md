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

The term comes from warehouse loading doors. "Dock" and "connection" lost. A folder counts because pipes can watch it. Doors are implemented with hexagonal architecture, as [[pipes-is-a-virtual-warehouse-with-in-side-and-out-side-doors]] describes.
