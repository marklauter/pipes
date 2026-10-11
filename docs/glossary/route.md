---
title: route
type: definition
summary: "A pairing of an in-side door with a success out-side door and a failure out-side door, for sending each digital artifact that arrives through the first to depart through the second, or through the third when a step or the departure fails."
status: evolving
has-a:
  - "[[in-side-door]]"
  - "[[out-side-door]]"
  - "[[step]]"
---

A pairing of an in-side door with a success out-side door and a failure out-side door, for sending each digital artifact that arrives through the first to depart through the second, or through the third when a step or the departure fails.

## Examples

- The `invoices` mailbox routed to the `accounting` folder, with the `review` folder for failures.

## Rationale

A route names its doors and its ordered steps. Fan-out is several routes that share an in-side door.
