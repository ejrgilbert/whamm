
Here is a high-level view of the grammar:
```
// Statements to initialize the global state of the instrumentation
global_statements;
...

// Function definitions to reuse code snippets
fn_name(fn_args) -> ret_val { fn_body; ... }
...

// An example of what a `probe` would look.
// There can be many of these in a monitor.
provider:package:event:mode / predicate / {
  probe_actions;
  ...
}
```