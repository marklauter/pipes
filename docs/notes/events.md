---
title: Events
type: note
summary: "Pipes writes one structured event per state change of a digital artifact as JSON lines on stdout; taps observe the stream read-only, and the host keeps it append-only off-host."
tags: [events, logging, observability]
created: 2026-10-11
status: evolving
---

# Events

Pipes writes one structured event each time a digital artifact changes state. The event stream is how operators and a UI see what pipes is doing. It never changes the flow.

## Events

| Event | When | Fields beyond the digital artifact |
| --- | --- | --- |
| arrived | A digital artifact [[arrive]]s through an [[in-side-door]]. | in-side door |
| sorted | The [[sorter]] matches the digital artifact against the [[sort-plan]]. | matched [[route]]s |
| departed | A digital artifact [[depart]]s through an [[out-side-door]]. | out-side door, and a reason (no route or failed departure) when that door is the [[attention-queue]] |
| departure failed | A departure through an out-side door fails. | out-side door, error |

Every event carries a timestamp and the digital artifact's id and name.

## Format

Events are `tracing` events, written to stdout as JSON lines by `tracing-subscriber`. The container runtime collects stdout, so pipes needs no log-shipping code.

## Taps

A tap is a read-only subscriber to the event stream, such as a UI showing live activity. A later `tracing` layer forwards each event to a `tokio::sync::broadcast` channel, and the UI reads it over Server-Sent Events. The event call sites stay unchanged. A tap that falls behind loses events, and the sorter never waits for a tap.

## Append-only storage

Pipes doesn't enforce append-only logs, because a process that can write its log can also rewrite it. The host forwards stdout to a store that the pipes service account can't modify. The client hosts locally, with no cloud infrastructure, so the store is one of three:

1. **Same host, OS enforced.** Podman sends stdout to journald. `chattr +a` on Linux, or an append-only ACL on Windows, blocks rewrites by anyone but root or an administrator.
2. **Tamper evident.** journald Forward Secure Sealing (`journalctl --setup-keys`) seals the log at intervals, and `journalctl --verify` detects later edits.
3. **Off-host and immutable.** syslog or Vector forwards to a log server (Grafana Loki, Graylog or OpenSearch) backed by MinIO with S3 Object Lock in compliance mode. No one, including an administrator, can delete a record before its retention period ends.

The client's requirement picks the tier: an audit or regulatory mandate needs tier 3, and tamper detection alone needs tier 2.

## Open questions

- Whether a tap replays history or sees only events after it connects. Replay could read from the log store, so pipes would still store nothing.
- How a digital artifact gets its id.
- Which tier the client requires.
- How the event schema is versioned once a UI depends on it.
