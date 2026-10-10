---
title: port
type: definition
summary: "An interface of an application, for one purposeful conversation with the world outside it."
status: evolving
disjoint-with:
  - "[[adapter]]"
---

An interface of an application, for one purposeful conversation with the world outside it.

## Examples

- The in-side port: the one conversation by which digital artifacts arrive in pipes, whatever the door.
- The out-side port: the one conversation by which digital artifacts depart pipes, whatever the door.

## Rationale

The term comes from Alistair Cockburn's Ports and Adapters pattern, also called hexagonal architecture. Cockburn writes that "a port identifies a purposeful conversation". The application defines the port in its own terms, so its logic never depends on a particular technology.

Cockburn separates driving ports, through which the outside world calls the application, from driven ports, through which the application calls the outside world. The in-side port is a driving port. The out-side port is a driven port.

Port and adapter are architecture terms, not warehouse terms. In the warehouse language, each side of pipes is a port, and each [[door]] is implemented by an adapter.

## References

- Alistair Cockburn, ["Hexagonal Architecture"](https://alistair.cockburn.us/hexagonal-architecture/), HaT Technical Report 2005.02, 2005-09-04.
- Alistair Cockburn and Juan Manuel Garrido de Paz, *Hexagonal Architecture Explained*, Humans and Technology, 2024, ISBN 978-1-7375197-8-2.
