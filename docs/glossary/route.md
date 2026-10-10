---
title: route
type: definition
summary: "A pairing of an in-side door with an out-side door, for sending the digital artifacts that arrive through the first to depart through the second."
status: evolving
has-a:
  - "[[in-side-door]]"
  - "[[out-side-door]]"
---

A pairing of an in-side door with an out-side door, for sending the digital artifacts that arrive through the first to depart through the second.

## Examples

- The `invoices` mailbox routed to the `accounting` folder.

## Rationale

A route names its doors. Fan-out is several routes that share an in-side door.
