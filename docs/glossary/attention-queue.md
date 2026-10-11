---
title: attention-queue
type: definition
summary: "An out-side door for digital artifacts awaiting human attention."
status: evolving
is-a: "[[out-side-door]]"
---

An out-side door for digital artifacts awaiting human attention.

## Examples

- A `review` folder that routes name as their failure door, where a person checks each digital artifact that failed.
- An issue-tracker queue, where each failed departure opens a ticket.

## Rationale

The warehouse equivalent is the exceptions area, and the messaging equivalent is a dead-letter queue. Pipes treats it as any other out-side door. A route names it, usually as its failure door, and its adapter tells people about what arrives, by notification, email or a failure count in a UI. A released digital artifact arrives again through an in-side door.
