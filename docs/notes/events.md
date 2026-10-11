---
title: Events
type: note
summary: "Pipes writes one structured event per state change of a digital artifact as JSON lines, to a file on Windows or stdout on Linux; taps observe the stream read-only, and append-only storage comes from the OS, a hash chain, or an immutable off-host store."
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

Events are `tracing` events, so each sink is a `tracing-subscriber` layer chosen in config and the event call sites never change. Pipes runs on Windows and Linux, so it supports two sinks:

- **JSON-lines file.** `tracing-appender` writes daily rolling files. This is the default on Windows, where pipes runs as a Windows service and has no stdout.
- **stdout.** JSON lines for a container or a systemd unit on Linux, where the host collects them.

Both sink writes are synchronous, so a slow disk or pipe stalls the thread that logs. A later enhancement wraps each sink in `tracing_appender::non_blocking`, which moves the writes to a background thread. Its buffer drops events when full by default, so the audit sinks set `lossy(false)`, which makes the logging thread wait instead.

OpenTelemetry is left out until the client adopts an OTel backend or pipes splits into services whose traces need correlating. Adding it then is a `tracing-opentelemetry` layer, with no change at the call sites.

## Taps

A tap is a read-only subscriber to the event stream, such as a UI showing live activity. A later `tracing` layer forwards each event to a `tokio::sync::broadcast` channel, and the UI reads it over Server-Sent Events. A tap that falls behind loses events, and the sorter never waits for a tap.

## Append-only storage

Pipes can't make its own log append-only, because a process that can write its log can also rewrite it. Three tiers work on Windows and Linux without cloud infrastructure:

1. **OS enforced.** The pipes service account gets append-data permission on the log folder but no write-data or delete permission. On Linux, `chattr +a` does the same. An administrator or root can still alter the files.
2. **Tamper evident.** Each event carries the hash of the previous event, and a verify command reports the first broken link. This works the same on both platforms, because pipes implements it.
3. **Off-host and immutable.** Vector, which runs on Windows and Linux, tails the files and ships them to a log server backed by MinIO with S3 Object Lock in compliance mode. No one, including an administrator, can delete a record before its retention period ends.

The client's requirement picks the tier: an audit or regulatory mandate needs tier 3, and tamper detection alone needs tier 2.

## Open questions

1. Whether a tap replays history or sees only events after it connects. Replay could read from the log store, so pipes would still store nothing.
2. How a digital artifact gets its id.
3. Which tier the client requires.
4. Whether to add a Windows Event Log sink for the client's existing Windows monitoring.
5. How the event schema is versioned once a UI depends on it.
