# Type bounds → operand polymorphism

Where `argN`/`localN` typing should go, once we want polymorphic ops on them.
Context: [#334](https://github.com/ejrgilbert/whamm/issues/334).

## Where we are

An operand's type is really per-site (`global.set`'s arg0 = the global's type;
`drop`/`select` are open). But it's declared once as a shared, event-scoped
`arg0: unknown` (`providers/packages/events/wasm-opcode-STAR.yaml`). #334: the old
code typed `(arg0: i32)` by mutating that shared record, so sibling probes binding
the same operand to different types clobbered each other (last writer won).

Current fix (`SymbolTableBuilder::add_type_bounds`): define a probe-local binding
that *shadows* the event-scoped one. Correct, but the operand type is still owned
by the event, not the probe — the binding is duplicated.

## Where it should go

- event/provider = **signature**: operand N exists (arity + any intrinsic
  constraint).
- probe = **instantiation**: a type var `α` per referenced `argN`/`localN`.
- bound `(arg0: i32)` = a **constraint** on `α` (later: `α: numeric`, `α ∈ {i32,i64}`).
- emit = **monomorphize** `α` per site.

One operand binding per probe, no shadow, and `α` is where polymorphism lands.

## Relocation

Move operand ownership event→probe. Snags found while doing #334:

- [ ] Two declaration styles to unify: literal variants (`arg0_unknown` → `arg0`)
  vs pattern names (`arg[0:9]+`, matched at emit time via `nth_prefixed`/
  `check_var_types` in `src/emitter/rewriting/rules/mod.rs`).
- [ ] Derived vars over operands, e.g. `effective_addr = arg0 + offset`.
- [ ] `restrict_probe_local_state` (`src/verifier/verifier.rs`) flips once operands
  are probe-local — would forbid `arg0` in global/`report var` inits. Decide.
- [ ] `event.def.bound_vars.len()` assert in `src/parser/tests/whamm_scripts.rs`.

## wei

Same template-vs-instantiation gap: two conflicting-bound probes on one opcode both
export `wasm:opcode:<opcode>` → duplicate export, invalid module. Hence the
conflicting-bound tests are rewriting-only.

- [ ] Disambiguate wei exports (probe id + type), or merge behind a runtime branch.

## Explicit cast: type parameter ↔ concrete type (follow-up)

Combining a type parameter with a concrete-typed operand (`arg0: T + arg1: i32`) is currently
a hard error (`reconcile_typeparam_literal`, `NotLiteral` arm in `src/verifier/verifier.rs`),
with the hint "an explicit cast (planned) will be needed". Implement that cast so users can
bridge the gap by writing `arg0 + (arg1 as T)` (which becomes `T + T`).

- Grammar already parses it: `cast = { "as" ~ TYPE_PRIMITIVE }` and `TYPE_PRIMITIVE` includes
  `TY_TYPEPARAM`, so `x as T` is already grammatical — no grammar change needed.
- Type checker: handle `UnOp::Cast { target: DataType::TypeParam(name) }` — result type is that
  type parameter (symbolic); record the op requirement on it like any other use.
- Emitter/mono (`src/emitter/rewriting/mono.rs`): when substituting the site binding, resolve
  the cast target `T -> concrete` and let the existing concrete-cast emit path produce the
  conversion (`i32 -> f64`, etc. — already implemented for concrete casts).
- Tests: a `core_suite/polymorphism` case `arg0: T + (arg1 as T)` that compiles + runs, plus a
  verifier unit test that `arg1 as T` type-checks.
- Cleanup: once it lands, update the `reconcile_typeparam_literal` `NotLiteral` message to point
  users at the cast instead of saying "(planned)".

## To file

- [ ] relocate operand ownership to probe scope
- [ ] wei duplicate-export on conflicting bounds
- [ ] explicit cast between a type parameter and a concrete type (`arg1 as T`) — see section above
