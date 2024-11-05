# Probes #
`<probe_rule> / <predicate> / { <actions> }`

We use the term `probe` to refer to this triple of `probe_rule`, `predicate` and `actions`.

When performing bytecode rewriting, `whamm!`:
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
This command provides documentation describing the match rule parts as well as the globals and functions in scope, which can help users learn about how to build their instrumentation.

## The Probe Match Rule ##
`provider:package:event:mode`

The `probe_rule` is a way to express some "location" you want to instrument for your program.

| _part_       | _description_                                                                                                                               |
|--------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| **provider** | The name of the `provider` that supports this instrumentation capability used in this probe.                                                |
| **package**  | The name of the `package` within the specified provider that supports the instrumentation capability used in this probe.                    |
| **event**    | The name of the `event` that would correlate with the location to insert this probe in the instrumented program.                            |
| **mode**     | The name of the `mode` that should be used when emitting the probe actions at the `event`'s location, such as `before`, `after`, and `alt`. |

Each part of the `probe_rule` gradually increases in specificity until reaching the `mode` of your probe.
Consider the following example match rule: `wasm:bytecode:br_if:before`.
This rule can be read as "Insert this probe _before_ each of the _br_if_ _Wasm_ _bytecode_ instructions in my program."

Read through our [instrumentable events](../events.md) documentation for what we currently support and our future goals.

## The Predicate ##
`/ <predicate> /`

The `predicate` is a way to express some "conditional" you want to evaluate to `true` for the probe's actions to be executed.
This aspect of a probe is optional to use.
If there is no `predicate` for some probe, the `actions` will always execute when the probe's location is reached during program execution.

## The Actions ##
`{ <actions> }`

The `actions` are statements that are executed at the `probe_rule`'s location if the `predicate` evaluates to `true`.
