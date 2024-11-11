# The Language #

`whamm!` enables tool implementers to express their instrumentation in terms of program _events_ and corresponding _predicated actions_;
"When _this event_ occurs during program execution, do _these actions_ if _this predicate_ (or conditional) evaluates to `true`."
This abstraction provides a high-level and intuitive syntax that can target events at various granularities in the instrumented program.

Read on for an overview of the syntax and semantics of the language.

## Language Concepts ##
- [Variables](syntax/variables.md) are used to store data.
- [Logical operations](syntax/logop.md) can be used to combine boolean expressions.
- [Ternary Expressions](syntax/ternary.md) can be used for succinct conditional variable assignments.
- [Primitive types](syntax/primitives.md) are numbers, booleans, and strings.
- Various [arithmetic](syntax/arith.md) operations can be used with numbers.
- [Strings](syntax/strings.md) are key for dealing with text, etc.
- [Tuples](syntax/tuples.md) allow using multiple values where one value is expected.
- [Maps](syntax/maps.md) are key for storing large amounts of data, but they're implemented quite differently in `whamm!`.
- [Function](syntax/functions.md) definitions can be used to reuse code snippets.
- [Conditionals](syntax/conditionals.md) are if/else/elif statements used for simple control flow
- `Whamm!` also provides helpful features to enable instrumentation expressiveness
  - [Report](syntax/report_vars.md) variables are key for flushing the dynamic data collected by instrumentation during an application's run
  - [Unshared](syntax/unshared_vars.md) variables are used to create an instance of a variable per probe match-site whose value is retained across site visits
  - [Shared](syntax/shared_vars.md) variables are used to create an instance of a variable that is _shared_ by every probe match-site
- And finally, [probes](syntax/probes.md) are used to express instrumentation.
- All of this syntax is used to write `whamm!` [scripts](syntax/scripts.md).
