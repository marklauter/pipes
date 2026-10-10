---
title: Pipes is a virtual warehouse with in-side and out-side doors
type: note
summary: "Files arrive through in-side doors, are routed in flight by the route map, and leave through out-side doors. Each side is a port, and each door is an adapter."
tags: [architecture, domain]
created: 2026-10-10
status: evolving
---

# Pipes is a virtual warehouse with in-side and out-side doors

Pipes is modeled on a retail distribution warehouse, such as Macy's. One truck of shoes arrives at the in-side. The shoes go onto a routing belt, which spreads them across many trucks waiting at the out-side. Logistics calls this cross-docking: goods are routed straight through rather than stored. Pipes does the same with files.

## Parts

- **In-side doors** are where files arrive. The known in-side doors are FTP/SFTP, email, web upload, and folder watch.
- **In flight** is the state of a file between the door it arrived through and the out-side doors the route map assigns. The plugin pipeline transforms or wraps in-flight files.
- **Out-side doors** are where files leave. Writing to a folder is the only out-side door known so far.

## Code shape

The warehouse terms map onto ports and adapters:

- The in-side is a port: one trait that every arriving file passes through.
- The out-side is a port: one trait that every departing file passes through.
- Each door is an adapter that implements its side's port. Adding a door means adding an adapter, and routing doesn't change.

The two ports are the first plug-in points. The plugin pipeline is the third.

## Route map

The route map steers in-flight files, and it connects doors, not folders. A location such as a folder path, a mailbox, or an SFTP host belongs to the door's adapter as one of its settings. The config therefore has two parts:

- **Doors.** Each door has a name, a side (in or out), an adapter type, and that adapter's settings.
- **Routes.** Each route connects an in-side door to an out-side door by name.

The build order is in [[pipes-builds-the-route-map-first-then-folder-and-email-intake]].

## Open

- Which out-side doors are needed beyond writing to a folder.
- Whether one arriving file can leave through several out-side doors, as one truck of shoes fills many trucks. If so, fan-out is several routes from the same in-side door.
