---
title: Config
type: specification
summary: "Pipes reads its config from three TOML files in one folder: the in-side doors, the out-side doors, and the sort plan, where each door and each route is a named table."
status: evolving
cites:
  - "[[in-side-door]]"
  - "[[out-side-door]]"
  - "[[sort-plan]]"
  - "[[route]]"
  - "[[attention-queue]]"
  - "[[digital-artifact]]"
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

**`out-side-doors.toml`** defines the [[out-side-door]]s, in the same form. An [[attention-queue]] is an out-side door like any other.

```toml
[archive]
adapter = "folder-write"
path = 'C:\pipes\archive'

[review]
adapter = "folder-write"
path = 'C:\pipes\review'
```

**`sort-plan.toml`** defines the [[sort-plan]]. The `attention-queue` key names the out-side door that receives each [[digital-artifact]] with no route or a failed departure. Each table is a route, with `from` naming an in-side door and `to` naming an out-side door. Several routes with the same `from` fan out a digital artifact to several out-side doors.

```toml
attention-queue = "review"

[acme-to-archive]
from = "acme"
to = "archive"
```

A route matches on its in-side door alone. The door's adapter design decides which digital artifacts arrive through it. For example, one folder adapter can watch every subfolder, and a file adapter can watch a single file.

## Validation

The parser rejects the config if any of the following is true:

- A door name appears in both door files.
- A table has a key its adapter type doesn't define, or is missing a key the adapter type requires.
- An `adapter` value names an unknown adapter type.
- A route's `from` names a door missing from `in-side-doors.toml`, or its `to` names a door missing from `out-side-doors.toml`.
- The `attention-queue` key is missing or names a door missing from `out-side-doors.toml`.

The parser reports all errors in one pass, each with its file, line and column.

## Open questions

1. Whether the sort plan names the attention-queue with one key, as above. Steps will later need a failure destination of their own.
2. Whether the parser rejects a cycle. An out-side door and an in-side door can share a location, which chains routes, and a chain can loop back to its start. A loop may be a valid mechanism, so the parser may allow it.
