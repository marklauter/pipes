---
title: pipes
type: definition
summary: "A service for routing digital artifacts between external systems."
status: evolving
---

A service for routing digital artifacts between external systems.

## Examples

- Invoices emailed to a mailbox depart as files in an accounting folder.
- Files uploaded over SFTP depart to two partner folders.

## Rationale

The kernel names no door or route, because door, route and route map are defined in terms of pipes, and naming them here would make the definitions circular. Pipes works like a cross-dock warehouse: digital artifacts pass straight through and pipes doesn't keep them.
