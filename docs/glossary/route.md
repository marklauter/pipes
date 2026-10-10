---
title: route
type: definition
summary: "A pairing of an in-side door with an out-side door, for sending the digital artifacts that arrive at the first to depart through the second."
status: evolving
has-a:
  - "[[in-side-door]]"
  - "[[out-side-door]]"
---

A pairing of an in-side door with an out-side door, for sending the digital artifacts that arrive at the first to depart through the second.

## Examples

- The `invoices` mailbox routed to the `accounting` folder.

## Rationale

A route pairs named doors. Fan-out, where one in-side door feeds several out-side doors, is several routes that share an in-side door.
