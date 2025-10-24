use toggle;

// deep nesting
wasm:opcode:*:before /
    @static toggle.should_inject(
        @static toggle.should_inject(
            @static toggle.get_value(),
            @static toggle.should_inject(
                @static toggle.get_value(),
                @static toggle.get_value()
            )
        ),
        @static toggle.get_value()
    ) as bool
/ {
    report var val: i32;
}