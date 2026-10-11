---
title: Config
type: specification
summary: "Pipes reads its config from three TOML files in one folder: the in-side doors, the out-side doors, and the sort plan, where each door and each route is a named table each route names a success door and a failure door, and each out-side door can name a failure door."
status: evolving
cites:
  - "[[in-side-door]]"
  - "[[out-side-door]]"
  - "[[sort-plan]]"
  - "[[route]]"
  - "[[attention-queue]]"
  - "[[digital-artifact]]"
  - "[[step]]"
---

# Config

Pipes reads its config from one folder that holds three TOML files. Each [[door]] and each [[route]] is a table, and the table's name is its identifier.

## Files

**`in-side-doors.toml`** defines the [[in-side-door]]s. Each table has an `adapter` key that names the adapter type, plus the keys that adapter type requires.

```toml
[acme]
adapter = "folder-watch"
path = 'C:\pipes\in\acme'
```

**`out-side-doors.toml`** defines the [[out-side-door]]s, in the same form. An optional `failure` key names the out-side door a digital artifact departs through when its departure through this door fails. Failure doors chain, and a chain ends at a door with no `failure` key. An [[attention-queue]] is an out-side door like any other.

```toml
[archive]
adapter = "folder-write"
path = 'C:\pipes\archive'
failure = "archive-attention"

[archive-attention]
adapter = "email-send"
mailbox = "archive-attention-queue@org.com"

[review]
adapter = "folder-write"
path = 'C:\pipes\review'
```

**`sort-plan.toml`** defines the [[sort-plan]]. Each table is a route. `from` names an in-side door, `success` names the out-side door a [[digital-artifact]] departs through, and `failure` names the out-side door it departs through when a [[step]] fails. Several routes with the same `from` fan out a digital artifact to several out-side doors.

```toml
[acme-to-archive]
from = "acme"
success = "archive"
failure = "review"
```

A route matches on its in-side door alone. The door's adapter design decides which digital artifacts arrive through it. For example, one folder adapter can watch every subfolder, and a file adapter can watch a single file.

## Validation

The parser rejects the config if any of the following is true:

- A door name appears in both door files.
- A table has a key its adapter type doesn't define, or is missing a key the adapter type requires.
- An `adapter` value names an unknown adapter type.
- A route's `from` names a door missing from `in-side-doors.toml`, or its `success` or `failure` names a door missing from `out-side-doors.toml`.
- An out-side door's `failure` names a door missing from `out-side-doors.toml`, or a chain of failure doors loops back to a door already in it.
- An in-side door has no route, so a digital artifact arriving through it would have no out-side door.

The parser reports all errors in one pass, each with its file, line and column.

## Open questions

1. Whether the parser rejects a cycle. An out-side door and an in-side door can share a location, which chains routes, and a chain can loop back to its start. A loop may be a valid mechanism, so the parser may allow it.
