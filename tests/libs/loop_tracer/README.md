# The LoopTracer Library #

Uses Wizard's Loop tracing backend.

To compile to Wasm:
```shell
$VIRGIL_HOME/bin/dev/v3c-wasm-wasi1 -fun-exprs -simple-bodies -program-name=tracer ./LoopTracer.v3 ./libs/*.v3 $VIRGIL_HOME/lib/util/*.v3 $WIZARD_HOME/src/util/PrefixTrie.v3 $WIZARD_HOME/src/util/Palette.v3 $WIZARD_HOME/src/util/TermColors.v3
```
