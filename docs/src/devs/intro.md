# For `whamm!` Developers #

Do you want to contribute to `whamm!` or just learn about the low-level details for fun?
Then you're in the right place.

## Resources ##

Parsing:
- The [Pest book](https://pest.rs/book/)

## `whamm!` Implementation Concepts ##

The [_four phases_ of compilation](compiler_phases.md):
1. [Parse](parsing.md)
2. Configure the `Whamm!` [Core Library](./core_lib.md) (if needed)
3. [Verify](verifying.md)
4. [Translate](translate.md) AST into the injection strategy's representation
5. [Emit](emit/emitting.md)

Other helpful concepts:
- The `whamm!` [CLI](cli.md)
- [Testing](testing.md)
- [Error Handling](error_handling.md)
