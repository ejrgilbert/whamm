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

## To file

- [ ] relocate operand ownership to probe scope
- [ ] wei duplicate-export on conflicting bounds
