(module
    ;; `strcmp` -- Compare two strings
    ;; Args:
    ;; - str0_offset: The offset in data that the first string starts at
    ;; - str0_size: The length of the first string
    ;; - str1_offset: The offset in data that the other string starts at
    ;; - str1_size: The length of the other string
    ;; Result:
    ;; - 0: if two strings are not equal
    ;; - non-zero: if two strings are equal
    (func $strcmp (param $str0_offset i32) (param $str0_size i32)
                  (param $str1_offset i32) (param $str1_size i32)
                  (result i32)
        (local $i i32) (local $str0_char i32) (local $str1_char i32)
        block $neq
            block $eq
                ;; 1. Check if sizes are equal, if not return 0
                local.get $str0_size
                local.get $str1_size
                i32.ne
                br_if $neq

                ;; 2. Check if mem offset is equal, if yes return non-zero
                ;;    (we are comparing the same data)
                local.get $str0_offset
                local.get $str1_offset
                i32.eq
                br_if $eq

                ;; 3. iterate over each string and check equivalence of chars, if any not equal, return 0
                i32.const 0
                local.set $i
                loop $cmp_char
                    ;; Check if we've reached the end of the string
                    local.get $i
                    local.get $str0_size  ;; (can compare with either str size, equal at this point)
    				i32.lt_u
    				i32.eqz
                    br_if $eq ;; We've reached the end without failing equality checks!

                    ;; get char for str0
                    local.get $str0_offset
                    local.get $i
                    i32.add
                    i32.load8_u
                    local.set $str0_char

                    ;; get char for str1
                    local.get $str1_offset
                    local.get $i
                    i32.add
                    i32.load8_u
                    local.set $str1_char

                    ;; compare the two chars
                    local.get $str0_char
                    local.get $str1_char
                    i32.ne
                    br_if $neq ;; If they are not equal, exit and return '0'

                    ;; Increment i and continue loop
                    local.get $i
                    i32.const 1
                    i32.add
                    local.set $i
                    br $cmp_char
                end
                ;; 4. Reached the end of each string without returning, return nonzero
                br $eq
            end
            ;; they are equal, return '1'
            i32.const 1
            return
        end
        ;; they are not equal, return '0'
        i32.const 0
        return
    )

    (func $test
        block $failed
            block $passed
                i32.const 0
                i32.const 0
                i32.const 0
                i32.const 0
                call $strcmp

                i32.const 1 ;; should be 1
                i32.ne
                br_if $failed

                i32.const 0
                i32.const 1
                i32.const 0
                i32.const 1
                call $strcmp

                i32.const 1 ;; should be 1
                i32.ne
                br_if $failed

                i32.const 0
                i32.const 1
                i32.const 1
                i32.const 1
                call $strcmp

                i32.const 0 ;; should be 0
                i32.ne
                br_if $failed

                i32.const 0
                i32.const 3
                i32.const 3
                i32.const 3
                call $strcmp

                i32.const 0 ;; should be 0
                i32.ne
                br_if $failed

                br $passed
            end
            return
        end
        unreachable ;; panic! something went wrong in testing
        return
    )

    (memory (export "memory") 1)
    (data (i32.const 0) "abcabd")

    ;; To test: `wasmtime strcmp.wat`
    (start $test) ;; Run the test function automatically
)