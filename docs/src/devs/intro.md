# For `Whamm` Developers #

Do you want to contribute to `Whamm` or just learn about the low-level details for fun?
Then you're in the right place.

## Resources ##

Parsing:
- The [Pest book](https://pest.rs/book/)

## `Whamm` Implementation Concepts ##

The [_four phases_ of compilation](compiler_phases.md):
1. [Parse](parsing.md)
2. [Verify](verifying.md)
3. [Transform](devs/transform_ast.md) AST into the injection strategy's representation
4. [Emit](emit/emitting.md)

Other helpful concepts:
- The `Whamm` [CLI](cli.md)
- [Testing](testing.md)
- [Error Handling](error_handling.md)
