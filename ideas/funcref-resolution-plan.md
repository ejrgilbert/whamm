# Funcref → fid resolution

## Problem

At call_indirect (and other funcref-handling) sites, we want user probes to
be able to reference the fid of the function being dispatched. The underlying
wasm value is a `funcref` pulled from a table by `i32` index. The naive
approach — a shadow table in wasm memory mapping `(table, idx) → fid` — is
stale the moment anyone mutates the real table, because the shadow isn't
updated in lockstep.

## Rejected alternatives

- **`ref.eq` to match each live funcref against constructed `ref.func $N`s.**
  Not possible: `funcref` is not a subtype of `eqref`; the reference hierarchies
  are disjoint in wasm by design.

- **Callee-side tagging** (each potentially-indirect-callable function writes
  its fid to a shadow global on entry). Rejected because it (1) can only resolve
  *after* the call has occurred, (2) adds per-invocation overhead to every
  candidate target, and (3) offers no reflection on funcrefs that point to
  imported functions.

- **Full dynamic shadow propagation** (parallel `i32` fid riding alongside
  every funcref through every local / global / param / return). Correct, but
  invasive — every funcref-handling instruction needs instrumentation, bloating
  the module.

- **Static dataflow to the originating `ref.func`.** Unbounded: funcrefs flow
  through `table.get ← table.set ← locals/calls/returns`; resolution collapses
  into symbolic execution.

## Chosen design: observable approximation

Maintain a shadow table per real table, populated **only from statically
recoverable sources**. Everything else writes a sentinel. The probe always
receives either a real fid or the sentinel, so imprecision is a first-class
output rather than a silent lie.

**Sentinel:** `UNKNOWN = -1`. Fids are non-negative, so user probes can test
`fid < 0` as a fast "unresolved?" check.

### How each event resolves

| Event                           | Funcref source           | How to resolve                          |
|---------------------------------|--------------------------|-----------------------------------------|
| `ref.func $N`                   | immediate                | Trivial: fid = `$N`                     |
| `call_indirect tbl, idx`        | `shadow_table[tbl][idx]` | Shadow read, bounds-checked             |
| `return_call_indirect tbl, idx` | same                     | same                                    |
| `table.get tbl`                 | `shadow_table[tbl][idx]` | Shadow read                             |
| `call_ref`                      | funcref on stack         | Single-instr lookback for `ref.func $N` |
| `return_call_ref`               | same                     | same                                    |
| `table.set tbl`                 | funcref on stack         | same                                    |

For shadow population, every `table.set` / `table.init` / `table.copy` /
`table.fill` whose target table has a shadow gets pre-instrumentation in a
post-rewrite pass. Lossy cases write `-1`; precise cases write the recovered
fid. `table.grow` is not instrumented — accesses past the shadow's bounds
resolve to `-1`.

## Implementation strategy

The shadow-table maintenance (user references a bound var in a `call_indirect`
probe → compiler auto-emits instrumentation at `table.set`/`table.init`/etc.
sites) is whamm's first instance of "bound value triggers side-effects across
a set of *other* events." Implemented one-off / hardcoded in the compiler pass;
generalize into a proper whamm feature once a second use case appears, to
avoid designing the abstraction blind.

## Known gaps (acceptable)

- Imported-function funcrefs → always `-1`.
- Host-written table entries → `-1` until overwritten by an instrumented op.
- Funcrefs that pass through a local / global / return → `-1`.
- `table.grow` not instrumented; new slots resolve to `-1`.

These gaps are the *point* of the design: imprecision is observable, not
hidden.

## Deferred follow-ups

- Extend the shadow on `table.grow` (currently any access past initial bounds
  resolves to `-1`).
- Precise `table.fill` (loop-store the recovered fid) and `table.init` (emit
  a per-passive-element-segment fid data segment and `memory.copy` it into
  the shadow).
- Thread shadow values through `table.get; call_ref` chains so the lossy
  call_ref case can recover the fid via the table.get's shadow read.
- `resolved_fid` on `ref.is_null` / `ref.as_non_null` (utility unclear,
  not yet wired).
- Wei-backend support for `resolve_funcref` (rewriting backend only today;
  `paper_eval/call_graph_resolved/` runs with Wei disabled).
