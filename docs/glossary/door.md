---
title: door
type: definition
summary: "A connection between pipes and an external system, for passing files across the pipes boundary."
status: evolving
---

A connection between pipes and an external system, for passing files across the pipes boundary.

## Examples

- A watched folder that files are dropped into.
- An IMAP mailbox whose attachments pipes collects.
- A folder that pipes writes delivered files to.

## Rationale

The term comes from a distribution warehouse, where trucks load and unload at doors. "Dock", "port", and "adapter" lost. Port and adapter name the code that implements a door, and they stay out of the domain language.
