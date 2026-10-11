---
title: step
type: definition
summary: "An operation on an in-flight digital artifact, for preparing it to depart through its route's out-side door."
status: evolving
---

An operation on an in-flight digital artifact, for preparing it to depart through its route's out-side door.

## Examples

- Wrapping a digital artifact in an envelope.
- Tagging a high-value digital artifact for special tracking.

## Rationale

A [[route]] runs its steps in order, so in fan-out each route prepares its own copy. A step ends in success, which passes the digital artifact on, or failure, which sends it to the [[attention-queue]] on a permanent fault or a transient fault past its retry limit. "Transformer" lost because a step can wrap without transforming. "Processor" is generic, "filter" suggests discarding, and "job" names a run.
