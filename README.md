# Instrumentation DSL #

This DSL is inspired by the dtrace D language.

## Tutorials ##

To run basic build:
```shell
cargo build
```

To run tests:
```shell
cargo test
cargo test -- --nocapture # With stdout tracing
```

To run project (there are example dscripts in `tests/dscripts` folder):
```shell
cargo run <app_wasm_path> <path_to_dscript> <path_for_compiled_output>
```

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run <path_to_app_wasm> <path_to_dscript> <path_for_compiled_output>
```

# Design Planning #

Steps:
1. Create the AST
    - Contains everything!
2. Create the symbol table by traversing the AST
    - Just top-level stuff (Dscript and Probes)
    - Eventually:
        - Support global vars at Dscript scope
        - Support function defs at Dscript scope
3. Pass both of these to a CodeGenerator
    - Traverse the AST
        - Add in functions/variables built into DtraceCore and ProbeTypes to the SymbolTable for lookup
            - This structure keeps a logical separation between the provider symbols and the user symbols
            - These functions should have a boolean `used` that is `true` if it's been used in the
              body/predicate to show that it should be injected into the bytecode
        - Enter the probe bodies/predicates
            - If field/function is used, mark it by manipulating the `used` boolean
            - Add user-defined variables to symbol table
4. At this point we have enough information to emit code, we're doing another phase here in order to
   support bytecode rewriting OR Virgil Monitor creation. If we directly generated Wasm bytecodes in
   the last phase (like would have been done in MiniJava, we'd have to duplicate the above logic
   between the two targets.
    - Initialize a CodeEmitter
        - Initializes the application to instrument as a Walrus module
    - Generate the `provider` functions by iterating over the relevant parts of the SymbolTable
        - Call function in CodeEmitter that generates each of these, will be hardcoded Wasm Walrus
          code writing.
    - Traverse the AST
        - Tell the CodeEmitter to switch to whatever the relevant context is (probe, wasm, call, alt)
            - This will use Walrus to find the bytecodes we want to instrument!
        - Enter the predicates
            - Generate the code that does the comparisons as Walrus Wasm instructions
                - `Call` will know the logic for the various fields
                - We've already injected the functions from `DtraceCore`, will need to get the fn_ids to
                  insert a call to those functions with the specified args
        - Enter the probe bodies
            - `Call` will know the logic for the various fields
5. At this point we have an instrumented version of the application Module. Have Walrus write it to
   the specified output file!