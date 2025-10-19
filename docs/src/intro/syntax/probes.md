# Probes #
`<probe_rule> / <predicate> / { <actions> }`

We use the term `probe` to refer to this triple of `probe_rule`, `predicate` and `actions`.

When performing bytecode rewriting, `Whamm`:
1. traverses the application's Wasm module to find the locations-of-interest as specified by each probe's `probe_rule`.
2. checks if the `predicate` evaluates to `false` statically
    - if it does evaluate to `false` it continues on, not injecting the probe's actions
    - if it does not evaluate to `false`, it injects the probe's actions at that location along with the folded `predicate`.
        - if the `predicate` evaluates to `true` statically, it will simply inject the actions into the program un-predicated.
        - if the `predicate` does not fold to a simple `boolean` value, it will inject predicated actions into this location.
          The predicate will then be evaluated dynamically when the application runs to conditionally execute the probe actions.

## Helpful `info` in CLI ##
`whamm info --help`

The `info` command provided by the CLI is a great resource to view what can be used as the probe match rule.
This command provides documentation describing the match rule parts as well as the bound variables and functions in scope, which can help users learn about how to build their instrumentation.

## The Probe Match Rule ##
`provider:package:event:mode`

The `probe_rule` is a way to express some "location" you want to instrument for your program.
It is a hierarchical with respect to `provider`, `package`, `event` and `mode.

| _part_       | _description_                                                                                                                               |
|--------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| **provider** | The name of the `provider` that supports this instrumentation capability used in this probe.                                                |
| **package**  | The name of the `package` within the specified provider that supports the instrumentation capability used in this probe.                    |
| **event**    | The name of the `event` that would correlate with the location to insert this probe in the instrumented program.                            |
| **mode**     | The name of the `mode` that should be used when emitting the probe actions at the `event`'s location, such as `before`, `after`, and `alt`. |

Each part of the `probe_rule` gradually increases in specificity until reaching the `mode` of your probe.
Consider the following example match rule: `wasm:opcode:br_if:before`.
This rule can be read as "Insert this probe _before_ each of the _br_if_ _Wasm_ _opcodes_ in my program."

Read through our [instrumentable events](../events.md) documentation for what we currently support and our future goals.

## The Predicate ##
`/ <predicate> /`

The `predicate` is a way to express some "conditional" you want to evaluate to `true` for the probe's actions to be executed.
It further constrains the match rule.
This aspect of a probe is optional to use.
If there is no `predicate` for some probe, the `actions` will always execute when the probe's location is reached during program execution.

### Constant Folding of Static Data ###

A probe can be predicated on both _static_ AND _dynamic_ data.
To support this, when targeting bytecode rewriting, the `whamm` compiler performs [constant propagation](https://en.wikipedia.org/wiki/Constant_folding) for statically-defined data.
Meaning that since static information is known at compile time, the values of those variables will be substituted and expressions can be partially evaluated.

As an example, consider the following probe:
`wasm:opcode:br_if:before / pc == 25 && arg0 == 1 / {..}`

This can be read as, attach a probe at the wasm opcode `br_if`, but only execute this logic if both the `pc` is 25 and the `arg0` evaluates to 1.
As we're traversing an application, we statically know when we're at the `pc` offset of 25 within a function.
BUT we don't know the value of `arg0` until the code is actually running!
This is where partial evaluation comes in.

Let's say we're at `pc` 0 within some function, constant propagation and partial evaluation would look like this:
```
0 == 25 && arg0 == 1
false && arg0 == 1
false
```
The predicate evaluated to `false` statically! This means we should _not_ attach the probe at this location, it's not a match!

Let's say we're at `pc` 25 within some function:
```
25 == 25 && arg0 == 1
true && arg0 == 1
arg0 == 1
```
Huh...we still have a part of the predicate left over! What should we do...
We still need to predicate on this part of the expression, we just need to inject this partially-evaluated expression to run at runtime!
So, we'd inject the probe body, wrapped with this condition!

Note that this does look a bit different for the `wei` target, `Whamm` has to do some other tinkering with the predicate there, which enables the engine to run the correct part of the predicate at the correct time (match time vs. runtime).

## The Actions ##
`{ <actions> }`

The `actions` are statements that are executed at the `probe_rule`'s location if the `predicate` evaluates to `true`.
