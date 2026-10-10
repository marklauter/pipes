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

- The `invoices` mailbox door routed to the `accounting` folder door.

## Rationale

A route pairs doors, not folders or mailboxes. A folder path or mailbox address is a setting of its door. Fan-out, where one in-side door feeds several out-side doors, is several routes that share an in-side door.
