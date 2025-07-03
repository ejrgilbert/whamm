
(component
  (import "log" (func $log (param "msg" string)))
  (core module $libc
    (memory (export "memory") 1)
    (func (export "canonical_abi_realloc") (param i32 i32 i32 i32) (result i32)
      unreachable)
  )

  (core module $my_module
    (import "env" "log-utf8" (func $log_utf8 (param i32 i32)))
    (import "env" "log-utf16" (func $log_utf16 (param i32 i32)))
    (import "env" "log-compact-utf16" (func $log_compact_utf16 (param i32 i32)))

    (func (export "log-utf8") (param i32 i32)
      local.get 0
      local.get 1
      call $log_utf8
    )
    (func (export "log-utf16") (param i32 i32)
      local.get 0
      local.get 1
      call $log_utf16
    )
    (func (export "log-compact-utf16") (param i32 i32)
      local.get 0
      local.get 1
      call $log_compact_utf16
    )
  )

  (core instance $libc (instantiate $libc))

  (alias core export $libc "canonical_abi_realloc" (core func $realloc))
  (alias core export $libc "memory" (core memory $memory))
  (core func $log_lower_utf8 (canon lower (func $log) string-encoding=utf8 (memory $memory) (realloc $realloc)))
  (core func $log_lower_utf16 (canon lower (func $log) string-encoding=utf16 (memory $memory) (realloc $realloc)))
  (core func $log_lower_compact_utf16 (canon lower (func $log) string-encoding=latin1+utf16 (memory $memory) (realloc $realloc)))

  (core instance $my_instance (instantiate $my_module
    (with "libc" (instance $libc))
    (with "env" (instance
      (export "log-utf8" (func $log_lower_utf8))
      (export "log-utf16" (func $log_lower_utf16))
      (export "log-compact-utf16" (func $log_lower_compact_utf16))
    ))
  ))

  (func (export "log1") (param "msg" string)
    (canon lift
      (core func $my_instance "log-utf8")
      string-encoding=utf8
      (memory $memory)
      (realloc $realloc)
    )
  )
  (func (export "log2") (param "msg" string)
    (canon lift
      (core func $my_instance "log-utf16")
      string-encoding=utf16
      (memory $memory)
      (realloc $realloc)
    )
  )
  (func (export "log3") (param "msg" string)
    (canon lift
      (core func $my_instance "log-compact-utf16")
      string-encoding=latin1+utf16
      (memory $memory)
      (realloc $realloc)
    )
  )
)