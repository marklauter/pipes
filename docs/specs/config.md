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

**`out-side-doors.toml`** defines the [[out-side-door]]s, in the same form. It must define a door named `attention-queue`, which is the [[attention-queue]].

```toml
[archive]
adapter = "folder-write"
path = 'C:\pipes\archive'

[attention-queue]
adapter = "folder-write"
path = 'C:\pipes\attention'
```

**`sort-plan.toml`** defines the [[sort-plan]]. Each table is a route, with `from` naming an in-side door and `to` naming an out-side door. Several routes with the same `from` fan out a [[digital-artifact]] to several out-side doors.

```toml
[acme-to-archive]
from = "acme"
to = "archive"
```

## Validation

The parser rejects the config if any of the following is true:

- A door name appears in both door files.
- A table has a key its adapter type doesn't define, or is missing a key the adapter type requires.
- An `adapter` value names an unknown adapter type.
- A route's `from` names a door missing from `in-side-doors.toml`, or its `to` names a door missing from `out-side-doors.toml`.
- `out-side-doors.toml` has no `attention-queue` door.

The parser reports all errors in one pass, each with its file, line and column.

## Open questions

1. Whether each file can become a folder of files, such as one in-side door per file.
2. Whether a route matches on anything beyond its in-side door, such as a file name pattern.
3. Whether a route names exactly one out-side door, so that fan-out is several routes sharing a `from`. The spec assumes yes, which matches [[route]].
4. Whether `sort-plan.toml` names its tables directly, without a `routes.` prefix. The spec assumes yes.
5. Whether `out-side-doors.toml` must define a door named `attention-queue`. The spec assumes yes.
6. Whether door names must be unique across both door files. The spec assumes yes.
