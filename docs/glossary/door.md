---
title: door
type: definition
summary: "A connection between pipes and an external system, for passing digital artifacts across the pipes boundary."
status: evolving
---

A connection between pipes and an external system, for passing digital artifacts across the pipes boundary.

## Examples

- A watched folder into which files are dropped.
- An IMAP mailbox whose attachments pipes collects.
- A folder to which departing files are written.

## Rationale

The term comes from a distribution warehouse, where trucks load and unload at doors. "Dock" lost. A door is implemented by an [[adapter]], and each side of pipes is a [[port]]. Those are the architecture terms, and door stays the domain term so the two vocabularies don't collide.
