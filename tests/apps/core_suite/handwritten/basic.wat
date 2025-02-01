(module
    (type (;0;) (func (param i32 i32)))
    (type (;1;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
    (type (;2;) (func (result i32)))
    (import "ic0" "debug_print" (func $_ZN3ic03ic011debug_print17h185bba3b3a7fc989E (type 0)))
    (import "ic0" "call_new" (func $_ZN3ic03ic08call_new17h9ec0b706f1330e5bE (type 1)))

    (func $should_not_instrument
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        call $_ZN3ic03ic08call_new17h9ec0b706f1330e5bE
    )

    (func $should
        i32.const 0
        i32.const 0
        call $_ZN3ic03ic011debug_print17h185bba3b3a7fc989E
    )

    (func $inject_synchronous_fault (type 2) (result i32)
        i32.const 1647358
        i32.const 51
        call $_ZN3ic03ic011debug_print17h185bba3b3a7fc989E
        i32.const -1
    )

    (func $instr_redirect_to_fault_injector (type 1)
        i32.const 0
        drop
    )

    (func $main (export "main")
        call $should_not_instrument
        call $should
        call $inject_synchronous_fault
        drop
    )

    (memory (export "memory") 1)
)