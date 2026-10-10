---
title: Pipes is a virtual warehouse with in-side and out-side doors
type: note
summary: "Digital artifacts arrive through in-side doors, are in flight while the sorter steers them by the sort plan, and depart through out-side doors. Doors are implemented with ports and adapters."
tags: [architecture, domain]
created: 2026-10-10
status: evolving
---

# Pipes is a virtual warehouse with in-side and out-side doors

Pipes is modeled on a retail distribution warehouse, such as Macy's. One truck of shoes arrives at the in-side. A sorter diverts the shoes across many trucks waiting at the out-side. Logistics calls this cross-docking: goods are routed straight through rather than stored. Pipes does the same with digital artifacts.

## Parts

- **In-side doors** are where digital artifacts arrive. The known kinds are FTP/SFTP servers, mailboxes, web upload, and watched folders.
- **In flight** is the state of a digital artifact between its arrival and its last departure. The plugin pipeline transforms or wraps in-flight digital artifacts.
- **The [[sorter]]** diverts in-flight digital artifacts to out-side doors by the [[sort-plan]].
- **Out-side doors** are where digital artifacts depart. The known kinds are folders and the [[attention-queue]], where digital artifacts wait for a person.

## Code shape

Doors are implemented with Alistair Cockburn's Ports and Adapters pattern, also called hexagonal architecture. These are build terms, so the glossary leaves them out.

- A **port** is an interface of the application for one purposeful conversation with the world outside it. The application defines the port in its own terms, so its logic never depends on a technology.
- An **adapter** converts between a port and one external technology, "the API definition to the signals needed by that device and vice versa".

Cockburn separates driving ports, through which the outside world calls the application, from driven ports, through which the application calls the outside world. In pipes:

- The in-side is a driving port: one trait through which every arriving digital artifact passes.
- The out-side is a driven port: one trait through which every departing digital artifact passes.
- Each kind of door, such as a watched folder or a mailbox, has an adapter for its side's port. Adding a kind of door means adding an adapter, and routing doesn't change.

Test doubles are adapters too, so pipes tests its core without servers.

The two ports are the first plugin points. The plugin pipeline is the third.

References:

- Alistair Cockburn, ["Hexagonal Architecture"](https://alistair.cockburn.us/hexagonal-architecture/), HaT Technical Report 2005.02, 2005-09-04.
- Alistair Cockburn and Juan Manuel Garrido de Paz, *Hexagonal Architecture Explained*, Humans and Technology, 2024, ISBN 978-1-7375197-8-2.

## Sorter and sort plan

The sorter diverts in-flight digital artifacts by the sort plan, whose routes connect doors by name. The config has two parts:

- **Doors.** Each door has a name, a side (in or out), an adapter type, and the location that identifies it, such as a folder path, a mailbox address, or an SFTP host.
- **Routes.** Each route connects an in-side door to an out-side door by name. Fan-out is several routes from the same in-side door.

The build order is in [[roadmap]].
