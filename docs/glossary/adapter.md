---
title: adapter
type: definition
summary: "A component for converting between a port and one external technology."
status: evolving
---

A component for converting between a port and one external technology.

## Examples

- The folder-watch adapter turns a finished file in a watched folder into an arrival at the in-side port.
- The email adapter turns each attachment in an IMAP mailbox into an arrival at the in-side port.
- The folder-write adapter turns a departure at the out-side port into a file written to a folder.

## Rationale

The term comes from Alistair Cockburn's Ports and Adapters pattern. Cockburn writes that for each external device "there is an adapter that converts the API definition to the signals needed by that device and vice versa". A port can have many adapters, and adding one leaves the application unchanged. Test doubles are adapters too, which is how pipes tests its core without servers.

Each [[door]] is implemented by an adapter. The door is the warehouse term, the adapter is the code.

## References

- Alistair Cockburn, ["Hexagonal Architecture"](https://alistair.cockburn.us/hexagonal-architecture/), HaT Technical Report 2005.02, 2005-09-04.
- Alistair Cockburn and Juan Manuel Garrido de Paz, *Hexagonal Architecture Explained*, Humans and Technology, 2024, ISBN 978-1-7375197-8-2.
