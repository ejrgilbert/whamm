# Operand polymorphism: `wei` (monitor-module) support

Design doc for how to support generic probes on the `wei` target.

## The core problem

The rewriting backend monomorphizes **per match site**: it sees the target app, so at each site it
knows the concrete operand types, binds each type parameter, and substitutes into a per-site clone
(`src/emitter/rewriting/mono.rs`).

wei can't do that. A wei monitor is a **standalone module applied to any app at bind time** — it
never sees concrete operand types at generation time. And a **core-wasm export has a fixed
signature**, so a single exported probe function physically cannot receive an `i32` at one site
and an `f64` at another. So the per-site monomorphization has to move somewhere else, or be
expressed differently.

The good news: the generic probe reaches wei as the **symbolic IR** (metadata collection runs
before the wei/rewrite split and does *not* monomorphize), so wei has the full information —
`type_params` with effective `GenericConstraint`s, and a body/predicate whose `done_on`/operand
types are still `TypeParam`. Nothing needs reconstructing; the question is only how to *lower* it.

## Direction: type reflection + bind-time monomorphization

Extend wei / the engine so it can **reflect** the concrete operand types at each match site (the
engine sees the app at bind time), then **synthesize a monomorphized probe function** for that site
from the generic template using **`func.new`** — a wasm proposal (dynamic function creation) already
implemented in Wizard, the engine the wei target runs on. The wei module ships the probe
*generically* and the engine instantiates it per site — the same work `mono.rs` does at compile time
for rewriting, moved to bind time.

Concretely:

- **wei emits the probe as a template**, not a concrete export: the symbolic body/predicate (operand
  and `done_on` types still `TypeParam`), the `type_params` with their effective
  `GenericConstraint`s, and the match rule — enough for the engine to re-lower.
- **At bind time, per matching site**, the engine:
  1. reflects the site's concrete operand types;
  2. checks each type parameter's constraint `admits` them, plus the self-consistency rule (a param
     reused across operands ⇒ those operands must share a concrete type) — reuse `GenericConstraint`
     semantics;
  3. monomorphizes the template (`T → concrete`), mirroring `mono.rs`;
  4. creates the concrete probe function (`func.new`) and wires it to the site.

Why this shape: handles open/`Top` (and future GC) bounds, avoids export blowup, and keeps the
symbolic generic IR as the single shared representation across both backends.

### Load-bearing pieces / open questions

- **`func.new` shape.** Understand the proposal's exact signature/semantics as implemented in
  Wizard (what it takes — a function type + body/closure? — and how the monomorphized body is
  handed to it).
- **Template encoding.** How does the wei module carry the generic body + constraints so the engine
  can re-lower it? Is the engine whamm-aware (reuses whamm's mono/lowering), or does it interpret an
  embedded template/IR shipped in the module?
- **Reflection surface.** How does the engine expose per-site operand types at bind time?
- **Share the mono logic.** Prefer reusing `mono.rs`'s substitution + `GenericConstraint` checks
  over reimplementing them engine-side.

## Alternatives considered (rejected)

- **Enumerate closed-bound leaves** — a concrete, type-suffixed export per combination of the
  constraints' leaves (`GenericConstraint::leaves()`). No engine change, but `Nᵏ` export blowup for
  multi-parameter probes and **cannot** express open/`Top` bounds. (This was the early plan,
  rejected for exactly these reasons.)
- **Boxed / runtime-dispatch ABI** — one export with a canonical/boxed value representation + a
  runtime type tag the body switches on. Single export, but needs a boxed ABI and per-firing
  dispatch cost; largest change.

## Intertwined: the duplicate-export bug

Independent of polymorphism, two conflicting-bound probes on one opcode can produce the same
export name (`wasm:opcode:<opcode>`) → duplicate export → invalid module. `probe_merge_key` /
`create_wei_match_rule` (`src/generator/wei/mod.rs`) are where export identity is decided. Whatever
approach is chosen, distinct instantiations must get **distinct export identities**, so this bug
should be fixed as part of the same work.

## Next steps

1. Learn `func.new`'s signature/semantics (Wizard's implementation) and Wizard's reflection API.
2. Decide how wei ships the generic template + constraints (module encoding) and where the mono
   logic lives (share `mono.rs` if possible).
3. wei-side: emit the generic template instead of erroring; remove/relax the `wei/mod.rs` guard;
   fix duplicate-export identity so instantiations stay distinct.
4. Engine-side: per matching site, reflect operand types → check constraints → monomorphize →
   `func.new` + install.
5. Tests: convert the `core_suite/polymorphism` `expected/wei/*.err` ("not yet supported") entries
   into real expected outputs. Follow the file-based suite convention (a `.err` shadows a sibling
   `.exp`).
6. Docs + bookkeeping: update the "Target support" note in `docs/src/intro/syntax/polymorphism.md`;
   update the memory.

Keep the generic probe IR (type parameters + constraints) as the shared representation — do not
monomorphize it away before the wei path.
