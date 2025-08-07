(component
  (type (;0;)
    (instance
      (type (;0;) (tuple string string))
      (type (;1;) (list 0))
      (type (;2;) (func (result 1)))
      (export (;0;) "get-environment" (func (type 2)))
    )
  )
  (import "wasi:cli/environment@0.2.3" (instance (;0;) (type 0)))
  (type (;1;)
    (instance
      (type (;0;) (result))
      (type (;1;) (func (param "status" 0)))
      (export (;0;) "exit" (func (type 1)))
    )
  )
  (import "wasi:cli/exit@0.2.3" (instance (;1;) (type 1)))
  (type (;2;)
    (instance
      (export (;0;) "error" (type (sub resource)))
    )
  )
  (import "wasi:io/error@0.2.3" (instance (;2;) (type 2)))
  (alias export 2 "error" (type (;3;)))
  (type (;4;)
    (instance
      (export (;0;) "output-stream" (type (sub resource)))
      (alias outer 1 3 (type (;1;)))
      (export (;2;) "error" (type (eq 1)))
      (type (;3;) (own 2))
      (type (;4;) (variant (case "last-operation-failed" 3) (case "closed")))
      (export (;5;) "stream-error" (type (eq 4)))
      (export (;6;) "input-stream" (type (sub resource)))
      (type (;7;) (borrow 0))
      (type (;8;) (result u64 (error 5)))
      (type (;9;) (func (param "self" 7) (result 8)))
      (export (;0;) "[method]output-stream.check-write" (func (type 9)))
      (type (;10;) (list u8))
      (type (;11;) (result (error 5)))
      (type (;12;) (func (param "self" 7) (param "contents" 10) (result 11)))
      (export (;1;) "[method]output-stream.write" (func (type 12)))
      (export (;2;) "[method]output-stream.blocking-write-and-flush" (func (type 12)))
      (type (;13;) (func (param "self" 7) (result 11)))
      (export (;3;) "[method]output-stream.blocking-flush" (func (type 13)))
    )
  )
  (import "wasi:io/streams@0.2.3" (instance (;3;) (type 4)))
  (alias export 3 "input-stream" (type (;5;)))
  (type (;6;)
    (instance
      (alias outer 1 5 (type (;0;)))
      (export (;1;) "input-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdin" (func (type 3)))
    )
  )
  (import "wasi:cli/stdin@0.2.3" (instance (;4;) (type 6)))
  (alias export 3 "output-stream" (type (;7;)))
  (type (;8;)
    (instance
      (alias outer 1 7 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdout" (func (type 3)))
    )
  )
  (import "wasi:cli/stdout@0.2.3" (instance (;5;) (type 8)))
  (alias export 3 "output-stream" (type (;9;)))
  (type (;10;)
    (instance
      (alias outer 1 9 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stderr" (func (type 3)))
    )
  )
  (import "wasi:cli/stderr@0.2.3" (instance (;6;) (type 10)))
  (type (;11;)
    (instance
      (type (;0;) (record (field "seconds" u64) (field "nanoseconds" u32)))
      (export (;1;) "datetime" (type (eq 0)))
    )
  )
  (import "wasi:clocks/wall-clock@0.2.3" (instance (;7;) (type 11)))
  (alias export 3 "output-stream" (type (;12;)))
  (alias export 7 "datetime" (type (;13;)))
  (alias export 3 "error" (type (;14;)))
  (type (;15;)
    (instance
      (export (;0;) "descriptor" (type (sub resource)))
      (type (;1;) u64)
      (export (;2;) "filesize" (type (eq 1)))
      (alias outer 1 12 (type (;3;)))
      (export (;4;) "output-stream" (type (eq 3)))
      (type (;5;) (enum "access" "would-block" "already" "bad-descriptor" "busy" "deadlock" "quota" "exist" "file-too-large" "illegal-byte-sequence" "in-progress" "interrupted" "invalid" "io" "is-directory" "loop" "too-many-links" "message-size" "name-too-long" "no-device" "no-entry" "no-lock" "insufficient-memory" "insufficient-space" "not-directory" "not-empty" "not-recoverable" "unsupported" "no-tty" "no-such-device" "overflow" "not-permitted" "pipe" "read-only" "invalid-seek" "text-file-busy" "cross-device"))
      (export (;6;) "error-code" (type (eq 5)))
      (type (;7;) (enum "unknown" "block-device" "character-device" "directory" "fifo" "symbolic-link" "regular-file" "socket"))
      (export (;8;) "descriptor-type" (type (eq 7)))
      (type (;9;) u64)
      (export (;10;) "link-count" (type (eq 9)))
      (alias outer 1 13 (type (;11;)))
      (export (;12;) "datetime" (type (eq 11)))
      (type (;13;) (option 12))
      (type (;14;) (record (field "type" 8) (field "link-count" 10) (field "size" 2) (field "data-access-timestamp" 13) (field "data-modification-timestamp" 13) (field "status-change-timestamp" 13)))
      (export (;15;) "descriptor-stat" (type (eq 14)))
      (alias outer 1 14 (type (;16;)))
      (export (;17;) "error" (type (eq 16)))
      (type (;18;) (borrow 0))
      (type (;19;) (own 4))
      (type (;20;) (result 19 (error 6)))
      (type (;21;) (func (param "self" 18) (param "offset" 2) (result 20)))
      (export (;0;) "[method]descriptor.write-via-stream" (func (type 21)))
      (type (;22;) (func (param "self" 18) (result 20)))
      (export (;1;) "[method]descriptor.append-via-stream" (func (type 22)))
      (type (;23;) (result 8 (error 6)))
      (type (;24;) (func (param "self" 18) (result 23)))
      (export (;2;) "[method]descriptor.get-type" (func (type 24)))
      (type (;25;) (result 15 (error 6)))
      (type (;26;) (func (param "self" 18) (result 25)))
      (export (;3;) "[method]descriptor.stat" (func (type 26)))
      (type (;27;) (borrow 17))
      (type (;28;) (option 6))
      (type (;29;) (func (param "err" 27) (result 28)))
      (export (;4;) "filesystem-error-code" (func (type 29)))
    )
  )
  (import "wasi:filesystem/types@0.2.3" (instance (;8;) (type 15)))
  (alias export 8 "descriptor" (type (;16;)))
  (type (;17;)
    (instance
      (alias outer 1 16 (type (;0;)))
      (export (;1;) "descriptor" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (tuple 2 string))
      (type (;4;) (list 3))
      (type (;5;) (func (result 4)))
      (export (;0;) "get-directories" (func (type 5)))
    )
  )
  (import "wasi:filesystem/preopens@0.2.3" (instance (;9;) (type 17)))
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32) (result i32)))
    (type (;3;) (func (param i32) (result i32)))
    (type (;4;) (func (param i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32)))
    (type (;7;) (func (param i32 i32 i32 i32)))
    (type (;8;) (func (param i32 i64 i32) (result i64)))
    (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;10;) (func (result i32)))
    (type (;11;) (func (param i32 i32 i32 i32 i32)))
    (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type (;13;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;14;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;15;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;16;) (func (param i64)))
    (type (;17;) (func (param f32)))
    (type (;18;) (func (param f64)))
    (type (;19;) (func (param i64 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h5858df6e6eba6e92E (;0;) (type 9)))
    (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;1;) (type 2)))
    (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;2;) (type 2)))
    (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;3;) (type 1)))
    (import "whamm_core" "putc" (func $putc (;4;) (type 1)))
    (import "whamm_core" "puts" (func $puts (;5;) (type 6)))
    (import "whamm_core" "putu8" (func $putu8 (;6;) (type 1)))
    (import "whamm_core" "puti8" (func $puti8 (;7;) (type 1)))
    (import "whamm_core" "putu16" (func $putu16 (;8;) (type 1)))
    (import "whamm_core" "puti16" (func $puti16 (;9;) (type 1)))
    (import "whamm_core" "putu32" (func $putu32 (;10;) (type 1)))
    (import "whamm_core" "puti32" (func $puti32 (;11;) (type 1)))
    (import "whamm_core" "putu64" (func $putu64 (;12;) (type 16)))
    (import "whamm_core" "puti64" (func $puti64 (;13;) (type 16)))
    (import "whamm_core" "putf32" (func $putf32 (;14;) (type 17)))
    (import "whamm_core" "putf64" (func $putf64 (;15;) (type 18)))
    (import "whamm_core" "putbool" (func $putbool (;16;) (type 1)))
    (table (;0;) 70 70 funcref)
    (memory (;0;) 17)
    (memory (;1;) 1)
    (memory (;2;) 1)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (global $GOT.data.internal.__memory_base (;1;) i32 i32.const 0)
    (global $GOT.data.internal.__table_base (;2;) i32 i32.const 1)
    (global $GOT.data.internal.__rust_no_alloc_shim_is_unstable (;3;) i32 i32.const 1055773)
    (global $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE (;4;) i32 i32.const 2)
    (global $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E (;5;) i32 i32.const 6)
    (global $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E (;6;) i32 i32.const 1051407)
    (global $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE (;7;) i32 i32.const 1055848)
    (global $GOT.data.internal.errno (;8;) i32 i32.const 1056384)
    (global $GOT.func.internal._ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E (;9;) i32 i32.const 9)
    (global $GOT.func.internal._ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE (;10;) i32 i32.const 10)
    (global $GOT.data.internal._RNvCscSpY9Juk0HT_7___rustc39___rust_alloc_error_handler_should_panic (;11;) i32 i32.const 1055772)
    (global $GOT.func.internal._ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h0c6ce12b01068af4E (;12;) i32 i32.const 6)
    (global $GOT.data.internal._ZN3std6thread7current7CURRENT17h17ed3b660549d676E (;13;) i32 i32.const 1055884)
    (global $GOT.data.internal._ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h79e1014250ce3477E (;14;) i32 i32.const 1055844)
    (global $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E (;15;) i32 i32.const 1055832)
    (global $GOT.func.internal._ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E (;16;) i32 i32.const 63)
    (global $GOT.func.internal._ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE (;17;) i32 i32.const 66)
    (global (;18;) (mut i32) i32.const 9998)
    (global (;19;) (mut i32) i32.const 0)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
    (export "__main_void" (func $__main_void))
    (export "calc" (func $calc))
    (export "print_x" (func $print_x))
    (export "opt_str" (func $opt_str))
    (export "main" (func $main))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE $main $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E $"#func66 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E $cabi_realloc $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E $_ZN4core3fmt5Write10write_char17ha202ea95069de918E $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E)
    (func $__wasm_call_ctors (;17;) (type 0))
    (func $_start (;18;) (type 0)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          global.get $GOT.data.internal.__memory_base
          i32.const 1055768
          i32.add
          i32.load
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          i32.const 1055768
          i32.add
          i32.const 1
          i32.store
          i32.const 0
          local.set 1
          local.get 1
          local.get 1
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $__wasm_call_ctors
          i32.const 32
          local.set 1
          local.get 1
          local.get 1
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $__main_void
          local.set 0
          i32.const 64
          local.set 1
          local.get 1
          local.get 1
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $__wasm_call_dtors
          local.get 0
          br_if 1 (;@1;)
          return
        end
        unreachable
      end
      local.get 0
      i32.const 96
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__wasi_proc_exit
      unreachable
    )
    (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE (;19;) (type 3) (param i32) (result i32)
      (local i32)
      local.get 0
      i32.load
      i32.const 128
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E (;20;) (type 1) (param i32)
      local.get 0
      call_indirect (type 0)
    )
    (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E (;21;) (type 3) (param i32) (result i32)
      (local i32)
      local.get 0
      i32.load
      i32.const 160
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE (;22;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.get 1
      i32.const 192
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $calc (;23;) (type 2) (param i32 i32) (result i32)
      local.get 1
      local.get 0
      i32.add
      local.get 1
      local.get 0
      i32.mul
      local.get 0
      i32.const 5
      i32.gt_s
      select
    )
    (func $print_x (;24;) (type 6) (param i32 i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      i32.store8 offset=15
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        global.get $GOT.data.internal.__table_base
        i32.const 0
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 2
        i32.const 52
        i32.add
        i64.extend_i32_u
        i64.or
        local.set 3
        loop ;; label = @2
          local.get 2
          i32.const 52
          i32.add
          local.get 2
          i32.const 15
          i32.add
          i32.const 224
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $opt_str
          local.get 2
          i32.const 2
          i32.store offset=20
          local.get 2
          global.get $GOT.data.internal.__memory_base
          i32.const 1054152
          i32.add
          i32.store offset=16
          local.get 2
          local.get 3
          i64.store offset=40
          local.get 2
          i64.const 1
          i64.store offset=28 align=4
          local.get 2
          local.get 2
          i32.const 40
          i32.add
          i32.store offset=24
          local.get 2
          i32.const 16
          i32.add
          i32.const 256
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
          block ;; label = @3
            local.get 2
            i32.load offset=52
            local.tee 0
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=56
            local.get 0
            i32.const 1
            i32.const 288
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 1
          i32.const -1
          i32.add
          local.tee 1
          br_if 0 (;@2;)
        end
      end
      local.get 2
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $opt_str (;25;) (type 6) (param i32 i32)
      (local i32 i32 i32)
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.load8_u
                    br_table 0 (;@7;) 1 (;@6;) 2 (;@5;) 0 (;@7;)
                  end
                  i32.const 2
                  local.set 2
                  i32.const 2
                  i32.const 1
                  i32.const 320
                  local.set 4
                  local.get 4
                  local.get 4
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                  local.tee 1
                  i32.eqz
                  br_if 3 (;@3;)
                  local.get 1
                  i32.const 26984
                  i32.store16 align=1
                  br 2 (;@4;)
                end
                i32.const 5
                local.set 2
                i32.const 5
                i32.const 1
                i32.const 352
                local.set 4
                local.get 4
                local.get 4
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                local.tee 1
                i32.eqz
                br_if 3 (;@2;)
                local.get 1
                global.get $GOT.data.internal.__memory_base
                i32.const 1048694
                i32.add
                local.tee 3
                i32.load align=1
                i32.store align=1
                local.get 1
                i32.const 4
                i32.add
                local.get 3
                i32.const 4
                i32.add
                i32.load8_u
                i32.store8
                br 1 (;@4;)
              end
              i32.const 4
              local.set 2
              i32.const 4
              i32.const 1
              i32.const 384
              local.set 4
              local.get 4
              local.get 4
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee 1
              i32.eqz
              br_if 3 (;@1;)
              local.get 1
              i32.const 1886745383
              i32.store align=1
            end
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 1
            i32.store offset=4
            local.get 0
            local.get 2
            i32.store
            return
          end
          i32.const 1
          i32.const 2
          global.get $GOT.data.internal.__memory_base
          i32.const 1054136
          i32.add
          i32.const 416
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
          unreachable
        end
        i32.const 1
        i32.const 5
        global.get $GOT.data.internal.__memory_base
        i32.const 1054136
        i32.add
        i32.const 448
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 1
      i32.const 4
      global.get $GOT.data.internal.__memory_base
      i32.const 1054136
      i32.add
      i32.const 480
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $main (;26;) (type 0)
      (local i32 i32 i32 i64 i64 i64 i32)
      block (type 0) ;; label = @1
        global.get $__stack_pointer
        i32.const 80
        i32.sub
        local.tee 0
        global.set $__stack_pointer
        local.get 0
        i32.const 0
        i32.store offset=64
        local.get 0
        i32.const 1
        i32.store offset=52
        local.get 0
        i64.const 4
        i64.store offset=56 align=4
        local.get 0
        global.get $GOT.data.internal.__memory_base
        local.tee 1
        i32.const 1054168
        i32.add
        i32.store offset=48
        local.get 0
        i32.const 48
        i32.add
        i32.const 512
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 9
        i32.store offset=12
        local.get 0
        i32.const 0
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 1
        i32.const 1054184
        i32.add
        local.tee 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.tee 3
        local.get 0
        i32.const 72
        i32.add
        i64.extend_i32_u
        i64.or
        local.tee 4
        i64.store offset=64
        local.get 0
        local.get 3
        local.get 0
        i32.const 16
        i32.add
        i64.extend_i32_u
        i64.or
        local.tee 5
        i64.store offset=56
        local.get 0
        local.get 3
        local.get 0
        i32.const 12
        i32.add
        i64.extend_i32_u
        i64.or
        local.tee 3
        i64.store offset=48
        local.get 0
        i32.const 9
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 544
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 8
        i32.store offset=12
        local.get 0
        i32.const 1
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 9
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 576
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 7
        i32.store offset=12
        local.get 0
        i32.const 2
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 9
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 608
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 6
        i32.store offset=12
        local.get 0
        i32.const 3
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 9
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 640
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 5
        i32.store offset=12
        local.get 0
        i32.const 4
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 20
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 672
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 4
        i32.store offset=12
        local.get 0
        i32.const 5
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 20
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 704
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 3
        i32.store offset=12
        local.get 0
        i32.const 6
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 18
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 736
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 2
        i32.store offset=12
        local.get 0
        i32.const 7
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 14
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 768
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 1
        i32.store offset=12
        local.get 0
        i32.const 8
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 8
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 800
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 0
        i32.store offset=12
        local.get 0
        i32.const 9
        i32.store offset=16
        local.get 0
        i32.const 4
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=20
        local.get 0
        i64.const 3
        i64.store offset=32 align=4
        local.get 0
        local.get 4
        i64.store offset=64
        local.get 0
        local.get 5
        i64.store offset=56
        local.get 0
        local.get 3
        i64.store offset=48
        local.get 0
        i32.const 0
        i32.store offset=72
        local.get 0
        local.get 0
        i32.const 48
        i32.add
        i32.store offset=28
        local.get 0
        i32.const 20
        i32.add
        i32.const 832
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 0
        i32.store offset=64
        local.get 0
        i32.const 1
        i32.store offset=52
        local.get 0
        local.get 1
        i32.const 1054176
        i32.add
        i32.store offset=48
        local.get 0
        i64.const 4
        i64.store offset=56 align=4
        local.get 0
        i32.const 48
        i32.add
        i32.const 864
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        local.get 0
        i32.const 0
        i32.store8 offset=47
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 896
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        local.get 1
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        global.get $GOT.data.internal.__table_base
        i32.const 0
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 0
        i32.const 20
        i32.add
        i64.extend_i32_u
        i64.or
        local.tee 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 928
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 960
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 992
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1024
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1056
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1088
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1120
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1152
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1184
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1216
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1248
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1280
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1312
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1344
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1376
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1408
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1440
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1472
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1504
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1536
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1568
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1600
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1632
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1664
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1696
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1728
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1760
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1792
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1824
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 1
        i32.store8 offset=47
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1856
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1888
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 1920
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 1952
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 1984
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2016
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2048
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2080
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2112
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2144
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2176
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2208
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2240
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2272
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2304
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2336
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2368
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2400
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2432
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2464
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2496
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2528
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2560
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2592
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2624
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2656
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2688
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2720
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2752
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2784
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 2
        i32.store8 offset=47
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2816
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2848
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2880
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 2912
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 2944
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 2976
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3008
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3040
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3072
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3104
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3136
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3168
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3200
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3232
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3264
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3296
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3328
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3360
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3392
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3424
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3456
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3488
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3520
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3552
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3584
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3616
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3648
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 20
        i32.add
        local.get 0
        i32.const 47
        i32.add
        i32.const 3680
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $opt_str
        local.get 0
        i32.const 2
        i32.store offset=52
        local.get 0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054152
        i32.add
        i32.store offset=48
        local.get 0
        local.get 4
        i64.store offset=72
        local.get 0
        i64.const 1
        i64.store offset=60 align=4
        local.get 0
        local.get 0
        i32.const 72
        i32.add
        i32.store offset=56
        local.get 0
        i32.const 48
        i32.add
        i32.const 3712
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 2
          i32.const 1
          i32.const 3744
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 80
        i32.add
        global.set $__stack_pointer
      end
      call $on_exit
    )
    (func $__main_void (;27;) (type 10) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      local.get 0
      global.get $GOT.data.internal.__table_base
      i32.const 2
      i32.add
      i32.store offset=12
      local.get 0
      i32.const 12
      i32.add
      global.get $GOT.data.internal.__memory_base
      i32.const 1054112
      i32.add
      i32.const 0
      i32.const 0
      i32.const 0
      i32.const 3776
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E
      local.set 1
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc (;28;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      local.get 1
      i32.const 3808
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc (;29;) (type 4) (param i32 i32 i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 3840
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc (;30;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 3872
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc
      local.set 4
      local.get 4
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler (;31;) (type 6) (param i32 i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 3904
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom
      return
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E (;32;) (type 6) (param i32 i32)
      local.get 0
      i64.const -245993367077761921
      i64.store offset=8
      local.get 0
      i64.const 6756087622182587336
      i64.store
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E (;33;) (type 6) (param i32 i32)
      local.get 0
      i64.const 7199936582794304877
      i64.store offset=8
      local.get 0
      i64.const -5076933981314334344
      i64.store
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E (;34;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      i32.load
      local.set 0
      block ;; label = @1
        local.get 1
        i32.load offset=8
        local.tee 2
        i32.const 33554432
        i32.and
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 2
          i32.const 67108864
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          i32.const 3936
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E
          return
        end
        local.get 0
        local.get 1
        i32.const 3968
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E
        return
      end
      local.get 0
      local.get 1
      i32.const 4000
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E (;35;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      i32.load
      local.get 1
      i32.const 4032
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E (;36;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 0
          i32.load8_u
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 2
          local.get 0
          i32.const 1
          i32.add
          i32.store offset=12
          local.get 1
          global.get $GOT.data.internal.__memory_base
          local.tee 0
          i32.const 1049038
          i32.add
          i32.const 4
          local.get 2
          i32.const 12
          i32.add
          local.get 0
          i32.const 1054220
          i32.add
          i32.const 4064
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E
          local.set 0
          br 1 (;@1;)
        end
        local.get 1
        global.get $GOT.data.internal.__memory_base
        i32.const 1049034
        i32.add
        i32.const 4
        i32.const 4096
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
        local.set 0
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E (;37;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 4128
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE (;38;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load
      local.set 4
      local.get 0
      i32.load
      local.set 1
      local.get 2
      i32.const 3
      i32.store offset=4
      local.get 2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054332
      i32.add
      i32.store
      local.get 2
      i64.const 3
      i64.store offset=12 align=4
      local.get 2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 5
      local.get 1
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 2
      local.get 5
      local.get 1
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 2
      global.get $GOT.data.internal.__table_base
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 1
      i64.extend_i32_u
      i64.or
      i64.store offset=24
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=8
      local.get 4
      local.get 3
      local.get 2
      i32.const 4160
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
      local.set 1
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE (;39;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 1
        i32.load offset=8
        local.tee 2
        i32.const 33554432
        i32.and
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 2
          i32.const 67108864
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          i32.const 4192
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
          return
        end
        local.get 0
        local.get 1
        i32.const 4224
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE
        return
      end
      local.get 0
      local.get 1
      i32.const 4256
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE
    )
    (func $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE (;40;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i64 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=4
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 1
              i32.const 65536
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=7
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=4
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=6
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=5
              i32.const 4
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=4
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            i32.const 3
            local.set 1
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=5
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=4
          i32.const 2
          local.set 1
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=4
        i32.const 1
        local.set 1
      end
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.load offset=8
      local.get 2
      i32.const 4
      i32.add
      local.get 1
      i32.const 4288
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E
      block ;; label = @1
        local.get 2
        i32.load8_u offset=8
        local.tee 1
        i32.const 4
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 3
        local.get 2
        i64.load offset=8
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 5
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 5
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 3
          i32.load
          local.set 6
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.add
            i32.load
            local.tee 5
            i32.load
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 5
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            local.get 5
            i32.load offset=8
            i32.const 4320
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
          i32.const 4352
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        local.get 4
        i64.store align=4
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
      i32.const 4
      i32.ne
    )
    (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E (;41;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        local.get 1
                        i32.load
                        local.tee 1
                        i32.load offset=16
                        br_if 0 (;@9;)
                        local.get 1
                        i32.const -1
                        i32.store offset=16
                        local.get 4
                        i32.const 8
                        i32.add
                        i32.const 10
                        local.get 2
                        local.get 3
                        i32.const 4384
                        local.set 14
                        local.get 14
                        local.get 14
                        i32.load 2 offset=28 align=1
                        i32.const 1
                        i32.add
                        i32.store 2 offset=28 align=1
                        call $_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E
                        block ;; label = @10
                          block ;; label = @11
                            local.get 4
                            i32.load offset=8
                            i32.const 1
                            i32.and
                            i32.eqz
                            br_if 0 (;@11;)
                            local.get 3
                            local.get 4
                            i32.load offset=12
                            i32.const 1
                            i32.add
                            local.tee 5
                            i32.ge_u
                            br_if 1 (;@10;)
                            local.get 4
                            i32.const 0
                            i32.store offset=32
                            local.get 4
                            i32.const 1
                            i32.store offset=20
                            local.get 4
                            i64.const 4
                            i64.store offset=24 align=4
                            local.get 4
                            global.get $GOT.data.internal.__memory_base
                            local.tee 1
                            i32.const 1054544
                            i32.add
                            i32.store offset=16
                            local.get 4
                            i32.const 16
                            i32.add
                            local.get 1
                            i32.const 1054552
                            i32.add
                            i32.const 4416
                            local.set 14
                            local.get 14
                            local.get 14
                            i32.load 2 offset=28 align=1
                            i32.const 1
                            i32.add
                            i32.store 2 offset=28 align=1
                            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                            unreachable
                          end
                          block ;; label = @11
                            local.get 1
                            i32.load offset=28
                            local.tee 6
                            br_if 0 (;@11;)
                            i32.const 0
                            local.set 6
                            br 8 (;@3;)
                          end
                          local.get 1
                          i32.load offset=24
                          local.tee 7
                          local.get 6
                          i32.add
                          i32.const -1
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.ne
                          br_if 7 (;@3;)
                          i32.const 0
                          local.set 8
                          loop ;; label = @11
                            local.get 4
                            local.get 6
                            local.get 8
                            i32.sub
                            local.tee 9
                            i32.store offset=44
                            local.get 4
                            local.get 7
                            local.get 8
                            i32.add
                            local.tee 10
                            i32.store offset=40
                            local.get 4
                            i32.const 16
                            i32.add
                            i32.const 1
                            local.get 4
                            i32.const 40
                            i32.add
                            i32.const 1
                            i32.const 4448
                            local.set 14
                            local.get 14
                            local.get 14
                            i32.load 2 offset=28 align=1
                            i32.const 1
                            i32.add
                            i32.store 2 offset=28 align=1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 4
                                      i32.load16_u offset=16
                                      i32.const 1
                                      i32.ne
                                      br_if 0 (;@16;)
                                      local.get 9
                                      local.set 11
                                      local.get 4
                                      i32.load16_u offset=18
                                      local.tee 5
                                      i32.const 8
                                      i32.eq
                                      br_if 1 (;@15;)
                                      local.get 1
                                      i32.const 0
                                      i32.store8 offset=32
                                      local.get 5
                                      i32.const 27
                                      i32.eq
                                      br_if 4 (;@12;)
                                      local.get 5
                                      i64.extend_i32_u
                                      i64.const 32
                                      i64.shl
                                      local.set 12
                                      br 2 (;@14;)
                                    end
                                    local.get 4
                                    i32.load offset=20
                                    local.set 11
                                  end
                                  local.get 1
                                  i32.const 0
                                  i32.store8 offset=32
                                  local.get 11
                                  br_if 1 (;@13;)
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054532
                                  i32.add
                                  i64.extend_i32_u
                                  i64.const 32
                                  i64.shl
                                  i64.const 2
                                  i64.or
                                  local.set 12
                                end
                                block ;; label = @14
                                  local.get 8
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  block ;; label = @15
                                    local.get 9
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    local.get 7
                                    local.get 10
                                    local.get 9
                                    memory.copy
                                  end
                                  local.get 1
                                  local.get 9
                                  i32.store offset=28
                                end
                                local.get 12
                                i64.const 255
                                i64.and
                                i64.const 4
                                i64.ne
                                br_if 5 (;@8;)
                                local.get 1
                                i32.load offset=28
                                local.set 6
                                br 10 (;@3;)
                              end
                              local.get 11
                              local.get 8
                              i32.add
                              local.set 8
                            end
                            local.get 8
                            local.get 6
                            i32.ge_u
                            br_if 7 (;@4;)
                            br 0 (;@11;)
                          end
                        end
                        block ;; label = @10
                          local.get 1
                          i32.load offset=28
                          local.tee 8
                          br_if 0 (;@10;)
                          local.get 5
                          i32.eqz
                          br_if 5 (;@5;)
                          local.get 2
                          local.set 6
                          local.get 5
                          local.set 8
                          loop ;; label = @11
                            local.get 4
                            local.get 8
                            i32.store offset=44
                            local.get 4
                            local.get 6
                            i32.store offset=40
                            local.get 4
                            i32.const 16
                            i32.add
                            i32.const 1
                            local.get 4
                            i32.const 40
                            i32.add
                            i32.const 1
                            i32.const 4480
                            local.set 14
                            local.get 14
                            local.get 14
                            i32.load 2 offset=28 align=1
                            i32.const 1
                            i32.add
                            i32.store 2 offset=28 align=1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.load16_u offset=16
                                    i32.const 1
                                    i32.ne
                                    br_if 0 (;@15;)
                                    local.get 4
                                    i64.load16_u offset=18
                                    local.tee 12
                                    i64.const 27
                                    i64.eq
                                    br_if 3 (;@12;)
                                    local.get 12
                                    i64.const 32
                                    i64.shl
                                    local.set 12
                                    br 1 (;@14;)
                                  end
                                  local.get 4
                                  i32.load offset=20
                                  local.tee 11
                                  br_if 1 (;@13;)
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054584
                                  i32.add
                                  i64.load
                                  local.set 12
                                end
                                local.get 12
                                i64.const 255
                                i64.and
                                i64.const 4
                                i64.eq
                                br_if 8 (;@5;)
                                local.get 12
                                i64.const -4294967041
                                i64.and
                                i64.const 34359738368
                                i64.eq
                                br_if 8 (;@5;)
                                local.get 0
                                local.get 12
                                i64.store align=4
                                br 11 (;@2;)
                              end
                              local.get 8
                              local.get 11
                              i32.lt_u
                              br_if 5 (;@7;)
                              local.get 6
                              local.get 11
                              i32.add
                              local.set 6
                              local.get 8
                              local.get 11
                              i32.sub
                              local.set 8
                            end
                            local.get 8
                            br_if 0 (;@11;)
                            br 6 (;@5;)
                          end
                        end
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              local.get 5
                              local.get 1
                              i32.load offset=20
                              local.get 8
                              i32.sub
                              i32.lt_u
                              br_if 0 (;@12;)
                              local.get 4
                              i32.const 16
                              i32.add
                              local.get 1
                              i32.const 20
                              i32.add
                              local.get 2
                              local.get 5
                              i32.const 4512
                              local.set 14
                              local.get 14
                              local.get 14
                              i32.load 2 offset=28 align=1
                              i32.const 1
                              i32.add
                              i32.store 2 offset=28 align=1
                              call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
                              local.get 4
                              i32.load8_u offset=16
                              i32.const 4
                              i32.eq
                              br_if 1 (;@11;)
                              local.get 0
                              local.get 4
                              i64.load offset=16
                              i64.store align=4
                              br 10 (;@2;)
                            end
                            block ;; label = @12
                              local.get 5
                              i32.eqz
                              br_if 0 (;@12;)
                              local.get 1
                              i32.load offset=24
                              local.get 8
                              i32.add
                              local.get 2
                              local.get 5
                              memory.copy
                            end
                            local.get 1
                            local.get 8
                            local.get 5
                            i32.add
                            local.tee 11
                            i32.store offset=28
                            br 1 (;@10;)
                          end
                          local.get 1
                          i32.load offset=28
                          local.set 11
                        end
                        local.get 11
                        i32.eqz
                        br_if 4 (;@5;)
                        local.get 1
                        i32.load offset=24
                        local.set 7
                        i32.const 0
                        local.set 8
                        loop ;; label = @10
                          local.get 4
                          local.get 11
                          local.get 8
                          i32.sub
                          local.tee 9
                          i32.store offset=44
                          local.get 4
                          local.get 7
                          local.get 8
                          i32.add
                          local.tee 10
                          i32.store offset=40
                          local.get 4
                          i32.const 16
                          i32.add
                          i32.const 1
                          local.get 4
                          i32.const 40
                          i32.add
                          i32.const 1
                          i32.const 4544
                          local.set 14
                          local.get 14
                          local.get 14
                          i32.load 2 offset=28 align=1
                          i32.const 1
                          i32.add
                          i32.store 2 offset=28 align=1
                          call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.load16_u offset=16
                                    i32.const 1
                                    i32.ne
                                    br_if 0 (;@15;)
                                    local.get 9
                                    local.set 6
                                    local.get 4
                                    i32.load16_u offset=18
                                    local.tee 13
                                    i32.const 8
                                    i32.eq
                                    br_if 1 (;@14;)
                                    local.get 1
                                    i32.const 0
                                    i32.store8 offset=32
                                    local.get 13
                                    i32.const 27
                                    i32.eq
                                    br_if 4 (;@11;)
                                    local.get 13
                                    i64.extend_i32_u
                                    i64.const 32
                                    i64.shl
                                    local.set 12
                                    br 2 (;@13;)
                                  end
                                  local.get 4
                                  i32.load offset=20
                                  local.set 6
                                end
                                local.get 1
                                i32.const 0
                                i32.store8 offset=32
                                local.get 6
                                br_if 1 (;@12;)
                                global.get $GOT.data.internal.__memory_base
                                i32.const 1054532
                                i32.add
                                i64.extend_i32_u
                                i64.const 32
                                i64.shl
                                i64.const 2
                                i64.or
                                local.set 12
                              end
                              block ;; label = @13
                                local.get 8
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  local.get 9
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  local.get 7
                                  local.get 10
                                  local.get 9
                                  memory.copy
                                end
                                local.get 1
                                local.get 9
                                i32.store offset=28
                              end
                              local.get 12
                              i64.const 255
                              i64.and
                              i64.const 4
                              i64.eq
                              br_if 7 (;@5;)
                              local.get 0
                              local.get 12
                              i64.store align=4
                              br 10 (;@2;)
                            end
                            local.get 6
                            local.get 8
                            i32.add
                            local.set 8
                          end
                          local.get 8
                          local.get 11
                          i32.ge_u
                          br_if 4 (;@6;)
                          br 0 (;@10;)
                        end
                      end
                      global.get $GOT.data.internal.__memory_base
                      i32.const 1054648
                      i32.add
                      i32.const 4576
                      local.set 14
                      local.get 14
                      local.get 14
                      i32.load 2 offset=28 align=1
                      i32.const 1
                      i32.add
                      i32.store 2 offset=28 align=1
                      call $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE
                      unreachable
                    end
                    local.get 0
                    local.get 12
                    i64.store align=4
                    br 5 (;@2;)
                  end
                  local.get 11
                  local.get 8
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054784
                  i32.add
                  i32.const 4608
                  local.set 14
                  local.get 14
                  local.get 14
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
                  unreachable
                end
                block ;; label = @6
                  local.get 8
                  local.get 11
                  i32.gt_u
                  br_if 0 (;@6;)
                  local.get 1
                  i32.const 0
                  i32.store offset=28
                  br 1 (;@5;)
                end
                local.get 8
                local.get 11
                global.get $GOT.data.internal.__memory_base
                i32.const 1054252
                i32.add
                i32.const 4640
                local.set 14
                local.get 14
                local.get 14
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
                unreachable
              end
              local.get 2
              local.get 5
              i32.add
              local.set 11
              block ;; label = @5
                local.get 3
                local.get 5
                i32.sub
                local.tee 8
                local.get 1
                i32.load offset=20
                local.get 1
                i32.load offset=28
                local.tee 6
                i32.sub
                i32.lt_u
                br_if 0 (;@5;)
                local.get 0
                local.get 1
                i32.const 20
                i32.add
                local.get 11
                local.get 8
                i32.const 4672
                local.set 14
                local.get 14
                local.get 14
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
                br 3 (;@2;)
              end
              block ;; label = @5
                local.get 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=24
                local.get 6
                i32.add
                local.get 11
                local.get 8
                memory.copy
              end
              local.get 0
              i32.const 4
              i32.store8
              local.get 1
              local.get 6
              local.get 8
              i32.add
              i32.store offset=28
              br 2 (;@2;)
            end
            local.get 8
            local.get 6
            i32.gt_u
            br_if 2 (;@1;)
            i32.const 0
            local.set 6
            local.get 1
            i32.const 0
            i32.store offset=28
          end
          block ;; label = @3
            local.get 3
            local.get 1
            i32.load offset=20
            local.get 6
            i32.sub
            i32.lt_u
            br_if 0 (;@3;)
            local.get 0
            local.get 1
            i32.const 20
            i32.add
            local.get 2
            local.get 3
            i32.const 4704
            local.set 14
            local.get 14
            local.get 14
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load offset=24
            local.get 6
            i32.add
            local.get 2
            local.get 3
            memory.copy
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 1
          local.get 6
          local.get 3
          i32.add
          i32.store offset=28
        end
        local.get 1
        local.get 1
        i32.load offset=16
        i32.const 1
        i32.add
        i32.store offset=16
        local.get 4
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 8
      local.get 6
      global.get $GOT.data.internal.__memory_base
      i32.const 1054252
      i32.add
      i32.const 4736
      local.set 14
      local.get 14
      local.get 14
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17ha202ea95069de918E (;42;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i64 i32 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 1
              i32.const 65536
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=12
        i32.const 1
        local.set 1
      end
      i32.const 0
      local.set 3
      block ;; label = @1
        i32.const 0
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.load offset=4
        local.tee 5
        local.get 4
        i64.load offset=8
        local.tee 6
        i64.const 4294967295
        local.get 6
        i64.const 4294967295
        i64.lt_u
        select
        i32.wrap_i64
        i32.sub
        local.tee 7
        local.get 7
        local.get 5
        i32.gt_u
        select
        local.tee 7
        local.get 1
        local.get 7
        local.get 1
        i32.lt_u
        select
        local.tee 8
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.load
        local.get 6
        local.get 5
        i64.extend_i32_u
        local.tee 9
        local.get 6
        local.get 9
        i64.lt_u
        select
        i32.wrap_i64
        i32.add
        local.get 2
        i32.const 12
        i32.add
        local.get 8
        memory.copy
      end
      local.get 4
      local.get 6
      local.get 8
      i64.extend_i32_u
      i64.add
      i64.store offset=8
      block ;; label = @1
        local.get 7
        local.get 1
        i32.ge_u
        br_if 0 (;@1;)
        global.get $GOT.data.internal.__memory_base
        i32.const 1054584
        i32.add
        i64.load
        local.tee 6
        i64.const 255
        i64.and
        i64.const 4
        i64.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 4
          i32.load
          local.set 7
          block ;; label = @3
            local.get 4
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 7
            local.get 5
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 7
            local.get 5
            local.get 1
            i32.load offset=8
            i32.const 4768
            local.set 10
            local.get 10
            local.get 10
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          i32.const 4800
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        local.get 6
        i64.store align=4
        i32.const 1
        local.set 3
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE (;43;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 1
              i32.const 65536
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=12
        i32.const 1
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      i32.const 4832
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE (;44;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 0
      local.set 4
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.eqz
          br_if 0 (;@2;)
          loop ;; label = @3
            local.get 3
            local.get 2
            i32.store offset=4
            local.get 3
            local.get 1
            i32.store
            local.get 3
            i32.const 8
            i32.add
            i32.const 2
            local.get 3
            i32.const 1
            i32.const 4864
            local.set 7
            local.get 7
            local.get 7
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.load16_u offset=8
                    i32.const 1
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 3
                    i64.load16_u offset=10
                    local.tee 5
                    i64.const 27
                    i64.eq
                    br_if 3 (;@4;)
                    local.get 5
                    i64.const 32
                    i64.shl
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 3
                  i32.load offset=12
                  local.tee 6
                  br_if 1 (;@5;)
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  local.set 5
                end
                local.get 5
                i64.const 255
                i64.and
                i64.const 4
                i64.eq
                br_if 3 (;@2;)
                local.get 0
                i32.load offset=4
                local.set 1
                block ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.load8_u
                    local.tee 2
                    i32.const 4
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 2
                    i32.const 3
                    i32.ne
                    br_if 1 (;@6;)
                  end
                  local.get 1
                  i32.load
                  local.set 6
                  block ;; label = @7
                    local.get 1
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 2
                    i32.load
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 6
                    local.get 4
                    call_indirect (type 1)
                  end
                  block ;; label = @7
                    local.get 2
                    i32.load offset=4
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 6
                    local.get 4
                    local.get 2
                    i32.load offset=8
                    i32.const 4896
                    local.set 7
                    local.get 7
                    local.get 7
                    i32.load 2 offset=28 align=1
                    i32.const 1
                    i32.add
                    i32.store 2 offset=28 align=1
                    call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  end
                  local.get 1
                  i32.const 12
                  i32.const 4
                  i32.const 4928
                  local.set 7
                  local.get 7
                  local.get 7
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                end
                local.get 0
                local.get 5
                i64.store align=4
                i32.const 1
                local.set 4
                br 3 (;@2;)
              end
              local.get 2
              local.get 6
              i32.lt_u
              br_if 3 (;@1;)
              local.get 1
              local.get 6
              i32.add
              local.set 1
              local.get 2
              local.get 6
              i32.sub
              local.set 2
            end
            local.get 2
            br_if 0 (;@3;)
          end
        end
        local.get 3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 4
        return
      end
      local.get 6
      local.get 2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      i32.const 4960
      local.set 7
      local.get 7
      local.get 7
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E (;45;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 1
              i32.const 65536
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=12
        i32.const 1
        local.set 1
      end
      block ;; label = @1
        local.get 1
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 1
        i32.const 1
        i32.const 1
        i32.const 4992
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get 0
        i32.load offset=8
        local.set 3
      end
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 2
        i32.const 12
        i32.add
        local.get 1
        memory.copy
      end
      local.get 0
      local.get 3
      local.get 1
      i32.add
      i32.store offset=8
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E (;46;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 2
            i32.add
            local.tee 2
            local.get 1
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 6
            br 1 (;@2;)
          end
          i32.const 0
          local.set 6
          block ;; label = @3
            local.get 3
            local.get 4
            i32.add
            i32.const -1
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            i32.and
            i64.extend_i32_u
            local.get 2
            local.get 0
            i32.load
            local.tee 1
            i32.const 1
            i32.shl
            local.tee 7
            local.get 2
            local.get 7
            i32.gt_u
            select
            local.tee 2
            i32.const 8
            i32.const 4
            local.get 4
            i32.const 1
            i32.eq
            select
            local.tee 7
            local.get 2
            local.get 7
            i32.gt_u
            select
            local.tee 7
            i64.extend_i32_u
            i64.mul
            local.tee 8
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            i32.eqz
            br_if 0 (;@3;)
            br 1 (;@2;)
          end
          local.get 8
          i32.wrap_i64
          local.tee 9
          i32.const -2147483648
          local.get 3
          i32.sub
          i32.gt_u
          br_if 0 (;@2;)
          i32.const 0
          local.set 2
          block ;; label = @3
            local.get 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 5
            local.get 1
            local.get 4
            i32.mul
            i32.store offset=28
            local.get 5
            local.get 0
            i32.load offset=4
            i32.store offset=20
            local.get 3
            local.set 2
          end
          local.get 5
          local.get 2
          i32.store offset=24
          local.get 5
          i32.const 8
          i32.add
          local.get 3
          local.get 9
          local.get 5
          i32.const 20
          i32.add
          i32.const 5024
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E
          local.get 5
          i32.load offset=8
          i32.const 1
          i32.ne
          br_if 1 (;@1;)
          local.get 5
          i32.load offset=16
          local.set 2
          local.get 5
          i32.load offset=12
          local.set 6
        end
        local.get 6
        local.get 2
        global.get $GOT.data.internal.__memory_base
        i32.const 1054284
        i32.add
        i32.const 5056
        local.set 10
        local.get 10
        local.get 10
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      local.get 5
      i32.load offset=12
      local.set 3
      local.get 0
      local.get 7
      i32.store
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 5
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E (;47;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054380
      i32.add
      local.get 1
      i32.const 5088
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E (;48;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054428
      i32.add
      local.get 1
      i32.const 5120
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E (;49;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054404
      i32.add
      local.get 1
      i32.const 5152
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE (;50;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054452
      i32.add
      local.get 1
      i32.const 5184
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE (;51;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054356
      i32.add
      local.get 1
      i32.const 5216
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN3std9panicking12default_hook17h8deeaf2f2b300de3E (;52;) (type 1) (param i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      i32.const 3
      local.set 2
      block ;; label = @1
        local.get 0
        i32.load8_u offset=13
        br_if 0 (;@1;)
        i32.const 1
        local.set 2
        global.get $GOT.data.internal.__memory_base
        i32.const 1055872
        i32.add
        i32.load
        i32.const 1
        i32.gt_u
        br_if 0 (;@1;)
        i32.const 5248
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std5panic19get_backtrace_style17h54380a7febe96116E
        i32.const 255
        i32.and
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store8 offset=15
      local.get 1
      local.get 0
      i32.load offset=8
      i32.store offset=16
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      i32.const 5280
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std9panicking14payload_as_str17h6b6acf98476ffb56E
      local.get 1
      local.get 1
      i64.load
      i64.store offset=20 align=4
      global.get $GOT.data.internal.__memory_base
      i32.const 1055775
      i32.add
      i32.load8_u
      local.set 0
      local.get 1
      local.get 1
      i32.const 15
      i32.add
      i32.store offset=36
      local.get 1
      local.get 1
      i32.const 20
      i32.add
      i32.store offset=32
      local.get 1
      local.get 1
      i32.const 16
      i32.add
      i32.store offset=28
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 0
                br_if 0 (;@5;)
                local.get 1
                i64.const 0
                i64.store offset=40 align=4
                br 1 (;@4;)
              end
              global.get $GOT.data.internal.__memory_base
              local.tee 0
              i32.const 1055775
              i32.add
              i32.const 1
              i32.store8
              local.get 0
              i32.const 1055880
              i32.add
              local.tee 2
              i32.load
              local.set 0
              local.get 2
              i32.const 0
              i32.store
              local.get 1
              i32.const 0
              i32.store offset=40
              local.get 1
              local.get 0
              i32.store offset=44
              local.get 0
              br_if 1 (;@3;)
            end
            global.get $GOT.data.internal.__memory_base
            local.set 0
            local.get 1
            i32.const 40
            i32.add
            i32.const 5312
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE
            local.get 1
            i32.const 28
            i32.add
            local.get 1
            i32.const 79
            i32.add
            local.get 0
            i32.const 1054984
            i32.add
            i32.const 5344
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE
            br 1 (;@2;)
          end
          local.get 0
          i32.load8_u offset=8
          local.set 2
          local.get 0
          i32.const 1
          i32.store8 offset=8
          local.get 1
          local.get 2
          i32.store8 offset=51
          local.get 2
          i32.const 1
          i32.eq
          br_if 1 (;@1;)
          local.get 1
          i32.const 28
          i32.add
          local.get 0
          i32.const 12
          i32.add
          global.get $GOT.data.internal.__memory_base
          local.tee 2
          i32.const 1054944
          i32.add
          i32.const 5376
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE
          local.get 0
          i32.const 0
          i32.store8 offset=8
          local.get 2
          i32.const 1055775
          i32.add
          i32.const 1
          i32.store8
          local.get 2
          i32.const 1055880
          i32.add
          local.tee 3
          i32.load
          local.set 2
          local.get 3
          local.get 0
          i32.store
          local.get 1
          local.get 2
          i32.store offset=56
          local.get 1
          i32.const 1
          i32.store offset=52
          local.get 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 2
          i32.load
          local.tee 0
          i32.const -1
          i32.add
          i32.store
          local.get 0
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 1
          i32.const 56
          i32.add
          i32.const 5408
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
        end
        local.get 1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 1
      i64.const 0
      i64.store offset=64 align=4
      local.get 1
      i64.const 17179869185
      i64.store offset=56 align=4
      local.get 1
      global.get $GOT.data.internal.__memory_base
      local.tee 0
      i32.const 1054800
      i32.add
      i32.store offset=52
      i32.const 0
      local.get 1
      i32.const 51
      i32.add
      global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
      local.get 1
      i32.const 52
      i32.add
      local.get 0
      i32.const 1054808
      i32.add
      i32.const 5440
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E (;53;) (type 1) (param i32)
      (local i32 i32 i32 i32)
      local.get 0
      i32.load offset=4
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load8_u
          local.tee 0
          i32.const 4
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 3
          i32.ne
          br_if 1 (;@1;)
        end
        local.get 1
        i32.load
        local.set 2
        block ;; label = @2
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 0
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          call_indirect (type 1)
        end
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          local.get 0
          i32.load offset=8
          i32.const 5472
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        i32.const 5504
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE (;54;) (type 1) (param i32)
      (local i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 2
        i32.const -1
        i32.add
        i32.store
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        i32.const 5536
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
      end
    )
    (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E (;55;) (type 1) (param i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 0
        i32.const 12
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 16
        i32.add
        i32.load
        local.get 1
        i32.const 1
        i32.const 5568
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      block ;; label = @1
        local.get 0
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const -1
        i32.add
        i32.store offset=4
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 24
        i32.const 4
        i32.const 5600
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E (;56;) (type 1) (param i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 1
        i32.const 1
        i32.const 5632
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E (;57;) (type 1) (param i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 1
        i32.const 1
        i32.const 5664
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE (;58;) (type 1) (param i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 1
        i32.const -2147483648
        i32.or
        i32.const -2147483648
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 1
        i32.const 1
        i32.const 5696
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E (;59;) (type 6) (param i32 i32)
      (local i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.and
          local.tee 0
          i32.const 4
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 3
          i32.ne
          br_if 1 (;@1;)
        end
        local.get 1
        i32.load
        local.set 2
        block ;; label = @2
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 0
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          call_indirect (type 1)
        end
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          local.get 0
          i32.load offset=8
          i32.const 5728
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        i32.const 5760
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE (;60;) (type 6) (param i32 i32)
      local.get 0
      i32.const 0
      i32.store
    )
    (func $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E (;61;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 2
      i32.store offset=12
      local.get 5
      local.get 1
      i32.store offset=8
      local.get 0
      local.get 5
      i32.const 8
      i32.add
      global.get $GOT.data.internal.__memory_base
      i32.const 1054236
      i32.add
      local.tee 2
      local.get 5
      i32.const 12
      i32.add
      local.get 2
      local.get 3
      local.get 4
      i32.const 5792
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E
      unreachable
    )
    (func $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE (;62;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 2)
    )
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E (;63;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          local.get 2
          i32.const 0
          i32.store offset=12
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 1
                i32.const 65536
                i32.lt_u
                br_if 0 (;@5;)
                local.get 2
                local.get 1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=15
                local.get 2
                local.get 1
                i32.const 18
                i32.shr_u
                i32.const 240
                i32.or
                i32.store8 offset=12
                local.get 2
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get 2
                local.get 1
                i32.const 12
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 4
                local.set 1
                br 2 (;@3;)
              end
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            i32.store8 offset=12
            i32.const 2
            local.set 1
          end
          block ;; label = @3
            local.get 1
            local.get 0
            i32.load
            local.get 0
            i32.load offset=8
            local.tee 3
            i32.sub
            i32.le_u
            br_if 0 (;@3;)
            local.get 0
            local.get 3
            local.get 1
            i32.const 1
            i32.const 1
            i32.const 5824
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get 0
            i32.load offset=8
            local.set 3
          end
          block ;; label = @3
            local.get 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            i32.load offset=4
            local.get 3
            i32.add
            local.get 2
            i32.const 12
            i32.add
            local.get 1
            memory.copy
          end
          local.get 0
          local.get 3
          local.get 1
          i32.add
          i32.store offset=8
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 0
          i32.load offset=8
          local.tee 3
          local.get 0
          i32.load
          i32.ne
          br_if 0 (;@2;)
          local.get 0
          global.get $GOT.data.internal.__memory_base
          i32.const 1054268
          i32.add
          i32.const 5856
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E
        end
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 1
        i32.store8
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=8
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E (;64;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        i32.const 1
        i32.const 1
        i32.const 5888
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get 0
        i32.load offset=8
        local.set 3
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 1
        local.get 2
        memory.copy
      end
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E (;65;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      block ;; label = @1
        local.get 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.load offset=4
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.load offset=8
                local.tee 4
                br_if 0 (;@5;)
                block ;; label = @6
                  local.get 2
                  br_if 0 (;@6;)
                  local.get 1
                  local.set 3
                  br 4 (;@2;)
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                br 2 (;@3;)
              end
              local.get 3
              i32.load
              local.get 4
              local.get 1
              local.get 2
              i32.const 5920
              local.set 5
              local.get 5
              local.get 5
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
              local.set 3
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              local.get 1
              local.set 3
              br 2 (;@2;)
            end
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
          end
          local.get 2
          local.get 1
          i32.const 5952
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.set 3
        end
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          i32.const 1
          i32.store
          return
        end
        local.get 0
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 0
      i32.const 0
      i32.store offset=4
      local.get 0
      i32.const 1
      i32.store
    )
    (func $"#func66 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" (@name "_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE") (;66;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.get 1
      i32.const 5984
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E (;67;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      i32.const 4
      i32.add
      i32.store offset=12
      local.get 1
      global.get $GOT.data.internal.__memory_base
      local.tee 3
      i32.const 1049005
      i32.add
      i32.const 9
      local.get 3
      i32.const 1049014
      i32.add
      i32.const 11
      local.get 0
      local.get 3
      i32.const 1054300
      i32.add
      local.get 3
      i32.const 1049025
      i32.add
      i32.const 9
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      i32.const 1054316
      i32.add
      i32.const 6016
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E (;68;) (type 1) (param i32)
      (local i32 i32 i64 i64 i64 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055774
                          i32.add
                          i32.load8_u
                          br_table 0 (;@10;) 2 (;@8;) 1 (;@9;) 9 (;@1;) 0 (;@10;)
                        end
                        global.get $GOT.data.internal.__memory_base
                        i32.const 1055774
                        i32.add
                        i32.const 2
                        i32.store8
                        local.get 0
                        i32.load8_u
                        local.set 2
                        local.get 0
                        i32.const 0
                        i32.store8
                        block ;; label = @10
                          local.get 2
                          i32.const 1
                          i32.ne
                          br_if 0 (;@10;)
                          local.get 1
                          i32.const 0
                          i32.store8 offset=39
                          block ;; label = @11
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055776
                            i32.add
                            i32.load8_u offset=40
                            i32.const 3
                            i32.eq
                            br_if 0 (;@11;)
                            local.get 1
                            i32.const 39
                            i32.add
                            i32.const 6048
                            local.set 11
                            local.get 11
                            local.get 11
                            i32.load 2 offset=28 align=1
                            i32.const 1
                            i32.add
                            i32.store 2 offset=28 align=1
                            call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E
                            local.get 1
                            i32.load8_u offset=39
                            i32.const 1
                            i32.and
                            br_if 9 (;@2;)
                          end
                          block ;; label = @11
                            global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
                            i64.load
                            local.tee 3
                            i64.const 0
                            i64.ne
                            br_if 0 (;@11;)
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055856
                            i32.add
                            i64.load
                            local.set 4
                            loop ;; label = @12
                              local.get 4
                              i64.const -1
                              i64.eq
                              br_if 5 (;@7;)
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055856
                              i32.add
                              local.tee 0
                              local.get 4
                              i64.const 1
                              i64.add
                              local.tee 3
                              local.get 0
                              i64.load
                              local.tee 5
                              local.get 5
                              local.get 4
                              i64.eq
                              local.tee 0
                              select
                              i64.store
                              local.get 5
                              local.set 4
                              local.get 0
                              i32.eqz
                              br_if 0 (;@12;)
                            end
                            global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
                            local.get 3
                            i64.store
                          end
                          block ;; label = @11
                            block ;; label = @12
                              local.get 3
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              i64.load
                              i64.eq
                              br_if 0 (;@12;)
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              local.tee 2
                              i32.load8_u offset=12
                              local.set 6
                              i32.const 1
                              local.set 0
                              local.get 2
                              i32.const 1
                              i32.store8 offset=12
                              local.get 6
                              br_if 10 (;@2;)
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              local.get 3
                              i64.store
                              br 1 (;@11;)
                            end
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055776
                            i32.add
                            i32.load offset=8
                            local.tee 0
                            i32.const -1
                            i32.eq
                            br_if 9 (;@2;)
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                          end
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          local.tee 2
                          local.get 0
                          i32.store offset=8
                          local.get 2
                          i32.load offset=16
                          br_if 4 (;@6;)
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          local.tee 0
                          i32.const -1
                          i32.store offset=16
                          local.get 0
                          i32.load8_u offset=32
                          br_if 7 (;@3;)
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          i32.load offset=28
                          local.tee 6
                          i32.eqz
                          br_if 7 (;@3;)
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          i32.load offset=24
                          local.set 7
                          i32.const 0
                          local.set 0
                          loop ;; label = @11
                            local.get 1
                            local.get 6
                            local.get 0
                            i32.sub
                            local.tee 8
                            i32.store offset=44
                            local.get 1
                            local.get 7
                            local.get 0
                            i32.add
                            local.tee 9
                            i32.store offset=40
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const 1
                            local.get 1
                            i32.const 40
                            i32.add
                            i32.const 1
                            i32.const 6080
                            local.set 11
                            local.get 11
                            local.get 11
                            i32.load 2 offset=28 align=1
                            i32.const 1
                            i32.add
                            i32.store 2 offset=28 align=1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 1
                                      i32.load16_u offset=12
                                      i32.const 1
                                      i32.ne
                                      br_if 0 (;@16;)
                                      local.get 8
                                      local.set 2
                                      local.get 1
                                      i32.load16_u offset=14
                                      local.tee 10
                                      i32.const -8
                                      i32.add
                                      br_table 1 (;@15;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 2 (;@14;) 4 (;@12;) 2 (;@14;)
                                    end
                                    local.get 1
                                    i32.load offset=16
                                    local.set 2
                                  end
                                  local.get 2
                                  br_if 1 (;@13;)
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054532
                                  i32.add
                                  i64.extend_i32_u
                                  i64.const 32
                                  i64.shl
                                  i64.const 2
                                  i64.or
                                  local.set 4
                                  br 9 (;@5;)
                                end
                                local.get 10
                                i64.extend_i32_u
                                i64.const 32
                                i64.shl
                                local.set 4
                                br 8 (;@5;)
                              end
                              local.get 2
                              local.get 0
                              i32.add
                              local.set 0
                            end
                            local.get 0
                            local.get 6
                            i32.ge_u
                            br_if 7 (;@4;)
                            br 0 (;@11;)
                          end
                        end
                        global.get $GOT.data.internal.__memory_base
                        i32.const 1054824
                        i32.add
                        i32.const 6112
                        local.set 11
                        local.get 11
                        local.get 11
                        i32.load 2 offset=28 align=1
                        i32.const 1
                        i32.add
                        i32.store 2 offset=28 align=1
                        call $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE
                        unreachable
                      end
                      local.get 1
                      i32.const 0
                      i32.store offset=28
                      local.get 1
                      i32.const 1
                      i32.store offset=16
                      local.get 1
                      i64.const 4
                      i64.store offset=20 align=4
                      local.get 1
                      global.get $GOT.data.internal.__memory_base
                      local.tee 0
                      i32.const 1055340
                      i32.add
                      i32.store offset=12
                      local.get 1
                      i32.const 12
                      i32.add
                      local.get 0
                      i32.const 1054492
                      i32.add
                      i32.const 6144
                      local.set 11
                      local.get 11
                      local.get 11
                      i32.load 2 offset=28 align=1
                      i32.const 1
                      i32.add
                      i32.store 2 offset=28 align=1
                      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                      unreachable
                    end
                    local.get 1
                    i32.const 0
                    i32.store offset=28
                    local.get 1
                    i32.const 1
                    i32.store offset=16
                    local.get 1
                    i64.const 4
                    i64.store offset=20 align=4
                    local.get 1
                    global.get $GOT.data.internal.__memory_base
                    local.tee 0
                    i32.const 1055332
                    i32.add
                    i32.store offset=12
                    local.get 1
                    i32.const 12
                    i32.add
                    local.get 0
                    i32.const 1054492
                    i32.add
                    i32.const 6176
                    local.set 11
                    local.get 11
                    local.get 11
                    i32.load 2 offset=28 align=1
                    i32.const 1
                    i32.add
                    i32.store 2 offset=28 align=1
                    call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                    unreachable
                  end
                  i32.const 6208
                  local.set 11
                  local.get 11
                  local.get 11
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
                  unreachable
                end
                global.get $GOT.data.internal.__memory_base
                i32.const 1054632
                i32.add
                i32.const 6240
                local.set 11
                local.get 11
                local.get 11
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE
                unreachable
              end
              local.get 4
              i32.wrap_i64
              local.set 2
              block ;; label = @5
                local.get 0
                i32.eqz
                br_if 0 (;@5;)
                local.get 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 7
                local.get 9
                local.get 8
                memory.copy
              end
              block ;; label = @5
                local.get 2
                i32.const 255
                i32.and
                local.tee 0
                i32.const 4
                i32.gt_u
                br_if 0 (;@5;)
                local.get 0
                i32.const 3
                i32.ne
                br_if 2 (;@3;)
              end
              local.get 4
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              local.tee 0
              i32.load
              local.set 6
              block ;; label = @5
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 2
                i32.load
                local.tee 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 8
                call_indirect (type 1)
              end
              block ;; label = @5
                local.get 2
                i32.load offset=4
                local.tee 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 8
                local.get 2
                i32.load offset=8
                i32.const 6272
                local.set 11
                local.get 11
                local.get 11
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              end
              local.get 0
              i32.const 12
              i32.const 4
              i32.const 6304
              local.set 11
              local.get 11
              local.get 11
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              br 1 (;@3;)
            end
            local.get 0
            local.get 6
            i32.le_u
            br_if 0 (;@3;)
            local.get 0
            local.get 6
            global.get $GOT.data.internal.__memory_base
            i32.const 1054252
            i32.add
            i32.const 6336
            local.set 11
            local.get 11
            local.get 11
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
            unreachable
          end
          block ;; label = @3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load offset=20
            local.tee 0
            i32.eqz
            br_if 0 (;@3;)
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load offset=24
            local.get 0
            i32.const 1
            i32.const 6368
            local.set 11
            local.get 11
            local.get 11
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          local.tee 0
          i32.const 0
          i32.store8 offset=32
          local.get 0
          i32.const 0
          i32.store offset=28
          local.get 0
          i64.const 4294967296
          i64.store offset=20 align=4
          local.get 0
          local.get 0
          i32.load offset=16
          i32.const 1
          i32.add
          i32.store offset=16
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const -1
          i32.add
          local.tee 2
          i32.store offset=8
          local.get 2
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          local.tee 0
          i32.const 0
          i32.store8 offset=12
          local.get 0
          i64.const 0
          i64.store
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055774
        i32.add
        i32.const 3
        i32.store8
      end
      local.get 1
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E (;69;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i64 i64 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          i64.load
          local.tee 6
          i64.const 0
          i64.ne
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          i32.const 1055856
          i32.add
          i64.load
          local.set 7
          loop ;; label = @3
            local.get 7
            i64.const -1
            i64.eq
            br_if 2 (;@1;)
            global.get $GOT.data.internal.__memory_base
            i32.const 1055856
            i32.add
            local.tee 8
            local.get 7
            i64.const 1
            i64.add
            local.tee 6
            local.get 8
            i64.load
            local.tee 9
            local.get 9
            local.get 7
            i64.eq
            local.tee 8
            select
            i64.store
            local.get 9
            local.set 7
            local.get 8
            i32.eqz
            br_if 0 (;@3;)
          end
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.get 6
          i64.store
        end
        global.get $GOT.data.internal.__memory_base
        local.tee 8
        i32.const 1055864
        i32.add
        local.get 6
        i64.store
        local.get 0
        local.get 1
        i32.load offset=20
        call_indirect (type 3)
        local.set 1
        block ;; label = @2
          local.get 8
          i32.const 1055774
          i32.add
          i32.load8_u
          i32.const 3
          i32.eq
          br_if 0 (;@2;)
          local.get 5
          i32.const 1
          i32.store8 offset=15
          local.get 5
          i32.const 15
          i32.add
          i32.const 6400
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E
        end
        local.get 5
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      i32.const 6432
      local.set 10
      local.get 10
      local.get 10
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E (;70;) (type 0)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      i32.const 1
      i32.store offset=12
      local.get 0
      i64.const 4
      i64.store offset=16 align=4
      local.get 0
      global.get $GOT.data.internal.__memory_base
      local.tee 1
      i32.const 1054508
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      i32.const 1054516
      i32.add
      i32.const 6464
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E (;71;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8 offset=8
      local.get 3
      local.get 1
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054404
            i32.add
            local.get 2
            i32.const 6496
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 0
            i32.store offset=40
            local.get 3
            i32.const 1
            i32.store offset=28
            local.get 3
            i64.const 4
            i64.store offset=32 align=4
            local.get 3
            global.get $GOT.data.internal.__memory_base
            local.tee 1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get 3
            i32.const 24
            i32.add
            local.get 1
            i32.const 1054704
            i32.add
            i32.const 6528
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 3
          i32.load offset=12
          local.set 2
          block ;; label = @3
            local.get 3
            i32.load8_u offset=8
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
          end
          local.get 2
          i32.load
          local.set 0
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            local.get 1
            i32.load offset=8
            i32.const 6560
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          i32.const 6592
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE (;72;) (type 0)
      (local i32)
      i32.const 6624
      local.set 0
      local.get 0
      local.get 0
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $abort
      unreachable
    )
    (func $_ZN3std3env11current_dir17h890335e8528685e2E (;73;) (type 1) (param i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      i32.const 512
      local.set 2
      block ;; label = @1
        block ;; label = @2
          i32.const 512
          i32.const 1
          i32.const 6656
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 3
          i32.store offset=8
          local.get 1
          i32.const 512
          i32.store offset=4
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 512
              i32.const 6688
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $getcwd
              br_if 0 (;@4;)
              i32.const 512
              local.set 2
              loop ;; label = @5
                block ;; label = @6
                  global.get $GOT.data.internal.errno
                  i32.load
                  local.tee 4
                  i32.const 68
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 0
                  local.get 4
                  i32.store offset=8
                  local.get 0
                  i64.const 2147483648
                  i64.store align=4
                  local.get 2
                  i32.eqz
                  br_if 3 (;@3;)
                  local.get 3
                  local.get 2
                  i32.const 1
                  i32.const 6720
                  local.set 6
                  local.get 6
                  local.get 6
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  br 3 (;@3;)
                end
                local.get 1
                local.get 2
                i32.store offset=12
                local.get 1
                i32.const 4
                i32.add
                local.get 2
                i32.const 1
                i32.const 1
                i32.const 1
                i32.const 6752
                local.set 6
                local.get 6
                local.get 6
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
                local.get 1
                i32.load offset=8
                local.tee 3
                local.get 1
                i32.load offset=4
                local.tee 2
                i32.const 6784
                local.set 6
                local.get 6
                local.get 6
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $getcwd
                i32.eqz
                br_if 0 (;@5;)
              end
            end
            local.get 1
            local.get 3
            i32.const 6816
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $strlen
            local.tee 4
            i32.store offset=12
            block ;; label = @4
              local.get 2
              local.get 4
              i32.le_u
              br_if 0 (;@4;)
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 5
                  local.get 3
                  local.get 2
                  i32.const 1
                  i32.const 6848
                  local.set 6
                  local.get 6
                  local.get 6
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  br 1 (;@5;)
                end
                local.get 3
                local.get 2
                i32.const 1
                local.get 4
                i32.const 6880
                local.set 6
                local.get 6
                local.get 6
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
                local.tee 5
                i32.eqz
                br_if 4 (;@1;)
              end
              local.get 1
              local.get 4
              i32.store offset=4
              local.get 1
              local.get 5
              i32.store offset=8
            end
            local.get 0
            local.get 1
            i64.load offset=4 align=4
            i64.store align=4
            local.get 0
            i32.const 8
            i32.add
            local.get 1
            i32.const 4
            i32.add
            i32.const 8
            i32.add
            i32.load
            i32.store
          end
          local.get 1
          i32.const 16
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1
        i32.const 512
        global.get $GOT.data.internal.__memory_base
        i32.const 1055300
        i32.add
        i32.const 6912
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 1
      local.get 4
      global.get $GOT.data.internal.__memory_base
      i32.const 1055316
      i32.add
      i32.const 6944
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3env7_var_os17hecfa64e4c3898426E (;74;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 416
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 383
            i32.gt_u
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              i32.const 20
              i32.add
              local.get 1
              local.get 2
              memory.copy
            end
            local.get 3
            i32.const 20
            i32.add
            local.get 2
            i32.add
            i32.const 0
            i32.store8
            local.get 3
            i32.const 404
            i32.add
            local.get 3
            i32.const 20
            i32.add
            local.get 2
            i32.const 1
            i32.add
            i32.const 6976
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h04ddcd8be7687b8aE
            block ;; label = @4
              local.get 3
              i32.load offset=404
              i32.const 1
              i32.ne
              br_if 0 (;@4;)
              local.get 3
              global.get $GOT.data.internal.__memory_base
              i32.const 1054888
              i32.add
              i64.load
              i64.store offset=12 align=4
              i32.const -2147483647
              local.set 2
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 3
              i32.load offset=408
              i32.const 7008
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $getenv
              local.tee 1
              br_if 0 (;@4;)
              i32.const -2147483648
              local.set 2
              br 2 (;@2;)
            end
            i32.const 0
            local.set 4
            local.get 1
            i32.const 7040
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $strlen
            local.tee 2
            i32.const 0
            i32.lt_s
            br_if 2 (;@1;)
            block ;; label = @4
              block ;; label = @5
                local.get 2
                br_if 0 (;@5;)
                i32.const 1
                local.set 5
                br 1 (;@4;)
              end
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set 4
              local.get 2
              i32.const 1
              i32.const 7072
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee 5
              i32.eqz
              br_if 3 (;@1;)
            end
            block ;; label = @4
              local.get 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              local.get 1
              local.get 2
              memory.copy
            end
            local.get 3
            local.get 2
            i32.store offset=16
            local.get 3
            local.get 5
            i32.store offset=12
            br 1 (;@2;)
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 1
          local.get 2
          i32.const 7104
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE
          local.get 3
          i32.load offset=8
          local.set 2
        end
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const -2147483647
            i32.eq
            br_if 0 (;@3;)
            local.get 0
            local.get 3
            i64.load offset=12 align=4
            i64.store offset=4 align=4
            local.get 0
            local.get 2
            i32.store
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 3
            i32.load8_u offset=12
            i32.const 3
            i32.ne
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=16
            local.tee 2
            i32.load
            local.set 5
            block ;; label = @4
              local.get 2
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              i32.load
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              local.get 4
              call_indirect (type 1)
            end
            block ;; label = @4
              local.get 1
              i32.load offset=4
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              local.get 4
              local.get 1
              i32.load offset=8
              i32.const 7136
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 2
            i32.const 12
            i32.const 4
            i32.const 7168
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 0
          i32.const -2147483648
          i32.store
        end
        local.get 3
        i32.const 416
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 4
      local.get 2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      i32.const 7200
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE (;75;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      local.get 2
      i32.const 7232
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.load
            local.tee 2
            i32.const -2147483648
            i32.ne
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=8
            local.set 1
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.load offset=4
                local.tee 4
                i32.const 7264
                local.set 8
                local.get 8
                local.get 8
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $getenv
                local.tee 5
                br_if 0 (;@5;)
                local.get 0
                i32.const -2147483648
                i32.store
                br 1 (;@4;)
              end
              i32.const 0
              local.set 6
              local.get 5
              i32.const 7296
              local.set 8
              local.get 8
              local.get 8
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $strlen
              local.tee 2
              i32.const 0
              i32.lt_s
              br_if 3 (;@1;)
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 7
                  br 1 (;@5;)
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                i32.const 1
                local.set 6
                local.get 2
                i32.const 1
                i32.const 7328
                local.set 8
                local.get 8
                local.get 8
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                local.tee 7
                i32.eqz
                br_if 4 (;@1;)
              end
              block ;; label = @5
                local.get 2
                i32.eqz
                br_if 0 (;@5;)
                local.get 7
                local.get 5
                local.get 2
                memory.copy
              end
              local.get 0
              local.get 2
              i32.store offset=8
              local.get 0
              local.get 7
              i32.store offset=4
              local.get 0
              local.get 2
              i32.store
            end
            local.get 4
            i32.const 0
            i32.store8
            local.get 1
            i32.eqz
            br_if 1 (;@2;)
            local.get 4
            local.get 1
            i32.const 1
            i32.const 7360
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            br 1 (;@2;)
          end
          local.get 0
          i32.const -2147483647
          i32.store
          local.get 0
          global.get $GOT.data.internal.__memory_base
          i32.const 1054888
          i32.add
          i64.load
          i64.store offset=4 align=4
          local.get 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          i32.load offset=4
          local.get 2
          i32.const 1
          i32.const 7392
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 6
      local.get 2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      i32.const 7424
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E (;76;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              local.get 1
              i32.load
              local.tee 5
              local.get 1
              i32.load offset=8
              local.tee 6
              i32.sub
              i32.le_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 6
                br_if 0 (;@5;)
                i32.const 0
                local.set 6
                br 1 (;@4;)
              end
              local.get 1
              i32.load offset=4
              local.set 7
              i32.const 0
              local.set 8
              loop ;; label = @5
                local.get 4
                local.get 6
                local.get 8
                i32.sub
                local.tee 9
                i32.store offset=4
                local.get 4
                local.get 7
                local.get 8
                i32.add
                local.tee 10
                i32.store
                local.get 4
                i32.const 8
                i32.add
                i32.const 1
                local.get 4
                i32.const 1
                i32.const 7456
                local.set 15
                local.get 15
                local.get 15
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          local.get 4
                          i32.load16_u offset=8
                          i32.const 1
                          i32.ne
                          br_if 0 (;@10;)
                          local.get 9
                          local.set 11
                          local.get 4
                          i32.load16_u offset=10
                          local.tee 12
                          i32.const 8
                          i32.eq
                          br_if 1 (;@9;)
                          local.get 1
                          i32.const 0
                          i32.store8 offset=12
                          local.get 12
                          i32.const 27
                          i32.eq
                          br_if 4 (;@6;)
                          local.get 12
                          i64.extend_i32_u
                          i64.const 32
                          i64.shl
                          local.set 13
                          br 2 (;@8;)
                        end
                        local.get 4
                        i32.load offset=12
                        local.set 11
                      end
                      local.get 1
                      i32.const 0
                      i32.store8 offset=12
                      local.get 11
                      br_if 1 (;@7;)
                      global.get $GOT.data.internal.__memory_base
                      i32.const 1054532
                      i32.add
                      i64.extend_i32_u
                      i64.const 32
                      i64.shl
                      i64.const 2
                      i64.or
                      local.set 13
                    end
                    block ;; label = @8
                      local.get 8
                      i32.eqz
                      br_if 0 (;@8;)
                      block ;; label = @9
                        local.get 9
                        i32.eqz
                        br_if 0 (;@9;)
                        local.get 7
                        local.get 10
                        local.get 9
                        memory.copy
                      end
                      local.get 1
                      local.get 9
                      i32.store offset=8
                      local.get 9
                      local.set 6
                    end
                    local.get 13
                    i64.const 255
                    i64.and
                    i64.const 4
                    i64.eq
                    br_if 3 (;@4;)
                    local.get 0
                    local.get 13
                    i64.store align=4
                    br 4 (;@3;)
                  end
                  local.get 11
                  local.get 8
                  i32.add
                  local.set 8
                end
                local.get 8
                local.get 6
                i32.lt_u
                br_if 0 (;@5;)
              end
              local.get 8
              local.get 6
              i32.gt_u
              br_if 2 (;@2;)
              i32.const 0
              local.set 6
              local.get 1
              i32.const 0
              i32.store offset=8
            end
            block ;; label = @4
              local.get 3
              local.get 5
              i32.ge_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=4
                local.get 6
                i32.add
                local.get 2
                local.get 3
                memory.copy
              end
              local.get 0
              i32.const 4
              i32.store8
              local.get 1
              local.get 6
              local.get 3
              i32.add
              i32.store offset=8
              br 1 (;@3;)
            end
            i64.const 0
            local.set 14
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.eqz
                    br_if 0 (;@7;)
                    loop ;; label = @8
                      local.get 4
                      local.get 3
                      i32.store offset=4
                      local.get 4
                      local.get 2
                      i32.store
                      local.get 4
                      i32.const 8
                      i32.add
                      i32.const 1
                      local.get 4
                      i32.const 1
                      i32.const 7488
                      local.set 15
                      local.get 15
                      local.get 15
                      i32.load 2 offset=28 align=1
                      i32.const 1
                      i32.add
                      i32.store 2 offset=28 align=1
                      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              local.get 4
                              i32.load16_u offset=8
                              i32.const 1
                              i32.ne
                              br_if 0 (;@12;)
                              local.get 4
                              i64.load16_u offset=10
                              local.tee 13
                              i64.const 27
                              i64.eq
                              br_if 3 (;@9;)
                              local.get 13
                              i64.const 32
                              i64.shl
                              local.set 13
                              br 1 (;@11;)
                            end
                            local.get 4
                            i32.load offset=12
                            local.tee 8
                            br_if 1 (;@10;)
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1054584
                            i32.add
                            i64.load
                            local.set 13
                          end
                          local.get 13
                          i64.const 32
                          i64.shr_u
                          local.set 14
                          local.get 13
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          local.tee 8
                          i32.const 4
                          i32.eq
                          br_if 4 (;@6;)
                          local.get 8
                          br_if 5 (;@5;)
                          local.get 14
                          i64.const 8
                          i64.ne
                          br_if 5 (;@5;)
                          i64.const 4
                          local.set 14
                          i64.const 0
                          local.set 13
                          br 6 (;@4;)
                        end
                        local.get 3
                        local.get 8
                        i32.lt_u
                        br_if 8 (;@1;)
                        local.get 2
                        local.get 8
                        i32.add
                        local.set 2
                        local.get 3
                        local.get 8
                        i32.sub
                        local.set 3
                      end
                      local.get 3
                      br_if 0 (;@8;)
                    end
                  end
                  i64.const 0
                  local.set 13
                end
                local.get 13
                i64.const 4294967040
                i64.and
                local.get 14
                i64.const 32
                i64.shl
                i64.or
                local.set 13
                i64.const 4
                local.set 14
                br 1 (;@4;)
              end
              local.get 13
              i64.const 255
              i64.and
              local.set 14
              local.get 13
              i64.const -256
              i64.and
              local.set 13
            end
            local.get 1
            i32.const 0
            i32.store8 offset=12
            local.get 0
            local.get 14
            local.get 13
            i64.or
            i64.store align=4
          end
          local.get 4
          i32.const 16
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get 8
        local.get 6
        global.get $GOT.data.internal.__memory_base
        i32.const 1054252
        i32.add
        i32.const 7520
        local.set 15
        local.get 15
        local.get 15
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
        unreachable
      end
      local.get 8
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      i32.const 7552
      local.set 15
      local.get 15
      local.get 15
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E (;77;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 1056
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 1024
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        i32.const 0
        i32.const 1024
        memory.fill
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 2
            i32.const 1024
            i32.const 7584
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $strerror_r
            i32.const 0
            i32.lt_s
            br_if 0 (;@3;)
            local.get 2
            i32.const 1024
            i32.add
            local.get 2
            local.get 2
            i32.const 7616
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $strlen
            i32.const 7648
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3str8converts9from_utf817hc11b0c33b11310b8E
            local.get 2
            i32.load offset=1024
            br_if 1 (;@2;)
            i32.const 0
            local.set 3
            local.get 2
            i32.load offset=1032
            local.tee 1
            i32.const 0
            i32.lt_s
            br_if 2 (;@1;)
            local.get 2
            i32.load offset=1028
            local.set 4
            block ;; label = @4
              block ;; label = @5
                local.get 1
                br_if 0 (;@5;)
                i32.const 1
                local.set 5
                br 1 (;@4;)
              end
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set 3
              local.get 1
              i32.const 1
              i32.const 7680
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee 5
              i32.eqz
              br_if 3 (;@1;)
            end
            block ;; label = @4
              local.get 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              local.get 4
              local.get 1
              memory.copy
            end
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 0
            local.get 5
            i32.store offset=4
            local.get 0
            local.get 1
            i32.store
            local.get 2
            i32.const 1056
            i32.add
            global.set $__stack_pointer
            return
          end
          local.get 2
          i32.const 0
          i32.store offset=1040
          local.get 2
          i32.const 1
          i32.store offset=1028
          local.get 2
          i64.const 4
          i64.store offset=1032 align=4
          local.get 2
          global.get $GOT.data.internal.__memory_base
          local.tee 1
          i32.const 1055276
          i32.add
          i32.store offset=1024
          local.get 2
          i32.const 1024
          i32.add
          local.get 1
          i32.const 1055284
          i32.add
          i32.const 7712
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
          unreachable
        end
        local.get 2
        local.get 2
        i64.load offset=1028 align=4
        i64.store offset=1048
        global.get $GOT.data.internal.__memory_base
        local.tee 1
        i32.const 1049240
        i32.add
        i32.const 43
        local.get 2
        i32.const 1048
        i32.add
        local.get 1
        i32.const 1055244
        i32.add
        local.get 1
        i32.const 1055260
        i32.add
        i32.const 7744
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE
        unreachable
      end
      local.get 3
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      i32.const 7776
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E (;78;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 0
                i32.load8_u
                br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 0 (;@5;)
              end
              local.get 2
              local.get 0
              i32.load offset=4
              local.tee 0
              i32.store offset=4
              local.get 2
              i32.const 8
              i32.add
              local.get 0
              i32.const 7808
              local.set 4
              local.get 4
              local.get 4
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E
              local.get 2
              i32.const 3
              i32.store offset=44
              local.get 2
              global.get $GOT.data.internal.__memory_base
              i32.const 1054592
              i32.add
              i32.store offset=40
              local.get 2
              i64.const 2
              i64.store offset=52 align=4
              local.get 2
              global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 2
              i32.const 4
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=32
              local.get 2
              global.get $GOT.data.internal.__table_base
              i32.const 7
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 2
              i32.const 8
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=24
              local.get 2
              local.get 2
              i32.const 24
              i32.add
              i32.store offset=48
              local.get 1
              i32.load
              local.get 1
              i32.load offset=4
              local.get 2
              i32.const 40
              i32.add
              i32.const 7840
              local.set 4
              local.get 4
              local.get 4
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
              local.set 0
              local.get 2
              i32.load offset=8
              local.tee 1
              i32.eqz
              br_if 3 (;@1;)
              local.get 2
              i32.load offset=12
              local.get 1
              i32.const 1
              i32.const 7872
              local.set 4
              local.get 4
              local.get 4
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              br 3 (;@1;)
            end
            local.get 0
            i32.load8_u offset=1
            local.set 3
            local.get 2
            i32.const 1
            i32.store offset=44
            local.get 2
            global.get $GOT.data.internal.__memory_base
            local.tee 0
            i32.const 1049356
            i32.add
            i32.store offset=40
            local.get 2
            i64.const 1
            i64.store offset=52 align=4
            local.get 2
            local.get 0
            i32.const 1051408
            i32.add
            local.get 3
            i32.const 2
            i32.shl
            local.tee 3
            i32.add
            i32.load
            i32.store offset=28
            local.get 2
            local.get 0
            i32.const 1055356
            i32.add
            local.get 3
            i32.add
            i32.load
            i32.store offset=24
            local.get 2
            global.get $GOT.data.internal.__table_base
            i32.const 6
            i32.add
            i64.extend_i32_u
            i64.const 32
            i64.shl
            local.get 2
            i32.const 24
            i32.add
            i64.extend_i32_u
            i64.or
            i64.store offset=8
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=48
            local.get 1
            i32.load
            local.get 1
            i32.load offset=4
            local.get 2
            i32.const 40
            i32.add
            i32.const 7904
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            local.set 0
            br 2 (;@1;)
          end
          local.get 0
          i32.load offset=4
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 1
          i32.const 7936
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
          local.set 0
          br 1 (;@1;)
        end
        local.get 0
        i32.load offset=4
        local.tee 0
        i32.load
        local.get 1
        local.get 0
        i32.load offset=4
        i32.load offset=16
        call_indirect (type 2)
        local.set 0
      end
      local.get 2
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E (;79;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      block ;; label = @1
        local.get 3
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        i32.le_u
        br_if 0 (;@1;)
        local.get 1
        local.get 4
        local.get 3
        i32.const 1
        i32.const 1
        i32.const 7968
        local.set 5
        local.get 5
        local.get 5
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get 1
        i32.load offset=8
        local.set 4
      end
      block ;; label = @1
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.load offset=4
        local.get 4
        i32.add
        local.get 2
        local.get 3
        memory.copy
      end
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 1
      local.get 4
      local.get 3
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E (;80;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          i32.const 0
          local.set 4
          br 1 (;@1;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 5
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 6
            br 1 (;@2;)
          end
          local.get 2
          i32.const 28
          i32.add
          local.set 7
          local.get 3
          i32.const -4
          i32.and
          local.set 8
          i32.const 0
          local.set 4
          i32.const 0
          local.set 6
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 7
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.const -16
            i32.add
            i32.load
            local.get 7
            i32.const -24
            i32.add
            i32.load
            local.get 4
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 4
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            local.get 8
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 5
          i32.eqz
          br_if 0 (;@2;)
          local.get 6
          i32.const 3
          i32.shl
          local.get 2
          i32.add
          i32.const 4
          i32.add
          local.set 7
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 4
            i32.add
            local.set 4
            local.get 7
            i32.const 8
            i32.add
            local.set 7
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            br_if 0 (;@3;)
          end
        end
        local.get 3
        i32.const 3
        i32.shl
        local.set 7
        block ;; label = @2
          local.get 4
          local.get 1
          i32.load
          local.get 1
          i32.load offset=8
          local.tee 5
          i32.sub
          i32.le_u
          br_if 0 (;@2;)
          local.get 1
          local.get 5
          local.get 4
          i32.const 1
          i32.const 1
          i32.const 8000
          local.set 9
          local.get 9
          local.get 9
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        end
        local.get 2
        local.get 7
        i32.add
        local.set 8
        local.get 1
        i32.load offset=8
        local.set 7
        loop ;; label = @2
          local.get 2
          i32.load
          local.set 6
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 5
            local.get 1
            i32.load
            local.get 7
            i32.sub
            i32.le_u
            br_if 0 (;@3;)
            local.get 1
            local.get 7
            local.get 5
            i32.const 1
            i32.const 1
            i32.const 8032
            local.set 9
            local.get 9
            local.get 9
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get 1
            i32.load offset=8
            local.set 7
          end
          block ;; label = @3
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load offset=4
            local.get 7
            i32.add
            local.get 6
            local.get 5
            memory.copy
          end
          local.get 1
          local.get 7
          local.get 5
          i32.add
          local.tee 7
          i32.store offset=8
          local.get 2
          i32.const 8
          i32.add
          local.tee 2
          local.get 8
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 0
      local.get 4
      i32.store offset=4
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE (;81;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E (;82;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      block ;; label = @1
        local.get 3
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        i32.le_u
        br_if 0 (;@1;)
        local.get 1
        local.get 4
        local.get 3
        i32.const 1
        i32.const 1
        i32.const 8064
        local.set 5
        local.get 5
        local.get 5
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get 1
        i32.load offset=8
        local.set 4
      end
      block ;; label = @1
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.load offset=4
        local.get 4
        i32.add
        local.get 2
        local.get 3
        memory.copy
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 1
      local.get 4
      local.get 3
      i32.add
      i32.store offset=8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E (;83;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 3
        i32.and
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 5
            i32.const 0
            local.set 6
            br 1 (;@2;)
          end
          local.get 2
          i32.const 28
          i32.add
          local.set 7
          local.get 3
          i32.const -4
          i32.and
          local.set 8
          i32.const 0
          local.set 5
          i32.const 0
          local.set 6
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 7
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.const -16
            i32.add
            i32.load
            local.get 7
            i32.const -24
            i32.add
            i32.load
            local.get 5
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 5
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            local.get 8
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 6
          i32.const 3
          i32.shl
          local.get 2
          i32.add
          i32.const 4
          i32.add
          local.set 7
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 5
            i32.add
            local.set 5
            local.get 7
            i32.const 8
            i32.add
            local.set 7
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            br_if 0 (;@3;)
          end
        end
        local.get 3
        i32.const 3
        i32.shl
        local.set 4
        block ;; label = @2
          local.get 5
          local.get 1
          i32.load
          local.get 1
          i32.load offset=8
          local.tee 7
          i32.sub
          i32.le_u
          br_if 0 (;@2;)
          local.get 1
          local.get 7
          local.get 5
          i32.const 1
          i32.const 1
          i32.const 8096
          local.set 9
          local.get 9
          local.get 9
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
          local.get 1
          i32.load offset=8
          local.set 7
        end
        local.get 2
        local.get 4
        i32.add
        local.set 6
        loop ;; label = @2
          local.get 2
          i32.load
          local.set 4
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 5
            local.get 1
            i32.load
            local.get 7
            i32.sub
            i32.le_u
            br_if 0 (;@3;)
            local.get 1
            local.get 7
            local.get 5
            i32.const 1
            i32.const 1
            i32.const 8128
            local.set 9
            local.get 9
            local.get 9
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get 1
            i32.load offset=8
            local.set 7
          end
          block ;; label = @3
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load offset=4
            local.get 7
            i32.add
            local.get 4
            local.get 5
            memory.copy
          end
          local.get 1
          local.get 7
          local.get 5
          i32.add
          local.tee 7
          i32.store offset=8
          local.get 2
          i32.const 8
          i32.add
          local.tee 2
          local.get 6
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E (;84;) (type 6) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE (;85;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          i32.const 4
          i32.add
          local.set 5
          local.get 3
          i32.const 3
          i32.shl
          local.set 6
          local.get 3
          i32.const -1
          i32.add
          i32.const 536870911
          i32.and
          i32.const 1
          i32.add
          local.set 7
          i32.const 0
          local.set 8
          block ;; label = @3
            loop ;; label = @4
              local.get 5
              i32.load
              br_if 1 (;@3;)
              local.get 5
              i32.const 8
              i32.add
              local.set 5
              local.get 8
              i32.const 1
              i32.add
              local.set 8
              local.get 6
              i32.const -8
              i32.add
              local.tee 6
              br_if 0 (;@4;)
            end
            local.get 7
            local.set 8
          end
          block ;; label = @3
            local.get 3
            local.get 8
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            local.get 8
            i32.eq
            br_if 1 (;@2;)
            local.get 3
            local.get 8
            i32.sub
            local.set 7
            local.get 2
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            local.set 9
            block ;; label = @4
              loop ;; label = @5
                local.get 4
                i32.const 8
                i32.add
                i32.const 2
                local.get 9
                local.get 7
                i32.const 8160
                local.set 12
                local.get 12
                local.get 12
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                block ;; label = @6
                  local.get 4
                  i32.load16_u offset=8
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 4
                  i64.load16_u offset=10
                  local.tee 10
                  i64.const 27
                  i64.eq
                  br_if 1 (;@5;)
                  local.get 0
                  local.get 10
                  i64.const 32
                  i64.shl
                  i64.store align=4
                  br 5 (;@1;)
                end
                block ;; label = @6
                  local.get 4
                  i32.load offset=12
                  local.tee 5
                  br_if 0 (;@6;)
                  local.get 0
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  i64.store align=4
                  br 5 (;@1;)
                end
                local.get 9
                i32.const 4
                i32.add
                local.set 8
                local.get 7
                i32.const 3
                i32.shl
                local.set 3
                local.get 7
                i32.const -1
                i32.add
                i32.const 536870911
                i32.and
                i32.const 1
                i32.add
                local.set 11
                i32.const 0
                local.set 6
                block ;; label = @6
                  loop ;; label = @7
                    local.get 5
                    local.get 8
                    i32.load
                    local.tee 2
                    i32.lt_u
                    br_if 1 (;@6;)
                    local.get 8
                    i32.const 8
                    i32.add
                    local.set 8
                    local.get 6
                    i32.const 1
                    i32.add
                    local.set 6
                    local.get 5
                    local.get 2
                    i32.sub
                    local.set 5
                    local.get 3
                    i32.const -8
                    i32.add
                    local.tee 3
                    br_if 0 (;@7;)
                  end
                  local.get 11
                  local.set 6
                end
                block ;; label = @6
                  local.get 7
                  local.get 6
                  i32.lt_u
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 7
                    local.get 6
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.eqz
                    br_if 5 (;@2;)
                    local.get 4
                    i32.const 0
                    i32.store offset=24
                    local.get 4
                    i32.const 1
                    i32.store offset=12
                    local.get 4
                    i64.const 4
                    i64.store offset=16 align=4
                    local.get 4
                    global.get $GOT.data.internal.__memory_base
                    local.tee 5
                    i32.const 1054736
                    i32.add
                    i32.store offset=8
                    local.get 4
                    i32.const 8
                    i32.add
                    local.get 5
                    i32.const 1054744
                    i32.add
                    i32.const 8192
                    local.set 12
                    local.get 12
                    local.get 12
                    i32.load 2 offset=28 align=1
                    i32.const 1
                    i32.add
                    i32.store 2 offset=28 align=1
                    call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                    unreachable
                  end
                  local.get 9
                  local.get 6
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 9
                  i32.load offset=4
                  local.tee 8
                  local.get 5
                  i32.lt_u
                  br_if 2 (;@4;)
                  local.get 7
                  local.get 6
                  i32.sub
                  local.set 7
                  local.get 9
                  local.get 8
                  local.get 5
                  i32.sub
                  i32.store offset=4
                  local.get 9
                  local.get 9
                  i32.load
                  local.get 5
                  i32.add
                  i32.store
                  br 1 (;@5;)
                end
              end
              local.get 6
              local.get 7
              global.get $GOT.data.internal.__memory_base
              i32.const 1054720
              i32.add
              i32.const 8224
              local.set 12
              local.get 12
              local.get 12
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
              unreachable
            end
            local.get 4
            i32.const 0
            i32.store offset=24
            local.get 4
            i32.const 1
            i32.store offset=12
            local.get 4
            i64.const 4
            i64.store offset=16 align=4
            local.get 4
            global.get $GOT.data.internal.__memory_base
            local.tee 5
            i32.const 1054760
            i32.add
            i32.store offset=8
            local.get 4
            i32.const 8
            i32.add
            local.get 5
            i32.const 1054768
            i32.add
            i32.const 8256
            local.set 12
            local.get 12
            local.get 12
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 8
          local.get 3
          global.get $GOT.data.internal.__memory_base
          i32.const 1054720
          i32.add
          i32.const 8288
          local.set 12
          local.get 12
          local.get 12
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
          unreachable
        end
        local.get 0
        i32.const 4
        i32.store8
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE (;86;) (type 0)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              global.get $GOT.data.internal.__memory_base
              i32.const 1055776
              i32.add
              i32.load8_u offset=40
              br_table 0 (;@4;) 0 (;@4;) 3 (;@1;) 1 (;@3;) 0 (;@4;)
            end
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.const 2
            i32.store8 offset=40
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
            i32.const 1024
            i32.const 1
            i32.const 8320
            local.set 3
            local.get 3
            local.get 3
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
            local.tee 1
            i32.eqz
            br_if 1 (;@2;)
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            local.tee 2
            i32.const 3
            i32.store8 offset=40
            local.get 2
            i32.const 0
            i32.store8 offset=32
            local.get 2
            i32.const 0
            i32.store offset=28
            local.get 2
            local.get 1
            i32.store offset=24
            local.get 2
            i64.const 4398046511104
            i64.store offset=16
            local.get 2
            i32.const 0
            i32.store8 offset=12
            local.get 2
            i32.const 0
            i32.store offset=8
            local.get 2
            i64.const 0
            i64.store
          end
          local.get 0
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1
        i32.const 1024
        global.get $GOT.data.internal.__memory_base
        i32.const 1054616
        i32.add
        i32.const 8352
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      i32.const 1
      i32.store offset=12
      local.get 0
      i64.const 4
      i64.store offset=16 align=4
      local.get 0
      global.get $GOT.data.internal.__memory_base
      local.tee 2
      i32.const 1055340
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.const 1054840
      i32.add
      i32.const 8384
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E (;87;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i64 i64 i64 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
      local.set 2
      local.get 0
      i32.load
      local.set 3
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i64.load
          local.tee 4
          i64.const 0
          i64.ne
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          i32.const 1055856
          i32.add
          i64.load
          local.set 5
          loop ;; label = @3
            local.get 5
            i64.const -1
            i64.eq
            br_if 2 (;@1;)
            global.get $GOT.data.internal.__memory_base
            i32.const 1055856
            i32.add
            local.tee 0
            local.get 5
            i64.const 1
            i64.add
            local.tee 4
            local.get 0
            i64.load
            local.tee 6
            local.get 6
            local.get 5
            i64.eq
            local.tee 0
            select
            i64.store
            local.get 6
            local.set 5
            local.get 0
            i32.eqz
            br_if 0 (;@3;)
          end
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.get 4
          i64.store
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 4
              local.get 3
              i64.load
              i64.eq
              br_if 0 (;@4;)
              local.get 3
              i32.load8_u offset=12
              local.set 0
              local.get 3
              i32.const 1
              i32.store8 offset=12
              local.get 1
              local.get 0
              i32.store8 offset=7
              local.get 0
              i32.eqz
              br_if 1 (;@3;)
              local.get 1
              i64.const 0
              i64.store offset=20 align=4
              local.get 1
              i64.const 17179869185
              i64.store offset=12 align=4
              local.get 1
              global.get $GOT.data.internal.__memory_base
              local.tee 0
              i32.const 1054800
              i32.add
              i32.store offset=8
              i32.const 0
              local.get 1
              i32.const 7
              i32.add
              global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
              local.get 1
              i32.const 8
              i32.add
              local.get 0
              i32.const 1054808
              i32.add
              i32.const 8416
              local.set 7
              local.get 7
              local.get 7
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
              unreachable
            end
            block ;; label = @4
              local.get 3
              i32.load offset=8
              local.tee 0
              i32.const -1
              i32.eq
              br_if 0 (;@4;)
              local.get 3
              local.get 0
              i32.const 1
              i32.add
              i32.store offset=8
              br 2 (;@2;)
            end
            global.get $GOT.data.internal.__memory_base
            local.tee 0
            i32.const 1050608
            i32.add
            i32.const 38
            local.get 0
            i32.const 1054856
            i32.add
            i32.const 8448
            local.set 7
            local.get 7
            local.get 7
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core6option13expect_failed17he15179d1cacc214eE
            unreachable
          end
          local.get 3
          i32.const 1
          i32.store offset=8
          local.get 3
          local.get 4
          i64.store
        end
        local.get 1
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get 3
        return
      end
      i32.const 8480
      local.set 7
      local.get 7
      local.get 7
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E (;88;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.load
      i32.const 8512
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E
      i32.store offset=4
      local.get 3
      i32.const 4
      i32.store8 offset=8
      global.get $GOT.data.internal.__memory_base
      local.set 1
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            i32.const 1054452
            i32.add
            local.get 2
            i32.const 8544
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 0
            i32.store offset=40
            local.get 3
            i32.const 1
            i32.store offset=28
            local.get 3
            i64.const 4
            i64.store offset=32 align=4
            local.get 3
            global.get $GOT.data.internal.__memory_base
            local.tee 1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get 3
            i32.const 24
            i32.add
            local.get 1
            i32.const 1054704
            i32.add
            i32.const 8576
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 3
          i32.load offset=12
          local.set 2
          block ;; label = @3
            local.get 3
            i32.load8_u offset=8
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
          end
          local.get 2
          i32.load
          local.set 0
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            local.get 1
            i32.load offset=8
            i32.const 8608
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          i32.const 8640
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
      end
      local.get 3
      i32.load offset=4
      local.tee 1
      local.get 1
      i32.load offset=8
      i32.const -1
      i32.add
      local.tee 2
      i32.store offset=8
      block ;; label = @1
        local.get 2
        br_if 0 (;@1;)
        local.get 1
        i32.const 0
        i32.store8 offset=12
        local.get 1
        i64.const 0
        i64.store
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE (;89;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055775
            i32.add
            i32.load8_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 2
            br 1 (;@2;)
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055880
          i32.add
          local.tee 3
          i32.load
          local.set 4
          i32.const 0
          local.set 2
          local.get 3
          i32.const 0
          i32.store
          local.get 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 4
          i32.load8_u offset=8
          local.set 2
          local.get 4
          i32.const 1
          i32.store8 offset=8
          local.get 1
          local.get 2
          i32.store8 offset=7
          local.get 2
          i32.const 1
          i32.eq
          br_if 1 (;@1;)
          local.get 1
          i32.const 8
          i32.add
          local.get 4
          i32.const 12
          i32.add
          local.get 0
          i32.const 8672
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E
          local.get 1
          i32.load offset=12
          local.set 0
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.load8_u offset=8
              local.tee 2
              i32.const 4
              i32.gt_u
              br_if 0 (;@4;)
              local.get 2
              i32.const 3
              i32.ne
              br_if 1 (;@3;)
            end
            local.get 0
            i32.load
            local.set 3
            block ;; label = @4
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 2
              i32.load
              local.tee 5
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 5
              call_indirect (type 1)
            end
            block ;; label = @4
              local.get 2
              i32.load offset=4
              local.tee 5
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 5
              local.get 2
              i32.load offset=8
              i32.const 8704
              local.set 6
              local.get 6
              local.get 6
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 0
            i32.const 12
            i32.const 4
            i32.const 8736
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 0
          i32.store8 offset=8
          global.get $GOT.data.internal.__memory_base
          i32.const 1055880
          i32.add
          local.tee 0
          i32.load
          local.set 2
          local.get 0
          local.get 4
          i32.store
          local.get 1
          local.get 2
          i32.store offset=8
          block ;; label = @3
            local.get 2
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 2
            i32.load
            local.tee 4
            i32.const -1
            i32.add
            i32.store
            local.get 4
            i32.const 1
            i32.ne
            br_if 0 (;@3;)
            local.get 1
            i32.const 8
            i32.add
            i32.const 8768
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
          end
          i32.const 1
          local.set 2
        end
        local.get 1
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get 2
        return
      end
      local.get 1
      i64.const 0
      i64.store offset=20 align=4
      local.get 1
      i64.const 17179869185
      i64.store offset=12 align=4
      local.get 1
      global.get $GOT.data.internal.__memory_base
      local.tee 4
      i32.const 1054800
      i32.add
      i32.store offset=8
      i32.const 0
      local.get 1
      i32.const 7
      i32.add
      global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
      local.get 1
      i32.const 8
      i32.add
      local.get 4
      i32.const 1054808
      i32.add
      i32.const 8800
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E (;90;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8 offset=8
      local.get 3
      local.get 1
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054380
            i32.add
            local.get 2
            i32.const 8832
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 0
            i32.store offset=40
            local.get 3
            i32.const 1
            i32.store offset=28
            local.get 3
            i64.const 4
            i64.store offset=32 align=4
            local.get 3
            global.get $GOT.data.internal.__memory_base
            local.tee 1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get 3
            i32.const 24
            i32.add
            local.get 1
            i32.const 1054704
            i32.add
            i32.const 8864
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 3
          i32.load offset=12
          local.set 2
          block ;; label = @3
            local.get 3
            i32.load8_u offset=8
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
          end
          local.get 2
          i32.load
          local.set 0
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            local.get 1
            i32.load offset=8
            i32.const 8896
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          i32.const 8928
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2io5stdio6_print17h68847dc224af8aecE (;91;) (type 1) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 6
      i32.store offset=12
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1050253
      i32.add
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 8960
          local.set 2
          local.get 2
          local.get 2
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE
          br_if 0 (;@2;)
          block ;; label = @3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load8_u offset=40
            i32.const 3
            i32.eq
            br_if 0 (;@3;)
            i32.const 8992
            local.set 2
            local.get 2
            local.get 2
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE
          end
          local.get 1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          i32.store offset=28
          local.get 1
          local.get 1
          i32.const 28
          i32.add
          i32.store offset=40
          local.get 1
          i32.const 16
          i32.add
          local.get 1
          i32.const 40
          i32.add
          local.get 0
          i32.const 9024
          local.set 2
          local.get 2
          local.get 2
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E
          local.get 1
          i32.load8_u offset=16
          i32.const 4
          i32.ne
          br_if 1 (;@1;)
        end
        local.get 1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 1
      local.get 1
      i64.load offset=16
      i64.store offset=32
      local.get 1
      i32.const 2
      i32.store offset=44
      local.get 1
      global.get $GOT.data.internal.__memory_base
      local.tee 0
      i32.const 1054664
      i32.add
      i32.store offset=40
      local.get 1
      i64.const 2
      i64.store offset=52 align=4
      local.get 1
      global.get $GOT.func.internal._ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 1
      i32.const 32
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=72
      local.get 1
      global.get $GOT.data.internal.__table_base
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 1
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=64
      local.get 1
      local.get 1
      i32.const 64
      i32.add
      i32.store offset=48
      local.get 1
      i32.const 40
      i32.add
      local.get 0
      i32.const 1054680
      i32.add
      i32.const 9056
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE (;92;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i64 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 8
      i32.add
      local.get 0
      i32.load offset=8
      local.get 1
      local.get 2
      i32.const 9088
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E
      block ;; label = @1
        local.get 3
        i32.load8_u offset=8
        local.tee 2
        i32.const 4
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 4
        local.get 3
        i64.load offset=8
        local.set 5
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 4
          i32.load
          local.set 6
          block ;; label = @3
            local.get 4
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            local.get 1
            i32.load offset=8
            i32.const 9120
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          i32.const 9152
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        local.get 5
        i64.store align=4
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
      i32.const 4
      i32.ne
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E (;93;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i64 i32 i32 i64 i32)
      i32.const 0
      local.set 3
      block ;; label = @1
        i32.const 0
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.load offset=4
        local.tee 5
        local.get 4
        i64.load offset=8
        local.tee 6
        i64.const 4294967295
        local.get 6
        i64.const 4294967295
        i64.lt_u
        select
        i32.wrap_i64
        i32.sub
        local.tee 7
        local.get 7
        local.get 5
        i32.gt_u
        select
        local.tee 7
        local.get 2
        local.get 7
        local.get 2
        i32.lt_u
        select
        local.tee 8
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.load
        local.get 6
        local.get 5
        i64.extend_i32_u
        local.tee 9
        local.get 6
        local.get 9
        i64.lt_u
        select
        i32.wrap_i64
        i32.add
        local.get 1
        local.get 8
        memory.copy
      end
      local.get 4
      local.get 6
      local.get 8
      i64.extend_i32_u
      i64.add
      i64.store offset=8
      block ;; label = @1
        local.get 7
        local.get 2
        i32.ge_u
        br_if 0 (;@1;)
        global.get $GOT.data.internal.__memory_base
        i32.const 1054584
        i32.add
        i64.load
        local.tee 6
        i64.const 255
        i64.and
        i64.const 4
        i64.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 2
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 2
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 4
          i32.load
          local.set 7
          block ;; label = @3
            local.get 4
            i32.const 4
            i32.add
            i32.load
            local.tee 2
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 7
            local.get 5
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 2
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 7
            local.get 5
            local.get 2
            i32.load offset=8
            i32.const 9184
            local.set 10
            local.get 10
            local.get 10
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          i32.const 9216
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 0
        local.get 6
        i64.store align=4
        i32.const 1
        local.set 3
      end
      local.get 3
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E (;94;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 2
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        i32.const 1
        i32.const 1
        i32.const 9248
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get 0
        i32.load offset=8
        local.set 3
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 1
        local.get 2
        memory.copy
      end
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE (;95;) (type 7) (param i32 i32 i32 i32)
      (local i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            loop ;; label = @4
              local.get 4
              local.get 3
              i32.store offset=4
              local.get 4
              local.get 2
              i32.store
              local.get 4
              i32.const 8
              i32.add
              i32.const 2
              local.get 4
              i32.const 1
              i32.const 9280
              local.set 7
              local.get 7
              local.get 7
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i32.load16_u offset=8
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 4
                  i64.load16_u offset=10
                  local.tee 5
                  i64.const 27
                  i64.eq
                  br_if 1 (;@5;)
                  local.get 0
                  local.get 5
                  i64.const 32
                  i64.shl
                  i64.store align=4
                  br 4 (;@2;)
                end
                block ;; label = @6
                  local.get 4
                  i32.load offset=12
                  local.tee 6
                  br_if 0 (;@6;)
                  local.get 0
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  i64.store align=4
                  br 4 (;@2;)
                end
                local.get 3
                local.get 6
                i32.lt_u
                br_if 4 (;@1;)
                local.get 2
                local.get 6
                i32.add
                local.set 2
                local.get 3
                local.get 6
                i32.sub
                local.set 3
              end
              local.get 3
              br_if 0 (;@4;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
        end
        local.get 4
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 6
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      i32.const 9312
      local.set 7
      local.get 7
      local.get 7
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h1459a55272857e0fE (;96;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8 offset=8
      local.get 3
      local.get 1
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054428
            i32.add
            local.get 2
            i32.const 9344
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 0
            i32.store offset=40
            local.get 3
            i32.const 1
            i32.store offset=28
            local.get 3
            i64.const 4
            i64.store offset=32 align=4
            local.get 3
            global.get $GOT.data.internal.__memory_base
            local.tee 1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get 3
            i32.const 24
            i32.add
            local.get 1
            i32.const 1054704
            i32.add
            i32.const 9376
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 3
          i32.load offset=12
          local.set 2
          block ;; label = @3
            local.get 3
            i32.load8_u offset=8
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
          end
          local.get 2
          i32.load
          local.set 0
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            local.get 1
            i32.load offset=8
            i32.const 9408
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          i32.const 9440
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std5panic19get_backtrace_style17h54380a7febe96116E (;97;) (type 10) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      i32.const 3
      local.set 1
      block ;; label = @1
        global.get $GOT.data.internal.__memory_base
        i32.const 1055824
        i32.add
        i32.load8_u
        i32.const -1
        i32.add
        local.tee 2
        i32.const 255
        i32.and
        i32.const 3
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1049226
        i32.add
        i32.const 14
        i32.const 9472
        local.set 5
        local.get 5
        local.get 5
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std3env7_var_os17hecfa64e4c3898426E
        i32.const 2
        local.set 2
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const -2147483648
          i32.eq
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=8
          local.set 4
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.load offset=12
                    i32.const -1
                    i32.add
                    br_table 1 (;@6;) 2 (;@5;) 2 (;@5;) 0 (;@7;) 2 (;@5;)
                  end
                  local.get 4
                  i32.load align=1
                  i32.const 1819047270
                  i32.ne
                  br_if 1 (;@5;)
                  i32.const 1
                  local.set 2
                  i32.const 2
                  local.set 1
                  local.get 3
                  br_if 3 (;@3;)
                  br 4 (;@2;)
                end
                local.get 4
                i32.load8_u
                i32.const 48
                i32.eq
                br_if 1 (;@4;)
              end
              i32.const 0
              local.set 2
              i32.const 1
              local.set 1
              local.get 3
              i32.eqz
              br_if 2 (;@2;)
              br 1 (;@3;)
            end
            i32.const 2
            local.set 2
            i32.const 3
            local.set 1
            local.get 3
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 4
          local.get 3
          i32.const 1
          i32.const 9504
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055824
        i32.add
        local.tee 3
        local.get 3
        i32.load8_u
        local.tee 3
        local.get 1
        local.get 3
        select
        i32.store8
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        i32.const 3
        local.set 2
        local.get 3
        i32.const 4
        i32.ge_u
        br_if 0 (;@1;)
        i32.const 33619971
        local.get 3
        i32.const 3
        i32.shl
        i32.const 248
        i32.and
        i32.shr_u
        local.set 2
      end
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
    )
    (func $_ZN3std7process5abort17had5be8ae244d01ebE (;98;) (type 0)
      (local i32)
      i32.const 9536
      local.set 0
      local.get 0
      local.get 0
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E (;99;) (type 1) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load8_u offset=40
            br_table 1 (;@2;) 1 (;@2;) 0 (;@3;) 2 (;@1;) 1 (;@2;)
          end
          local.get 1
          i32.const 0
          i32.store offset=24
          local.get 1
          i32.const 1
          i32.store offset=12
          local.get 1
          i64.const 4
          i64.store offset=16 align=4
          local.get 1
          global.get $GOT.data.internal.__memory_base
          local.tee 0
          i32.const 1055340
          i32.add
          i32.store offset=8
          local.get 1
          i32.const 8
          i32.add
          local.get 0
          i32.const 1054840
          i32.add
          i32.const 9568
          local.set 2
          local.get 2
          local.get 2
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
          unreachable
        end
        local.get 0
        i32.const 1
        i32.store8
        global.get $GOT.data.internal.__memory_base
        i32.const 1055776
        i32.add
        local.tee 0
        i32.const 3
        i32.store8 offset=40
        local.get 0
        i32.const 0
        i32.store8 offset=32
        local.get 0
        i64.const 1
        i64.store offset=24
        local.get 0
        i64.const 0
        i64.store offset=16
        local.get 0
        i32.const 0
        i32.store8 offset=12
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        i64.const 0
        i64.store
      end
      local.get 1
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E (;100;) (type 10) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      global.get $GOT.data.internal.__memory_base
      i32.const 1055825
      i32.add
      local.tee 1
      i32.load8_u
      local.set 2
      local.get 1
      i32.const 1
      i32.store8
      local.get 0
      local.get 2
      i32.store8 offset=7
      block ;; label = @1
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i64.const 0
        i64.store offset=20 align=4
        local.get 0
        i64.const 17179869185
        i64.store offset=12 align=4
        local.get 0
        global.get $GOT.data.internal.__memory_base
        local.tee 2
        i32.const 1054800
        i32.add
        i32.store offset=8
        i32.const 0
        local.get 0
        i32.const 7
        i32.add
        global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.const 1054808
        i32.add
        i32.const 9600
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
        unreachable
      end
      global.get $GOT.data.internal.__memory_base
      local.set 2
      local.get 0
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 2
      i32.const 1055825
      i32.add
    )
    (func $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E (;101;) (type 7) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      i32.const 1
      i32.store offset=12
      local.get 4
      i64.const 1
      i64.store offset=20 align=4
      local.get 4
      global.get $GOT.data.internal.__memory_base
      i32.const 1049356
      i32.add
      i32.store offset=8
      local.get 4
      local.get 3
      i32.store8 offset=47
      local.get 4
      global.get $GOT.func.internal._ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 4
      i32.const 47
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 4
      local.get 4
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 0
      local.get 1
      local.get 4
      i32.const 8
      i32.add
      local.get 2
      call_indirect (type 4)
      local.get 4
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE (;102;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i64 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load
      local.set 4
      local.get 0
      i32.load8_u
      local.set 0
      local.get 2
      i32.const 4
      i32.add
      i32.const 9632
      local.set 10
      local.get 10
      local.get 10
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3env11current_dir17h890335e8528685e2E
      local.get 2
      i64.load offset=8 align=4
      local.set 5
      block ;; label = @1
        local.get 2
        i32.load offset=4
        local.tee 1
        i32.const -2147483648
        i32.ne
        br_if 0 (;@1;)
        local.get 5
        i64.const 255
        i64.and
        i64.const 3
        i64.ne
        br_if 0 (;@1;)
        local.get 5
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee 6
        i32.load
        local.set 7
        block ;; label = @2
          local.get 6
          i32.const 4
          i32.add
          i32.load
          local.tee 8
          i32.load
          local.tee 9
          i32.eqz
          br_if 0 (;@2;)
          local.get 7
          local.get 9
          call_indirect (type 1)
        end
        block ;; label = @2
          local.get 8
          i32.load offset=4
          local.tee 9
          i32.eqz
          br_if 0 (;@2;)
          local.get 7
          local.get 9
          local.get 8
          i32.load offset=8
          i32.const 9664
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 6
        i32.const 12
        i32.const 4
        i32.const 9696
        local.set 10
        local.get 10
        local.get 10
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 4
            global.get $GOT.data.internal.__memory_base
            i32.const 1050726
            i32.add
            i32.const 17
            local.get 3
            i32.load offset=12
            local.tee 3
            call_indirect (type 5)
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 0
              i32.const 1
              i32.and
              br_if 0 (;@4;)
              local.get 4
              global.get $GOT.data.internal.__memory_base
              i32.const 1050743
              i32.add
              i32.const 88
              local.get 3
              call_indirect (type 5)
              br_if 1 (;@3;)
            end
            i32.const 0
            local.set 4
            local.get 1
            i32.const -2147483648
            i32.or
            i32.const -2147483648
            i32.eq
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          i32.const 1
          local.set 4
          local.get 1
          i32.const -2147483648
          i32.or
          i32.const -2147483648
          i32.eq
          br_if 1 (;@1;)
        end
        local.get 5
        i32.wrap_i64
        local.get 1
        i32.const 1
        i32.const 9728
        local.set 10
        local.get 10
        local.get 10
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE (;103;) (type 1) (param i32)
      (local i32)
      local.get 0
      i32.const 9760
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE
      unreachable
    )
    (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE (;104;) (type 1) (param i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.tee 2
      i32.load offset=12
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load offset=4
              br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
            end
            local.get 3
            br_if 1 (;@2;)
            i32.const 1
            local.set 2
            i32.const 0
            local.set 3
            br 2 (;@1;)
          end
          local.get 3
          br_if 0 (;@2;)
          local.get 2
          i32.load
          local.tee 2
          i32.load offset=4
          local.set 3
          local.get 2
          i32.load
          local.set 2
          br 1 (;@1;)
        end
        local.get 1
        i32.const -2147483648
        i32.store
        global.get $GOT.data.internal.__memory_base
        local.set 2
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 1
        local.get 2
        i32.const 1055144
        i32.add
        local.get 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load8_u offset=8
        local.get 0
        i32.load8_u offset=9
        i32.const 9792
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E
        unreachable
      end
      local.get 1
      local.get 3
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055116
      i32.add
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      i32.const 9824
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E
      unreachable
    )
    (func $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E (;105;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        global.get $GOT.data.internal._RNvCscSpY9Juk0HT_7___rustc39___rust_alloc_error_handler_should_panic
        i32.load8_u
        br_if 0 (;@1;)
        local.get 2
        i32.const 2
        i32.store offset=12
        local.get 2
        global.get $GOT.data.internal.__memory_base
        i32.const 1054896
        i32.add
        i32.store offset=8
        local.get 2
        i64.const 1
        i64.store offset=20 align=4
        local.get 2
        global.get $GOT.func.internal._ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h0c6ce12b01068af4E
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 2
        i32.const 40
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=32
        local.get 2
        local.get 1
        i32.store offset=40
        local.get 2
        local.get 2
        i32.const 32
        i32.add
        i32.store offset=16
        local.get 2
        local.get 2
        i32.const 47
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i32.const 9856
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
        local.get 2
        i32.load offset=4
        local.set 3
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 3
          i32.load
          local.set 4
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            local.get 1
            i32.load offset=8
            i32.const 9888
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
          i32.const 9920
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 2
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 2
      i32.const 2
      i32.store offset=12
      local.get 2
      i64.const 1
      i64.store offset=20 align=4
      local.get 2
      global.get $GOT.data.internal.__memory_base
      local.tee 3
      i32.const 1054912
      i32.add
      i32.store offset=8
      local.get 2
      local.get 1
      i32.store
      local.get 2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h0c6ce12b01068af4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 2
      local.get 2
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.const 1054928
      i32.add
      i32.const 9952
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc (;106;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 8
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            local.get 0
            i32.le_u
            br_if 1 (;@2;)
          end
          local.get 2
          i32.const 0
          i32.store offset=12
          local.get 2
          i32.const 12
          i32.add
          local.get 1
          i32.const 4
          local.get 1
          i32.const 4
          i32.gt_u
          select
          local.get 0
          i32.const 9984
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $posix_memalign
          local.set 1
          i32.const 0
          local.get 2
          i32.load offset=12
          local.get 1
          select
          local.set 1
          br 1 (;@1;)
        end
        local.get 0
        i32.const 10016
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $malloc
        local.set 1
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc (;107;) (type 4) (param i32 i32 i32)
      (local i32)
      local.get 0
      i32.const 10048
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $free
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc (;108;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 8
            i32.gt_u
            br_if 0 (;@3;)
            local.get 2
            local.get 3
            i32.le_u
            br_if 1 (;@2;)
          end
          i32.const 0
          local.set 5
          local.get 4
          i32.const 0
          i32.store offset=12
          local.get 4
          i32.const 12
          i32.add
          local.get 2
          i32.const 4
          local.get 2
          i32.const 4
          i32.gt_u
          select
          local.get 3
          i32.const 10080
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $posix_memalign
          br_if 1 (;@1;)
          local.get 4
          i32.load offset=12
          local.tee 2
          i32.eqz
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 3
            local.get 1
            local.get 3
            local.get 1
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 0
            local.get 3
            memory.copy
          end
          local.get 0
          i32.const 10112
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $free
          local.get 2
          local.set 5
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i32.const 10144
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $realloc
        local.set 5
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 5
    )
    (func $_ZN3std9panicking14payload_as_str17h6b6acf98476ffb56E (;109;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      local.get 2
      i32.load offset=12
      local.tee 4
      call_indirect (type 6)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i64.load
            i64.const -5076933981314334344
            i64.ne
            br_if 0 (;@3;)
            i32.const 4
            local.set 2
            local.get 1
            local.set 5
            local.get 3
            i64.load offset=8
            i64.const 7199936582794304877
            i64.eq
            br_if 1 (;@2;)
          end
          local.get 3
          local.get 1
          local.get 4
          call_indirect (type 6)
          global.get $GOT.data.internal.__memory_base
          i32.const 1051014
          i32.add
          local.set 2
          i32.const 12
          local.set 4
          local.get 3
          i64.load
          i64.const 6756087622182587336
          i64.ne
          br_if 1 (;@1;)
          local.get 3
          i64.load offset=8
          i64.const -245993367077761921
          i64.ne
          br_if 1 (;@1;)
          local.get 1
          i32.const 4
          i32.add
          local.set 5
          i32.const 8
          local.set 2
        end
        local.get 1
        local.get 2
        i32.add
        i32.load
        local.set 4
        local.get 5
        i32.load
        local.set 2
      end
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE (;110;) (type 4) (param i32 i32 i32)
      (local i32 i32 i64 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 10176
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E
      local.set 4
      local.get 0
      i64.load align=4
      local.set 5
      local.get 3
      local.get 2
      i32.store offset=20
      local.get 3
      local.get 1
      i32.store offset=16
      local.get 3
      local.get 5
      i64.store offset=8 align=4
      block ;; label = @1
        block ;; label = @2
          global.get $GOT.data.internal._ZN3std6thread7current7CURRENT17h17ed3b660549d676E
          i32.load
          local.tee 6
          i32.const 2
          i32.gt_u
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          i32.const 1055864
          i32.add
          i64.load
          local.set 5
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.set 6
          block ;; label = @3
            block ;; label = @4
              local.get 5
              i64.eqz
              br_if 0 (;@4;)
              local.get 6
              i64.load
              local.get 5
              i64.eq
              br_if 1 (;@3;)
            end
            local.get 3
            i32.const 8
            i32.add
            i32.const 0
            local.get 3
            i32.const 10208
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
            br 2 (;@1;)
          end
          local.get 3
          i32.const 8
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1049222
          i32.add
          i32.const 4
          i32.const 10240
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 6
          i32.load offset=8
          local.tee 7
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          i32.const 8
          i32.add
          local.get 7
          local.get 6
          i32.const 12
          i32.add
          i32.load
          i32.const -1
          i32.add
          i32.const 10272
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br 1 (;@1;)
        end
        global.get $GOT.data.internal.__memory_base
        local.set 7
        block ;; label = @2
          local.get 6
          i64.load
          local.get 7
          i32.const 1055864
          i32.add
          i64.load
          i64.ne
          br_if 0 (;@2;)
          local.get 3
          i32.const 8
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1049222
          i32.add
          i32.const 4
          i32.const 10304
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br 1 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        i32.const 0
        local.get 3
        i32.const 10336
        local.set 8
        local.get 8
        local.get 8
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load offset=8
              i32.load8_u
              br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
            end
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            local.get 2
            i32.load offset=36
            i32.const 0
            i32.const 10368
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
            local.get 3
            i32.load8_u offset=8
            local.get 3
            i32.load offset=12
            i32.const 10400
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
            br 2 (;@1;)
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 1
          local.get 2
          i32.load offset=36
          i32.const 1
          i32.const 10432
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
          local.get 3
          i32.load8_u offset=8
          local.get 3
          i32.load offset=12
          i32.const 10464
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br 1 (;@1;)
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055080
        i32.add
        local.tee 0
        i32.load8_u
        local.set 6
        local.get 0
        i32.const 0
        i32.store8
        local.get 6
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 0
        i32.store offset=24
        local.get 3
        i32.const 1
        i32.store offset=12
        local.get 3
        global.get $GOT.data.internal.__memory_base
        i32.const 1055024
        i32.add
        i32.store offset=8
        local.get 3
        i64.const 4
        i64.store offset=16 align=4
        local.get 3
        local.get 1
        local.get 3
        i32.const 8
        i32.add
        local.get 2
        i32.load offset=36
        call_indirect (type 4)
        local.get 3
        i32.load8_u
        local.get 3
        i32.load offset=4
        i32.const 10496
        local.set 8
        local.get 8
        local.get 8
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
      end
      local.get 4
      i32.const 0
      i32.store8
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E (;111;) (type 4) (param i32 i32 i32)
      (local i32 i64 i64 i64 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 592
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 2
      i32.const 9
      local.get 1
      select
      i32.store offset=4
      local.get 3
      local.get 1
      global.get $GOT.data.internal.__memory_base
      local.tee 2
      i32.const 1050981
      i32.add
      local.get 1
      select
      i32.store
      block ;; label = @1
        i32.const 512
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 8
        i32.add
        i32.const 0
        i32.const 512
        memory.fill
      end
      local.get 3
      i64.const 0
      i64.store offset=528
      local.get 3
      i32.const 512
      i32.store offset=524
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=520
      local.get 0
      i64.load32_u
      local.set 4
      local.get 0
      i64.load32_u offset=4
      local.set 5
      local.get 3
      local.get 2
      i32.const 1055048
      i32.add
      i32.store offset=544
      local.get 3
      i64.const 3
      i64.store offset=556 align=4
      local.get 3
      local.get 5
      global.get $GOT.data.internal.__table_base
      local.tee 1
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 6
      i64.or
      local.tee 5
      i64.store offset=584
      local.get 3
      local.get 4
      local.get 1
      i32.const 10
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.or
      local.tee 4
      i64.store offset=576
      local.get 3
      local.get 6
      local.get 3
      i64.extend_i32_u
      i64.or
      local.tee 6
      i64.store offset=568
      local.get 3
      local.get 3
      i32.const 568
      i32.add
      i32.store offset=552
      local.get 3
      i32.const 4
      i32.store offset=548
      local.get 3
      i32.const 536
      i32.add
      local.get 3
      i32.const 520
      i32.add
      local.get 3
      i32.const 544
      i32.add
      i32.const 10528
      local.set 9
      local.get 9
      local.get 9
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std2io5Write9write_fmt17h1459a55272857e0fE
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.load8_u offset=536
            local.tee 1
            i32.const 4
            i32.ne
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=528
            local.tee 1
            i32.const 513
            i32.ge_u
            br_if 2 (;@1;)
            local.get 3
            i32.const 568
            i32.add
            local.get 0
            i32.load offset=8
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            local.get 0
            i32.load offset=12
            i32.load offset=28
            call_indirect (type 7)
            local.get 3
            i32.load offset=572
            local.set 1
            block ;; label = @4
              local.get 3
              i32.load8_u offset=568
              local.tee 0
              i32.const 4
              i32.gt_u
              br_if 0 (;@4;)
              local.get 0
              i32.const 3
              i32.ne
              br_if 2 (;@2;)
            end
            local.get 1
            i32.load
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 0
              i32.load
              local.tee 7
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 7
              call_indirect (type 1)
            end
            block ;; label = @4
              local.get 0
              i32.load offset=4
              local.tee 7
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 7
              local.get 0
              i32.load offset=8
              i32.const 10560
              local.set 9
              local.get 9
              local.get 9
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 1
            i32.const 12
            i32.const 4
            i32.const 10592
            local.set 9
            local.get 9
            local.get 9
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 1
            i32.const 3
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=540
            local.tee 1
            i32.load
            local.set 7
            block ;; label = @4
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 2
              i32.load
              local.tee 8
              i32.eqz
              br_if 0 (;@4;)
              local.get 7
              local.get 8
              call_indirect (type 1)
            end
            block ;; label = @4
              local.get 2
              i32.load offset=4
              local.tee 8
              i32.eqz
              br_if 0 (;@4;)
              local.get 7
              local.get 8
              local.get 2
              i32.load offset=8
              i32.const 10624
              local.set 9
              local.get 9
              local.get 9
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 1
            i32.const 12
            i32.const 4
            i32.const 10656
            local.set 9
            local.get 9
            local.get 9
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 0
          i32.load offset=12
          i32.const 36
          i32.add
          i32.load
          local.set 1
          global.get $GOT.data.internal.__memory_base
          local.set 2
          local.get 0
          i32.load offset=8
          local.set 0
          local.get 3
          local.get 2
          i32.const 1055048
          i32.add
          i32.store offset=544
          local.get 3
          i64.const 3
          i64.store offset=556 align=4
          local.get 3
          local.get 5
          i64.store offset=584
          local.get 3
          local.get 4
          i64.store offset=576
          local.get 3
          local.get 6
          i64.store offset=568
          local.get 3
          local.get 3
          i32.const 568
          i32.add
          i32.store offset=552
          local.get 3
          i32.const 4
          i32.store offset=548
          local.get 3
          i32.const 536
          i32.add
          local.get 0
          local.get 3
          i32.const 544
          i32.add
          local.get 1
          call_indirect (type 4)
          local.get 3
          i32.load offset=540
          local.set 1
          block ;; label = @3
            local.get 3
            i32.load8_u offset=536
            local.tee 0
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 0
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 1
          i32.load
          local.set 2
          block ;; label = @3
            local.get 1
            i32.const 4
            i32.add
            i32.load
            local.tee 0
            i32.load
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 7
            call_indirect (type 1)
          end
          block ;; label = @3
            local.get 0
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 7
            local.get 0
            i32.load offset=8
            i32.const 10688
            local.set 9
            local.get 9
            local.get 9
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 1
          i32.const 12
          i32.const 4
          i32.const 10720
          local.set 9
          local.get 9
          local.get 9
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 3
        i32.const 592
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 1
      i32.const 512
      global.get $GOT.data.internal.__memory_base
      i32.const 1055032
      i32.add
      i32.const 10752
      local.set 9
      local.get 9
      local.get 9
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN3std9panicking11panic_count8increase17hbcaad6b35138193cE (;112;) (type 3) (param i32) (result i32)
      (local i32 i32)
      global.get $GOT.data.internal._ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h79e1014250ce3477E
      local.tee 1
      local.get 1
      i32.load
      local.tee 2
      i32.const 1
      i32.add
      i32.store
      i32.const 0
      local.set 1
      block ;; label = @1
        local.get 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@1;)
        i32.const 1
        local.set 1
        global.get $GOT.data.internal.__memory_base
        i32.const 1055876
        i32.add
        i32.load8_u
        br_if 0 (;@1;)
        global.get $GOT.data.internal.__memory_base
        local.tee 1
        i32.const 1055876
        i32.add
        local.get 0
        i32.store8
        local.get 1
        i32.const 1055872
        i32.add
        local.tee 1
        local.get 1
        i32.load
        i32.const 1
        i32.add
        i32.store
        i32.const 2
        local.set 1
      end
      local.get 1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind (;113;) (type 1) (param i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i64.load align=4
      local.set 2
      local.get 1
      local.get 0
      i32.store offset=12
      local.get 1
      local.get 2
      i64.store offset=4 align=4
      local.get 1
      i32.const 4
      i32.add
      i32.const 10784
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE
      unreachable
    )
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E (;114;) (type 6) (param i32 i32)
      (local i32 i32 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 1
        i32.load
        i32.const -2147483648
        i32.ne
        br_if 0 (;@1;)
        local.get 1
        i32.load offset=12
        local.set 3
        local.get 2
        i32.const 28
        i32.add
        i32.const 8
        i32.add
        local.tee 4
        i32.const 0
        i32.store
        local.get 2
        i64.const 4294967296
        i64.store offset=28 align=4
        local.get 2
        i32.const 40
        i32.add
        i32.const 8
        i32.add
        local.get 3
        i32.load
        local.tee 3
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        i32.const 40
        i32.add
        i32.const 16
        i32.add
        local.get 3
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        local.get 3
        i64.load align=4
        i64.store offset=40
        local.get 2
        i32.const 28
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1054356
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 10816
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        drop
        local.get 2
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        local.get 4
        i32.load
        local.tee 3
        i32.store
        local.get 2
        local.get 2
        i64.load offset=28 align=4
        local.tee 5
        i64.store offset=16
        local.get 1
        i32.const 8
        i32.add
        local.get 3
        i32.store
        local.get 1
        local.get 5
        i64.store align=4
      end
      local.get 1
      i64.load align=4
      local.set 5
      local.get 1
      i64.const 4294967296
      i64.store align=4
      local.get 2
      i32.const 8
      i32.add
      local.tee 3
      local.get 1
      i32.const 8
      i32.add
      local.tee 1
      i32.load
      i32.store
      local.get 1
      i32.const 0
      i32.store
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      local.get 2
      local.get 5
      i64.store
      block ;; label = @1
        i32.const 12
        i32.const 4
        i32.const 10848
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 12
        i32.const 10880
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get 1
      local.get 2
      i64.load
      i64.store align=4
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.load
      i32.store
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055084
      i32.add
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE (;115;) (type 6) (param i32 i32)
      (local i32 i32 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 1
        i32.load
        i32.const -2147483648
        i32.ne
        br_if 0 (;@1;)
        local.get 1
        i32.load offset=12
        local.set 3
        local.get 2
        i32.const 12
        i32.add
        i32.const 8
        i32.add
        local.tee 4
        i32.const 0
        i32.store
        local.get 2
        i64.const 4294967296
        i64.store offset=12 align=4
        local.get 2
        i32.const 24
        i32.add
        i32.const 8
        i32.add
        local.get 3
        i32.load
        local.tee 3
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        i32.const 24
        i32.add
        i32.const 16
        i32.add
        local.get 3
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        local.get 3
        i64.load align=4
        i64.store offset=24
        local.get 2
        i32.const 12
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1054356
        i32.add
        local.get 2
        i32.const 24
        i32.add
        i32.const 10912
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        drop
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        i32.load
        local.tee 3
        i32.store
        local.get 2
        local.get 2
        i64.load offset=12 align=4
        local.tee 5
        i64.store
        local.get 1
        i32.const 8
        i32.add
        local.get 3
        i32.store
        local.get 1
        local.get 5
        i64.store align=4
      end
      local.get 0
      local.get 1
      i32.store
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055084
      i32.add
      i32.store offset=4
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE (;116;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          i32.const -2147483648
          i32.eq
          br_if 0 (;@2;)
          local.get 1
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=8
          i32.const 10944
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
          local.set 0
          br 1 (;@1;)
        end
        local.get 2
        i32.const 8
        i32.add
        i32.const 8
        i32.add
        local.get 0
        i32.load offset=12
        i32.load
        local.tee 0
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        i32.const 8
        i32.add
        i32.const 16
        i32.add
        local.get 0
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        local.get 0
        i64.load align=4
        i64.store offset=8
        local.get 1
        i32.load
        local.get 1
        i32.load offset=4
        local.get 2
        i32.const 8
        i32.add
        i32.const 10976
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        local.set 0
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E (;117;) (type 6) (param i32 i32)
      (local i32 i32 i32)
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      local.get 1
      i32.load offset=4
      local.set 2
      local.get 1
      i32.load
      local.set 3
      block ;; label = @1
        i32.const 8
        i32.const 4
        i32.const 11008
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 8
        i32.const 11040
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get 1
      local.get 2
      i32.store offset=4
      local.get 1
      local.get 3
      i32.store
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055100
      i32.add
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E (;118;) (type 6) (param i32 i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055100
      i32.add
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE (;119;) (type 6) (param i32 i32)
      local.get 0
      local.get 1
      i64.load align=4
      i64.store
    )
    (func $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E (;120;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      i32.const 11072
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
    )
    (func $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E (;121;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 96
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 1
      i32.store offset=32
      local.get 5
      local.get 0
      i32.store offset=28
      local.get 5
      local.get 2
      i32.store offset=36
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              i32.const 1
              i32.const 11104
              local.set 8
              local.get 8
              local.get 8
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN3std9panicking11panic_count8increase17hbcaad6b35138193cE
              i32.const 255
              i32.and
              local.tee 6
              i32.const 2
              i32.eq
              br_if 0 (;@4;)
              local.get 6
              i32.const 1
              i32.and
              i32.eqz
              br_if 1 (;@3;)
              local.get 5
              i32.const 16
              i32.add
              local.get 0
              local.get 1
              i32.load offset=24
              call_indirect (type 6)
              local.get 5
              local.get 5
              i32.load offset=20
              i32.const 0
              local.get 5
              i32.load offset=16
              local.tee 1
              select
              i32.store offset=44
              local.get 5
              local.get 1
              i32.const 1
              local.get 1
              select
              i32.store offset=40
              local.get 5
              i32.const 3
              i32.store offset=76
              local.get 5
              global.get $GOT.data.internal.__memory_base
              i32.const 1055196
              i32.add
              i32.store offset=72
              local.get 5
              i64.const 2
              i64.store offset=84 align=4
              local.get 5
              global.get $GOT.data.internal.__table_base
              local.tee 1
              i32.const 6
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 5
              i32.const 40
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=64
              local.get 5
              local.get 1
              i32.const 10
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 5
              i32.const 36
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=56
              local.get 5
              local.get 5
              i32.const 56
              i32.add
              i32.store offset=80
              local.get 5
              i32.const 48
              i32.add
              local.get 5
              i32.const 48
              i32.add
              local.get 5
              i32.const 72
              i32.add
              i32.const 11136
              local.set 8
              local.get 8
              local.get 8
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
              local.get 5
              i32.load8_u offset=48
              local.get 5
              i32.load offset=52
              i32.const 11168
              local.set 8
              local.get 8
              local.get 8
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
              br 3 (;@1;)
            end
            global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
            i32.load
            local.tee 6
            i32.const -1
            i32.gt_s
            br_if 1 (;@2;)
            local.get 5
            i32.const 1
            i32.store offset=76
            local.get 5
            global.get $GOT.data.internal.__memory_base
            i32.const 1055348
            i32.add
            i32.store offset=72
            local.get 5
            i64.const 0
            i64.store offset=84 align=4
            local.get 5
            local.get 5
            i32.const 48
            i32.add
            i32.store offset=80
            local.get 5
            i32.const 56
            i32.add
            local.get 5
            i32.const 48
            i32.add
            local.get 5
            i32.const 72
            i32.add
            i32.const 11200
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
            local.get 5
            i32.load8_u offset=56
            local.get 5
            i32.load offset=60
            i32.const 11232
            local.set 8
            local.get 8
            local.get 8
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
            br 2 (;@1;)
          end
          local.get 5
          i32.const 3
          i32.store offset=76
          local.get 5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055172
          i32.add
          i32.store offset=72
          local.get 5
          i64.const 2
          i64.store offset=84 align=4
          local.get 5
          global.get $GOT.data.internal.__table_base
          local.tee 1
          i32.const 11
          i32.add
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 28
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=64
          local.get 5
          local.get 1
          i32.const 10
          i32.add
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 36
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=56
          local.get 5
          local.get 5
          i32.const 56
          i32.add
          i32.store offset=80
          local.get 5
          i32.const 48
          i32.add
          local.get 5
          i32.const 48
          i32.add
          local.get 5
          i32.const 72
          i32.add
          i32.const 11264
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get 5
          i32.load8_u offset=48
          local.get 5
          i32.load offset=52
          i32.const 11296
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br 1 (;@1;)
        end
        global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
        local.tee 7
        local.get 6
        i32.const 1
        i32.add
        i32.store
        block ;; label = @2
          block ;; label = @3
            local.get 7
            i32.load offset=4
            i32.eqz
            br_if 0 (;@3;)
            local.get 5
            i32.const 8
            i32.add
            local.get 0
            local.get 1
            i32.load offset=20
            call_indirect (type 6)
            local.get 5
            local.get 4
            i32.store8 offset=85
            local.get 5
            local.get 3
            i32.store8 offset=84
            local.get 5
            local.get 2
            i32.store offset=80
            local.get 5
            local.get 5
            i64.load offset=8
            i64.store offset=72 align=4
            global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
            local.tee 2
            i32.load offset=4
            local.get 5
            i32.const 72
            i32.add
            local.get 2
            i32.load offset=8
            i32.load offset=20
            call_indirect (type 6)
            br 1 (;@2;)
          end
          local.get 5
          local.get 0
          local.get 1
          i32.load offset=20
          call_indirect (type 6)
          local.get 5
          local.get 4
          i32.store8 offset=85
          local.get 5
          local.get 3
          i32.store8 offset=84
          local.get 5
          local.get 2
          i32.store offset=80
          local.get 5
          local.get 5
          i64.load
          i64.store offset=72 align=4
          local.get 5
          i32.const 72
          i32.add
          i32.const 11328
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std9panicking12default_hook17h8deeaf2f2b300de3E
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055876
        i32.add
        i32.const 0
        i32.store8
        global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
        local.tee 2
        local.get 2
        i32.load
        i32.const -1
        i32.add
        i32.store
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          local.get 5
          i32.const 0
          i32.store offset=88
          local.get 5
          i32.const 1
          i32.store offset=76
          local.get 5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055220
          i32.add
          i32.store offset=72
          local.get 5
          i64.const 4
          i64.store offset=80 align=4
          local.get 5
          i32.const 56
          i32.add
          local.get 5
          i32.const 48
          i32.add
          local.get 5
          i32.const 72
          i32.add
          i32.const 11360
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get 5
          i32.load8_u offset=56
          local.get 5
          i32.load offset=60
          i32.const 11392
          local.set 8
          local.get 8
          local.get 8
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br 1 (;@1;)
        end
        local.get 0
        local.get 1
        i32.const 11424
        local.set 8
        local.get 8
        local.get 8
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc10rust_panic
        unreachable
      end
      i32.const 11456
      local.set 8
      local.get 8
      local.get 8
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc10rust_panic (;122;) (type 6) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      local.get 1
      i32.const 11488
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc18___rust_start_panic
      i32.store offset=12
      local.get 2
      i32.const 2
      i32.store offset=28
      local.get 2
      global.get $GOT.data.internal.__memory_base
      i32.const 1055228
      i32.add
      i32.store offset=24
      local.get 2
      i64.const 1
      i64.store offset=36 align=4
      local.get 2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=48
      local.get 2
      local.get 2
      i32.const 48
      i32.add
      i32.store offset=32
      local.get 2
      i32.const 16
      i32.add
      local.get 2
      i32.const 63
      i32.add
      local.get 2
      i32.const 24
      i32.add
      i32.const 11520
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
      local.get 2
      i32.load8_u offset=16
      local.get 2
      i32.load offset=20
      i32.const 11552
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
      i32.const 11584
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $cabi_realloc (;123;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
            local.get 3
            local.get 2
            i32.const 11616
            local.set 4
            local.get 4
            local.get 4
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
            local.tee 2
            i32.eqz
            br_if 1 (;@2;)
            br 2 (;@1;)
          end
          local.get 0
          local.get 1
          local.get 2
          local.get 3
          i32.const 11648
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
          local.tee 2
          br_if 1 (;@1;)
        end
        i32.const 11680
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
        unreachable
      end
      local.get 2
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE (;124;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      local.get 3
      i32.store offset=4
      local.get 4
      local.get 2
      i32.store
      local.get 4
      i32.const 8
      i32.add
      i32.const 2
      local.get 4
      i32.const 1
      i32.const 11712
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=8
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i64.load16_u offset=10
          i64.const 32
          i64.shl
          i64.store align=4
          br 1 (;@1;)
        end
        local.get 0
        local.get 4
        i32.load offset=12
        i32.store offset=4
        local.get 0
        i32.const 4
        i32.store8
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E (;125;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      i32.const 8
      i32.add
      i32.const 2
      local.get 2
      local.get 3
      i32.const 11744
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=8
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i64.load16_u offset=10
          i64.const 32
          i64.shl
          i64.store align=4
          br 1 (;@1;)
        end
        local.get 0
        local.get 4
        i32.load offset=12
        i32.store offset=4
        local.get 0
        i32.const 4
        i32.store8
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE (;126;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E (;127;) (type 6) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E (;128;) (type 6) (param i32 i32)
      (local i32 i32 i32)
      global.get $GOT.data.internal.__table_base
      local.set 2
      local.get 0
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055828
      i32.add
      i32.load
      local.tee 3
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      select
      call_indirect (type 6)
      i32.const 11776
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std7process5abort17had5be8ae244d01ebE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom (;129;) (type 6) (param i32 i32)
      (local i32)
      local.get 1
      local.get 0
      i32.const 11808
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc18___rust_start_panic (;130;) (type 2) (param i32 i32) (result i32)
      unreachable
    )
    (func $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE (;131;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 2
          local.get 3
          local.get 4
          i32.const 12
          i32.add
          i32.const 11840
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h5858df6e6eba6e92E
          local.tee 3
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=12
          i32.store offset=4
          i32.const 0
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i32.store16 offset=2
        i32.const 1
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store16
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $malloc (;132;) (type 3) (param i32) (result i32)
      (local i32)
      local.get 0
      i32.const 11872
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $dlmalloc
    )
    (func $dlmalloc (;133;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1055912
                                local.tee 2
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1056360
                                  local.tee 3
                                  br_if 0 (;@14;)
                                  i32.const 0
                                  i64.const -1
                                  i64.store offset=1056372 align=4
                                  i32.const 0
                                  i64.const 281474976776192
                                  i64.store offset=1056364 align=4
                                  i32.const 0
                                  local.get 1
                                  i32.const 8
                                  i32.add
                                  i32.const -16
                                  i32.and
                                  i32.const 1431655768
                                  i32.xor
                                  local.tee 3
                                  i32.store offset=1056360
                                  i32.const 0
                                  i32.const 0
                                  i32.store offset=1056380
                                  i32.const 0
                                  i32.const 0
                                  i32.store offset=1056332
                                end
                                i32.const 1114112
                                i32.const 1056432
                                i32.lt_u
                                br_if 1 (;@12;)
                                i32.const 0
                                local.set 2
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                i32.const 89
                                i32.lt_u
                                br_if 0 (;@13;)
                                i32.const 0
                                local.set 4
                                i32.const 0
                                i32.const 1056432
                                i32.store offset=1056336
                                i32.const 0
                                i32.const 1056432
                                i32.store offset=1055904
                                i32.const 0
                                local.get 3
                                i32.store offset=1055924
                                i32.const 0
                                i32.const -1
                                i32.store offset=1055920
                                i32.const 0
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                local.tee 3
                                i32.store offset=1056340
                                i32.const 0
                                local.get 3
                                i32.store offset=1056324
                                i32.const 0
                                local.get 3
                                i32.store offset=1056320
                                loop ;; label = @14
                                  local.get 4
                                  i32.const 1055948
                                  i32.add
                                  local.get 4
                                  i32.const 1055936
                                  i32.add
                                  local.tee 3
                                  i32.store
                                  local.get 3
                                  local.get 4
                                  i32.const 1055928
                                  i32.add
                                  local.tee 5
                                  i32.store
                                  local.get 4
                                  i32.const 1055940
                                  i32.add
                                  local.get 5
                                  i32.store
                                  local.get 4
                                  i32.const 1055956
                                  i32.add
                                  local.get 4
                                  i32.const 1055944
                                  i32.add
                                  local.tee 5
                                  i32.store
                                  local.get 5
                                  local.get 3
                                  i32.store
                                  local.get 4
                                  i32.const 1055964
                                  i32.add
                                  local.get 4
                                  i32.const 1055952
                                  i32.add
                                  local.tee 3
                                  i32.store
                                  local.get 3
                                  local.get 5
                                  i32.store
                                  local.get 4
                                  i32.const 1055960
                                  i32.add
                                  local.get 3
                                  i32.store
                                  local.get 4
                                  i32.const 32
                                  i32.add
                                  local.tee 4
                                  i32.const 256
                                  i32.ne
                                  br_if 0 (;@14;)
                                end
                                i32.const 1114112
                                i32.const -52
                                i32.add
                                i32.const 56
                                i32.store
                                i32.const 0
                                i32.const 0
                                i32.load offset=1056376
                                i32.store offset=1055916
                                i32.const 0
                                i32.const 1056432
                                i32.const -8
                                i32.const 1056432
                                i32.sub
                                i32.const 15
                                i32.and
                                local.tee 4
                                i32.add
                                local.tee 2
                                i32.store offset=1055912
                                i32.const 0
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                local.get 4
                                i32.sub
                                i32.const -56
                                i32.add
                                local.tee 4
                                i32.store offset=1055900
                                local.get 2
                                local.get 4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                              end
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 0
                                  i32.const 236
                                  i32.gt_u
                                  br_if 0 (;@14;)
                                  block ;; label = @15
                                    i32.const 0
                                    i32.load offset=1055888
                                    local.tee 6
                                    i32.const 16
                                    local.get 0
                                    i32.const 19
                                    i32.add
                                    i32.const 496
                                    i32.and
                                    local.get 0
                                    i32.const 11
                                    i32.lt_u
                                    select
                                    local.tee 5
                                    i32.const 3
                                    i32.shr_u
                                    local.tee 3
                                    i32.shr_u
                                    local.tee 4
                                    i32.const 3
                                    i32.and
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    block ;; label = @16
                                      block ;; label = @17
                                        local.get 4
                                        i32.const 1
                                        i32.and
                                        local.get 3
                                        i32.or
                                        i32.const 1
                                        i32.xor
                                        local.tee 5
                                        i32.const 3
                                        i32.shl
                                        local.tee 3
                                        i32.const 1055928
                                        i32.add
                                        local.tee 4
                                        local.get 3
                                        i32.const 1055936
                                        i32.add
                                        i32.load
                                        local.tee 3
                                        i32.load offset=8
                                        local.tee 0
                                        i32.ne
                                        br_if 0 (;@17;)
                                        i32.const 0
                                        local.get 6
                                        i32.const -2
                                        local.get 5
                                        i32.rotl
                                        i32.and
                                        i32.store offset=1055888
                                        br 1 (;@16;)
                                      end
                                      local.get 4
                                      local.get 0
                                      i32.store offset=8
                                      local.get 0
                                      local.get 4
                                      i32.store offset=12
                                    end
                                    local.get 3
                                    i32.const 8
                                    i32.add
                                    local.set 4
                                    local.get 3
                                    local.get 5
                                    i32.const 3
                                    i32.shl
                                    local.tee 5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    local.get 3
                                    local.get 5
                                    i32.add
                                    local.tee 3
                                    local.get 3
                                    i32.load offset=4
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    br 14 (;@1;)
                                  end
                                  local.get 5
                                  i32.const 0
                                  i32.load offset=1055896
                                  local.tee 7
                                  i32.le_u
                                  br_if 1 (;@13;)
                                  block ;; label = @15
                                    local.get 4
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    block ;; label = @16
                                      block ;; label = @17
                                        local.get 4
                                        local.get 3
                                        i32.shl
                                        i32.const 2
                                        local.get 3
                                        i32.shl
                                        local.tee 4
                                        i32.const 0
                                        local.get 4
                                        i32.sub
                                        i32.or
                                        i32.and
                                        i32.ctz
                                        local.tee 3
                                        i32.const 3
                                        i32.shl
                                        local.tee 4
                                        i32.const 1055928
                                        i32.add
                                        local.tee 0
                                        local.get 4
                                        i32.const 1055936
                                        i32.add
                                        i32.load
                                        local.tee 4
                                        i32.load offset=8
                                        local.tee 8
                                        i32.ne
                                        br_if 0 (;@17;)
                                        i32.const 0
                                        local.get 6
                                        i32.const -2
                                        local.get 3
                                        i32.rotl
                                        i32.and
                                        local.tee 6
                                        i32.store offset=1055888
                                        br 1 (;@16;)
                                      end
                                      local.get 0
                                      local.get 8
                                      i32.store offset=8
                                      local.get 8
                                      local.get 0
                                      i32.store offset=12
                                    end
                                    local.get 4
                                    local.get 5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    local.get 4
                                    local.get 3
                                    i32.const 3
                                    i32.shl
                                    local.tee 3
                                    i32.add
                                    local.get 3
                                    local.get 5
                                    i32.sub
                                    local.tee 0
                                    i32.store
                                    local.get 4
                                    local.get 5
                                    i32.add
                                    local.tee 8
                                    local.get 0
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    block ;; label = @16
                                      local.get 7
                                      i32.eqz
                                      br_if 0 (;@16;)
                                      local.get 7
                                      i32.const -8
                                      i32.and
                                      i32.const 1055928
                                      i32.add
                                      local.set 5
                                      i32.const 0
                                      i32.load offset=1055908
                                      local.set 3
                                      block ;; label = @17
                                        block ;; label = @18
                                          local.get 6
                                          i32.const 1
                                          local.get 7
                                          i32.const 3
                                          i32.shr_u
                                          i32.shl
                                          local.tee 9
                                          i32.and
                                          br_if 0 (;@18;)
                                          i32.const 0
                                          local.get 6
                                          local.get 9
                                          i32.or
                                          i32.store offset=1055888
                                          local.get 5
                                          local.set 9
                                          br 1 (;@17;)
                                        end
                                        local.get 5
                                        i32.load offset=8
                                        local.set 9
                                      end
                                      local.get 9
                                      local.get 3
                                      i32.store offset=12
                                      local.get 5
                                      local.get 3
                                      i32.store offset=8
                                      local.get 3
                                      local.get 5
                                      i32.store offset=12
                                      local.get 3
                                      local.get 9
                                      i32.store offset=8
                                    end
                                    local.get 4
                                    i32.const 8
                                    i32.add
                                    local.set 4
                                    i32.const 0
                                    local.get 8
                                    i32.store offset=1055908
                                    i32.const 0
                                    local.get 0
                                    i32.store offset=1055896
                                    br 14 (;@1;)
                                  end
                                  i32.const 0
                                  i32.load offset=1055892
                                  local.tee 10
                                  i32.eqz
                                  br_if 1 (;@13;)
                                  local.get 10
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 1056192
                                  i32.add
                                  i32.load
                                  local.tee 8
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 5
                                  i32.sub
                                  local.set 3
                                  local.get 8
                                  local.set 0
                                  block ;; label = @15
                                    loop ;; label = @16
                                      block ;; label = @17
                                        local.get 0
                                        i32.load offset=16
                                        local.tee 4
                                        br_if 0 (;@17;)
                                        local.get 0
                                        i32.load offset=20
                                        local.tee 4
                                        i32.eqz
                                        br_if 2 (;@15;)
                                      end
                                      local.get 4
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 5
                                      i32.sub
                                      local.tee 0
                                      local.get 3
                                      local.get 0
                                      local.get 3
                                      i32.lt_u
                                      local.tee 0
                                      select
                                      local.set 3
                                      local.get 4
                                      local.get 8
                                      local.get 0
                                      select
                                      local.set 8
                                      local.get 4
                                      local.set 0
                                      br 0 (;@16;)
                                    end
                                  end
                                  local.get 8
                                  i32.load offset=24
                                  local.set 2
                                  block ;; label = @15
                                    local.get 8
                                    i32.load offset=12
                                    local.tee 4
                                    local.get 8
                                    i32.eq
                                    br_if 0 (;@15;)
                                    local.get 8
                                    i32.load offset=8
                                    local.tee 0
                                    local.get 4
                                    i32.store offset=12
                                    local.get 4
                                    local.get 0
                                    i32.store offset=8
                                    br 13 (;@2;)
                                  end
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 8
                                      i32.load offset=20
                                      local.tee 0
                                      i32.eqz
                                      br_if 0 (;@16;)
                                      local.get 8
                                      i32.const 20
                                      i32.add
                                      local.set 9
                                      br 1 (;@15;)
                                    end
                                    local.get 8
                                    i32.load offset=16
                                    local.tee 0
                                    i32.eqz
                                    br_if 4 (;@11;)
                                    local.get 8
                                    i32.const 16
                                    i32.add
                                    local.set 9
                                  end
                                  loop ;; label = @15
                                    local.get 9
                                    local.set 11
                                    local.get 0
                                    local.tee 4
                                    i32.const 20
                                    i32.add
                                    local.set 9
                                    local.get 4
                                    i32.load offset=20
                                    local.tee 0
                                    br_if 0 (;@15;)
                                    local.get 4
                                    i32.const 16
                                    i32.add
                                    local.set 9
                                    local.get 4
                                    i32.load offset=16
                                    local.tee 0
                                    br_if 0 (;@15;)
                                  end
                                  local.get 11
                                  i32.const 0
                                  i32.store
                                  br 12 (;@2;)
                                end
                                i32.const -1
                                local.set 5
                                local.get 0
                                i32.const -65
                                i32.gt_u
                                br_if 0 (;@13;)
                                local.get 0
                                i32.const 19
                                i32.add
                                local.tee 4
                                i32.const -16
                                i32.and
                                local.set 5
                                i32.const 0
                                i32.load offset=1055892
                                local.tee 10
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 31
                                local.set 7
                                block ;; label = @14
                                  local.get 0
                                  i32.const 16777196
                                  i32.gt_u
                                  br_if 0 (;@14;)
                                  local.get 5
                                  i32.const 38
                                  local.get 4
                                  i32.const 8
                                  i32.shr_u
                                  i32.clz
                                  local.tee 4
                                  i32.sub
                                  i32.shr_u
                                  i32.const 1
                                  i32.and
                                  local.get 4
                                  i32.const 1
                                  i32.shl
                                  i32.sub
                                  i32.const 62
                                  i32.add
                                  local.set 7
                                end
                                i32.const 0
                                local.get 5
                                i32.sub
                                local.set 3
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      block ;; label = @17
                                        local.get 7
                                        i32.const 2
                                        i32.shl
                                        i32.const 1056192
                                        i32.add
                                        i32.load
                                        local.tee 0
                                        br_if 0 (;@17;)
                                        i32.const 0
                                        local.set 4
                                        i32.const 0
                                        local.set 9
                                        br 1 (;@16;)
                                      end
                                      i32.const 0
                                      local.set 4
                                      local.get 5
                                      i32.const 0
                                      i32.const 25
                                      local.get 7
                                      i32.const 1
                                      i32.shr_u
                                      i32.sub
                                      local.get 7
                                      i32.const 31
                                      i32.eq
                                      select
                                      i32.shl
                                      local.set 8
                                      i32.const 0
                                      local.set 9
                                      loop ;; label = @17
                                        block ;; label = @18
                                          local.get 0
                                          i32.load offset=4
                                          i32.const -8
                                          i32.and
                                          local.get 5
                                          i32.sub
                                          local.tee 6
                                          local.get 3
                                          i32.ge_u
                                          br_if 0 (;@18;)
                                          local.get 6
                                          local.set 3
                                          local.get 0
                                          local.set 9
                                          local.get 6
                                          br_if 0 (;@18;)
                                          i32.const 0
                                          local.set 3
                                          local.get 0
                                          local.set 9
                                          local.get 0
                                          local.set 4
                                          br 3 (;@15;)
                                        end
                                        local.get 4
                                        local.get 0
                                        i32.load offset=20
                                        local.tee 6
                                        local.get 6
                                        local.get 0
                                        local.get 8
                                        i32.const 29
                                        i32.shr_u
                                        i32.const 4
                                        i32.and
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i32.load
                                        local.tee 11
                                        i32.eq
                                        select
                                        local.get 4
                                        local.get 6
                                        select
                                        local.set 4
                                        local.get 8
                                        i32.const 1
                                        i32.shl
                                        local.set 8
                                        local.get 11
                                        local.set 0
                                        local.get 11
                                        br_if 0 (;@17;)
                                      end
                                    end
                                    block ;; label = @16
                                      local.get 4
                                      local.get 9
                                      i32.or
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.set 9
                                      i32.const 2
                                      local.get 7
                                      i32.shl
                                      local.tee 4
                                      i32.const 0
                                      local.get 4
                                      i32.sub
                                      i32.or
                                      local.get 10
                                      i32.and
                                      local.tee 4
                                      i32.eqz
                                      br_if 3 (;@13;)
                                      local.get 4
                                      i32.ctz
                                      i32.const 2
                                      i32.shl
                                      i32.const 1056192
                                      i32.add
                                      i32.load
                                      local.set 4
                                    end
                                    local.get 4
                                    i32.eqz
                                    br_if 1 (;@14;)
                                  end
                                  loop ;; label = @15
                                    local.get 4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 6
                                    local.get 3
                                    i32.lt_u
                                    local.set 8
                                    block ;; label = @16
                                      local.get 4
                                      i32.load offset=16
                                      local.tee 0
                                      br_if 0 (;@16;)
                                      local.get 4
                                      i32.load offset=20
                                      local.set 0
                                    end
                                    local.get 6
                                    local.get 3
                                    local.get 8
                                    select
                                    local.set 3
                                    local.get 4
                                    local.get 9
                                    local.get 8
                                    select
                                    local.set 9
                                    local.get 0
                                    local.set 4
                                    local.get 0
                                    br_if 0 (;@15;)
                                  end
                                end
                                local.get 9
                                i32.eqz
                                br_if 0 (;@13;)
                                local.get 3
                                i32.const 0
                                i32.load offset=1055896
                                local.get 5
                                i32.sub
                                i32.ge_u
                                br_if 0 (;@13;)
                                local.get 9
                                i32.load offset=24
                                local.set 11
                                block ;; label = @14
                                  local.get 9
                                  i32.load offset=12
                                  local.tee 4
                                  local.get 9
                                  i32.eq
                                  br_if 0 (;@14;)
                                  local.get 9
                                  i32.load offset=8
                                  local.tee 0
                                  local.get 4
                                  i32.store offset=12
                                  local.get 4
                                  local.get 0
                                  i32.store offset=8
                                  br 11 (;@3;)
                                end
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 9
                                    i32.load offset=20
                                    local.tee 0
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    local.get 9
                                    i32.const 20
                                    i32.add
                                    local.set 8
                                    br 1 (;@14;)
                                  end
                                  local.get 9
                                  i32.load offset=16
                                  local.tee 0
                                  i32.eqz
                                  br_if 4 (;@10;)
                                  local.get 9
                                  i32.const 16
                                  i32.add
                                  local.set 8
                                end
                                loop ;; label = @14
                                  local.get 8
                                  local.set 6
                                  local.get 0
                                  local.tee 4
                                  i32.const 20
                                  i32.add
                                  local.set 8
                                  local.get 4
                                  i32.load offset=20
                                  local.tee 0
                                  br_if 0 (;@14;)
                                  local.get 4
                                  i32.const 16
                                  i32.add
                                  local.set 8
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 0
                                  br_if 0 (;@14;)
                                end
                                local.get 6
                                i32.const 0
                                i32.store
                                br 10 (;@3;)
                              end
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1055896
                                local.tee 4
                                local.get 5
                                i32.lt_u
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.load offset=1055908
                                local.set 3
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    local.get 5
                                    i32.sub
                                    local.tee 0
                                    i32.const 16
                                    i32.lt_u
                                    br_if 0 (;@15;)
                                    local.get 3
                                    local.get 5
                                    i32.add
                                    local.tee 8
                                    local.get 0
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    local.get 3
                                    local.get 4
                                    i32.add
                                    local.get 0
                                    i32.store
                                    local.get 3
                                    local.get 5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    br 1 (;@14;)
                                  end
                                  local.get 3
                                  local.get 4
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 4
                                  i32.add
                                  local.tee 4
                                  local.get 4
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  i32.const 0
                                  local.set 8
                                  i32.const 0
                                  local.set 0
                                end
                                i32.const 0
                                local.get 0
                                i32.store offset=1055896
                                i32.const 0
                                local.get 8
                                i32.store offset=1055908
                                local.get 3
                                i32.const 8
                                i32.add
                                local.set 4
                                br 12 (;@1;)
                              end
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1055900
                                local.tee 0
                                local.get 5
                                i32.le_u
                                br_if 0 (;@13;)
                                local.get 2
                                local.get 5
                                i32.add
                                local.tee 4
                                local.get 0
                                local.get 5
                                i32.sub
                                local.tee 3
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                i32.const 0
                                local.get 4
                                i32.store offset=1055912
                                i32.const 0
                                local.get 3
                                i32.store offset=1055900
                                local.get 2
                                local.get 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 2
                                i32.const 8
                                i32.add
                                local.set 4
                                br 12 (;@1;)
                              end
                              block ;; label = @13
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1056360
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  i32.const 0
                                  i32.load offset=1056368
                                  local.set 3
                                  br 1 (;@13;)
                                end
                                i32.const 0
                                i64.const -1
                                i64.store offset=1056372 align=4
                                i32.const 0
                                i64.const 281474976776192
                                i64.store offset=1056364 align=4
                                i32.const 0
                                local.get 1
                                i32.const 12
                                i32.add
                                i32.const -16
                                i32.and
                                i32.const 1431655768
                                i32.xor
                                i32.store offset=1056360
                                i32.const 0
                                i32.const 0
                                i32.store offset=1056380
                                i32.const 0
                                i32.const 0
                                i32.store offset=1056332
                                i32.const 65536
                                local.set 3
                              end
                              i32.const 0
                              local.set 4
                              block ;; label = @13
                                local.get 3
                                local.get 5
                                i32.const 71
                                i32.add
                                local.tee 11
                                i32.add
                                local.tee 8
                                i32.const 0
                                local.get 3
                                i32.sub
                                local.tee 6
                                i32.and
                                local.tee 9
                                local.get 5
                                i32.gt_u
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.const 48
                                i32.store offset=1056384
                                br 12 (;@1;)
                              end
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1056328
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1056320
                                  local.tee 3
                                  local.get 9
                                  i32.add
                                  local.tee 7
                                  local.get 3
                                  i32.le_u
                                  br_if 0 (;@14;)
                                  local.get 7
                                  local.get 4
                                  i32.le_u
                                  br_if 1 (;@13;)
                                end
                                i32.const 0
                                local.set 4
                                i32.const 0
                                i32.const 48
                                i32.store offset=1056384
                                br 12 (;@1;)
                              end
                              i32.const 0
                              i32.load8_u offset=1056332
                              i32.const 4
                              i32.and
                              br_if 5 (;@7;)
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 2
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    i32.const 1056336
                                    local.set 4
                                    loop ;; label = @16
                                      block ;; label = @17
                                        local.get 4
                                        i32.load
                                        local.tee 3
                                        local.get 2
                                        i32.gt_u
                                        br_if 0 (;@17;)
                                        local.get 3
                                        local.get 4
                                        i32.load offset=4
                                        i32.add
                                        local.get 2
                                        i32.gt_u
                                        br_if 3 (;@14;)
                                      end
                                      local.get 4
                                      i32.load offset=8
                                      local.tee 4
                                      br_if 0 (;@16;)
                                    end
                                  end
                                  i32.const 0
                                  i32.const 11904
                                  local.set 12
                                  local.get 12
                                  local.get 12
                                  i32.load 2 offset=28 align=1
                                  i32.const 1
                                  i32.add
                                  i32.store 2 offset=28 align=1
                                  call $sbrk
                                  local.tee 8
                                  i32.const -1
                                  i32.eq
                                  br_if 6 (;@8;)
                                  local.get 9
                                  local.set 6
                                  block ;; label = @15
                                    i32.const 0
                                    i32.load offset=1056364
                                    local.tee 4
                                    i32.const -1
                                    i32.add
                                    local.tee 3
                                    local.get 8
                                    i32.and
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    local.get 9
                                    local.get 8
                                    i32.sub
                                    local.get 3
                                    local.get 8
                                    i32.add
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.and
                                    i32.add
                                    local.set 6
                                  end
                                  local.get 6
                                  local.get 5
                                  i32.le_u
                                  br_if 6 (;@8;)
                                  local.get 6
                                  i32.const 2147483646
                                  i32.gt_u
                                  br_if 6 (;@8;)
                                  block ;; label = @15
                                    i32.const 0
                                    i32.load offset=1056328
                                    local.tee 4
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    i32.load offset=1056320
                                    local.tee 3
                                    local.get 6
                                    i32.add
                                    local.tee 0
                                    local.get 3
                                    i32.le_u
                                    br_if 7 (;@8;)
                                    local.get 0
                                    local.get 4
                                    i32.gt_u
                                    br_if 7 (;@8;)
                                  end
                                  local.get 6
                                  i32.const 11936
                                  local.set 12
                                  local.get 12
                                  local.get 12
                                  i32.load 2 offset=28 align=1
                                  i32.const 1
                                  i32.add
                                  i32.store 2 offset=28 align=1
                                  call $sbrk
                                  local.tee 4
                                  local.get 8
                                  i32.ne
                                  br_if 1 (;@13;)
                                  br 8 (;@6;)
                                end
                                local.get 8
                                local.get 0
                                i32.sub
                                local.get 6
                                i32.and
                                local.tee 6
                                i32.const 2147483646
                                i32.gt_u
                                br_if 5 (;@8;)
                                local.get 6
                                i32.const 11968
                                local.set 12
                                local.get 12
                                local.get 12
                                i32.load 2 offset=28 align=1
                                i32.const 1
                                i32.add
                                i32.store 2 offset=28 align=1
                                call $sbrk
                                local.tee 8
                                local.get 4
                                i32.load
                                local.get 4
                                i32.load offset=4
                                i32.add
                                i32.eq
                                br_if 4 (;@9;)
                                local.get 8
                                local.set 4
                              end
                              block ;; label = @13
                                local.get 6
                                local.get 5
                                i32.const 72
                                i32.add
                                i32.ge_u
                                br_if 0 (;@13;)
                                local.get 4
                                i32.const -1
                                i32.eq
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  local.get 11
                                  local.get 6
                                  i32.sub
                                  i32.const 0
                                  i32.load offset=1056368
                                  local.tee 3
                                  i32.add
                                  i32.const 0
                                  local.get 3
                                  i32.sub
                                  i32.and
                                  local.tee 3
                                  i32.const 2147483646
                                  i32.le_u
                                  br_if 0 (;@14;)
                                  local.get 4
                                  local.set 8
                                  br 8 (;@6;)
                                end
                                block ;; label = @14
                                  local.get 3
                                  i32.const 12000
                                  local.set 12
                                  local.get 12
                                  local.get 12
                                  i32.load 2 offset=28 align=1
                                  i32.const 1
                                  i32.add
                                  i32.store 2 offset=28 align=1
                                  call $sbrk
                                  i32.const -1
                                  i32.eq
                                  br_if 0 (;@14;)
                                  local.get 3
                                  local.get 6
                                  i32.add
                                  local.set 6
                                  local.get 4
                                  local.set 8
                                  br 8 (;@6;)
                                end
                                i32.const 0
                                local.get 6
                                i32.sub
                                i32.const 12032
                                local.set 12
                                local.get 12
                                local.get 12
                                i32.load 2 offset=28 align=1
                                i32.const 1
                                i32.add
                                i32.store 2 offset=28 align=1
                                call $sbrk
                                drop
                                br 5 (;@8;)
                              end
                              local.get 4
                              local.set 8
                              local.get 4
                              i32.const -1
                              i32.ne
                              br_if 6 (;@6;)
                              br 4 (;@8;)
                            end
                            unreachable
                          end
                          i32.const 0
                          local.set 4
                          br 8 (;@2;)
                        end
                        i32.const 0
                        local.set 4
                        br 6 (;@3;)
                      end
                      local.get 8
                      i32.const -1
                      i32.ne
                      br_if 2 (;@6;)
                    end
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056332
                    i32.const 4
                    i32.or
                    i32.store offset=1056332
                  end
                  local.get 9
                  i32.const 2147483646
                  i32.gt_u
                  br_if 1 (;@5;)
                  local.get 9
                  i32.const 12064
                  local.set 12
                  local.get 12
                  local.get 12
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $sbrk
                  local.set 8
                  i32.const 0
                  i32.const 12096
                  local.set 12
                  local.get 12
                  local.get 12
                  i32.load 2 offset=28 align=1
                  i32.const 1
                  i32.add
                  i32.store 2 offset=28 align=1
                  call $sbrk
                  local.set 4
                  local.get 8
                  i32.const -1
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 4
                  i32.const -1
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 8
                  local.get 4
                  i32.ge_u
                  br_if 1 (;@5;)
                  local.get 4
                  local.get 8
                  i32.sub
                  local.tee 6
                  local.get 5
                  i32.const 56
                  i32.add
                  i32.le_u
                  br_if 1 (;@5;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=1056320
                local.get 6
                i32.add
                local.tee 4
                i32.store offset=1056320
                block ;; label = @6
                  local.get 4
                  i32.const 0
                  i32.load offset=1056324
                  i32.le_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 4
                  i32.store offset=1056324
                end
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1055912
                        local.tee 3
                        i32.eqz
                        br_if 0 (;@9;)
                        i32.const 1056336
                        local.set 4
                        loop ;; label = @10
                          local.get 8
                          local.get 4
                          i32.load
                          local.tee 0
                          local.get 4
                          i32.load offset=4
                          local.tee 9
                          i32.add
                          i32.eq
                          br_if 2 (;@8;)
                          local.get 4
                          i32.load offset=8
                          local.tee 4
                          br_if 0 (;@10;)
                          br 3 (;@7;)
                        end
                      end
                      block ;; label = @9
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1055904
                          local.tee 4
                          i32.eqz
                          br_if 0 (;@10;)
                          local.get 8
                          local.get 4
                          i32.ge_u
                          br_if 1 (;@9;)
                        end
                        i32.const 0
                        local.get 8
                        i32.store offset=1055904
                      end
                      i32.const 0
                      local.set 4
                      i32.const 0
                      local.get 6
                      i32.store offset=1056340
                      i32.const 0
                      local.get 8
                      i32.store offset=1056336
                      i32.const 0
                      i32.const -1
                      i32.store offset=1055920
                      i32.const 0
                      i32.const 0
                      i32.load offset=1056360
                      i32.store offset=1055924
                      i32.const 0
                      i32.const 0
                      i32.store offset=1056348
                      loop ;; label = @9
                        local.get 4
                        i32.const 1055948
                        i32.add
                        local.get 4
                        i32.const 1055936
                        i32.add
                        local.tee 3
                        i32.store
                        local.get 3
                        local.get 4
                        i32.const 1055928
                        i32.add
                        local.tee 0
                        i32.store
                        local.get 4
                        i32.const 1055940
                        i32.add
                        local.get 0
                        i32.store
                        local.get 4
                        i32.const 1055956
                        i32.add
                        local.get 4
                        i32.const 1055944
                        i32.add
                        local.tee 0
                        i32.store
                        local.get 0
                        local.get 3
                        i32.store
                        local.get 4
                        i32.const 1055964
                        i32.add
                        local.get 4
                        i32.const 1055952
                        i32.add
                        local.tee 3
                        i32.store
                        local.get 3
                        local.get 0
                        i32.store
                        local.get 4
                        i32.const 1055960
                        i32.add
                        local.get 3
                        i32.store
                        local.get 4
                        i32.const 32
                        i32.add
                        local.tee 4
                        i32.const 256
                        i32.ne
                        br_if 0 (;@9;)
                      end
                      local.get 8
                      i32.const -8
                      local.get 8
                      i32.sub
                      i32.const 15
                      i32.and
                      local.tee 4
                      i32.add
                      local.tee 3
                      local.get 6
                      i32.const -56
                      i32.add
                      local.tee 0
                      local.get 4
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      i32.const 0
                      i32.const 0
                      i32.load offset=1056376
                      i32.store offset=1055916
                      i32.const 0
                      local.get 4
                      i32.store offset=1055900
                      i32.const 0
                      local.get 3
                      i32.store offset=1055912
                      local.get 8
                      local.get 0
                      i32.add
                      i32.const 56
                      i32.store offset=4
                      br 2 (;@6;)
                    end
                    local.get 3
                    local.get 8
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 3
                    local.get 0
                    i32.lt_u
                    br_if 0 (;@7;)
                    local.get 4
                    i32.load offset=12
                    i32.const 8
                    i32.and
                    br_if 0 (;@7;)
                    local.get 3
                    i32.const -8
                    local.get 3
                    i32.sub
                    i32.const 15
                    i32.and
                    local.tee 0
                    i32.add
                    local.tee 8
                    i32.const 0
                    i32.load offset=1055900
                    local.get 6
                    i32.add
                    local.tee 11
                    local.get 0
                    i32.sub
                    local.tee 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 4
                    local.get 9
                    local.get 6
                    i32.add
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056376
                    i32.store offset=1055916
                    i32.const 0
                    local.get 0
                    i32.store offset=1055900
                    i32.const 0
                    local.get 8
                    i32.store offset=1055912
                    local.get 3
                    local.get 11
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 1 (;@6;)
                  end
                  block ;; label = @7
                    local.get 8
                    i32.const 0
                    i32.load offset=1055904
                    i32.ge_u
                    br_if 0 (;@7;)
                    i32.const 0
                    local.get 8
                    i32.store offset=1055904
                  end
                  local.get 8
                  local.get 6
                  i32.add
                  local.set 0
                  i32.const 1056336
                  local.set 4
                  block ;; label = @7
                    block ;; label = @8
                      loop ;; label = @9
                        local.get 4
                        i32.load
                        local.tee 9
                        local.get 0
                        i32.eq
                        br_if 1 (;@8;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@9;)
                        br 2 (;@7;)
                      end
                    end
                    local.get 4
                    i32.load8_u offset=12
                    i32.const 8
                    i32.and
                    i32.eqz
                    br_if 3 (;@4;)
                  end
                  i32.const 1056336
                  local.set 4
                  block ;; label = @7
                    loop ;; label = @8
                      block ;; label = @9
                        local.get 4
                        i32.load
                        local.tee 0
                        local.get 3
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        local.get 4
                        i32.load offset=4
                        i32.add
                        local.tee 0
                        local.get 3
                        i32.gt_u
                        br_if 2 (;@7;)
                      end
                      local.get 4
                      i32.load offset=8
                      local.set 4
                      br 0 (;@8;)
                    end
                  end
                  local.get 8
                  i32.const -8
                  local.get 8
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee 4
                  i32.add
                  local.tee 11
                  local.get 6
                  i32.const -56
                  i32.add
                  local.tee 9
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 8
                  local.get 9
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  local.get 3
                  local.get 0
                  i32.const 55
                  local.get 0
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.add
                  i32.const -63
                  i32.add
                  local.tee 9
                  local.get 9
                  local.get 3
                  i32.const 16
                  i32.add
                  i32.lt_u
                  select
                  local.tee 9
                  i32.const 35
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056376
                  i32.store offset=1055916
                  i32.const 0
                  local.get 4
                  i32.store offset=1055900
                  i32.const 0
                  local.get 11
                  i32.store offset=1055912
                  local.get 9
                  i32.const 16
                  i32.add
                  i32.const 0
                  i64.load offset=1056344 align=4
                  i64.store align=4
                  local.get 9
                  i32.const 0
                  i64.load offset=1056336 align=4
                  i64.store offset=8 align=4
                  i32.const 0
                  local.get 9
                  i32.const 8
                  i32.add
                  i32.store offset=1056344
                  i32.const 0
                  local.get 6
                  i32.store offset=1056340
                  i32.const 0
                  local.get 8
                  i32.store offset=1056336
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056348
                  local.get 9
                  i32.const 36
                  i32.add
                  local.set 4
                  loop ;; label = @7
                    local.get 4
                    i32.const 7
                    i32.store
                    local.get 4
                    i32.const 4
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.lt_u
                    br_if 0 (;@7;)
                  end
                  local.get 9
                  local.get 3
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 9
                  local.get 9
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 9
                  local.get 9
                  local.get 3
                  i32.sub
                  local.tee 8
                  i32.store
                  local.get 3
                  local.get 8
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  block ;; label = @7
                    block ;; label = @8
                      local.get 8
                      i32.const 255
                      i32.gt_u
                      br_if 0 (;@8;)
                      local.get 8
                      i32.const -8
                      i32.and
                      i32.const 1055928
                      i32.add
                      local.set 4
                      block ;; label = @9
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1055888
                          local.tee 0
                          i32.const 1
                          local.get 8
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 8
                          i32.and
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 0
                          local.get 8
                          i32.or
                          i32.store offset=1055888
                          local.get 4
                          local.set 0
                          br 1 (;@9;)
                        end
                        local.get 4
                        i32.load offset=8
                        local.set 0
                      end
                      local.get 0
                      local.get 3
                      i32.store offset=12
                      local.get 4
                      local.get 3
                      i32.store offset=8
                      i32.const 12
                      local.set 8
                      i32.const 8
                      local.set 9
                      br 1 (;@7;)
                    end
                    i32.const 31
                    local.set 4
                    block ;; label = @8
                      local.get 8
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@8;)
                      local.get 8
                      i32.const 38
                      local.get 8
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 4
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 4
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set 4
                    end
                    local.get 3
                    local.get 4
                    i32.store offset=28
                    local.get 3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 4
                    i32.const 2
                    i32.shl
                    i32.const 1056192
                    i32.add
                    local.set 0
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1055892
                          local.tee 9
                          i32.const 1
                          local.get 4
                          i32.shl
                          local.tee 6
                          i32.and
                          br_if 0 (;@10;)
                          local.get 0
                          local.get 3
                          i32.store
                          i32.const 0
                          local.get 9
                          local.get 6
                          i32.or
                          i32.store offset=1055892
                          local.get 3
                          local.get 0
                          i32.store offset=24
                          br 1 (;@9;)
                        end
                        local.get 8
                        i32.const 0
                        i32.const 25
                        local.get 4
                        i32.const 1
                        i32.shr_u
                        i32.sub
                        local.get 4
                        i32.const 31
                        i32.eq
                        select
                        i32.shl
                        local.set 4
                        local.get 0
                        i32.load
                        local.set 9
                        loop ;; label = @10
                          local.get 9
                          local.tee 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 8
                          i32.eq
                          br_if 2 (;@8;)
                          local.get 4
                          i32.const 29
                          i32.shr_u
                          local.set 9
                          local.get 4
                          i32.const 1
                          i32.shl
                          local.set 4
                          local.get 0
                          local.get 9
                          i32.const 4
                          i32.and
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 6
                          i32.load
                          local.tee 9
                          br_if 0 (;@10;)
                        end
                        local.get 6
                        local.get 3
                        i32.store
                        local.get 3
                        local.get 0
                        i32.store offset=24
                      end
                      i32.const 8
                      local.set 8
                      i32.const 12
                      local.set 9
                      local.get 3
                      local.set 0
                      local.get 3
                      local.set 4
                      br 1 (;@7;)
                    end
                    local.get 0
                    i32.load offset=8
                    local.set 4
                    local.get 0
                    local.get 3
                    i32.store offset=8
                    local.get 4
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 4
                    i32.store offset=8
                    i32.const 0
                    local.set 4
                    i32.const 24
                    local.set 8
                    i32.const 12
                    local.set 9
                  end
                  local.get 3
                  local.get 9
                  i32.add
                  local.get 0
                  i32.store
                  local.get 3
                  local.get 8
                  i32.add
                  local.get 4
                  i32.store
                end
                i32.const 0
                i32.load offset=1055900
                local.tee 4
                local.get 5
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                i32.load offset=1055912
                local.tee 3
                local.get 5
                i32.add
                local.tee 0
                local.get 4
                local.get 5
                i32.sub
                local.tee 4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.get 4
                i32.store offset=1055900
                i32.const 0
                local.get 0
                i32.store offset=1055912
                local.get 3
                local.get 5
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 3
                i32.const 8
                i32.add
                local.set 4
                br 4 (;@1;)
              end
              i32.const 0
              local.set 4
              i32.const 0
              i32.const 48
              i32.store offset=1056384
              br 3 (;@1;)
            end
            local.get 4
            local.get 8
            i32.store
            local.get 4
            local.get 4
            i32.load offset=4
            local.get 6
            i32.add
            i32.store offset=4
            local.get 8
            local.get 9
            local.get 5
            i32.const 12128
            local.set 12
            local.get 12
            local.get 12
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $prepend_alloc
            local.set 4
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 11
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 9
                local.get 9
                i32.load offset=28
                local.tee 8
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee 0
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 0
                local.get 4
                i32.store
                local.get 4
                br_if 1 (;@4;)
                i32.const 0
                local.get 10
                i32.const -2
                local.get 8
                i32.rotl
                i32.and
                local.tee 10
                i32.store offset=1055892
                br 2 (;@3;)
              end
              local.get 11
              i32.const 16
              i32.const 20
              local.get 11
              i32.load offset=16
              local.get 9
              i32.eq
              select
              i32.add
              local.get 4
              i32.store
              local.get 4
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 4
            local.get 11
            i32.store offset=24
            block ;; label = @4
              local.get 9
              i32.load offset=16
              local.tee 0
              i32.eqz
              br_if 0 (;@4;)
              local.get 4
              local.get 0
              i32.store offset=16
              local.get 0
              local.get 4
              i32.store offset=24
            end
            local.get 9
            i32.load offset=20
            local.tee 0
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 0
            i32.store offset=20
            local.get 0
            local.get 4
            i32.store offset=24
          end
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
              local.get 9
              local.get 3
              local.get 5
              i32.or
              local.tee 4
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 9
              local.get 4
              i32.add
              local.tee 4
              local.get 4
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@3;)
            end
            local.get 9
            local.get 5
            i32.add
            local.tee 8
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 3
            i32.add
            local.get 3
            i32.store
            block ;; label = @4
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const -8
              i32.and
              i32.const 1055928
              i32.add
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  i32.const 0
                  i32.load offset=1055888
                  local.tee 5
                  i32.const 1
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 3
                  i32.and
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 5
                  local.get 3
                  i32.or
                  i32.store offset=1055888
                  local.get 4
                  local.set 3
                  br 1 (;@5;)
                end
                local.get 4
                i32.load offset=8
                local.set 3
              end
              local.get 3
              local.get 8
              i32.store offset=12
              local.get 4
              local.get 8
              i32.store offset=8
              local.get 8
              local.get 4
              i32.store offset=12
              local.get 8
              local.get 3
              i32.store offset=8
              br 1 (;@3;)
            end
            i32.const 31
            local.set 4
            block ;; label = @4
              local.get 3
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const 38
              local.get 3
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee 4
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get 4
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 4
            end
            local.get 8
            local.get 4
            i32.store offset=28
            local.get 8
            i64.const 0
            i64.store offset=16 align=4
            local.get 4
            i32.const 2
            i32.shl
            i32.const 1056192
            i32.add
            local.set 5
            block ;; label = @4
              local.get 10
              i32.const 1
              local.get 4
              i32.shl
              local.tee 0
              i32.and
              br_if 0 (;@4;)
              local.get 5
              local.get 8
              i32.store
              i32.const 0
              local.get 10
              local.get 0
              i32.or
              i32.store offset=1055892
              local.get 8
              local.get 5
              i32.store offset=24
              local.get 8
              local.get 8
              i32.store offset=8
              local.get 8
              local.get 8
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.const 25
            local.get 4
            i32.const 1
            i32.shr_u
            i32.sub
            local.get 4
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set 4
            local.get 5
            i32.load
            local.set 0
            block ;; label = @4
              loop ;; label = @5
                local.get 0
                local.tee 5
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 3
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const 29
                i32.shr_u
                local.set 0
                local.get 4
                i32.const 1
                i32.shl
                local.set 4
                local.get 5
                local.get 0
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 6
                i32.load
                local.tee 0
                br_if 0 (;@5;)
              end
              local.get 6
              local.get 8
              i32.store
              local.get 8
              local.get 5
              i32.store offset=24
              local.get 8
              local.get 8
              i32.store offset=12
              local.get 8
              local.get 8
              i32.store offset=8
              br 1 (;@3;)
            end
            local.get 5
            i32.load offset=8
            local.tee 4
            local.get 8
            i32.store offset=12
            local.get 5
            local.get 8
            i32.store offset=8
            local.get 8
            i32.const 0
            i32.store offset=24
            local.get 8
            local.get 5
            i32.store offset=12
            local.get 8
            local.get 4
            i32.store offset=8
          end
          local.get 9
          i32.const 8
          i32.add
          local.set 4
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 2
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 8
              local.get 8
              i32.load offset=28
              local.tee 9
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee 0
              i32.load
              i32.ne
              br_if 0 (;@4;)
              local.get 0
              local.get 4
              i32.store
              local.get 4
              br_if 1 (;@3;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 9
              i32.rotl
              i32.and
              i32.store offset=1055892
              br 2 (;@2;)
            end
            local.get 2
            i32.const 16
            i32.const 20
            local.get 2
            i32.load offset=16
            local.get 8
            i32.eq
            select
            i32.add
            local.get 4
            i32.store
            local.get 4
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 4
          local.get 2
          i32.store offset=24
          block ;; label = @3
            local.get 8
            i32.load offset=16
            local.tee 0
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 0
            i32.store offset=16
            local.get 0
            local.get 4
            i32.store offset=24
          end
          local.get 8
          i32.load offset=20
          local.tee 0
          i32.eqz
          br_if 0 (;@2;)
          local.get 4
          local.get 0
          i32.store offset=20
          local.get 0
          local.get 4
          i32.store offset=24
        end
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 8
            local.get 3
            local.get 5
            i32.or
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@2;)
          end
          local.get 8
          local.get 5
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 8
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @3
            local.get 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 7
            i32.const -8
            i32.and
            i32.const 1055928
            i32.add
            local.set 5
            i32.const 0
            i32.load offset=1055908
            local.set 4
            block ;; label = @4
              block ;; label = @5
                i32.const 1
                local.get 7
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 9
                local.get 6
                i32.and
                br_if 0 (;@5;)
                i32.const 0
                local.get 9
                local.get 6
                i32.or
                i32.store offset=1055888
                local.get 5
                local.set 9
                br 1 (;@4;)
              end
              local.get 5
              i32.load offset=8
              local.set 9
            end
            local.get 9
            local.get 4
            i32.store offset=12
            local.get 5
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 5
            i32.store offset=12
            local.get 4
            local.get 9
            i32.store offset=8
          end
          i32.const 0
          local.get 0
          i32.store offset=1055908
          i32.const 0
          local.get 3
          i32.store offset=1055896
        end
        local.get 8
        i32.const 8
        i32.add
        local.set 4
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $prepend_alloc (;134;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      local.get 0
      i32.const -8
      local.get 0
      i32.sub
      i32.const 15
      i32.and
      i32.add
      local.tee 3
      local.get 2
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 1
      i32.const -8
      local.get 1
      i32.sub
      i32.const 15
      i32.and
      i32.add
      local.tee 4
      local.get 3
      local.get 2
      i32.add
      local.tee 5
      i32.sub
      local.set 0
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.const 0
          i32.load offset=1055912
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          local.get 5
          i32.store offset=1055912
          i32.const 0
          i32.const 0
          i32.load offset=1055900
          local.get 0
          i32.add
          local.tee 2
          i32.store offset=1055900
          local.get 5
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 4
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          local.get 5
          i32.store offset=1055908
          i32.const 0
          i32.const 0
          i32.load offset=1055896
          local.get 0
          i32.add
          local.tee 2
          i32.store offset=1055896
          local.get 5
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 2
          i32.add
          local.get 2
          i32.store
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 4
          i32.load offset=4
          local.tee 1
          i32.const 3
          i32.and
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 1
          i32.const -8
          i32.and
          local.set 6
          local.get 4
          i32.load offset=12
          local.set 2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 2
                local.get 4
                i32.load offset=8
                local.tee 7
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1055888
                i32.const -2
                local.get 1
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=1055888
                br 2 (;@3;)
              end
              local.get 2
              local.get 7
              i32.store offset=8
              local.get 7
              local.get 2
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 4
            i32.load offset=24
            local.set 8
            block ;; label = @4
              block ;; label = @5
                local.get 2
                local.get 4
                i32.eq
                br_if 0 (;@5;)
                local.get 4
                i32.load offset=8
                local.tee 1
                local.get 2
                i32.store offset=12
                local.get 2
                local.get 1
                i32.store offset=8
                br 1 (;@4;)
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 4
                    i32.load offset=20
                    local.tee 1
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 4
                    i32.const 20
                    i32.add
                    local.set 7
                    br 1 (;@6;)
                  end
                  local.get 4
                  i32.load offset=16
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@5;)
                  local.get 4
                  i32.const 16
                  i32.add
                  local.set 7
                end
                loop ;; label = @6
                  local.get 7
                  local.set 9
                  local.get 1
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 7
                  local.get 2
                  i32.load offset=20
                  local.tee 1
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 7
                  local.get 2
                  i32.load offset=16
                  local.tee 1
                  br_if 0 (;@6;)
                end
                local.get 9
                i32.const 0
                i32.store
                br 1 (;@4;)
              end
              i32.const 0
              local.set 2
            end
            local.get 8
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 4
                local.get 4
                i32.load offset=28
                local.tee 7
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee 1
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 1
                local.get 2
                i32.store
                local.get 2
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get 7
                i32.rotl
                i32.and
                i32.store offset=1055892
                br 2 (;@3;)
              end
              local.get 8
              i32.const 16
              i32.const 20
              local.get 8
              i32.load offset=16
              local.get 4
              i32.eq
              select
              i32.add
              local.get 2
              i32.store
              local.get 2
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 2
            local.get 8
            i32.store offset=24
            block ;; label = @4
              local.get 4
              i32.load offset=16
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 1
              i32.store offset=16
              local.get 1
              local.get 2
              i32.store offset=24
            end
            local.get 4
            i32.load offset=20
            local.tee 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 1
            i32.store offset=20
            local.get 1
            local.get 2
            i32.store offset=24
          end
          local.get 6
          local.get 0
          i32.add
          local.set 0
          local.get 4
          local.get 6
          i32.add
          local.tee 4
          i32.load offset=4
          local.set 1
        end
        local.get 4
        local.get 1
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 5
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 5
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1055888
              local.tee 1
              i32.const 1
              local.get 0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 0
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              local.get 0
              i32.or
              i32.store offset=1055888
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            i32.load offset=8
            local.set 0
          end
          local.get 0
          local.get 5
          i32.store offset=12
          local.get 2
          local.get 5
          i32.store offset=8
          local.get 5
          local.get 2
          i32.store offset=12
          local.get 5
          local.get 0
          i32.store offset=8
          br 1 (;@1;)
        end
        i32.const 31
        local.set 2
        block ;; label = @2
          local.get 0
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 38
          local.get 0
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee 2
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get 2
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set 2
        end
        local.get 5
        local.get 2
        i32.store offset=28
        local.get 5
        i64.const 0
        i64.store offset=16 align=4
        local.get 2
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set 1
        block ;; label = @2
          i32.const 0
          i32.load offset=1055892
          local.tee 7
          i32.const 1
          local.get 2
          i32.shl
          local.tee 4
          i32.and
          br_if 0 (;@2;)
          local.get 1
          local.get 5
          i32.store
          i32.const 0
          local.get 7
          local.get 4
          i32.or
          i32.store offset=1055892
          local.get 5
          local.get 1
          i32.store offset=24
          local.get 5
          local.get 5
          i32.store offset=8
          local.get 5
          local.get 5
          i32.store offset=12
          br 1 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 2
        local.get 1
        i32.load
        local.set 7
        block ;; label = @2
          loop ;; label = @3
            local.get 7
            local.tee 1
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 0
            i32.eq
            br_if 1 (;@2;)
            local.get 2
            i32.const 29
            i32.shr_u
            local.set 7
            local.get 2
            i32.const 1
            i32.shl
            local.set 2
            local.get 1
            local.get 7
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 4
            i32.load
            local.tee 7
            br_if 0 (;@3;)
          end
          local.get 4
          local.get 5
          i32.store
          local.get 5
          local.get 1
          i32.store offset=24
          local.get 5
          local.get 5
          i32.store offset=12
          local.get 5
          local.get 5
          i32.store offset=8
          br 1 (;@1;)
        end
        local.get 1
        i32.load offset=8
        local.tee 2
        local.get 5
        i32.store offset=12
        local.get 1
        local.get 5
        i32.store offset=8
        local.get 5
        i32.const 0
        i32.store offset=24
        local.get 5
        local.get 1
        i32.store offset=12
        local.get 5
        local.get 2
        i32.store offset=8
      end
      local.get 3
      i32.const 8
      i32.add
    )
    (func $free (;135;) (type 1) (param i32)
      (local i32)
      local.get 0
      i32.const 12160
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $dlfree
    )
    (func $dlfree (;136;) (type 1) (param i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const -8
        i32.add
        local.tee 1
        local.get 0
        i32.const -4
        i32.add
        i32.load
        local.tee 2
        i32.const -8
        i32.and
        local.tee 0
        i32.add
        local.set 3
        block ;; label = @2
          local.get 2
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 2
          i32.const 2
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 1
          i32.load
          local.tee 4
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1055904
          i32.lt_u
          br_if 1 (;@1;)
          local.get 4
          local.get 0
          i32.add
          local.set 0
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 1
                  i32.const 0
                  i32.load offset=1055908
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 1
                  i32.load offset=12
                  local.set 2
                  block ;; label = @7
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 2
                    local.get 1
                    i32.load offset=8
                    local.tee 5
                    i32.ne
                    br_if 2 (;@5;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055888
                    i32.const -2
                    local.get 4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1055888
                    br 5 (;@2;)
                  end
                  local.get 1
                  i32.load offset=24
                  local.set 6
                  block ;; label = @7
                    local.get 2
                    local.get 1
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 1
                    i32.load offset=8
                    local.tee 4
                    local.get 2
                    i32.store offset=12
                    local.get 2
                    local.get 4
                    i32.store offset=8
                    br 4 (;@3;)
                  end
                  block ;; label = @7
                    block ;; label = @8
                      local.get 1
                      i32.load offset=20
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 1
                      i32.const 20
                      i32.add
                      local.set 5
                      br 1 (;@7;)
                    end
                    local.get 1
                    i32.load offset=16
                    local.tee 4
                    i32.eqz
                    br_if 3 (;@4;)
                    local.get 1
                    i32.const 16
                    i32.add
                    local.set 5
                  end
                  loop ;; label = @7
                    local.get 5
                    local.set 7
                    local.get 4
                    local.tee 2
                    i32.const 20
                    i32.add
                    local.set 5
                    local.get 2
                    i32.load offset=20
                    local.tee 4
                    br_if 0 (;@7;)
                    local.get 2
                    i32.const 16
                    i32.add
                    local.set 5
                    local.get 2
                    i32.load offset=16
                    local.tee 4
                    br_if 0 (;@7;)
                  end
                  local.get 7
                  i32.const 0
                  i32.store
                  br 3 (;@3;)
                end
                local.get 3
                i32.load offset=4
                local.tee 2
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if 3 (;@2;)
                local.get 3
                local.get 2
                i32.const -2
                i32.and
                i32.store offset=4
                i32.const 0
                local.get 0
                i32.store offset=1055896
                local.get 3
                local.get 0
                i32.store
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                return
              end
              local.get 2
              local.get 5
              i32.store offset=8
              local.get 5
              local.get 2
              i32.store offset=12
              br 2 (;@2;)
            end
            i32.const 0
            local.set 2
          end
          local.get 6
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 1
              local.get 1
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@3;)
              i32.const 0
              i32.const 0
              i32.load offset=1055892
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1055892
              br 2 (;@2;)
            end
            local.get 6
            i32.const 16
            i32.const 20
            local.get 6
            i32.load offset=16
            local.get 1
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 2
          local.get 6
          i32.store offset=24
          block ;; label = @3
            local.get 1
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 2
            i32.store offset=24
          end
          local.get 1
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 4
          i32.store offset=20
          local.get 4
          local.get 2
          i32.store offset=24
        end
        local.get 1
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 3
        i32.load offset=4
        local.tee 4
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i32.const 2
                  i32.and
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 3
                    i32.const 0
                    i32.load offset=1055912
                    i32.ne
                    br_if 0 (;@7;)
                    i32.const 0
                    local.get 1
                    i32.store offset=1055912
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055900
                    local.get 0
                    i32.add
                    local.tee 0
                    i32.store offset=1055900
                    local.get 1
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if 6 (;@1;)
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055896
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055908
                    return
                  end
                  block ;; label = @7
                    local.get 3
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if 0 (;@7;)
                    i32.const 0
                    local.get 1
                    i32.store offset=1055908
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055896
                    local.get 0
                    i32.add
                    local.tee 0
                    i32.store offset=1055896
                    local.get 1
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.add
                    local.get 0
                    i32.store
                    return
                  end
                  local.get 4
                  i32.const -8
                  i32.and
                  local.get 0
                  i32.add
                  local.set 0
                  local.get 3
                  i32.load offset=12
                  local.set 2
                  block ;; label = @7
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 2
                      local.get 3
                      i32.load offset=8
                      local.tee 5
                      i32.ne
                      br_if 0 (;@8;)
                      i32.const 0
                      i32.const 0
                      i32.load offset=1055888
                      i32.const -2
                      local.get 4
                      i32.const 3
                      i32.shr_u
                      i32.rotl
                      i32.and
                      i32.store offset=1055888
                      br 5 (;@3;)
                    end
                    local.get 2
                    local.get 5
                    i32.store offset=8
                    local.get 5
                    local.get 2
                    i32.store offset=12
                    br 4 (;@3;)
                  end
                  local.get 3
                  i32.load offset=24
                  local.set 6
                  block ;; label = @7
                    local.get 2
                    local.get 3
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 3
                    i32.load offset=8
                    local.tee 4
                    local.get 2
                    i32.store offset=12
                    local.get 2
                    local.get 4
                    i32.store offset=8
                    br 3 (;@4;)
                  end
                  block ;; label = @7
                    block ;; label = @8
                      local.get 3
                      i32.load offset=20
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 3
                      i32.const 20
                      i32.add
                      local.set 5
                      br 1 (;@7;)
                    end
                    local.get 3
                    i32.load offset=16
                    local.tee 4
                    i32.eqz
                    br_if 2 (;@5;)
                    local.get 3
                    i32.const 16
                    i32.add
                    local.set 5
                  end
                  loop ;; label = @7
                    local.get 5
                    local.set 7
                    local.get 4
                    local.tee 2
                    i32.const 20
                    i32.add
                    local.set 5
                    local.get 2
                    i32.load offset=20
                    local.tee 4
                    br_if 0 (;@7;)
                    local.get 2
                    i32.const 16
                    i32.add
                    local.set 5
                    local.get 2
                    i32.load offset=16
                    local.tee 4
                    br_if 0 (;@7;)
                  end
                  local.get 7
                  i32.const 0
                  i32.store
                  br 2 (;@4;)
                end
                local.get 3
                local.get 4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 1
                local.get 0
                i32.add
                local.get 0
                i32.store
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                br 3 (;@2;)
              end
              i32.const 0
              local.set 2
            end
            local.get 6
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 3
                local.get 3
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee 4
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 4
                local.get 2
                i32.store
                local.get 2
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1055892
                br 2 (;@3;)
              end
              local.get 6
              i32.const 16
              i32.const 20
              local.get 6
              i32.load offset=16
              local.get 3
              i32.eq
              select
              i32.add
              local.get 2
              i32.store
              local.get 2
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 2
            local.get 6
            i32.store offset=24
            block ;; label = @4
              local.get 3
              i32.load offset=16
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 4
              i32.store offset=16
              local.get 4
              local.get 2
              i32.store offset=24
            end
            local.get 3
            i32.load offset=20
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 4
            i32.store offset=20
            local.get 4
            local.get 2
            i32.store offset=24
          end
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1055896
          return
        end
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1055888
              local.tee 4
              i32.const 1
              local.get 0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 0
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 0
              i32.or
              i32.store offset=1055888
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            i32.load offset=8
            local.set 0
          end
          local.get 0
          local.get 1
          i32.store offset=12
          local.get 2
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 2
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 2
        block ;; label = @2
          local.get 0
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 38
          local.get 0
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee 2
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get 2
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set 2
        end
        local.get 1
        local.get 2
        i32.store offset=28
        local.get 1
        i64.const 0
        i64.store offset=16 align=4
        local.get 2
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set 3
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                i32.const 0
                i32.load offset=1055892
                local.tee 4
                i32.const 1
                local.get 2
                i32.shl
                local.tee 5
                i32.and
                br_if 0 (;@5;)
                i32.const 0
                local.get 4
                local.get 5
                i32.or
                i32.store offset=1055892
                i32.const 8
                local.set 0
                i32.const 24
                local.set 2
                local.get 3
                local.set 5
                br 1 (;@4;)
              end
              local.get 0
              i32.const 0
              i32.const 25
              local.get 2
              i32.const 1
              i32.shr_u
              i32.sub
              local.get 2
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set 2
              local.get 3
              i32.load
              local.set 5
              loop ;; label = @5
                local.get 5
                local.tee 4
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 0
                i32.eq
                br_if 2 (;@3;)
                local.get 2
                i32.const 29
                i32.shr_u
                local.set 5
                local.get 2
                i32.const 1
                i32.shl
                local.set 2
                local.get 4
                local.get 5
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 3
                i32.load
                local.tee 5
                br_if 0 (;@5;)
              end
              i32.const 8
              local.set 0
              i32.const 24
              local.set 2
              local.get 4
              local.set 5
            end
            local.get 1
            local.set 4
            local.get 1
            local.set 7
            br 1 (;@2;)
          end
          local.get 4
          i32.load offset=8
          local.tee 5
          local.get 1
          i32.store offset=12
          i32.const 8
          local.set 2
          local.get 4
          i32.const 8
          i32.add
          local.set 3
          i32.const 0
          local.set 7
          i32.const 24
          local.set 0
        end
        local.get 3
        local.get 1
        i32.store
        local.get 1
        local.get 2
        i32.add
        local.get 5
        i32.store
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 1
        local.get 0
        i32.add
        local.get 7
        i32.store
        i32.const 0
        i32.const 0
        i32.load offset=1055920
        i32.const -1
        i32.add
        local.tee 1
        i32.const -1
        local.get 1
        select
        i32.store offset=1055920
      end
    )
    (func $calloc (;137;) (type 2) (param i32 i32) (result i32)
      (local i32 i64 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          i32.const 0
          local.set 2
          br 1 (;@1;)
        end
        local.get 0
        i64.extend_i32_u
        local.get 1
        i64.extend_i32_u
        i64.mul
        local.tee 3
        i32.wrap_i64
        local.set 2
        local.get 1
        local.get 0
        i32.or
        i32.const 65536
        i32.lt_u
        br_if 0 (;@1;)
        i32.const -1
        local.get 2
        local.get 3
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.const 0
        i32.ne
        select
        local.set 2
      end
      block ;; label = @1
        local.get 2
        i32.const 12192
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dlmalloc
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const -4
        i32.add
        i32.load8_u
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 0
        local.get 2
        i32.const 12224
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $memset
        drop
      end
      local.get 0
    )
    (func $realloc (;138;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 1
        i32.const 12256
        local.set 13
        local.get 13
        local.get 13
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dlmalloc
        return
      end
      block ;; label = @1
        local.get 1
        i32.const -64
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1056384
        i32.const 0
        return
      end
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.set 2
      local.get 0
      i32.const -4
      i32.add
      local.tee 3
      i32.load
      local.tee 4
      i32.const -8
      i32.and
      local.set 5
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 4
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.const 4
            i32.or
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.sub
            i32.const 0
            i32.load offset=1056368
            i32.const 1
            i32.shl
            i32.le_u
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          local.get 0
          i32.const -8
          i32.add
          local.tee 6
          local.get 5
          i32.add
          local.set 7
          block ;; label = @3
            local.get 5
            local.get 2
            i32.lt_u
            br_if 0 (;@3;)
            local.get 5
            local.get 2
            i32.sub
            local.tee 1
            i32.const 16
            i32.lt_u
            br_if 2 (;@1;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 2
            i32.add
            local.tee 2
            local.get 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 7
            local.get 7
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            i32.const 12288
            local.set 13
            local.get 13
            local.get 13
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $dispose_chunk
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1055912
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1055900
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.le_u
            br_if 1 (;@2;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            i32.const 0
            local.get 6
            local.get 2
            i32.add
            local.tee 1
            i32.store offset=1055912
            i32.const 0
            local.get 5
            local.get 2
            i32.sub
            local.tee 2
            i32.store offset=1055900
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1055908
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1055896
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 1 (;@2;)
            block ;; label = @4
              block ;; label = @5
                local.get 5
                local.get 2
                i32.sub
                local.tee 1
                i32.const 16
                i32.lt_u
                br_if 0 (;@5;)
                local.get 3
                local.get 2
                local.get 4
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 6
                local.get 2
                i32.add
                local.tee 2
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 6
                local.get 5
                i32.add
                local.tee 5
                local.get 1
                i32.store
                local.get 5
                local.get 5
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                br 1 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const 1
              i32.and
              local.get 5
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 5
              i32.add
              local.tee 1
              local.get 1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.set 1
              i32.const 0
              local.set 2
            end
            i32.const 0
            local.get 2
            i32.store offset=1055908
            i32.const 0
            local.get 1
            i32.store offset=1055896
            local.get 0
            return
          end
          local.get 7
          i32.load offset=4
          local.tee 8
          i32.const 2
          i32.and
          br_if 0 (;@2;)
          local.get 8
          i32.const -8
          i32.and
          local.get 5
          i32.add
          local.tee 9
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
          local.get 9
          local.get 2
          i32.sub
          local.set 10
          local.get 7
          i32.load offset=12
          local.set 1
          block ;; label = @3
            block ;; label = @4
              local.get 8
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 1
                local.get 7
                i32.load offset=8
                local.tee 5
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1055888
                i32.const -2
                local.get 8
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=1055888
                br 2 (;@3;)
              end
              local.get 1
              local.get 5
              i32.store offset=8
              local.get 5
              local.get 1
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 7
            i32.load offset=24
            local.set 11
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 7
                i32.eq
                br_if 0 (;@5;)
                local.get 7
                i32.load offset=8
                local.tee 5
                local.get 1
                i32.store offset=12
                local.get 1
                local.get 5
                i32.store offset=8
                br 1 (;@4;)
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 7
                    i32.load offset=20
                    local.tee 5
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 7
                    i32.const 20
                    i32.add
                    local.set 8
                    br 1 (;@6;)
                  end
                  local.get 7
                  i32.load offset=16
                  local.tee 5
                  i32.eqz
                  br_if 1 (;@5;)
                  local.get 7
                  i32.const 16
                  i32.add
                  local.set 8
                end
                loop ;; label = @6
                  local.get 8
                  local.set 12
                  local.get 5
                  local.tee 1
                  i32.const 20
                  i32.add
                  local.set 8
                  local.get 1
                  i32.load offset=20
                  local.tee 5
                  br_if 0 (;@6;)
                  local.get 1
                  i32.const 16
                  i32.add
                  local.set 8
                  local.get 1
                  i32.load offset=16
                  local.tee 5
                  br_if 0 (;@6;)
                end
                local.get 12
                i32.const 0
                i32.store
                br 1 (;@4;)
              end
              i32.const 0
              local.set 1
            end
            local.get 11
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 7
                local.get 7
                i32.load offset=28
                local.tee 8
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee 5
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 5
                local.get 1
                i32.store
                local.get 1
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get 8
                i32.rotl
                i32.and
                i32.store offset=1055892
                br 2 (;@3;)
              end
              local.get 11
              i32.const 16
              i32.const 20
              local.get 11
              i32.load offset=16
              local.get 7
              i32.eq
              select
              i32.add
              local.get 1
              i32.store
              local.get 1
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 1
            local.get 11
            i32.store offset=24
            block ;; label = @4
              local.get 7
              i32.load offset=16
              local.tee 5
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              local.get 5
              i32.store offset=16
              local.get 5
              local.get 1
              i32.store offset=24
            end
            local.get 7
            i32.load offset=20
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            local.get 5
            i32.store offset=20
            local.get 5
            local.get 1
            i32.store offset=24
          end
          block ;; label = @3
            local.get 10
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 9
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 9
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          local.get 10
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 9
          i32.add
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 10
          i32.const 12320
          local.set 13
          local.get 13
          local.get 13
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $dispose_chunk
          local.get 0
          return
        end
        block ;; label = @2
          local.get 1
          i32.const 12352
          local.set 13
          local.get 13
          local.get 13
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $dlmalloc
          local.tee 2
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 2
        local.get 0
        i32.const -4
        i32.const -8
        local.get 3
        i32.load
        local.tee 5
        i32.const 3
        i32.and
        select
        local.get 5
        i32.const -8
        i32.and
        i32.add
        local.tee 5
        local.get 1
        local.get 5
        local.get 1
        i32.lt_u
        select
        i32.const 12384
        local.set 13
        local.get 13
        local.get 13
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $memcpy
        local.set 1
        local.get 0
        i32.const 12416
        local.set 13
        local.get 13
        local.get 13
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dlfree
        local.get 1
        local.set 0
      end
      local.get 0
    )
    (func $dispose_chunk (;139;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      local.get 0
      local.get 1
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 3
          i32.const 2
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.load
          local.tee 4
          local.get 1
          i32.add
          local.set 1
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  local.get 4
                  i32.sub
                  local.tee 0
                  i32.const 0
                  i32.load offset=1055908
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 0
                  i32.load offset=12
                  local.set 3
                  block ;; label = @7
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 3
                    local.get 0
                    i32.load offset=8
                    local.tee 5
                    i32.ne
                    br_if 2 (;@5;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055888
                    i32.const -2
                    local.get 4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1055888
                    br 5 (;@2;)
                  end
                  local.get 0
                  i32.load offset=24
                  local.set 6
                  block ;; label = @7
                    local.get 3
                    local.get 0
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 0
                    i32.load offset=8
                    local.tee 4
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 4
                    i32.store offset=8
                    br 4 (;@3;)
                  end
                  block ;; label = @7
                    block ;; label = @8
                      local.get 0
                      i32.load offset=20
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 20
                      i32.add
                      local.set 5
                      br 1 (;@7;)
                    end
                    local.get 0
                    i32.load offset=16
                    local.tee 4
                    i32.eqz
                    br_if 3 (;@4;)
                    local.get 0
                    i32.const 16
                    i32.add
                    local.set 5
                  end
                  loop ;; label = @7
                    local.get 5
                    local.set 7
                    local.get 4
                    local.tee 3
                    i32.const 20
                    i32.add
                    local.set 5
                    local.get 3
                    i32.load offset=20
                    local.tee 4
                    br_if 0 (;@7;)
                    local.get 3
                    i32.const 16
                    i32.add
                    local.set 5
                    local.get 3
                    i32.load offset=16
                    local.tee 4
                    br_if 0 (;@7;)
                  end
                  local.get 7
                  i32.const 0
                  i32.store
                  br 3 (;@3;)
                end
                local.get 2
                i32.load offset=4
                local.tee 3
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if 3 (;@2;)
                local.get 2
                local.get 3
                i32.const -2
                i32.and
                i32.store offset=4
                i32.const 0
                local.get 1
                i32.store offset=1055896
                local.get 2
                local.get 1
                i32.store
                local.get 0
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                return
              end
              local.get 3
              local.get 5
              i32.store offset=8
              local.get 5
              local.get 3
              i32.store offset=12
              br 2 (;@2;)
            end
            i32.const 0
            local.set 3
          end
          local.get 6
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 0
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.get 3
              i32.store
              local.get 3
              br_if 1 (;@3;)
              i32.const 0
              i32.const 0
              i32.load offset=1055892
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1055892
              br 2 (;@2;)
            end
            local.get 6
            i32.const 16
            i32.const 20
            local.get 6
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 3
            i32.store
            local.get 3
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 3
          local.get 6
          i32.store offset=24
          block ;; label = @3
            local.get 0
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 3
            i32.store offset=24
          end
          local.get 0
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          local.get 4
          i32.store offset=20
          local.get 4
          local.get 3
          i32.store offset=24
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  i32.load offset=4
                  local.tee 4
                  i32.const 2
                  i32.and
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 2
                    i32.const 0
                    i32.load offset=1055912
                    i32.ne
                    br_if 0 (;@7;)
                    i32.const 0
                    local.get 0
                    i32.store offset=1055912
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055900
                    local.get 1
                    i32.add
                    local.tee 1
                    i32.store offset=1055900
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if 6 (;@1;)
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055896
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055908
                    return
                  end
                  block ;; label = @7
                    local.get 2
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if 0 (;@7;)
                    i32.const 0
                    local.get 0
                    i32.store offset=1055908
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055896
                    local.get 1
                    i32.add
                    local.tee 1
                    i32.store offset=1055896
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.add
                    local.get 1
                    i32.store
                    return
                  end
                  local.get 4
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.add
                  local.set 1
                  local.get 2
                  i32.load offset=12
                  local.set 3
                  block ;; label = @7
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 3
                      local.get 2
                      i32.load offset=8
                      local.tee 5
                      i32.ne
                      br_if 0 (;@8;)
                      i32.const 0
                      i32.const 0
                      i32.load offset=1055888
                      i32.const -2
                      local.get 4
                      i32.const 3
                      i32.shr_u
                      i32.rotl
                      i32.and
                      i32.store offset=1055888
                      br 5 (;@3;)
                    end
                    local.get 3
                    local.get 5
                    i32.store offset=8
                    local.get 5
                    local.get 3
                    i32.store offset=12
                    br 4 (;@3;)
                  end
                  local.get 2
                  i32.load offset=24
                  local.set 6
                  block ;; label = @7
                    local.get 3
                    local.get 2
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 2
                    i32.load offset=8
                    local.tee 4
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 4
                    i32.store offset=8
                    br 3 (;@4;)
                  end
                  block ;; label = @7
                    block ;; label = @8
                      local.get 2
                      i32.load offset=20
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 2
                      i32.const 20
                      i32.add
                      local.set 5
                      br 1 (;@7;)
                    end
                    local.get 2
                    i32.load offset=16
                    local.tee 4
                    i32.eqz
                    br_if 2 (;@5;)
                    local.get 2
                    i32.const 16
                    i32.add
                    local.set 5
                  end
                  loop ;; label = @7
                    local.get 5
                    local.set 7
                    local.get 4
                    local.tee 3
                    i32.const 20
                    i32.add
                    local.set 5
                    local.get 3
                    i32.load offset=20
                    local.tee 4
                    br_if 0 (;@7;)
                    local.get 3
                    i32.const 16
                    i32.add
                    local.set 5
                    local.get 3
                    i32.load offset=16
                    local.tee 4
                    br_if 0 (;@7;)
                  end
                  local.get 7
                  i32.const 0
                  i32.store
                  br 2 (;@4;)
                end
                local.get 2
                local.get 4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 0
                local.get 1
                i32.add
                local.get 1
                i32.store
                local.get 0
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                br 3 (;@2;)
              end
              i32.const 0
              local.set 3
            end
            local.get 6
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 2
                local.get 2
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee 4
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 4
                local.get 3
                i32.store
                local.get 3
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1055892
                br 2 (;@3;)
              end
              local.get 6
              i32.const 16
              i32.const 20
              local.get 6
              i32.load offset=16
              local.get 2
              i32.eq
              select
              i32.add
              local.get 3
              i32.store
              local.get 3
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 3
            local.get 6
            i32.store offset=24
            block ;; label = @4
              local.get 2
              i32.load offset=16
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 4
              i32.store offset=16
              local.get 4
              local.get 3
              i32.store offset=24
            end
            local.get 2
            i32.load offset=20
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 3
            local.get 4
            i32.store offset=20
            local.get 4
            local.get 3
            i32.store offset=24
          end
          local.get 0
          local.get 1
          i32.add
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1055896
          return
        end
        block ;; label = @2
          local.get 1
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set 3
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1055888
              local.tee 4
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 1
              i32.or
              i32.store offset=1055888
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 3
            i32.load offset=8
            local.set 1
          end
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 3
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 3
        block ;; label = @2
          local.get 1
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const 38
          local.get 1
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee 3
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get 3
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store offset=28
        local.get 0
        i64.const 0
        i64.store offset=16 align=4
        local.get 3
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set 4
        block ;; label = @2
          i32.const 0
          i32.load offset=1055892
          local.tee 5
          i32.const 1
          local.get 3
          i32.shl
          local.tee 2
          i32.and
          br_if 0 (;@2;)
          local.get 4
          local.get 0
          i32.store
          i32.const 0
          local.get 5
          local.get 2
          i32.or
          i32.store offset=1055892
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 0
          i32.store offset=12
          return
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 3
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 3
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        local.get 4
        i32.load
        local.set 5
        block ;; label = @2
          loop ;; label = @3
            local.get 5
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 1
            i32.eq
            br_if 1 (;@2;)
            local.get 3
            i32.const 29
            i32.shr_u
            local.set 5
            local.get 3
            i32.const 1
            i32.shl
            local.set 3
            local.get 4
            local.get 5
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 2
            i32.load
            local.tee 5
            br_if 0 (;@3;)
          end
          local.get 2
          local.get 0
          i32.store
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 0
          i32.store offset=8
          return
        end
        local.get 4
        i32.load offset=8
        local.tee 1
        local.get 0
        i32.store offset=12
        local.get 4
        local.get 0
        i32.store offset=8
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        local.get 4
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
      end
    )
    (func $posix_memalign (;140;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 16
            i32.ne
            br_if 0 (;@3;)
            local.get 2
            i32.const 12448
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $dlmalloc
            local.set 1
            br 1 (;@2;)
          end
          i32.const 28
          local.set 3
          local.get 1
          i32.const 4
          i32.lt_u
          br_if 1 (;@1;)
          local.get 1
          i32.const 3
          i32.and
          br_if 1 (;@1;)
          local.get 1
          i32.const 2
          i32.shr_u
          local.tee 4
          local.get 4
          i32.const -1
          i32.add
          i32.and
          br_if 1 (;@1;)
          block ;; label = @3
            i32.const -64
            local.get 1
            i32.sub
            local.get 2
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 48
            return
          end
          local.get 1
          i32.const 16
          local.get 1
          i32.const 16
          i32.gt_u
          select
          local.get 2
          i32.const 12480
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $internal_memalign
          local.set 1
        end
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          i32.const 48
          return
        end
        local.get 0
        local.get 1
        i32.store
        i32.const 0
        local.set 3
      end
      local.get 3
    )
    (func $internal_memalign (;141;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 16
          local.get 0
          i32.const 16
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const -1
          i32.add
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        i32.const 32
        local.set 3
        loop ;; label = @2
          local.get 3
          local.tee 0
          i32.const 1
          i32.shl
          local.set 3
          local.get 0
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        i32.const -64
        local.get 0
        i32.sub
        local.get 1
        i32.gt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1056384
        i32.const 0
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 16
        local.get 1
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.get 1
        i32.const 11
        i32.lt_u
        select
        local.tee 1
        i32.add
        i32.const 12
        i32.add
        i32.const 12512
        local.set 7
        local.get 7
        local.get 7
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dlmalloc
        local.tee 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 3
      i32.const -8
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const -1
          i32.add
          local.get 3
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        local.get 3
        i32.const -4
        i32.add
        local.tee 4
        i32.load
        local.tee 5
        i32.const -8
        i32.and
        local.get 3
        local.get 0
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 3
        i32.const 0
        local.get 0
        local.get 3
        local.get 2
        i32.sub
        i32.const 15
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 3
        i32.sub
        local.set 6
        block ;; label = @2
          local.get 5
          i32.const 3
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 6
          i32.store offset=4
          local.get 0
          local.get 2
          i32.load
          local.get 3
          i32.add
          i32.store
          br 1 (;@1;)
        end
        local.get 0
        local.get 6
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 6
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 4
        local.get 3
        local.get 4
        i32.load
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 2
        local.get 3
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 2
        local.get 3
        i32.const 12544
        local.set 7
        local.get 7
        local.get 7
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dispose_chunk
      end
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const -8
        i32.and
        local.tee 2
        local.get 1
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 3
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.tee 3
        local.get 2
        local.get 1
        i32.sub
        local.tee 1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 3
        local.get 1
        i32.const 12576
        local.set 7
        local.get 7
        local.get 7
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $dispose_chunk
      end
      local.get 0
      i32.const 8
      i32.add
    )
    (func $_Exit (;142;) (type 1) (param i32)
      (local i32)
      local.get 0
      i32.const 12608
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__wasi_proc_exit
      unreachable
    )
    (func $__wasilibc_ensure_environ (;143;) (type 0)
      (local i32)
      block ;; label = @1
        i32.const 0
        i32.load offset=1055524
        i32.const -1
        i32.ne
        br_if 0 (;@1;)
        i32.const 12640
        local.set 0
        local.get 0
        local.get 0
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $__wasilibc_initialize_environ
      end
    )
    (func $__wasilibc_initialize_environ (;144;) (type 0)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 12
          i32.add
          local.get 0
          i32.const 8
          i32.add
          i32.const 12672
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $__wasi_environ_sizes_get
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 0
            i32.load offset=12
            local.tee 1
            br_if 0 (;@3;)
            i32.const 1056388
            local.set 1
            br 2 (;@1;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 1
              i32.add
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              i32.load offset=8
              i32.const 12704
              local.set 3
              local.get 3
              local.get 3
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $malloc
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 4
              i32.const 12736
              local.set 3
              local.get 3
              local.get 3
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $calloc
              local.tee 1
              br_if 1 (;@3;)
              local.get 2
              i32.const 12768
              local.set 3
              local.get 3
              local.get 3
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $free
            end
            i32.const 70
            i32.const 12800
            local.set 3
            local.get 3
            local.get 3
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_Exit
            unreachable
          end
          local.get 1
          local.get 2
          i32.const 12832
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $__wasi_environ_get
          i32.eqz
          br_if 1 (;@1;)
          local.get 2
          i32.const 12864
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $free
          local.get 1
          i32.const 12896
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $free
        end
        i32.const 71
        i32.const 12928
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_Exit
        unreachable
      end
      i32.const 0
      local.get 1
      i32.store offset=1055524
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $__wasi_environ_get (;145;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 12960
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__imported_wasi_snapshot_preview1_environ_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_environ_sizes_get (;146;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 12992
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__imported_wasi_snapshot_preview1_environ_sizes_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_proc_exit (;147;) (type 1) (param i32)
      (local i32)
      local.get 0
      call $on_exit
      i32.const 13024
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__imported_wasi_snapshot_preview1_proc_exit
      unreachable
    )
    (func $abort (;148;) (type 0)
      unreachable
    )
    (func $getcwd (;149;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      i32.const 0
      i32.load offset=1055528
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          local.get 2
          i32.const 13056
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $strdup
          local.tee 0
          br_if 1 (;@1;)
          i32.const 0
          i32.const 48
          i32.store offset=1056384
          i32.const 0
          return
        end
        block ;; label = @2
          local.get 2
          i32.const 13088
          local.set 3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $strlen
          i32.const 1
          i32.add
          local.get 1
          i32.le_u
          br_if 0 (;@2;)
          i32.const 0
          i32.const 68
          i32.store offset=1056384
          i32.const 0
          return
        end
        local.get 0
        local.get 2
        i32.const 13120
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $strcpy
        local.set 0
      end
      local.get 0
    )
    (func $sbrk (;150;) (type 3) (param i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        memory.size
        i32.const 16
        i32.shl
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 65535
        i32.and
        br_if 0 (;@1;)
        local.get 0
        i32.const -1
        i32.le_s
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 0
          i32.const -1
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          i32.const 48
          i32.store offset=1056384
          i32.const -1
          return
        end
        local.get 0
        i32.const 16
        i32.shl
        return
      end
      i32.const 13152
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $abort
      unreachable
    )
    (func $getenv (;151;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32)
      i32.const 13184
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__wasilibc_ensure_environ
      block ;; label = @1
        local.get 0
        i32.const 61
        i32.const 13216
        local.set 5
        local.get 5
        local.get 5
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $__strchrnul
        local.tee 1
        local.get 0
        i32.ne
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      i32.const 0
      local.set 2
      block ;; label = @1
        local.get 0
        local.get 1
        local.get 0
        i32.sub
        local.tee 3
        i32.add
        i32.load8_u
        br_if 0 (;@1;)
        i32.const 0
        i32.load offset=1055524
        local.tee 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.const 4
        i32.add
        local.set 4
        block ;; label = @2
          loop ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 1
              local.get 3
              i32.const 13248
              local.set 5
              local.get 5
              local.get 5
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $strncmp
              br_if 0 (;@4;)
              local.get 1
              local.get 3
              i32.add
              local.tee 1
              i32.load8_u
              i32.const 61
              i32.eq
              br_if 2 (;@2;)
            end
            local.get 4
            i32.load
            local.set 1
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 1
            br_if 0 (;@3;)
            br 2 (;@1;)
          end
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 2
      end
      local.get 2
    )
    (func $dummy (;152;) (type 0))
    (func $__wasm_call_dtors (;153;) (type 0)
      (local i32)
      i32.const 13280
      local.set 0
      local.get 0
      local.get 0
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $dummy
      i32.const 13312
      local.set 0
      local.get 0
      local.get 0
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__stdio_exit
    )
    (func $"#func154 dummy" (@name "dummy") (;154;) (type 2) (param i32 i32) (result i32)
      local.get 0
    )
    (func $__lctrans (;155;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 13344
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $"#func154 dummy"
    )
    (func $__ofl_lock (;156;) (type 10) (result i32)
      i32.const 1056420
    )
    (func $__stdio_exit (;157;) (type 0)
      (local i32 i32 i32 i32)
      block ;; label = @1
        i32.const 13376
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $__ofl_lock
        i32.load
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        loop ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load offset=20
            local.get 0
            i32.load offset=24
            i32.eq
            br_if 0 (;@3;)
            local.get 0
            i32.const 0
            i32.const 0
            local.get 0
            i32.load offset=32
            call_indirect (type 5)
            drop
          end
          block ;; label = @3
            local.get 0
            i32.load offset=4
            local.tee 1
            local.get 0
            i32.load offset=8
            local.tee 2
            i32.eq
            br_if 0 (;@3;)
            local.get 0
            local.get 1
            local.get 2
            i32.sub
            i64.extend_i32_s
            i32.const 1
            local.get 0
            i32.load offset=36
            call_indirect (type 8)
            drop
          end
          local.get 0
          i32.load offset=52
          local.tee 0
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        i32.const 0
        i32.load offset=1056424
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=24
          i32.eq
          br_if 0 (;@2;)
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 5)
          drop
        end
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get 0
        i32.load offset=36
        call_indirect (type 8)
        drop
      end
      block ;; label = @1
        i32.const 0
        i32.load offset=1056424
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=24
          i32.eq
          br_if 0 (;@2;)
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 5)
          drop
        end
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get 0
        i32.load offset=36
        call_indirect (type 8)
        drop
      end
      block ;; label = @1
        i32.const 0
        i32.load offset=1056424
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=24
          i32.eq
          br_if 0 (;@2;)
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 5)
          drop
        end
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get 0
        i32.load offset=36
        call_indirect (type 8)
        drop
      end
    )
    (func $memcpy (;158;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 32
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            local.get 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            local.get 1
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.set 4
            local.get 1
            i32.const 1
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 2
            i32.const -2
            i32.add
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.set 4
            local.get 1
            i32.const 2
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 2
            i32.const -3
            i32.add
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.set 4
            local.get 1
            i32.const 3
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get 2
            i32.const -4
            i32.add
            local.set 3
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 1
            i32.const 4
            i32.add
            local.set 5
            br 2 (;@1;)
          end
          local.get 0
          local.get 1
          local.get 2
          memory.copy
          local.get 0
          return
        end
        local.get 2
        local.set 3
        local.get 0
        local.set 4
        local.get 1
        local.set 5
      end
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.const 3
          i32.and
          local.tee 2
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 16
              i32.ge_u
              br_if 0 (;@4;)
              local.get 3
              local.set 2
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 3
              i32.const -16
              i32.add
              local.tee 2
              i32.const 16
              i32.and
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              i32.const 16
              i32.add
              local.set 4
              local.get 5
              i32.const 16
              i32.add
              local.set 5
              local.get 2
              local.set 3
            end
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            local.set 2
            loop ;; label = @4
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              local.get 5
              i64.load offset=16 align=4
              i64.store offset=16 align=4
              local.get 4
              local.get 5
              i64.load offset=24 align=4
              i64.store offset=24 align=4
              local.get 4
              i32.const 32
              i32.add
              local.set 4
              local.get 5
              i32.const 32
              i32.add
              local.set 5
              local.get 2
              i32.const -32
              i32.add
              local.tee 2
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
            end
          end
          block ;; label = @3
            local.get 2
            i32.const 8
            i32.lt_u
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 5
            i32.const 8
            i32.add
            local.set 5
            local.get 4
            i32.const 8
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load
            i32.store
            local.get 5
            i32.const 4
            i32.add
            local.set 5
            local.get 4
            i32.const 4
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 2
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load16_u align=1
            i32.store16 align=1
            local.get 4
            i32.const 2
            i32.add
            local.set 4
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 2
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 5
          i32.load8_u
          i32.store8
          local.get 0
          return
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.const 32
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 4
                  local.get 5
                  i32.load
                  local.tee 3
                  i32.store8
                  block ;; label = @7
                    block ;; label = @8
                      local.get 2
                      i32.const -1
                      i32.add
                      br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 3 (;@5;)
                    end
                    local.get 4
                    local.get 3
                    i32.const 8
                    i32.shr_u
                    i32.store8 offset=1
                    local.get 4
                    local.get 5
                    i32.const 6
                    i32.add
                    i64.load align=2
                    i64.store offset=6 align=4
                    local.get 4
                    local.get 5
                    i32.load offset=4
                    i32.const 16
                    i32.shl
                    local.get 3
                    i32.const 16
                    i32.shr_u
                    i32.or
                    i32.store offset=2
                    local.get 4
                    i32.const 18
                    i32.add
                    local.set 2
                    local.get 5
                    i32.const 18
                    i32.add
                    local.set 1
                    i32.const 14
                    local.set 6
                    local.get 5
                    i32.const 14
                    i32.add
                    i32.load align=2
                    local.set 5
                    i32.const 14
                    local.set 3
                    br 3 (;@4;)
                  end
                  local.get 4
                  local.get 5
                  i32.const 5
                  i32.add
                  i64.load align=1
                  i64.store offset=5 align=4
                  local.get 4
                  local.get 5
                  i32.load offset=4
                  i32.const 24
                  i32.shl
                  local.get 3
                  i32.const 8
                  i32.shr_u
                  i32.or
                  i32.store offset=1
                  local.get 4
                  i32.const 17
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 17
                  i32.add
                  local.set 1
                  i32.const 13
                  local.set 6
                  local.get 5
                  i32.const 13
                  i32.add
                  i32.load align=1
                  local.set 5
                  i32.const 15
                  local.set 3
                  br 2 (;@4;)
                end
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.const 16
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 4
                    local.set 2
                    local.get 5
                    local.set 1
                    br 1 (;@6;)
                  end
                  local.get 4
                  local.get 5
                  i32.load8_u
                  i32.store8
                  local.get 4
                  local.get 5
                  i32.load offset=1 align=1
                  i32.store offset=1 align=1
                  local.get 4
                  local.get 5
                  i64.load offset=5 align=1
                  i64.store offset=5 align=1
                  local.get 4
                  local.get 5
                  i32.load16_u offset=13 align=1
                  i32.store16 offset=13 align=1
                  local.get 4
                  local.get 5
                  i32.load8_u offset=15
                  i32.store8 offset=15
                  local.get 4
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 16
                  i32.add
                  local.set 1
                end
                local.get 3
                i32.const 8
                i32.and
                br_if 2 (;@3;)
                br 3 (;@2;)
              end
              local.get 4
              local.get 3
              i32.const 16
              i32.shr_u
              i32.store8 offset=2
              local.get 4
              local.get 3
              i32.const 8
              i32.shr_u
              i32.store8 offset=1
              local.get 4
              local.get 5
              i32.const 7
              i32.add
              i64.load align=1
              i64.store offset=7 align=4
              local.get 4
              local.get 5
              i32.load offset=4
              i32.const 8
              i32.shl
              local.get 3
              i32.const 24
              i32.shr_u
              i32.or
              i32.store offset=3
              local.get 4
              i32.const 19
              i32.add
              local.set 2
              local.get 5
              i32.const 19
              i32.add
              local.set 1
              i32.const 15
              local.set 6
              local.get 5
              i32.const 15
              i32.add
              i32.load align=1
              local.set 5
              i32.const 13
              local.set 3
            end
            local.get 4
            local.get 6
            i32.add
            local.get 5
            i32.store
          end
          local.get 2
          local.get 1
          i64.load align=1
          i64.store align=1
          local.get 2
          i32.const 8
          i32.add
          local.set 2
          local.get 1
          i32.const 8
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load align=1
          i32.store align=1
          local.get 2
          i32.const 4
          i32.add
          local.set 2
          local.get 1
          i32.const 4
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load16_u align=1
          i32.store16 align=1
          local.get 2
          i32.const 2
          i32.add
          local.set 2
          local.get 1
          i32.const 2
          i32.add
          local.set 1
        end
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
      end
      local.get 0
    )
    (func $memset (;159;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i64)
      block ;; label = @1
        local.get 2
        i32.const 33
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        memory.fill
        local.get 0
        return
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8
        local.get 0
        local.get 2
        i32.add
        local.tee 3
        i32.const -1
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 3
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8 offset=2
        local.get 0
        local.get 1
        i32.store8 offset=1
        local.get 3
        i32.const -3
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const -2
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 7
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8 offset=3
        local.get 3
        i32.const -4
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 9
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.tee 3
        i32.store
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        i32.const 60
        i32.and
        local.tee 1
        i32.add
        local.tee 2
        i32.const -4
        i32.add
        local.get 3
        i32.store
        local.get 1
        i32.const 9
        i32.lt_u
        br_if 0 (;@1;)
        local.get 5
        local.get 3
        i32.store offset=8
        local.get 5
        local.get 3
        i32.store offset=4
        local.get 2
        i32.const -8
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -12
        i32.add
        local.get 3
        i32.store
        local.get 1
        i32.const 25
        i32.lt_u
        br_if 0 (;@1;)
        local.get 5
        local.get 3
        i32.store offset=24
        local.get 5
        local.get 3
        i32.store offset=20
        local.get 5
        local.get 3
        i32.store offset=16
        local.get 5
        local.get 3
        i32.store offset=12
        local.get 2
        i32.const -16
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -20
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -24
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -28
        i32.add
        local.get 3
        i32.store
        local.get 1
        local.get 5
        i32.const 4
        i32.and
        i32.const 24
        i32.or
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 32
        i32.lt_u
        br_if 0 (;@1;)
        local.get 3
        i64.extend_i32_u
        i64.const 4294967297
        i64.mul
        local.set 6
        local.get 5
        local.get 2
        i32.add
        local.set 2
        loop ;; label = @2
          local.get 2
          local.get 6
          i64.store offset=24
          local.get 2
          local.get 6
          i64.store offset=16
          local.get 2
          local.get 6
          i64.store offset=8
          local.get 2
          local.get 6
          i64.store
          local.get 2
          i32.const 32
          i32.add
          local.set 2
          local.get 1
          i32.const -32
          i32.add
          local.tee 1
          i32.const 31
          i32.gt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
    )
    (func $__strchrnul (;160;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 255
              i32.and
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@2;)
              block ;; label = @5
                local.get 0
                i32.load8_u
                local.tee 3
                br_if 0 (;@5;)
                local.get 0
                return
              end
              local.get 3
              local.get 1
              i32.const 255
              i32.and
              i32.ne
              br_if 1 (;@3;)
              local.get 0
              return
            end
            local.get 0
            local.get 0
            i32.const 13408
            local.set 5
            local.get 5
            local.get 5
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $strlen
            i32.add
            return
          end
          block ;; label = @3
            local.get 0
            i32.const 1
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 0
            i32.const 2
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 0
            i32.const 3
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.set 0
        end
        block ;; label = @2
          block ;; label = @3
            i32.const 16843008
            local.get 0
            i32.load
            local.tee 3
            i32.sub
            local.get 3
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if 0 (;@3;)
            local.get 0
            local.set 2
            br 1 (;@2;)
          end
          local.get 2
          i32.const 16843009
          i32.mul
          local.set 4
          loop ;; label = @3
            block ;; label = @4
              i32.const 16843008
              local.get 3
              local.get 4
              i32.xor
              local.tee 3
              i32.sub
              local.get 3
              i32.or
              i32.const -2139062144
              i32.and
              i32.const -2139062144
              i32.eq
              br_if 0 (;@4;)
              local.get 0
              local.set 2
              br 2 (;@2;)
            end
            local.get 0
            i32.load offset=4
            local.set 3
            local.get 0
            i32.const 4
            i32.add
            local.tee 2
            local.set 0
            local.get 3
            i32.const 16843008
            local.get 3
            i32.sub
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if 0 (;@3;)
          end
        end
        local.get 2
        i32.const -1
        i32.add
        local.set 3
        loop ;; label = @2
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          i32.load8_u
          local.tee 0
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          local.get 1
          i32.const 255
          i32.and
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 3
    )
    (func $__stpcpy (;161;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 0
            i32.xor
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load8_u
            local.set 2
            br 1 (;@2;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 1
              local.set 3
              br 1 (;@3;)
            end
            local.get 0
            local.get 1
            i32.load8_u
            local.tee 2
            i32.store8
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              local.get 0
              return
            end
            local.get 0
            i32.const 1
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 2
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 2
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 3
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 3
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 1
            i32.const 4
            i32.add
            local.set 3
          end
          block ;; label = @3
            i32.const 16843008
            local.get 3
            i32.load
            local.tee 2
            i32.sub
            local.get 2
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if 0 (;@3;)
            local.get 3
            local.set 1
            br 1 (;@2;)
          end
          loop ;; label = @3
            local.get 0
            local.get 2
            i32.store
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 3
            i32.load offset=4
            local.set 2
            local.get 3
            i32.const 4
            i32.add
            local.tee 1
            local.set 3
            local.get 2
            i32.const 16843008
            local.get 2
            i32.sub
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if 0 (;@3;)
          end
        end
        local.get 0
        local.get 2
        i32.store8
        block ;; label = @2
          local.get 2
          i32.const 255
          i32.and
          br_if 0 (;@2;)
          local.get 0
          return
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        local.set 2
        loop ;; label = @2
          local.get 2
          local.get 3
          i32.load8_u
          local.tee 0
          i32.store8 offset=1
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 0
          br_if 0 (;@2;)
        end
      end
      local.get 2
    )
    (func $strcpy (;162;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 13440
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__stpcpy
      drop
      local.get 0
    )
    (func $strdup (;163;) (type 3) (param i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.const 13472
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $strlen
        i32.const 1
        i32.add
        local.tee 1
        i32.const 13504
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $malloc
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 0
        local.get 1
        i32.const 13536
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $memcpy
        drop
      end
      local.get 2
    )
    (func $strerror (;164;) (type 3) (param i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        i32.const 0
        i32.load offset=1056416
        local.tee 1
        br_if 0 (;@1;)
        i32.const 1056392
        local.set 1
        i32.const 0
        i32.const 1056392
        i32.store offset=1056416
      end
      i32.const 0
      local.get 0
      local.get 0
      i32.const 76
      i32.gt_u
      select
      i32.const 1
      i32.shl
      i32.const 1053136
      i32.add
      i32.load16_u
      i32.const 1051578
      i32.add
      local.get 1
      i32.load offset=20
      i32.const 13568
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $__lctrans
    )
    (func $strerror_r (;165;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 13600
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $strerror
          local.tee 0
          i32.const 13632
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $strlen
          local.tee 3
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
          i32.const 68
          local.set 3
          local.get 2
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.const 13664
          local.set 4
          local.get 4
          local.get 4
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $memcpy
          local.get 2
          i32.add
          i32.const 0
          i32.store8
          i32.const 68
          return
        end
        local.get 1
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.const 13696
        local.set 4
        local.get 4
        local.get 4
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $memcpy
        drop
        i32.const 0
        local.set 3
      end
      local.get 3
    )
    (func $strlen (;166;) (type 3) (param i32) (result i32)
      (local i32 i32 i32)
      local.get 0
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 0
            i32.load8_u
            br_if 0 (;@3;)
            local.get 0
            local.get 0
            i32.sub
            return
          end
          local.get 0
          i32.const 1
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 2
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 3
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          br_if 1 (;@1;)
        end
        local.get 1
        i32.const -4
        i32.add
        local.set 2
        local.get 1
        i32.const -5
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          i32.const 16843008
          local.get 2
          i32.const 4
          i32.add
          local.tee 2
          i32.load
          local.tee 3
          i32.sub
          local.get 3
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.eq
          br_if 0 (;@2;)
        end
        loop ;; label = @2
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.load8_u
          local.set 3
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 3
          br_if 0 (;@2;)
        end
      end
      local.get 1
      local.get 0
      i32.sub
    )
    (func $strncmp (;167;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 2
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load8_u
          local.tee 3
          br_if 0 (;@2;)
          i32.const 0
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        block ;; label = @2
          loop ;; label = @3
            local.get 3
            i32.const 255
            i32.and
            local.get 1
            i32.load8_u
            local.tee 4
            i32.ne
            br_if 1 (;@2;)
            local.get 4
            i32.eqz
            br_if 1 (;@2;)
            local.get 2
            i32.const 0
            i32.eq
            br_if 1 (;@2;)
            local.get 2
            i32.const -1
            i32.add
            local.set 2
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            i32.load8_u
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            local.get 3
            br_if 0 (;@3;)
          end
          i32.const 0
          local.set 3
        end
        local.get 3
        i32.const 255
        i32.and
        local.set 3
      end
      local.get 3
      local.get 1
      i32.load8_u
      i32.sub
    )
    (func $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE (;168;) (type 1) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 0
      i32.store offset=24
      local.get 1
      i32.const 1
      i32.store offset=12
      local.get 1
      i64.const 4
      i64.store offset=16 align=4
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055532
      i32.add
      i32.store offset=8
      local.get 1
      i32.const 8
      i32.add
      local.get 0
      i32.const 13728
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E (;169;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 3
        i32.const 1
        i32.shl
        local.tee 4
        i32.const 8
        local.get 4
        i32.const 8
        i32.gt_u
        select
        local.tee 4
        i32.const 0
        i32.ge_s
        br_if 0 (;@1;)
        i32.const 0
        i32.const 0
        local.get 1
        i32.const 13760
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 0
      local.set 5
      block ;; label = @1
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 3
        i32.store offset=28
        local.get 2
        local.get 0
        i32.load offset=4
        i32.store offset=20
        i32.const 1
        local.set 5
      end
      local.get 2
      local.get 5
      i32.store offset=24
      local.get 2
      i32.const 8
      i32.add
      i32.const 1
      local.get 4
      local.get 2
      i32.const 20
      i32.add
      i32.const 13792
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE
      block ;; label = @1
        local.get 2
        i32.load offset=8
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 2
        i32.load offset=12
        local.get 2
        i32.load offset=16
        local.get 1
        i32.const 13824
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      local.get 2
      i32.load offset=12
      local.set 3
      local.get 0
      local.get 4
      i32.store
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE (;170;) (type 4) (param i32 i32 i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.const 13856
        local.set 3
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get 2
      i32.const 13888
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE
      unreachable
    )
    (func $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE (;171;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32)
      block ;; label = @1
        local.get 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.load offset=4
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.load offset=8
                local.tee 4
                br_if 0 (;@5;)
                block ;; label = @6
                  local.get 2
                  br_if 0 (;@6;)
                  local.get 1
                  local.set 3
                  br 4 (;@2;)
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                br 2 (;@3;)
              end
              local.get 3
              i32.load
              local.get 4
              local.get 1
              local.get 2
              i32.const 13920
              local.set 5
              local.get 5
              local.get 5
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
              local.set 3
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              local.get 1
              local.set 3
              br 2 (;@2;)
            end
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
          end
          local.get 2
          local.get 1
          i32.const 13952
          local.set 5
          local.get 5
          local.get 5
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.set 3
        end
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          i32.const 1
          i32.store
          return
        end
        local.get 0
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 0
      i32.const 0
      i32.store offset=4
      local.get 0
      i32.const 1
      i32.store
    )
    (func $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E (;172;) (type 6) (param i32 i32)
      (local i32)
      local.get 1
      local.get 0
      i32.const 13984
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler
      unreachable
    )
    (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE (;173;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const -1
            i32.eq
            br_if 0 (;@3;)
            i32.const 0
            local.set 4
            block ;; label = @4
              local.get 2
              i32.const 1
              i32.add
              local.tee 5
              i32.const 0
              i32.lt_s
              br_if 0 (;@4;)
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set 4
              local.get 5
              i32.const 1
              i32.const 14016
              local.set 7
              local.get 7
              local.get 7
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee 6
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 2
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 1
                local.get 2
                memory.copy
              end
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  i32.const 7
                  i32.gt_u
                  br_if 0 (;@6;)
                  local.get 2
                  i32.eqz
                  br_if 4 (;@2;)
                  block ;; label = @7
                    local.get 1
                    i32.load8_u
                    br_if 0 (;@7;)
                    i32.const 0
                    local.set 4
                    br 2 (;@5;)
                  end
                  i32.const 1
                  local.set 4
                  local.get 2
                  i32.const 1
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=1
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 2
                  local.set 4
                  local.get 2
                  i32.const 2
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=2
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 3
                  local.set 4
                  local.get 2
                  i32.const 3
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=3
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 4
                  local.set 4
                  local.get 2
                  i32.const 4
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=4
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 5
                  local.set 4
                  local.get 2
                  i32.const 5
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=5
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 6
                  local.set 4
                  local.get 2
                  i32.const 6
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=6
                  i32.eqz
                  br_if 1 (;@5;)
                  br 4 (;@2;)
                end
                local.get 3
                i32.const 8
                i32.add
                i32.const 0
                local.get 1
                local.get 2
                i32.const 14048
                local.set 7
                local.get 7
                local.get 7
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4core5slice6memchr14memchr_aligned17hf4db372f52bc45e4E
                local.get 3
                i32.load offset=8
                i32.const 1
                i32.and
                i32.eqz
                br_if 3 (;@2;)
                local.get 3
                i32.load offset=12
                local.set 4
              end
              local.get 0
              local.get 4
              i32.store offset=12
              local.get 0
              local.get 2
              i32.store offset=8
              local.get 0
              local.get 6
              i32.store offset=4
              local.get 0
              local.get 5
              i32.store
              br 3 (;@1;)
            end
            local.get 4
            local.get 5
            global.get $GOT.data.internal.__memory_base
            i32.const 1055540
            i32.add
            i32.const 14080
            local.set 7
            local.get 7
            local.get 7
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
            unreachable
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055556
          i32.add
          i32.const 14112
          local.set 7
          local.get 7
          local.get 7
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE
          unreachable
        end
        local.get 3
        local.get 2
        i32.store offset=28
        local.get 3
        local.get 6
        i32.store offset=24
        local.get 3
        local.get 5
        i32.store offset=20
        local.get 3
        local.get 3
        i32.const 20
        i32.add
        i32.const 14144
        local.set 7
        local.get 7
        local.get 7
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE
        local.get 0
        local.get 3
        i64.load
        i64.store offset=4 align=4
        local.get 0
        i32.const -2147483648
        i32.store
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE (;174;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 1
        i32.load
        local.tee 3
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.ne
        br_if 0 (;@1;)
        i32.const 0
        local.set 5
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 4
              i32.const 1
              i32.add
              local.tee 3
              i32.const 0
              i32.ge_s
              br_if 0 (;@4;)
              br 1 (;@3;)
            end
            i32.const 0
            local.set 5
            block ;; label = @4
              local.get 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 4
              i32.store offset=28
              local.get 2
              local.get 1
              i32.load offset=4
              i32.store offset=20
              i32.const 1
              local.set 5
            end
            local.get 2
            local.get 5
            i32.store offset=24
            local.get 2
            i32.const 8
            i32.add
            i32.const 1
            local.get 3
            local.get 2
            i32.const 20
            i32.add
            i32.const 14176
            local.set 6
            local.get 6
            local.get 6
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE
            local.get 2
            i32.load offset=8
            i32.const 1
            i32.ne
            br_if 1 (;@2;)
            local.get 2
            i32.load offset=16
            local.set 1
            local.get 2
            i32.load offset=12
            local.set 5
          end
          local.get 5
          local.get 1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055572
          i32.add
          i32.const 14208
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
          unreachable
        end
        local.get 2
        i32.load offset=12
        local.set 5
        local.get 1
        local.get 3
        i32.store
        local.get 1
        local.get 5
        i32.store offset=4
      end
      local.get 1
      local.get 4
      i32.const 1
      i32.add
      local.tee 5
      i32.store offset=8
      local.get 1
      i32.load offset=4
      local.tee 1
      local.get 4
      i32.add
      i32.const 0
      i32.store8
      block ;; label = @1
        block ;; label = @2
          local.get 3
          local.get 5
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          local.set 4
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 5
          br_if 0 (;@2;)
          i32.const 1
          local.set 4
          local.get 1
          local.get 3
          i32.const 1
          i32.const 14240
          local.set 6
          local.get 6
          local.get 6
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 1
        local.get 3
        i32.const 1
        local.get 5
        i32.const 14272
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
        local.tee 4
        br_if 0 (;@1;)
        i32.const 1
        local.get 5
        i32.const 14304
        local.set 6
        local.get 6
        local.get 6
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      local.get 4
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E (;175;) (type 4) (param i32 i32 i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 14336
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE (;176;) (type 4) (param i32 i32 i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 14368
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE
      unreachable
    )
    (func $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E (;177;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.const 402653184
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.const 268435456
                i32.and
                br_if 0 (;@5;)
                local.get 2
                i32.const 16
                i32.lt_u
                br_if 1 (;@4;)
                local.get 1
                local.get 2
                i32.const 14400
                local.set 10
                local.get 10
                local.get 10
                i32.load 2 offset=28 align=1
                i32.const 1
                i32.add
                i32.store 2 offset=28 align=1
                call $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E
                local.set 4
                br 2 (;@3;)
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.load16_u offset=14
                    local.tee 5
                    br_if 0 (;@7;)
                    i32.const 0
                    local.set 2
                    br 1 (;@6;)
                  end
                  local.get 1
                  local.get 2
                  i32.add
                  local.set 6
                  i32.const 0
                  local.set 2
                  local.get 5
                  local.set 7
                  local.get 1
                  local.set 8
                  loop ;; label = @7
                    local.get 8
                    local.tee 4
                    local.get 6
                    i32.eq
                    br_if 2 (;@5;)
                    block ;; label = @8
                      block ;; label = @9
                        local.get 4
                        i32.load8_s
                        local.tee 8
                        i32.const -1
                        i32.le_s
                        br_if 0 (;@9;)
                        local.get 4
                        i32.const 1
                        i32.add
                        local.set 8
                        br 1 (;@8;)
                      end
                      block ;; label = @9
                        local.get 8
                        i32.const -32
                        i32.ge_u
                        br_if 0 (;@9;)
                        local.get 4
                        i32.const 2
                        i32.add
                        local.set 8
                        br 1 (;@8;)
                      end
                      block ;; label = @9
                        local.get 8
                        i32.const -16
                        i32.ge_u
                        br_if 0 (;@9;)
                        local.get 4
                        i32.const 3
                        i32.add
                        local.set 8
                        br 1 (;@8;)
                      end
                      local.get 4
                      i32.const 4
                      i32.add
                      local.set 8
                    end
                    local.get 8
                    local.get 4
                    i32.sub
                    local.get 2
                    i32.add
                    local.set 2
                    local.get 7
                    i32.const -1
                    i32.add
                    local.tee 7
                    br_if 0 (;@7;)
                  end
                end
                i32.const 0
                local.set 7
              end
              local.get 5
              local.get 7
              i32.sub
              local.set 4
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              i32.const 0
              local.set 2
              i32.const 0
              local.set 4
              br 1 (;@3;)
            end
            local.get 2
            i32.const 3
            i32.and
            local.set 6
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.const 4
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                local.set 4
                i32.const 0
                local.set 7
                br 1 (;@4;)
              end
              local.get 2
              i32.const 12
              i32.and
              local.set 5
              i32.const 0
              local.set 4
              i32.const 0
              local.set 7
              loop ;; label = @5
                local.get 4
                local.get 1
                local.get 7
                i32.add
                local.tee 8
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 8
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 8
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 8
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set 4
                local.get 5
                local.get 7
                i32.const 4
                i32.add
                local.tee 7
                i32.ne
                br_if 0 (;@5;)
              end
            end
            local.get 6
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            local.get 7
            i32.add
            local.set 8
            loop ;; label = @4
              local.get 4
              local.get 8
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 4
              local.get 8
              i32.const 1
              i32.add
              local.set 8
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              br_if 0 (;@4;)
            end
          end
          local.get 4
          local.get 0
          i32.load16_u offset=12
          local.tee 8
          i32.ge_u
          br_if 0 (;@2;)
          local.get 8
          local.get 4
          i32.sub
          local.set 9
          i32.const 0
          local.set 4
          i32.const 0
          local.set 5
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.const 29
                i32.shr_u
                i32.const 3
                i32.and
                br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;) 2 (;@3;)
              end
              local.get 9
              local.set 5
              br 1 (;@3;)
            end
            local.get 9
            i32.const 65534
            i32.and
            i32.const 1
            i32.shr_u
            local.set 5
          end
          local.get 3
          i32.const 2097151
          i32.and
          local.set 6
          local.get 0
          i32.load offset=4
          local.set 7
          local.get 0
          i32.load
          local.set 0
          block ;; label = @3
            loop ;; label = @4
              local.get 4
              i32.const 65535
              i32.and
              local.get 5
              i32.const 65535
              i32.and
              i32.ge_u
              br_if 1 (;@3;)
              i32.const 1
              local.set 8
              local.get 4
              i32.const 1
              i32.add
              local.set 4
              local.get 0
              local.get 6
              local.get 7
              i32.load offset=16
              call_indirect (type 2)
              br_if 3 (;@1;)
              br 0 (;@4;)
            end
          end
          i32.const 1
          local.set 8
          local.get 0
          local.get 1
          local.get 2
          local.get 7
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          i32.const 0
          local.set 4
          local.get 9
          local.get 5
          i32.sub
          i32.const 65535
          i32.and
          local.set 2
          loop ;; label = @3
            local.get 4
            i32.const 65535
            i32.and
            local.tee 5
            local.get 2
            i32.lt_u
            local.set 8
            local.get 5
            local.get 2
            i32.ge_u
            br_if 2 (;@1;)
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 0
            local.get 6
            local.get 7
            i32.load offset=16
            call_indirect (type 2)
            br_if 2 (;@1;)
            br 0 (;@3;)
          end
        end
        local.get 0
        i32.load
        local.get 1
        local.get 2
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 5)
        local.set 8
      end
      local.get 8
    )
    (func $_ZN4core9panicking5panic17hd836709591dfc35fE (;178;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 0
      i32.store offset=16
      local.get 3
      i32.const 1
      i32.store offset=4
      local.get 3
      i64.const 4
      i64.store offset=8 align=4
      local.get 3
      local.get 1
      i32.store offset=28
      local.get 3
      local.get 0
      i32.store offset=24
      local.get 3
      local.get 3
      i32.const 24
      i32.add
      i32.store
      local.get 3
      local.get 2
      i32.const 14432
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE (;179;) (type 6) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 1
      i32.store16 offset=12
      local.get 2
      local.get 1
      i32.store offset=8
      local.get 2
      local.get 0
      i32.store offset=4
      local.get 2
      i32.const 4
      i32.add
      i32.const 14464
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind
      unreachable
    )
    (func $_ZN4core3fmt5write17h19dbf2ffaf30f068E (;180;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      local.get 0
      i32.store
      local.get 3
      i64.const 3758096416
      i64.store offset=8 align=4
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.load offset=16
                local.tee 4
                i32.eqz
                br_if 0 (;@5;)
                local.get 2
                i32.load offset=20
                local.tee 1
                br_if 1 (;@4;)
                br 2 (;@3;)
              end
              local.get 2
              i32.load offset=12
              local.tee 0
              i32.eqz
              br_if 1 (;@3;)
              local.get 2
              i32.load offset=8
              local.tee 1
              local.get 0
              i32.const 3
              i32.shl
              i32.add
              local.set 5
              local.get 0
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 6
              local.get 2
              i32.load
              local.set 0
              loop ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load
                  local.get 0
                  i32.load
                  local.get 7
                  local.get 3
                  i32.load offset=4
                  i32.load offset=12
                  call_indirect (type 5)
                  i32.eqz
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 1
                  br 5 (;@1;)
                end
                block ;; label = @6
                  local.get 1
                  i32.load
                  local.get 3
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  call_indirect (type 2)
                  i32.eqz
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 1
                  br 5 (;@1;)
                end
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 1
                i32.const 8
                i32.add
                local.tee 1
                local.get 5
                i32.eq
                br_if 3 (;@2;)
                br 0 (;@5;)
              end
            end
            local.get 1
            i32.const 24
            i32.mul
            local.set 8
            local.get 1
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 6
            local.get 2
            i32.load offset=8
            local.set 9
            local.get 2
            i32.load
            local.set 0
            i32.const 0
            local.set 7
            loop ;; label = @4
              block ;; label = @5
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@5;)
                local.get 3
                i32.load
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=4
                i32.load offset=12
                call_indirect (type 5)
                i32.eqz
                br_if 0 (;@5;)
                i32.const 1
                local.set 1
                br 4 (;@1;)
              end
              i32.const 0
              local.set 5
              i32.const 0
              local.set 10
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 4
                    local.get 7
                    i32.add
                    local.tee 1
                    i32.const 8
                    i32.add
                    i32.load16_u
                    br_table 0 (;@7;) 1 (;@6;) 2 (;@5;) 0 (;@7;)
                  end
                  local.get 1
                  i32.const 10
                  i32.add
                  i32.load16_u
                  local.set 10
                  br 1 (;@5;)
                end
                local.get 9
                local.get 1
                i32.const 12
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                i32.load16_u offset=4
                local.set 10
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.load16_u
                    br_table 0 (;@7;) 1 (;@6;) 2 (;@5;) 0 (;@7;)
                  end
                  local.get 1
                  i32.const 2
                  i32.add
                  i32.load16_u
                  local.set 5
                  br 1 (;@5;)
                end
                local.get 9
                local.get 1
                i32.const 4
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                i32.load16_u offset=4
                local.set 5
              end
              local.get 3
              local.get 5
              i32.store16 offset=14
              local.get 3
              local.get 10
              i32.store16 offset=12
              local.get 3
              local.get 1
              i32.const 20
              i32.add
              i32.load
              i32.store offset=8
              block ;; label = @5
                local.get 9
                local.get 1
                i32.const 16
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                local.tee 1
                i32.load
                local.get 3
                local.get 1
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 2)
                i32.eqz
                br_if 0 (;@5;)
                i32.const 1
                local.set 1
                br 4 (;@1;)
              end
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 8
              local.get 7
              i32.const 24
              i32.add
              local.tee 7
              i32.eq
              br_if 2 (;@2;)
              br 0 (;@4;)
            end
          end
          i32.const 0
          local.set 6
        end
        block ;; label = @2
          local.get 6
          local.get 2
          i32.load offset=4
          i32.ge_u
          br_if 0 (;@2;)
          local.get 3
          i32.load
          local.get 2
          i32.load
          local.get 6
          i32.const 3
          i32.shl
          i32.add
          local.tee 1
          i32.load
          local.get 1
          i32.load offset=4
          local.get 3
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          i32.eqz
          br_if 0 (;@2;)
          i32.const 1
          local.set 1
          br 1 (;@1;)
        end
        i32.const 0
        local.set 1
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE (;181;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 10
      local.set 4
      local.get 0
      local.set 5
      block ;; label = @1
        local.get 0
        i32.const 1000
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 10
        local.set 4
        local.get 0
        local.set 6
        loop ;; label = @2
          local.get 3
          i32.const 6
          i32.add
          local.get 4
          i32.add
          local.tee 7
          i32.const -4
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1053532
          i32.add
          local.tee 8
          local.get 6
          local.get 6
          i32.const 10000
          i32.div_u
          local.tee 5
          i32.const 10000
          i32.mul
          i32.sub
          local.tee 9
          i32.const 65535
          i32.and
          i32.const 100
          i32.div_u
          local.tee 10
          i32.const 1
          i32.shl
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          local.get 7
          i32.const -2
          i32.add
          local.get 8
          local.get 9
          local.get 10
          i32.const 100
          i32.mul
          i32.sub
          i32.const 65535
          i32.and
          i32.const 1
          i32.shl
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const -4
          i32.add
          local.set 4
          local.get 6
          i32.const 9999999
          i32.gt_u
          local.set 7
          local.get 5
          local.set 6
          local.get 7
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        block ;; label = @2
          local.get 5
          i32.const 9
          i32.gt_u
          br_if 0 (;@2;)
          local.get 5
          local.set 6
          br 1 (;@1;)
        end
        local.get 3
        i32.const 6
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get 5
        local.get 5
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 6
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
      end
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.eqz
          br_if 0 (;@2;)
          local.get 6
          i32.eqz
          br_if 1 (;@1;)
        end
        local.get 3
        i32.const 6
        i32.add
        local.get 4
        i32.const -1
        i32.add
        local.tee 4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 30
        i32.and
        i32.add
        i32.const 1
        i32.add
        i32.load8_u
        i32.store8
      end
      local.get 2
      local.get 1
      i32.const 1
      i32.const 0
      local.get 3
      i32.const 6
      i32.add
      local.get 4
      i32.add
      i32.const 10
      local.get 4
      i32.sub
      i32.const 14496
      local.set 11
      local.get 11
      local.get 11
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 6
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 6
    )
    (func $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE (;182;) (type 13) (param i32 i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          local.get 5
          i32.const 1
          i32.add
          local.set 6
          local.get 0
          i32.load offset=8
          local.set 7
          i32.const 45
          local.set 8
          br 1 (;@1;)
        end
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load offset=8
        local.tee 7
        i32.const 2097152
        i32.and
        local.tee 1
        select
        local.set 8
        local.get 1
        i32.const 21
        i32.shr_u
        local.get 5
        i32.add
        local.set 6
      end
      block ;; label = @1
        block ;; label = @2
          local.get 7
          i32.const 8388608
          i32.and
          br_if 0 (;@2;)
          i32.const 0
          local.set 2
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          i32.const 14528
          local.set 15
          local.get 15
          local.get 15
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E
          local.get 6
          i32.add
          local.set 6
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          i32.const 0
          local.get 6
          i32.add
          local.set 6
          br 1 (;@1;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 9
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 1
            i32.const 0
            local.set 10
            br 1 (;@2;)
          end
          local.get 3
          i32.const 12
          i32.and
          local.set 11
          i32.const 0
          local.set 1
          i32.const 0
          local.set 10
          loop ;; label = @3
            local.get 1
            local.get 2
            local.get 10
            i32.add
            local.tee 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 11
            local.get 10
            i32.const 4
            i32.add
            local.tee 10
            i32.ne
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 9
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 10
          i32.add
          local.set 12
          loop ;; label = @3
            local.get 1
            local.get 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 12
            i32.const 1
            i32.add
            local.set 12
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@3;)
          end
        end
        local.get 1
        local.get 6
        i32.add
        local.set 6
      end
      block ;; label = @1
        block ;; label = @2
          local.get 6
          local.get 0
          i32.load16_u offset=12
          local.tee 11
          i32.ge_u
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 7
                i32.const 16777216
                i32.and
                br_if 0 (;@5;)
                local.get 11
                local.get 6
                i32.sub
                local.set 13
                i32.const 0
                local.set 1
                i32.const 0
                local.set 11
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 7
                      i32.const 29
                      i32.shr_u
                      i32.const 3
                      i32.and
                      br_table 2 (;@6;) 0 (;@8;) 1 (;@7;) 0 (;@8;) 2 (;@6;)
                    end
                    local.get 13
                    local.set 11
                    br 1 (;@6;)
                  end
                  local.get 13
                  i32.const 65534
                  i32.and
                  i32.const 1
                  i32.shr_u
                  local.set 11
                end
                local.get 7
                i32.const 2097151
                i32.and
                local.set 6
                local.get 0
                i32.load offset=4
                local.set 9
                local.get 0
                i32.load
                local.set 10
                loop ;; label = @6
                  local.get 1
                  i32.const 65535
                  i32.and
                  local.get 11
                  i32.const 65535
                  i32.and
                  i32.ge_u
                  br_if 2 (;@4;)
                  i32.const 1
                  local.set 12
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 10
                  local.get 6
                  local.get 9
                  i32.load offset=16
                  call_indirect (type 2)
                  i32.eqz
                  br_if 0 (;@6;)
                  br 5 (;@1;)
                end
              end
              local.get 0
              local.get 0
              i64.load offset=8 align=4
              local.tee 14
              i32.wrap_i64
              i32.const -1612709888
              i32.and
              i32.const 536870960
              i32.or
              i32.store offset=8
              i32.const 1
              local.set 12
              local.get 0
              i32.load
              local.tee 10
              local.get 0
              i32.load offset=4
              local.tee 9
              local.get 8
              local.get 2
              local.get 3
              i32.const 14560
              local.set 15
              local.get 15
              local.get 15
              i32.load 2 offset=28 align=1
              i32.const 1
              i32.add
              i32.store 2 offset=28 align=1
              call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
              br_if 3 (;@1;)
              i32.const 0
              local.set 1
              local.get 11
              local.get 6
              i32.sub
              i32.const 65535
              i32.and
              local.set 2
              loop ;; label = @5
                local.get 1
                i32.const 65535
                i32.and
                local.get 2
                i32.ge_u
                br_if 2 (;@3;)
                i32.const 1
                local.set 12
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                local.get 10
                i32.const 48
                local.get 9
                i32.load offset=16
                call_indirect (type 2)
                i32.eqz
                br_if 0 (;@5;)
                br 4 (;@1;)
              end
            end
            i32.const 1
            local.set 12
            local.get 10
            local.get 9
            local.get 8
            local.get 2
            local.get 3
            i32.const 14592
            local.set 15
            local.get 15
            local.get 15
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
            br_if 2 (;@1;)
            local.get 10
            local.get 4
            local.get 5
            local.get 9
            i32.load offset=12
            call_indirect (type 5)
            br_if 2 (;@1;)
            i32.const 0
            local.set 1
            local.get 13
            local.get 11
            i32.sub
            i32.const 65535
            i32.and
            local.set 0
            loop ;; label = @4
              local.get 1
              i32.const 65535
              i32.and
              local.tee 2
              local.get 0
              i32.lt_u
              local.set 12
              local.get 2
              local.get 0
              i32.ge_u
              br_if 3 (;@1;)
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 10
              local.get 6
              local.get 9
              i32.load offset=16
              call_indirect (type 2)
              i32.eqz
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          i32.const 1
          local.set 12
          local.get 10
          local.get 4
          local.get 5
          local.get 9
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 0
          local.get 14
          i64.store offset=8 align=4
          i32.const 0
          return
        end
        i32.const 1
        local.set 12
        local.get 0
        i32.load
        local.tee 1
        local.get 0
        i32.load offset=4
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        i32.const 14624
        local.set 15
        local.get 15
        local.get 15
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
        br_if 0 (;@1;)
        local.get 1
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 5)
        local.set 12
      end
      local.get 12
    )
    (func $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E (;183;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 0
          i32.const 3
          i32.add
          i32.const -4
          i32.and
          local.tee 2
          local.get 0
          i32.sub
          local.tee 3
          i32.lt_u
          br_if 0 (;@2;)
          local.get 1
          local.get 3
          i32.sub
          local.tee 4
          i32.const 4
          i32.lt_u
          br_if 0 (;@2;)
          local.get 4
          i32.const 3
          i32.and
          local.set 5
          i32.const 0
          local.set 6
          i32.const 0
          local.set 1
          block ;; label = @3
            local.get 2
            local.get 0
            i32.eq
            local.tee 7
            br_if 0 (;@3;)
            i32.const 0
            local.set 1
            block ;; label = @4
              block ;; label = @5
                local.get 0
                local.get 2
                i32.sub
                local.tee 8
                i32.const -4
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.set 9
                br 1 (;@4;)
              end
              i32.const 0
              local.set 9
              loop ;; label = @5
                local.get 1
                local.get 0
                local.get 9
                i32.add
                local.tee 2
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set 1
                local.get 9
                i32.const 4
                i32.add
                local.tee 9
                br_if 0 (;@5;)
              end
            end
            local.get 7
            br_if 0 (;@3;)
            local.get 0
            local.get 9
            i32.add
            local.set 2
            loop ;; label = @4
              local.get 1
              local.get 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 2
              i32.const 1
              i32.add
              local.set 2
              local.get 8
              i32.const 1
              i32.add
              local.tee 8
              br_if 0 (;@4;)
            end
          end
          local.get 0
          local.get 3
          i32.add
          local.set 0
          block ;; label = @3
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 4
            i32.const -4
            i32.and
            i32.add
            local.tee 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            local.set 6
            local.get 5
            i32.const 1
            i32.eq
            br_if 0 (;@3;)
            local.get 6
            local.get 2
            i32.load8_s offset=1
            i32.const -65
            i32.gt_s
            i32.add
            local.set 6
            local.get 5
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            local.get 6
            local.get 2
            i32.load8_s offset=2
            i32.const -65
            i32.gt_s
            i32.add
            local.set 6
          end
          local.get 4
          i32.const 2
          i32.shr_u
          local.set 8
          local.get 6
          local.get 1
          i32.add
          local.set 3
          loop ;; label = @3
            local.get 0
            local.set 4
            local.get 8
            i32.eqz
            br_if 2 (;@1;)
            local.get 8
            i32.const 192
            local.get 8
            i32.const 192
            i32.lt_u
            select
            local.tee 6
            i32.const 3
            i32.and
            local.set 7
            local.get 6
            i32.const 2
            i32.shl
            local.set 5
            i32.const 0
            local.set 2
            block ;; label = @4
              local.get 8
              i32.const 4
              i32.lt_u
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              i32.const 1008
              i32.and
              i32.add
              local.set 9
              i32.const 0
              local.set 2
              local.get 4
              local.set 1
              loop ;; label = @5
                local.get 1
                i32.const 12
                i32.add
                i32.load
                local.tee 0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.const 8
                i32.add
                i32.load
                local.tee 0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.load
                local.tee 0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 2
                i32.add
                i32.add
                i32.add
                i32.add
                local.set 2
                local.get 1
                i32.const 16
                i32.add
                local.tee 1
                local.get 9
                i32.ne
                br_if 0 (;@5;)
              end
            end
            local.get 8
            local.get 6
            i32.sub
            local.set 8
            local.get 4
            local.get 5
            i32.add
            local.set 0
            local.get 2
            i32.const 8
            i32.shr_u
            i32.const 16711935
            i32.and
            local.get 2
            i32.const 16711935
            i32.and
            i32.add
            i32.const 65537
            i32.mul
            i32.const 16
            i32.shr_u
            local.get 3
            i32.add
            local.set 3
            local.get 7
            i32.eqz
            br_if 0 (;@3;)
          end
          local.get 4
          local.get 6
          i32.const 252
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.tee 2
          i32.load
          local.tee 1
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 1
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.set 1
          block ;; label = @3
            local.get 7
            i32.const 1
            i32.eq
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=4
            local.tee 0
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get 0
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get 1
            i32.add
            local.set 1
            local.get 7
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=8
            local.tee 2
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get 2
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get 1
            i32.add
            local.set 1
          end
          local.get 1
          i32.const 8
          i32.shr_u
          i32.const 459007
          i32.and
          local.get 1
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 3
          i32.add
          return
        end
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 1
        i32.const 3
        i32.and
        local.set 9
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 3
            i32.const 0
            local.set 2
            br 1 (;@2;)
          end
          local.get 1
          i32.const -4
          i32.and
          local.set 8
          i32.const 0
          local.set 3
          i32.const 0
          local.set 2
          loop ;; label = @3
            local.get 3
            local.get 0
            local.get 2
            i32.add
            local.tee 1
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 3
            local.get 8
            local.get 2
            i32.const 4
            i32.add
            local.tee 2
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 9
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 2
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 3
          local.get 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 3
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@2;)
        end
      end
      local.get 3
    )
    (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E (;184;) (type 2) (param i32 i32) (result i32)
      (local i32)
      global.get $GOT.data.internal.__memory_base
      local.set 2
      local.get 1
      i32.load
      local.get 2
      i32.const 1053348
      i32.add
      i32.const 14
      local.get 1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE (;185;) (type 1) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 1
      i32.store offset=12
      local.get 1
      i64.const 1
      i64.store offset=20 align=4
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055588
      i32.add
      i32.store offset=8
      local.get 1
      global.get $GOT.func.internal._ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 1
      i32.const 47
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 1
      local.get 1
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 1
      i32.const 8
      i32.add
      local.get 0
      i32.const 14656
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h04ddcd8be7687b8aE (;186;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.const 7
                i32.gt_u
                br_if 0 (;@5;)
                local.get 2
                i32.eqz
                br_if 3 (;@2;)
                local.get 1
                i32.load8_u
                br_if 1 (;@4;)
                i32.const 0
                local.set 3
                br 4 (;@1;)
              end
              block ;; label = @5
                block ;; label = @6
                  local.get 1
                  i32.const 3
                  i32.add
                  i32.const -4
                  i32.and
                  local.get 1
                  i32.sub
                  local.tee 4
                  i32.eqz
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 3
                  loop ;; label = @7
                    local.get 1
                    local.get 3
                    i32.add
                    i32.load8_u
                    i32.eqz
                    br_if 6 (;@1;)
                    local.get 4
                    local.get 3
                    i32.const 1
                    i32.add
                    local.tee 3
                    i32.ne
                    br_if 0 (;@7;)
                  end
                  local.get 4
                  local.get 2
                  i32.const -8
                  i32.add
                  local.tee 5
                  i32.le_u
                  br_if 1 (;@5;)
                  br 3 (;@3;)
                end
                local.get 2
                i32.const -8
                i32.add
                local.set 5
              end
              loop ;; label = @5
                i32.const 16843008
                local.get 1
                local.get 4
                i32.add
                local.tee 3
                i32.load
                local.tee 6
                i32.sub
                local.get 6
                i32.or
                i32.const 16843008
                local.get 3
                i32.const 4
                i32.add
                i32.load
                local.tee 3
                i32.sub
                local.get 3
                i32.or
                i32.and
                i32.const -2139062144
                i32.and
                i32.const -2139062144
                i32.ne
                br_if 2 (;@3;)
                local.get 4
                i32.const 8
                i32.add
                local.tee 4
                local.get 5
                i32.le_u
                br_if 0 (;@5;)
                br 2 (;@3;)
              end
            end
            i32.const 1
            local.set 3
            local.get 2
            i32.const 1
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=1
            i32.eqz
            br_if 2 (;@1;)
            i32.const 2
            local.set 3
            local.get 2
            i32.const 2
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=2
            i32.eqz
            br_if 2 (;@1;)
            i32.const 3
            local.set 3
            local.get 2
            i32.const 3
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 4
            local.set 3
            local.get 2
            i32.const 4
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=4
            i32.eqz
            br_if 2 (;@1;)
            i32.const 5
            local.set 3
            local.get 2
            i32.const 5
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=5
            i32.eqz
            br_if 2 (;@1;)
            i32.const 6
            local.set 3
            local.get 2
            i32.const 6
            i32.eq
            br_if 1 (;@2;)
            local.get 1
            i32.load8_u offset=6
            br_if 1 (;@2;)
            br 2 (;@1;)
          end
          local.get 2
          local.get 4
          i32.eq
          br_if 0 (;@2;)
          loop ;; label = @3
            block ;; label = @4
              local.get 1
              local.get 4
              i32.add
              i32.load8_u
              br_if 0 (;@4;)
              local.get 4
              local.set 3
              br 3 (;@1;)
            end
            local.get 2
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 0
        i32.const 1
        i32.store offset=4
        local.get 0
        i32.const 1
        i32.store
        return
      end
      block ;; label = @1
        local.get 3
        i32.const 1
        i32.add
        local.get 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        i32.store offset=8
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 1
        i32.store
        return
      end
      local.get 0
      local.get 2
      i32.store offset=8
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 0
      i32.store
    )
    (func $_ZN4core3str8converts9from_utf817hc11b0c33b11310b8E (;187;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i64 i64 i32)
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        i32.const 0
        local.get 2
        i32.const -7
        i32.add
        local.tee 3
        local.get 3
        local.get 2
        i32.gt_u
        select
        local.set 4
        local.get 1
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.get 1
        i32.sub
        local.set 5
        i32.const 0
        local.set 3
        loop ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 1
                  local.get 3
                  i32.add
                  i32.load8_u
                  local.tee 6
                  i32.extend8_s
                  local.tee 7
                  i32.const 0
                  i32.lt_s
                  br_if 0 (;@6;)
                  local.get 5
                  local.get 3
                  i32.sub
                  i32.const 3
                  i32.and
                  br_if 1 (;@5;)
                  local.get 3
                  local.get 4
                  i32.ge_u
                  br_if 2 (;@4;)
                  loop ;; label = @7
                    local.get 1
                    local.get 3
                    i32.add
                    local.tee 6
                    i32.const 4
                    i32.add
                    i32.load
                    local.get 6
                    i32.load
                    i32.or
                    i32.const -2139062144
                    i32.and
                    br_if 3 (;@4;)
                    local.get 3
                    i32.const 8
                    i32.add
                    local.tee 3
                    local.get 4
                    i32.lt_u
                    br_if 0 (;@7;)
                    br 3 (;@4;)
                  end
                end
                i64.const 1099511627776
                local.set 8
                i64.const 4294967296
                local.set 9
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      block ;; label = @17
                                        global.get $GOT.data.internal.__memory_base
                                        i32.const 1053773
                                        i32.add
                                        local.get 6
                                        i32.add
                                        i32.load8_u
                                        i32.const -2
                                        i32.add
                                        br_table 0 (;@17;) 1 (;@16;) 2 (;@15;) 10 (;@7;)
                                      end
                                      local.get 3
                                      i32.const 1
                                      i32.add
                                      local.tee 6
                                      local.get 2
                                      i32.lt_u
                                      br_if 2 (;@14;)
                                      i64.const 0
                                      local.set 8
                                      i64.const 0
                                      local.set 9
                                      br 9 (;@7;)
                                    end
                                    i64.const 0
                                    local.set 8
                                    local.get 3
                                    i32.const 1
                                    i32.add
                                    local.tee 10
                                    local.get 2
                                    i32.lt_u
                                    br_if 2 (;@13;)
                                    i64.const 0
                                    local.set 9
                                    br 8 (;@7;)
                                  end
                                  i64.const 0
                                  local.set 8
                                  local.get 3
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  local.get 2
                                  i32.lt_u
                                  br_if 2 (;@12;)
                                  i64.const 0
                                  local.set 9
                                  br 7 (;@7;)
                                end
                                i64.const 1099511627776
                                local.set 8
                                i64.const 4294967296
                                local.set 9
                                local.get 1
                                local.get 6
                                i32.add
                                i32.load8_s
                                i32.const -65
                                i32.gt_s
                                br_if 6 (;@7;)
                                br 7 (;@6;)
                              end
                              local.get 1
                              local.get 10
                              i32.add
                              i32.load8_s
                              local.set 10
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 6
                                    i32.const -224
                                    i32.add
                                    br_table 0 (;@15;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 1 (;@14;) 2 (;@13;)
                                  end
                                  local.get 10
                                  i32.const -32
                                  i32.and
                                  i32.const -96
                                  i32.eq
                                  br_if 4 (;@10;)
                                  br 3 (;@11;)
                                end
                                local.get 10
                                i32.const -97
                                i32.gt_s
                                br_if 2 (;@11;)
                                br 3 (;@10;)
                              end
                              block ;; label = @13
                                local.get 7
                                i32.const 31
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 12
                                i32.lt_u
                                br_if 0 (;@13;)
                                local.get 7
                                i32.const -2
                                i32.and
                                i32.const -18
                                i32.ne
                                br_if 2 (;@11;)
                                local.get 10
                                i32.const -64
                                i32.lt_s
                                br_if 3 (;@10;)
                                br 2 (;@11;)
                              end
                              local.get 10
                              i32.const -64
                              i32.lt_s
                              br_if 2 (;@10;)
                              br 1 (;@11;)
                            end
                            local.get 1
                            local.get 10
                            i32.add
                            i32.load8_s
                            local.set 10
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 6
                                    i32.const -240
                                    i32.add
                                    br_table 1 (;@14;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 2 (;@13;) 0 (;@15;)
                                  end
                                  local.get 7
                                  i32.const 15
                                  i32.add
                                  i32.const 255
                                  i32.and
                                  i32.const 2
                                  i32.gt_u
                                  br_if 3 (;@11;)
                                  local.get 10
                                  i32.const -64
                                  i32.ge_s
                                  br_if 3 (;@11;)
                                  br 2 (;@12;)
                                end
                                local.get 10
                                i32.const 112
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 48
                                i32.ge_u
                                br_if 2 (;@11;)
                                br 1 (;@12;)
                              end
                              local.get 10
                              i32.const -113
                              i32.gt_s
                              br_if 1 (;@11;)
                            end
                            block ;; label = @12
                              local.get 3
                              i32.const 2
                              i32.add
                              local.tee 6
                              local.get 2
                              i32.lt_u
                              br_if 0 (;@12;)
                              i64.const 0
                              local.set 9
                              br 5 (;@7;)
                            end
                            local.get 1
                            local.get 6
                            i32.add
                            i32.load8_s
                            i32.const -65
                            i32.gt_s
                            br_if 2 (;@9;)
                            i64.const 0
                            local.set 9
                            local.get 3
                            i32.const 3
                            i32.add
                            local.tee 6
                            local.get 2
                            i32.ge_u
                            br_if 4 (;@7;)
                            local.get 1
                            local.get 6
                            i32.add
                            i32.load8_s
                            i32.const -64
                            i32.lt_s
                            br_if 5 (;@6;)
                            i64.const 3298534883328
                            local.set 8
                            br 3 (;@8;)
                          end
                          i64.const 1099511627776
                          local.set 8
                          br 2 (;@8;)
                        end
                        i64.const 0
                        local.set 9
                        local.get 3
                        i32.const 2
                        i32.add
                        local.tee 6
                        local.get 2
                        i32.ge_u
                        br_if 2 (;@7;)
                        local.get 1
                        local.get 6
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.le_s
                        br_if 3 (;@6;)
                      end
                      i64.const 2199023255552
                      local.set 8
                    end
                    i64.const 4294967296
                    local.set 9
                  end
                  local.get 0
                  local.get 8
                  local.get 3
                  i64.extend_i32_u
                  i64.or
                  local.get 9
                  i64.or
                  i64.store offset=4 align=4
                  local.get 0
                  i32.const 1
                  i32.store
                  return
                end
                local.get 6
                i32.const 1
                i32.add
                local.set 3
                br 2 (;@3;)
              end
              local.get 3
              i32.const 1
              i32.add
              local.set 3
              br 1 (;@3;)
            end
            local.get 3
            local.get 2
            i32.ge_u
            br_if 0 (;@3;)
            loop ;; label = @4
              local.get 1
              local.get 3
              i32.add
              i32.load8_s
              i32.const 0
              i32.lt_s
              br_if 1 (;@3;)
              local.get 2
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.ne
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          local.get 3
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
      local.get 2
      i32.store offset=8
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 0
      i32.store
    )
    (func $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE (;188;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      i32.const 1
      local.set 6
      block ;; label = @1
        local.get 0
        i32.load8_u offset=4
        br_if 0 (;@1;)
        local.get 0
        i32.load8_u offset=5
        local.set 7
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 8
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@2;)
          i32.const 1
          local.set 6
          global.get $GOT.data.internal.__memory_base
          local.set 9
          local.get 8
          i32.load
          local.get 9
          i32.const 1053516
          i32.add
          local.get 9
          i32.const 1053513
          i32.add
          local.get 7
          i32.const 1
          i32.and
          local.tee 7
          select
          i32.const 2
          i32.const 3
          local.get 7
          select
          local.get 8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 8
          i32.load
          local.get 1
          local.get 2
          local.get 8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          global.get $GOT.data.internal.__memory_base
          local.set 2
          local.get 8
          i32.load
          local.get 2
          i32.const 1053507
          i32.add
          i32.const 2
          local.get 8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 3
          local.get 8
          local.get 4
          i32.load offset=12
          call_indirect (type 2)
          local.set 6
          br 1 (;@1;)
        end
        i32.const 1
        local.set 6
        block ;; label = @2
          local.get 7
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          local.set 7
          local.get 8
          i32.load
          local.get 7
          i32.const 1053518
          i32.add
          i32.const 3
          local.get 8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        i32.const 1
        local.set 6
        local.get 5
        i32.const 1
        i32.store8 offset=15
        local.get 5
        global.get $GOT.data.internal.__memory_base
        i32.const 1055668
        i32.add
        i32.store offset=20
        local.get 5
        local.get 8
        i64.load align=4
        i64.store align=4
        local.get 5
        local.get 8
        i64.load offset=8 align=4
        i64.store offset=24 align=4
        local.get 5
        local.get 5
        i32.const 15
        i32.add
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 1
        local.get 2
        i32.const 14688
        local.set 10
        local.get 10
        local.get 10
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E
        br_if 0 (;@1;)
        local.get 5
        global.get $GOT.data.internal.__memory_base
        i32.const 1053507
        i32.add
        i32.const 2
        i32.const 14720
        local.set 10
        local.get 10
        local.get 10
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E
        br_if 0 (;@1;)
        local.get 3
        local.get 5
        i32.const 16
        i32.add
        local.get 4
        i32.load offset=12
        call_indirect (type 2)
        br_if 0 (;@1;)
        global.get $GOT.data.internal.__memory_base
        local.set 6
        local.get 5
        i32.load offset=16
        local.get 6
        i32.const 1053521
        i32.add
        i32.const 2
        local.get 5
        i32.load offset=20
        i32.load offset=12
        call_indirect (type 5)
        local.set 6
      end
      local.get 0
      i32.const 1
      i32.store8 offset=5
      local.get 0
      local.get 6
      i32.store8 offset=4
      local.get 5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E (;189;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 3
      local.set 3
      local.get 0
      i32.load8_u
      local.tee 0
      local.set 4
      block ;; label = @1
        local.get 0
        i32.const 10
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 1
        local.set 3
        local.get 2
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get 0
        local.get 0
        i32.const 100
        i32.div_u
        local.tee 4
        i32.const 100
        i32.mul
        i32.sub
        i32.const 255
        i32.and
        i32.const 1
        i32.shl
        i32.add
        i32.load16_u align=1
        i32.store16 offset=14 align=1
      end
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.eqz
          br_if 0 (;@2;)
          local.get 4
          i32.eqz
          br_if 1 (;@1;)
        end
        local.get 2
        i32.const 13
        i32.add
        local.get 3
        i32.const -1
        i32.add
        local.tee 3
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get 4
        i32.const 1
        i32.shl
        i32.const 254
        i32.and
        i32.add
        i32.const 1
        i32.add
        i32.load8_u
        i32.store8
      end
      local.get 1
      i32.const 1
      i32.const 1
      i32.const 0
      local.get 2
      i32.const 13
      i32.add
      local.get 3
      i32.add
      i32.const 3
      local.get 3
      i32.sub
      i32.const 14752
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE (;190;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 1
      i32.store offset=12
      local.get 5
      local.get 0
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 2
      i32.store offset=16
      local.get 5
      i32.const 2
      i32.store offset=28
      local.get 5
      global.get $GOT.data.internal.__memory_base
      i32.const 1055652
      i32.add
      i32.store offset=24
      local.get 5
      i64.const 2
      i64.store offset=36 align=4
      local.get 5
      global.get $GOT.data.internal.__table_base
      local.tee 1
      i32.const 63
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 5
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get 5
      local.get 1
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 5
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=48
      local.get 5
      local.get 5
      i32.const 48
      i32.add
      i32.store offset=32
      local.get 5
      i32.const 24
      i32.add
      local.get 4
      i32.const 14784
      local.set 6
      local.get 6
      local.get 6
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E (;191;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      i32.load
      i32.const 1
      local.get 1
      i32.const 14816
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE
    )
    (func $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE (;192;) (type 1) (param i32)
      (local i32)
      global.get $GOT.data.internal.__memory_base
      i32.const 1053380
      i32.add
      i32.const 43
      local.get 0
      i32.const 14848
      local.set 1
      local.get 1
      local.get 1
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking5panic17hd836709591dfc35fE
      unreachable
    )
    (func $_ZN4core6option13expect_failed17he15179d1cacc214eE (;193;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 3
      local.get 0
      i32.store offset=8
      local.get 3
      i32.const 1
      i32.store offset=20
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1053340
      i32.add
      i32.store offset=16
      local.get 3
      i64.const 1
      i64.store offset=28 align=4
      local.get 3
      global.get $GOT.data.internal.__table_base
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 3
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 3
      local.get 3
      i32.const 40
      i32.add
      i32.store offset=24
      local.get 3
      i32.const 16
      i32.add
      local.get 2
      i32.const 14880
      local.set 4
      local.get 4
      local.get 4
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E (;194;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      i32.const 14912
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE (;195;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      i32.const 0
      local.set 3
      loop ;; label = @1
        local.get 2
        local.get 3
        i32.add
        i32.const 127
        i32.add
        local.get 0
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 87
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 3
        i32.const -1
        i32.add
        local.set 3
        local.get 0
        i32.const 15
        i32.gt_u
        local.set 4
        local.get 0
        i32.const 4
        i32.shr_u
        local.set 0
        local.get 4
        br_if 0 (;@1;)
      end
      local.get 1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get 2
      local.get 3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 3
      i32.sub
      i32.const 14944
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E (;196;) (type 14) (param i32 i32 i32 i32 i32 i32 i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 7
      global.set $__stack_pointer
      local.get 7
      local.get 2
      i32.store offset=12
      local.get 7
      local.get 1
      i32.store offset=8
      local.get 7
      local.get 4
      i32.store offset=20
      local.get 7
      local.get 3
      i32.store offset=16
      local.get 7
      global.get $GOT.data.internal.__memory_base
      local.tee 2
      i32.const 1054100
      i32.add
      local.get 0
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      local.tee 1
      i32.add
      i32.load
      i32.store offset=28
      local.get 7
      local.get 2
      i32.const 1055756
      i32.add
      local.get 1
      i32.add
      i32.load
      i32.store offset=24
      block ;; label = @1
        local.get 5
        i32.load
        i32.eqz
        br_if 0 (;@1;)
        local.get 7
        i32.const 32
        i32.add
        i32.const 16
        i32.add
        local.get 5
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get 7
        i32.const 32
        i32.add
        i32.const 8
        i32.add
        local.get 5
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get 7
        local.get 5
        i64.load align=4
        i64.store offset=32
        local.get 7
        i32.const 4
        i32.store offset=92
        local.get 7
        global.get $GOT.data.internal.__memory_base
        i32.const 1055620
        i32.add
        i32.store offset=88
        local.get 7
        i64.const 4
        i64.store offset=100 align=4
        local.get 7
        global.get $GOT.data.internal.__table_base
        local.tee 5
        i32.const 63
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.tee 8
        local.get 7
        i32.const 16
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=80
        local.get 7
        local.get 8
        local.get 7
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=72
        local.get 7
        global.get $GOT.func.internal._ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 7
        i32.const 32
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=64
        local.get 7
        local.get 5
        i32.const 64
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 7
        i32.const 24
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=56
        local.get 7
        local.get 7
        i32.const 56
        i32.add
        i32.store offset=96
        local.get 7
        i32.const 88
        i32.add
        local.get 6
        i32.const 14976
        local.set 9
        local.get 9
        local.get 9
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
        unreachable
      end
      local.get 7
      i32.const 3
      i32.store offset=92
      local.get 7
      i64.const 3
      i64.store offset=100 align=4
      local.get 7
      global.get $GOT.data.internal.__memory_base
      i32.const 1055596
      i32.add
      i32.store offset=88
      local.get 7
      global.get $GOT.data.internal.__table_base
      local.tee 5
      i32.const 63
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 8
      local.get 7
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=72
      local.get 7
      local.get 8
      local.get 7
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=64
      local.get 7
      local.get 5
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 7
      i32.const 24
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get 7
      local.get 7
      i32.const 56
      i32.add
      i32.store offset=96
      local.get 7
      i32.const 88
      i32.add
      local.get 6
      i32.const 15008
      local.set 9
      local.get 9
      local.get 9
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E (;197;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 2)
    )
    (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE (;198;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 1
      i32.load
      local.get 1
      i32.load offset=4
      local.get 0
      i32.const 15040
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E (;199;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      local.get 1
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.load offset=4
      local.set 4
      local.get 0
      i32.load
      local.set 5
      local.get 0
      i32.load offset=8
      local.set 6
      i32.const 0
      local.set 7
      i32.const 0
      local.set 8
      i32.const 0
      local.set 9
      i32.const 0
      local.set 10
      block ;; label = @1
        loop ;; label = @2
          local.get 10
          i32.const 1
          i32.and
          br_if 1 (;@1;)
          block ;; label = @3
            block ;; label = @4
              local.get 2
              local.get 9
              i32.lt_u
              br_if 0 (;@4;)
              loop ;; label = @5
                local.get 1
                local.get 9
                i32.add
                local.set 10
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        local.get 2
                        local.get 9
                        i32.sub
                        local.tee 11
                        i32.const 7
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 2
                        local.get 9
                        i32.ne
                        br_if 1 (;@8;)
                        local.get 2
                        local.set 9
                        br 5 (;@4;)
                      end
                      block ;; label = @9
                        block ;; label = @10
                          local.get 10
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 12
                          local.get 10
                          i32.sub
                          local.tee 13
                          i32.eqz
                          br_if 0 (;@10;)
                          i32.const 0
                          local.set 0
                          loop ;; label = @11
                            local.get 10
                            local.get 0
                            i32.add
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 5 (;@6;)
                            local.get 13
                            local.get 0
                            i32.const 1
                            i32.add
                            local.tee 0
                            i32.ne
                            br_if 0 (;@11;)
                          end
                          local.get 13
                          local.get 11
                          i32.const -8
                          i32.add
                          local.tee 14
                          i32.le_u
                          br_if 1 (;@9;)
                          br 3 (;@7;)
                        end
                        local.get 11
                        i32.const -8
                        i32.add
                        local.set 14
                      end
                      loop ;; label = @9
                        i32.const 16843008
                        local.get 12
                        i32.load
                        local.tee 0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get 0
                        i32.or
                        i32.const 16843008
                        local.get 12
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get 0
                        i32.or
                        i32.and
                        i32.const -2139062144
                        i32.and
                        i32.const -2139062144
                        i32.ne
                        br_if 2 (;@7;)
                        local.get 12
                        i32.const 8
                        i32.add
                        local.set 12
                        local.get 13
                        i32.const 8
                        i32.add
                        local.tee 13
                        local.get 14
                        i32.le_u
                        br_if 0 (;@9;)
                        br 2 (;@7;)
                      end
                    end
                    i32.const 0
                    local.set 0
                    loop ;; label = @8
                      local.get 10
                      local.get 0
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.eq
                      br_if 2 (;@6;)
                      local.get 11
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 2
                    local.set 9
                    br 3 (;@4;)
                  end
                  block ;; label = @7
                    local.get 11
                    local.get 13
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 2
                    local.set 9
                    br 3 (;@4;)
                  end
                  loop ;; label = @7
                    block ;; label = @8
                      local.get 10
                      local.get 13
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 13
                      local.set 0
                      br 2 (;@6;)
                    end
                    local.get 11
                    local.get 13
                    i32.const 1
                    i32.add
                    local.tee 13
                    i32.ne
                    br_if 0 (;@7;)
                  end
                  local.get 2
                  local.set 9
                  br 2 (;@4;)
                end
                local.get 0
                local.get 9
                i32.add
                local.tee 13
                i32.const 1
                i32.add
                local.set 9
                block ;; label = @6
                  local.get 13
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 10
                  local.get 0
                  i32.add
                  i32.load8_u
                  i32.const 10
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 10
                  local.get 9
                  local.set 12
                  local.get 9
                  local.set 0
                  br 3 (;@3;)
                end
                local.get 9
                local.get 2
                i32.le_u
                br_if 0 (;@5;)
              end
            end
            local.get 2
            local.get 8
            i32.eq
            br_if 2 (;@1;)
            i32.const 1
            local.set 10
            local.get 8
            local.set 12
            local.get 2
            local.set 0
          end
          block ;; label = @3
            block ;; label = @4
              local.get 6
              i32.load8_u
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              global.get $GOT.data.internal.__memory_base
              i32.const 1053509
              i32.add
              i32.const 4
              local.get 4
              i32.load offset=12
              call_indirect (type 5)
              br_if 1 (;@3;)
            end
            local.get 0
            local.get 8
            i32.sub
            local.set 11
            i32.const 0
            local.set 13
            block ;; label = @4
              local.get 0
              local.get 8
              i32.eq
              br_if 0 (;@4;)
              local.get 3
              local.get 0
              i32.add
              i32.load8_u
              i32.const 10
              i32.eq
              local.set 13
            end
            local.get 1
            local.get 8
            i32.add
            local.set 0
            local.get 6
            local.get 13
            i32.store8
            local.get 12
            local.set 8
            local.get 5
            local.get 0
            local.get 11
            local.get 4
            i32.load offset=12
            call_indirect (type 5)
            i32.eqz
            br_if 1 (;@2;)
          end
        end
        i32.const 1
        local.set 7
      end
      local.get 7
    )
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE (;200;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      i32.load offset=4
      local.set 2
      local.get 0
      i32.load
      local.set 3
      block ;; label = @1
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load8_u
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        global.get $GOT.data.internal.__memory_base
        i32.const 1053509
        i32.add
        i32.const 4
        local.get 2
        i32.load offset=12
        call_indirect (type 5)
        i32.eqz
        br_if 0 (;@1;)
        i32.const 1
        return
      end
      local.get 0
      local.get 1
      i32.const 10
      i32.eq
      i32.store8
      local.get 3
      local.get 1
      local.get 2
      i32.load offset=16
      call_indirect (type 2)
    )
    (func $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E (;201;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055668
      i32.add
      local.get 1
      i32.const 15072
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E (;202;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      block ;; label = @1
        local.get 2
        i32.const 1114112
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 2
        local.get 1
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@1;)
        i32.const 1
        return
      end
      block ;; label = @1
        local.get 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 0
      local.get 3
      local.get 4
      local.get 1
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E (;203;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E (;204;) (type 15) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 11
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 5)
      local.set 2
      local.get 11
      i32.const 0
      i32.store8 offset=13
      local.get 11
      local.get 2
      i32.store8 offset=12
      local.get 11
      local.get 0
      i32.store offset=8
      local.get 11
      i32.const 8
      i32.add
      local.get 3
      local.get 4
      local.get 5
      local.get 6
      i32.const 15104
      local.set 12
      local.get 12
      local.get 12
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE
      local.get 7
      local.get 8
      local.get 9
      local.get 10
      i32.const 15136
      local.set 12
      local.get 12
      local.get 12
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE
      local.set 10
      local.get 11
      i32.load8_u offset=13
      local.tee 2
      local.get 11
      i32.load8_u offset=12
      local.tee 1
      i32.or
      local.set 0
      block ;; label = @1
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 1
        i32.const 1
        i32.and
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 10
          i32.load
          local.tee 0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          local.set 2
          local.get 0
          i32.load
          local.get 2
          i32.const 1053524
          i32.add
          i32.const 2
          local.get 0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          local.set 0
          br 1 (;@1;)
        end
        global.get $GOT.data.internal.__memory_base
        local.set 2
        local.get 0
        i32.load
        local.get 2
        i32.const 1053523
        i32.add
        i32.const 1
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 5)
        local.set 0
      end
      local.get 11
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
      i32.const 1
      i32.and
    )
    (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E (;205;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      i32.const 1
      local.set 6
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 7
        local.get 1
        local.get 2
        local.get 0
        i32.load offset=4
        local.tee 8
        i32.load offset=12
        local.tee 9
        call_indirect (type 5)
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u offset=10
            i32.const 128
            i32.and
            br_if 0 (;@3;)
            i32.const 1
            local.set 6
            local.get 7
            global.get $GOT.data.internal.__memory_base
            i32.const 1053526
            i32.add
            i32.const 1
            local.get 9
            call_indirect (type 5)
            br_if 2 (;@1;)
            local.get 3
            local.get 0
            local.get 4
            i32.load offset=12
            call_indirect (type 2)
            i32.eqz
            br_if 1 (;@2;)
            br 2 (;@1;)
          end
          local.get 7
          global.get $GOT.data.internal.__memory_base
          i32.const 1053527
          i32.add
          i32.const 2
          local.get 9
          call_indirect (type 5)
          br_if 1 (;@1;)
          i32.const 1
          local.set 6
          local.get 5
          i32.const 1
          i32.store8 offset=15
          local.get 5
          local.get 8
          i32.store offset=4
          local.get 5
          local.get 7
          i32.store
          local.get 5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055668
          i32.add
          i32.store offset=20
          local.get 5
          local.get 0
          i64.load offset=8 align=4
          i64.store offset=24 align=4
          local.get 5
          local.get 5
          i32.const 15
          i32.add
          i32.store offset=8
          local.get 5
          local.get 5
          i32.store offset=16
          local.get 3
          local.get 5
          i32.const 16
          i32.add
          local.get 4
          i32.load offset=12
          call_indirect (type 2)
          br_if 1 (;@1;)
          global.get $GOT.data.internal.__memory_base
          local.set 1
          local.get 5
          i32.load offset=16
          local.get 1
          i32.const 1053521
          i32.add
          i32.const 2
          local.get 5
          i32.load offset=20
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        block ;; label = @2
          local.get 2
          br_if 0 (;@2;)
          local.get 0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@2;)
          global.get $GOT.data.internal.__memory_base
          local.set 2
          i32.const 1
          local.set 6
          local.get 0
          i32.load
          local.get 2
          i32.const 1053529
          i32.add
          i32.const 1
          local.get 0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        global.get $GOT.data.internal.__memory_base
        local.set 6
        local.get 0
        i32.load
        local.get 6
        i32.const 1053337
        i32.add
        i32.const 1
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 5)
        local.set 6
      end
      local.get 5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 6
    )
    (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E (;206;) (type 2) (param i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load8_u
        br_if 0 (;@1;)
        local.get 1
        global.get $GOT.data.internal.__memory_base
        i32.const 1053732
        i32.add
        i32.const 5
        i32.const 15168
        local.set 2
        local.get 2
        local.get 2
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
        call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
        return
      end
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053737
      i32.add
      i32.const 4
      i32.const 15200
      local.set 2
      local.get 2
      local.get 2
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE (;207;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      local.get 2
      local.get 0
      local.get 1
      i32.const 15232
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core5slice6memchr14memchr_aligned17hf4db372f52bc45e4E (;208;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.tee 4
              local.get 2
              i32.eq
              br_if 0 (;@4;)
              local.get 3
              local.get 4
              local.get 2
              i32.sub
              local.tee 4
              local.get 3
              local.get 4
              i32.lt_u
              select
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              i32.const 0
              local.set 5
              local.get 1
              i32.const 255
              i32.and
              local.set 6
              i32.const 1
              local.set 7
              loop ;; label = @5
                local.get 2
                local.get 5
                i32.add
                i32.load8_u
                local.get 6
                i32.eq
                br_if 4 (;@1;)
                local.get 4
                local.get 5
                i32.const 1
                i32.add
                local.tee 5
                i32.ne
                br_if 0 (;@5;)
              end
              local.get 4
              local.get 3
              i32.const -8
              i32.add
              local.tee 8
              i32.gt_u
              br_if 2 (;@2;)
              br 1 (;@3;)
            end
            local.get 3
            i32.const -8
            i32.add
            local.set 8
            i32.const 0
            local.set 4
          end
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 5
          loop ;; label = @3
            i32.const 16843008
            local.get 2
            local.get 4
            i32.add
            local.tee 6
            i32.load
            local.get 5
            i32.xor
            local.tee 7
            i32.sub
            local.get 7
            i32.or
            i32.const 16843008
            local.get 6
            i32.const 4
            i32.add
            i32.load
            local.get 5
            i32.xor
            local.tee 6
            i32.sub
            local.get 6
            i32.or
            i32.and
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.ne
            br_if 1 (;@2;)
            local.get 4
            i32.const 8
            i32.add
            local.tee 4
            local.get 8
            i32.le_u
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 3
          local.get 4
          i32.eq
          br_if 0 (;@2;)
          local.get 1
          i32.const 255
          i32.and
          local.set 5
          i32.const 1
          local.set 7
          loop ;; label = @3
            block ;; label = @4
              local.get 2
              local.get 4
              i32.add
              i32.load8_u
              local.get 5
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.set 5
              br 3 (;@1;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.ne
            br_if 0 (;@3;)
          end
        end
        i32.const 0
        local.set 7
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      local.get 7
      i32.store
    )
    (func $_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E (;209;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      local.get 3
      i32.const 0
      local.get 3
      local.get 2
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get 2
      i32.sub
      local.tee 4
      i32.sub
      i32.const 7
      i32.and
      local.get 3
      local.get 4
      i32.lt_u
      select
      local.tee 5
      i32.sub
      local.set 6
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              local.get 5
              i32.lt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 5
                i32.eqz
                br_if 0 (;@5;)
                block ;; label = @6
                  block ;; label = @7
                    local.get 2
                    local.get 3
                    i32.add
                    local.tee 7
                    i32.const -1
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -1
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 2
                  local.get 6
                  i32.add
                  local.tee 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -2
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -2
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -3
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -3
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -4
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -4
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -5
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -5
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -6
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -6
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  block ;; label = @7
                    local.get 7
                    i32.const -7
                    i32.add
                    local.tee 8
                    i32.load8_u
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const -7
                    i32.add
                    local.set 5
                    br 1 (;@6;)
                  end
                  local.get 9
                  local.get 8
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 5
                  i32.const -8
                  i32.or
                  local.set 5
                end
                local.get 5
                local.get 6
                i32.add
                local.set 5
                br 3 (;@2;)
              end
              local.get 4
              local.get 3
              local.get 3
              local.get 4
              i32.gt_u
              select
              local.set 9
              local.get 1
              i32.const 255
              i32.and
              i32.const 16843009
              i32.mul
              local.set 4
              block ;; label = @5
                loop ;; label = @6
                  local.get 6
                  local.tee 5
                  local.get 9
                  i32.le_u
                  br_if 1 (;@5;)
                  local.get 5
                  i32.const -8
                  i32.add
                  local.set 6
                  i32.const 16843008
                  local.get 2
                  local.get 5
                  i32.add
                  local.tee 8
                  i32.const -8
                  i32.add
                  i32.load
                  local.get 4
                  i32.xor
                  local.tee 7
                  i32.sub
                  local.get 7
                  i32.or
                  i32.const 16843008
                  local.get 8
                  i32.const -4
                  i32.add
                  i32.load
                  local.get 4
                  i32.xor
                  local.tee 8
                  i32.sub
                  local.get 8
                  i32.or
                  i32.and
                  i32.const -2139062144
                  i32.and
                  i32.const -2139062144
                  i32.eq
                  br_if 0 (;@6;)
                end
              end
              local.get 5
              local.get 3
              i32.gt_u
              br_if 1 (;@3;)
              local.get 2
              i32.const -1
              i32.add
              local.set 4
              local.get 1
              i32.const 255
              i32.and
              local.set 8
              loop ;; label = @5
                block ;; label = @6
                  local.get 5
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 5 (;@1;)
                end
                local.get 4
                local.get 5
                i32.add
                local.set 6
                local.get 5
                i32.const -1
                i32.add
                local.set 5
                local.get 6
                i32.load8_u
                local.get 8
                i32.eq
                br_if 3 (;@2;)
                br 0 (;@5;)
              end
            end
            local.get 6
            local.get 3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055692
            i32.add
            i32.const 15264
            local.set 10
            local.get 10
            local.get 10
            i32.load 2 offset=28 align=1
            i32.const 1
            i32.add
            i32.store 2 offset=28 align=1
            call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
            unreachable
          end
          local.get 5
          local.get 3
          global.get $GOT.data.internal.__memory_base
          i32.const 1055708
          i32.add
          i32.const 15296
          local.set 10
          local.get 10
          local.get 10
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
          call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
          unreachable
        end
        i32.const 1
        local.set 6
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      local.get 6
      i32.store
    )
    (func $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE (;210;) (type 4) (param i32 i32 i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      local.get 0
      i32.store
      local.get 3
      i32.const 2
      i32.store offset=12
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1055724
      i32.add
      i32.store offset=8
      local.get 3
      i64.const 2
      i64.store offset=20 align=4
      local.get 3
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 4
      local.get 3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 3
      local.get 4
      local.get 3
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      i32.const 15328
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE (;211;) (type 4) (param i32 i32 i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      local.get 0
      i32.store
      local.get 3
      i32.const 2
      i32.store offset=12
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1055740
      i32.add
      i32.store offset=8
      local.get 3
      i64.const 2
      i64.store offset=20 align=4
      local.get 3
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 4
      local.get 3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 3
      local.get 4
      local.get 3
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      i32.const 15360
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E (;212;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load8_u
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        local.get 3
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 55
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i32.const 255
        i32.and
        local.tee 4
        i32.const 4
        i32.shr_u
        local.set 3
        local.get 4
        i32.const 15
        i32.gt_u
        br_if 0 (;@1;)
      end
      local.get 1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      i32.const 15392
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E (;213;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load8_u
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        local.get 3
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 87
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i32.const 255
        i32.and
        local.tee 4
        i32.const 4
        i32.shr_u
        local.set 3
        local.get 4
        i32.const 15
        i32.gt_u
        br_if 0 (;@1;)
      end
      local.get 1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      i32.const 15424
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE (;214;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      i32.const 0
      local.set 3
      loop ;; label = @1
        local.get 2
        local.get 3
        i32.add
        i32.const 127
        i32.add
        local.get 0
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 55
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 3
        i32.const -1
        i32.add
        local.set 3
        local.get 0
        i32.const 15
        i32.gt_u
        local.set 4
        local.get 0
        i32.const 4
        i32.shr_u
        local.set 0
        local.get 4
        br_if 0 (;@1;)
      end
      local.get 1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get 2
      local.get 3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 3
      i32.sub
      i32.const 15456
      local.set 5
      local.get 5
      local.get 5
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE (;215;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      i32.load
      local.tee 0
      local.get 0
      i32.const 31
      i32.shr_s
      local.tee 2
      i32.xor
      local.get 2
      i32.sub
      local.get 0
      i32.const -1
      i32.xor
      i32.const 31
      i32.shr_u
      local.get 1
      i32.const 15488
      local.set 3
      local.get 3
      local.get 3
      i32.load 2 offset=28 align=1
      i32.const 1
      i32.add
      i32.store 2 offset=28 align=1
      call $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE
    )
    (func $puts_internal (;216;) (type 6) (param i32 i32)
      (local i32)
      loop ;; label = @1
        local.get 2
        local.get 1
        i32.lt_u
        i32.eqz
        br_if 1
        local.get 0
        local.get 2
        i32.add
        i32.load8_u 1
        call $putc
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        br 0 (;@1;)
      end
    )
    (func $on_exit (;217;) (type 0)
      call $flush_reports
    )
    (func $flush_var_metadata (;218;) (type 19) (param i64 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
      local.get 1
      i32.load 2 align=1
      local.set 2
      local.get 1
      i32.load8_u 2 offset=4
      local.set 3
      local.get 1
      i32.load 2 offset=5 align=1
      local.set 4
      local.get 1
      i32.load 2 offset=9 align=1
      local.set 5
      local.get 1
      i32.load 2 offset=13 align=1
      local.set 6
      local.get 1
      i32.load8_u 2 offset=17
      local.set 7
      local.get 1
      i32.load8_u 2 offset=18
      local.set 8
      local.get 1
      i32.load 2 offset=19 align=1
      local.set 9
      local.get 1
      i32.load8_u 2 offset=23
      local.set 10
      local.get 1
      call $puti32
      i32.const 9969
      i32.const 11
      call $puts_internal
      local.get 6
      local.get 7
      call $puts_internal
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 0
      i64.const -2005570450770458313
      i64.eq
      if ;; label = @1
        i32.const 9988
        i32.const 5
        call $puts_internal
        i32.const 9993
        i32.const 5
        call $puts_internal
      else
        unreachable
      end
      i32.const 9982
      i32.const 6
      call $puts_internal
      local.get 8
      call $puti32
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 2
      local.get 3
      call $puts_internal
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 4
      call $puti32
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 5
      call $puti32
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 9
      local.get 10
      call $puts_internal
      i32.const 9980
      i32.const 2
      call $puts_internal
      local.get 1
      i32.const 24
      i32.add
    )
    (func $flush_u32_vars (;219;) (type 0)
      (local i32 i32)
      block ;; label = @1
        global.get 19
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        global.get 19
        local.set 0
        loop ;; label = @2
          local.get 0
          i32.load 2 align=1
          local.tee 1
          i32.const -1
          i32.ne
          if ;; label = @3
            local.get 1
            i32.eqz
            if ;; label = @4
              unreachable
            end
            local.get 0
            local.get 1
            i32.add
            local.set 1
          end
          i32.const 4
          local.get 0
          i32.add
          local.set 0
          i64.const -2005570450770458313
          local.get 0
          call $flush_var_metadata
          local.tee 0
          i32.load 2 align=1
          call $putu32
          i32.const 10
          call $putc
          local.get 1
          i32.const -1
          i32.ne
          if ;; label = @3
            local.get 1
            local.set 0
            br 1 (;@2;)
          end
        end
      end
    )
    (func $flush_reports (;220;) (type 0)
      i32.const 9793
      i32.const 176
      call $puts_internal
      i32.const 10
      call $putc
      call $flush_u32_vars
    )
    (data $.rodata (;0;) (i32.const 1048576) "/Users/evgilber/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs world!\0ahello==== CALC ====\0a\0a==== PRINT ====\0acalc(, ) -> \0alibrary/std/src/panicking.rs: /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/vec/mod.rs/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/string.rs/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/raw_vec/mod.rsUtf8Errorvalid_up_toerror_lenNoneSome:/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/slice.rslibrary/std/src/rt.rslibrary/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhaustedmainRUST_BACKTRACEcalled `Result::unwrap()` on an `Err` valuefailed to write the buffered datalibrary/std/src/io/buffered/bufwriter.rs\01\00\00\00\00\00\00\00library/std/src/io/buffered/linewritershim.rsmid > lenfailed to write whole bufferentity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filequota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryin progressother erroruncategorized error (os error )library/std/src/io/stdio.rsfailed printing to stdoutlibrary/std/src/io/mod.rsa formatting trait implementation returned an error when the underlying stream did notadvancing io slices beyond their lengthadvancing IoSlice beyond its lengthlibrary/std/src/sys/io/io_slice/wasi.rspanicked at :\0acannot recursively acquire mutexlibrary/std/src/sys/sync/mutex/no_threads.rslibrary/std/src/sync/poison/once.rslock count overflow in reentrant mutexlibrary/std/src/sync/reentrant_lock.rsfile name contained an unexpected NUL bytestack backtrace:\0anote: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0amemory allocation of  bytes failed\0a bytes failedlibrary/std/src/alloc.rsnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a<unnamed>\0athread '' panicked at \0aBox<dyn Any>aborting due to panic at \0athread panicked while processing panic. aborting.\0athread caused non-unwinding panic. aborting.\0afatal runtime error: failed to initiate panic, error library/std/src/sys/pal/wasip2/../wasi/os.rsstrerror_r failureOnce instance has previously been poisonedone-time initialization may not be performed recursivelyfatal runtime error: rwlock locked for writing\0a\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\0e\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\0b\00\00\00\13\00\00\00/\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05capacity overflowlibrary/alloc/src/ffi/c_str.rs)\00\00\01\00\00\00\00\00\00\00BorrowMutErroralready borrowed: called `Option::unwrap()` on a `None` value==!=matchesassertion `left  right` failed\0a  left: \0a right:  right` failed: \0a  left: :      { ,  {\0a,\0a} }((\0a,0x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetruelibrary/core/src/slice/memchr.rs\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00range start index  out of range for slice of length range end index \00\00\00\02\00\00\00\02\00\00\00\07\00\00\00")
    (data $.data (;1;) (i32.const 1054112) "\00\00\00\00\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\05\00\00\00\00\00\10\00n\00\00\00\be\01\00\00\1d\00\00\00\01\00\00\00\00\00\00\00n\00\10\00\08\00\00\00{\00\10\00\0f\00\00\00\8a\00\10\00\11\00\00\00\9b\00\10\00\05\00\00\00\a0\00\10\00\02\00\00\00\a2\00\10\00\05\00\00\00\a7\00\10\00\01\00\00\00\0e\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\0f\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\c6\00\10\00L\00\00\00V\0a\00\00$\00\00\00\12\01\10\00K\00\00\00}\05\00\00\1b\00\00\00]\01\10\00P\00\00\00.\02\00\00\11\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\11\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\12\00\00\00\01\00\00\00\00\00\00\00\d2\01\10\00\01\00\00\00\d2\01\10\00\01\00\00\00\13\00\00\00\0c\00\00\00\04\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\1e\00\00\00\1f\00\00\00 \00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00!\00\00\00\22\00\00\00#\00\00\00\d3\01\10\00J\00\00\00\be\01\00\00\1d\00\00\00\1d\02\10\00\15\00\00\00\86\00\00\00\0d\00\00\00O\02\10\007\00\00\002\02\10\00\1d\00\00\00\a9\04\00\00\0d\00\00\00\c3\02\10\00!\00\00\00\17\00\00\00A\03\10\00\09\00\00\00\14\03\10\00-\00\00\00\16\01\00\00)\00\00\00J\03\10\00\1c\00\00\00\17\00\00\00\00\00\00\00\02\00\00\00h\17\10\00\01\00\00\00\00\00\00\00S\06\10\00\0b\00\00\00^\06\10\00\01\00\00\00\e4\02\10\00(\00\00\00z\00\00\00!\00\00\00_\06\10\00\1b\00\00\00\e3\02\00\00\13\00\00\00_\06\10\00\1b\00\00\00\5c\03\00\00\14\00\00\00z\06\10\00\13\00\00\00\c4\00\10\00\02\00\00\00_\06\10\00\1b\00\00\00\8d\04\00\00\09\00\00\00\ac\06\10\00V\00\00\00\93\06\10\00\19\00\00\00\88\02\00\00\11\00\00\00\93\06\10\00\19\00\00\00\08\06\00\00 \00\00\00\02\07\10\00'\00\00\00\93\06\10\00\19\00\00\00\0a\06\00\00\0d\00\00\00)\07\10\00#\00\00\00L\07\10\00'\00\00\00\14\00\00\00\0d\00\00\00\93\06\10\00\19\00\00\00\09\07\00\00$\00\00\00\81\07\10\00 \00\00\00\a1\07\10\00,\00\00\00\13\00\00\00\09\00\00\00\cd\07\10\00#\00\00\00\9b\00\00\002\00\00\00\cd\07\10\00#\00\00\00\d6\00\00\00\14\00\00\00\16\08\10\00&\00\00\00\1f\01\00\00-\00\00\00<\08\10\00*\00\00\00\14\00\00\00\00\00\00\00\02\00\00\00\98\18\10\00\cf\08\10\00\15\00\00\00\e4\08\10\00\0e\00\00\00\cf\08\10\00\15\00\00\00\f2\08\10\00\0d\00\00\00\ff\08\10\00\18\00\00\00d\01\00\00\09\00\00\00$\00\00\00\0c\00\00\00\04\00\00\00%\00\00\00&\00\00\00'\00\00\00(\00\00\00)\00\00\00*\00\00\00+\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00,\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\001\00\00\002\00\00\00\17\09\10\00N\00\00\00\a8\00\10\00\1c\00\00\00\1d\01\00\00.\00\00\00n\09\10\00\09\00\00\00w\09\10\00\0e\00\00\00\7f\07\10\00\02\00\00\00\85\09\10\00\01\00\00\00\01\00\00\00\13\00\00\00\0c\00\00\00\04\00\00\003\00\00\00\00\00\00\00\08\00\00\00\04\00\00\004\00\00\00\00\00\00\00\08\00\00\00\04\00\00\005\00\00\006\00\00\007\00\00\008\00\00\009\00\00\00\10\00\00\00\04\00\00\00:\00\00\00;\00\00\00<\00\00\00=\00\00\00\92\09\10\00\19\00\00\00\7f\07\10\00\02\00\00\00\85\09\10\00\01\00\00\00s\07\10\00\0c\00\00\00\7f\07\10\00\02\00\00\00\ab\09\10\003\00\00\00\de\09\10\00-\00\00\00\0b\0a\10\005\00\00\00\85\09\10\00\01\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00>\00\00\00@\0a\10\00,\00\00\00A\00\00\006\00\00\00l\0a\10\00\12\00\00\00@\0a\10\00,\00\00\00?\00\00\00\0d\00\00\00@\0a\10\00,\00\00\00F\00\00\00\13\00\00\00@\0a\10\00,\00\00\00M\00\00\00\15\00\00\00~\0a\10\00*\00\00\00\a8\0a\10\008\00\00\00\e0\0a\10\00/\00\00\00f\03\10\00v\03\10\00\87\03\10\00\99\03\10\00\a9\03\10\00\b9\03\10\00\cc\03\10\00\de\03\10\00\eb\03\10\00\f9\03\10\00\0e\04\10\00\1a\04\10\00%\04\10\00:\04\10\00O\04\10\00^\04\10\00l\04\10\00\7f\04\10\00\a5\04\10\00\dd\04\10\00\f6\04\10\00\0d\05\10\00\19\05\10\00\22\05\10\00,\05\10\00<\05\10\00S\05\10\00a\05\10\00o\05\10\00|\05\10\00\90\05\10\00\98\05\10\00\b3\05\10\00\c1\05\10\00\d1\05\10\00\e7\05\10\00\fc\05\10\00\07\06\10\00\1d\06\10\00*\06\10\005\06\10\00@\06\10\00\ff\ff\ff\ff\b8\0b\10\00j\12\10\00\11\00\00\00{\12\10\00\1e\00\00\00\1a\01\00\00\1e\00\00\00{\12\10\00\1e\00\00\00\16\01\00\007\00\00\00{\12\10\00\1e\00\00\00U\01\00\00\0b\00\00\00\b2\12\10\00\12\00\00\00\fa\12\10\00\10\00\00\00\0a\13\10\00\17\00\00\00!\13\10\00\09\00\00\00\fa\12\10\00\10\00\00\00*\13\10\00\10\00\00\00:\13\10\00\09\00\00\00!\13\10\00\09\00\00\00\01\00\00\00\00\00\00\00C\13\10\00\02\00\00\00\00\00\00\00\0c\00\00\00\04\00\00\00C\00\00\00D\00\00\00E\00\00\00-\14\10\00 \00\00\00\84\00\00\00\1e\00\00\00-\14\10\00 \00\00\00\a0\00\00\00\09\00\00\00M\15\10\00\12\00\00\00_\15\10\00\22\00\00\00\81\15\10\00\10\00\00\00_\15\10\00\22\00\00\00\ef\12\10\00\f1\12\10\00\f3\12\10\00")
    (data (;2;) (memory 1) (i32.const 0) "count")
    (data (;3;) (memory 1) (i32.const 5) "0_wasm:opcode:call:before")
    (data (;4;) (memory 1) (i32.const 30) "_start")
    (data (;5;) (memory 1) (i32.const 36) "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE")
    (data (;6;) (memory 1) (i32.const 108) "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E")
    (data (;7;) (memory 1) (i32.const 198) "_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE")
    (data (;8;) (memory 1) (i32.const 287) "print_x")
    (data (;9;) (memory 1) (i32.const 294) "opt_str")
    (data (;10;) (memory 1) (i32.const 301) "main")
    (data (;11;) (memory 1) (i32.const 305) "__main_void")
    (data (;12;) (memory 1) (i32.const 316) "_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc")
    (data (;13;) (memory 1) (i32.const 358) "_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc")
    (data (;14;) (memory 1) (i32.const 402) "_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc")
    (data (;15;) (memory 1) (i32.const 446) "_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler")
    (data (;16;) (memory 1) (i32.const 502) "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E")
    (data (;17;) (memory 1) (i32.const 573) "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E")
    (data (;18;) (memory 1) (i32.const 644) "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E")
    (data (;19;) (memory 1) (i32.const 715) "_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E")
    (data (;20;) (memory 1) (i32.const 788) "_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE")
    (data (;21;) (memory 1) (i32.const 861) "_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE")
    (data (;22;) (memory 1) (i32.const 955) "_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE")
    (data (;23;) (memory 1) (i32.const 1005) "_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E")
    (data (;24;) (memory 1) (i32.const 1101) "_ZN4core3fmt5Write10write_char17ha202ea95069de918E")
    (data (;25;) (memory 1) (i32.const 1151) "_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE")
    (data (;26;) (memory 1) (i32.const 1201) "_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE")
    (data (;27;) (memory 1) (i32.const 1317) "_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E")
    (data (;28;) (memory 1) (i32.const 1367) "_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E")
    (data (;29;) (memory 1) (i32.const 1457) "_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E")
    (data (;30;) (memory 1) (i32.const 1505) "_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E")
    (data (;31;) (memory 1) (i32.const 1553) "_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E")
    (data (;32;) (memory 1) (i32.const 1601) "_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE")
    (data (;33;) (memory 1) (i32.const 1649) "_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE")
    (data (;34;) (memory 1) (i32.const 1697) "_ZN3std9panicking12default_hook17h8deeaf2f2b300de3E")
    (data (;35;) (memory 1) (i32.const 1748) "_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E")
    (data (;36;) (memory 1) (i32.const 1902) "_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE")
    (data (;37;) (memory 1) (i32.const 2136) "_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E")
    (data (;38;) (memory 1) (i32.const 2198) "_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E")
    (data (;39;) (memory 1) (i32.const 2274) "_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E")
    (data (;40;) (memory 1) (i32.const 2354) "_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE")
    (data (;41;) (memory 1) (i32.const 2465) "_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E")
    (data (;42;) (memory 1) (i32.const 2580) "_ZN4core9panicking13assert_failed17heafbb113157aa4f0E")
    (data (;43;) (memory 1) (i32.const 2633) "_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E")
    (data (;44;) (memory 1) (i32.const 2728) "_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E")
    (data (;45;) (memory 1) (i32.const 2821) "_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E")
    (data (;46;) (memory 1) (i32.const 2871) "_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E")
    (data (;47;) (memory 1) (i32.const 2964) "_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E")
    (data (;48;) (memory 1) (i32.const 3027) "_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E")
    (data (;49;) (memory 1) (i32.const 3078) "_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E")
    (data (;50;) (memory 1) (i32.const 3135) "_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E")
    (data (;51;) (memory 1) (i32.const 3181) "_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE")
    (data (;52;) (memory 1) (i32.const 3247) "_ZN3std3env11current_dir17h890335e8528685e2E")
    (data (;53;) (memory 1) (i32.const 3291) "_ZN3std3env7_var_os17hecfa64e4c3898426E")
    (data (;54;) (memory 1) (i32.const 3330) "_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE")
    (data (;55;) (memory 1) (i32.const 3414) "_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E")
    (data (;56;) (memory 1) (i32.const 3499) "_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E")
    (data (;57;) (memory 1) (i32.const 3558) "_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E")
    (data (;58;) (memory 1) (i32.const 3647) "_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E")
    (data (;59;) (memory 1) (i32.const 3765) "_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E")
    (data (;60;) (memory 1) (i32.const 3893) "_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E")
    (data (;61;) (memory 1) (i32.const 4015) "_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E")
    (data (;62;) (memory 1) (i32.const 4147) "_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE")
    (data (;63;) (memory 1) (i32.const 4203) "_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE")
    (data (;64;) (memory 1) (i32.const 4276) "_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E")
    (data (;65;) (memory 1) (i32.const 4324) "_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E")
    (data (;66;) (memory 1) (i32.const 4420) "_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE")
    (data (;67;) (memory 1) (i32.const 4489) "_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E")
    (data (;68;) (memory 1) (i32.const 4535) "_ZN3std2io5stdio6_print17h68847dc224af8aecE")
    (data (;69;) (memory 1) (i32.const 4578) "_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE")
    (data (;70;) (memory 1) (i32.const 4694) "_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E")
    (data (;71;) (memory 1) (i32.const 4810) "_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E")
    (data (;72;) (memory 1) (i32.const 4926) "_ZN3std2io5Write9write_all17h25f1158500ddcd1aE")
    (data (;73;) (memory 1) (i32.const 4972) "_ZN3std2io5Write9write_fmt17h1459a55272857e0fE")
    (data (;74;) (memory 1) (i32.const 5018) "_ZN3std5panic19get_backtrace_style17h54380a7febe96116E")
    (data (;75;) (memory 1) (i32.const 5072) "_ZN3std7process5abort17had5be8ae244d01ebE")
    (data (;76;) (memory 1) (i32.const 5113) "_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E")
    (data (;77;) (memory 1) (i32.const 5186) "_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E")
    (data (;78;) (memory 1) (i32.const 5232) "_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE")
    (data (;79;) (memory 1) (i32.const 5359) "_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE")
    (data (;80;) (memory 1) (i32.const 5428) "_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE")
    (data (;81;) (memory 1) (i32.const 5516) "_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E")
    (data (;82;) (memory 1) (i32.const 5575) "_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc")
    (data (;83;) (memory 1) (i32.const 5616) "_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc")
    (data (;84;) (memory 1) (i32.const 5659) "_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc")
    (data (;85;) (memory 1) (i32.const 5702) "_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE")
    (data (;86;) (memory 1) (i32.const 5783) "_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E")
    (data (;87;) (memory 1) (i32.const 5894) "_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind")
    (data (;88;) (memory 1) (i32.const 5940) "_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E")
    (data (;89;) (memory 1) (i32.const 6077) "_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE")
    (data (;90;) (memory 1) (i32.const 6209) "_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE")
    (data (;91;) (memory 1) (i32.const 6333) "_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E")
    (data (;92;) (memory 1) (i32.const 6466) "_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E")
    (data (;93;) (memory 1) (i32.const 6587) "_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E")
    (data (;94;) (memory 1) (i32.const 6646) "_RNvCscSpY9Juk0HT_7___rustc10rust_panic")
    (data (;95;) (memory 1) (i32.const 6685) "cabi_realloc")
    (data (;96;) (memory 1) (i32.const 6697) "_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE")
    (data (;97;) (memory 1) (i32.const 6792) "_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E")
    (data (;98;) (memory 1) (i32.const 6897) "_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E")
    (data (;99;) (memory 1) (i32.const 6939) "_RNvCscSpY9Juk0HT_7___rustc8___rg_oom")
    (data (;100;) (memory 1) (i32.const 6976) "_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE")
    (data (;101;) (memory 1) (i32.const 7028) "malloc")
    (data (;102;) (memory 1) (i32.const 7034) "dlmalloc")
    (data (;103;) (memory 1) (i32.const 7042) "free")
    (data (;104;) (memory 1) (i32.const 7046) "calloc")
    (data (;105;) (memory 1) (i32.const 7052) "realloc")
    (data (;106;) (memory 1) (i32.const 7059) "posix_memalign")
    (data (;107;) (memory 1) (i32.const 7073) "internal_memalign")
    (data (;108;) (memory 1) (i32.const 7090) "_Exit")
    (data (;109;) (memory 1) (i32.const 7095) "__wasilibc_ensure_environ")
    (data (;110;) (memory 1) (i32.const 7120) "__wasilibc_initialize_environ")
    (data (;111;) (memory 1) (i32.const 7149) "__wasi_environ_get")
    (data (;112;) (memory 1) (i32.const 7167) "__wasi_environ_sizes_get")
    (data (;113;) (memory 1) (i32.const 7191) "__wasi_proc_exit")
    (data (;114;) (memory 1) (i32.const 7207) "getcwd")
    (data (;115;) (memory 1) (i32.const 7213) "sbrk")
    (data (;116;) (memory 1) (i32.const 7217) "getenv")
    (data (;117;) (memory 1) (i32.const 7223) "__wasm_call_dtors")
    (data (;118;) (memory 1) (i32.const 7240) "__lctrans")
    (data (;119;) (memory 1) (i32.const 7249) "__stdio_exit")
    (data (;120;) (memory 1) (i32.const 7261) "__strchrnul")
    (data (;121;) (memory 1) (i32.const 7272) "strcpy")
    (data (;122;) (memory 1) (i32.const 7278) "strdup")
    (data (;123;) (memory 1) (i32.const 7284) "strerror")
    (data (;124;) (memory 1) (i32.const 7292) "strerror_r")
    (data (;125;) (memory 1) (i32.const 7302) "_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE")
    (data (;126;) (memory 1) (i32.const 7358) "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E")
    (data (;127;) (memory 1) (i32.const 7425) "_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE")
    (data (;128;) (memory 1) (i32.const 7476) "_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE")
    (data (;129;) (memory 1) (i32.const 7526) "_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E")
    (data (;130;) (memory 1) (i32.const 7581) "_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE")
    (data (;131;) (memory 1) (i32.const 7693) "_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE")
    (data (;132;) (memory 1) (i32.const 7761) "_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E")
    (data (;133;) (memory 1) (i32.const 7829) "_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE")
    (data (;134;) (memory 1) (i32.const 7895) "_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E")
    (data (;135;) (memory 1) (i32.const 7941) "_ZN4core9panicking5panic17hd836709591dfc35fE")
    (data (;136;) (memory 1) (i32.const 7985) "_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE")
    (data (;137;) (memory 1) (i32.const 8033) "_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE")
    (data (;138;) (memory 1) (i32.const 8101) "_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE")
    (data (;139;) (memory 1) (i32.const 8157) "_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE")
    (data (;140;) (memory 1) (i32.const 8214) "_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE")
    (data (;141;) (memory 1) (i32.const 8274) "_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E")
    (data (;142;) (memory 1) (i32.const 8371) "_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE")
    (data (;143;) (memory 1) (i32.const 8421) "_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E")
    (data (;144;) (memory 1) (i32.const 8519) "_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE")
    (data (;145;) (memory 1) (i32.const 8569) "_ZN4core6option13expect_failed17he15179d1cacc214eE")
    (data (;146;) (memory 1) (i32.const 8619) "_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E")
    (data (;147;) (memory 1) (i32.const 8692) "_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE")
    (data (;148;) (memory 1) (i32.const 8787) "_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E")
    (data (;149;) (memory 1) (i32.const 8846) "_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE")
    (data (;150;) (memory 1) (i32.const 8934) "_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E")
    (data (;151;) (memory 1) (i32.const 8982) "_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E")
    (data (;152;) (memory 1) (i32.const 9052) "_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E")
    (data (;153;) (memory 1) (i32.const 9124) "_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE")
    (data (;154;) (memory 1) (i32.const 9195) "_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E")
    (data (;155;) (memory 1) (i32.const 9244) "_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE")
    (data (;156;) (memory 1) (i32.const 9329) "_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE")
    (data (;157;) (memory 1) (i32.const 9412) "_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E")
    (data (;158;) (memory 1) (i32.const 9506) "_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E")
    (data (;159;) (memory 1) (i32.const 9600) "_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE")
    (data (;160;) (memory 1) (i32.const 9695) "_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE")
    (data (;161;) (memory 1) (i32.const 9793) "\0a================================= REPORT CSV FLUSH ====================================\0aid, id_type, name, whamm_type, wasm_type, script_id, fname, fid, pc, probe_id, value(s)")
    (data (;162;) (memory 1) (i32.const 9969) ", memaddr, ")
    (data (;163;) (memory 1) (i32.const 9980) ", ")
    (data (;164;) (memory 1) (i32.const 9982) "script")
    (data (;165;) (memory 1) (i32.const 9988) "u32, ")
    (data (;166;) (memory 1) (i32.const 9993) "i32, ")
    (data (;167;) (memory 2) (i32.const 0) " \00\00\00\1e\00\00\00\06\05\00\00\00\0c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1e\00\00\00\06\05\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1e\00\00\00\06\05\00\00\00\0f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1e\00\00\00\06\05\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00$\00\00\00H\06\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00l\00\00\00Z\08\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\c6\00\00\00Y\09\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1f\01\00\00\07\0b\00\00\00\1f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1f\01\00\00\07\0b\00\00\006\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1f\01\00\00\07\0b\00\00\00A\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\00\12\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\00\1f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\008\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\00P\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\00X\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00&\01\00\00\07\0c\00\00\00`\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00T\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00x\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\9c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\c0\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e4\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\08\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00,\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00P\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00t\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\98\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\aa\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\b4\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\d6\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e1\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e9\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\00\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\0b\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\13\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00*\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\005\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00=\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00T\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00_\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00g\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00~\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\89\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\91\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\a8\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\b3\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\bb\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\d2\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\dd\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e5\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\fc\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\07\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\0f\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00&\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\001\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\009\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00P\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00[\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00f\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00}\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\88\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\90\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\a7\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\b2\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\ba\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\d1\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\dc\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e4\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\fb\03\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\06\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\0e\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00%\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\000\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\008\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00O\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00Z\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00b\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00y\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\84\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\8c\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\a3\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\ae\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\b6\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\cd\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\d8\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\e0\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\f7\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\02\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\0d\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00$\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00/\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\007\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00N\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00Y\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00a\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00x\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\83\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\8b\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\a2\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\ad\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\b5\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\cc\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\d7\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\df\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\f6\05\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\01\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\09\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00 \06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00+\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\003\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00J\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00U\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00]\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00t\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\7f\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\87\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\9e\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\01\00\00\04\0d\00\00\00\a9\06\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\001\01\00\00\0b\0e\00\00\00\13\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00<\01\00\00*\0f\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00f\01\00\00,\10\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\92\01\00\00,\11\00\00\00\04\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\be\01\00\008\12\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\01\00\00G\15\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\01\00\00G\15\00\00\00\16\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\01\00\00G\15\00\00\00\1b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00=\02\00\00G\16\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\84\02\00\00G\17\00\00\00\1f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\84\02\00\00G\17\00\00\00(\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\cb\02\00\00I\18\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\14\03\00\00I\19\00\00\00@\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00]\03\00\00^\1a\00\00\00\0e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00]\03\00\00^\1a\00\00\00\13\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00]\03\00\00^\1a\00\00\00\18\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\03\00\002\1b\00\00\00x\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\03\00\002\1b\00\00\00\ae\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\03\00\002\1b\00\00\00\b3\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\1c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00B\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00p\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\e7\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00<\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00}\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\dc\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\e9\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\fb\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00\15\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00F\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\03\00\00`\1c\00\00\00o\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00M\04\00\002\1d\00\00\00\e2\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00M\04\00\002\1d\00\00\00\e7\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\7f\04\00\002\1e\00\00\00t\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b1\04\00\00t\1f\00\00\00\19\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b1\04\00\00t\1f\00\00\00j\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b1\04\00\00t\1f\00\00\00o\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b1\04\00\00t\1f\00\00\00\95\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00%\05\00\002 \00\00\00\80\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00W\05\00\00Z!\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00W\05\00\00Z!\00\00\00y\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b1\05\00\000\22\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e1\05\00\000#\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\11\06\00\000$\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00A\06\00\000%\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00q\06\00\000&\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00\14\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00%\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00q\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00\8b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00\b4\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\06\00\003'\00\00\00\d3\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d4\06\00\00\9a(\00\00\00+\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d4\06\00\00\9a(\00\00\000\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00n\07\00\00\ea)\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00X\08\00\00>*\00\00\00\10\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00X\08\00\00>*\00\00\00%\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\96\08\00\00L+\00\00\00\0a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e2\08\00\00P,\00\00\00\0a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\002\09\00\00o-\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\09\00\00s.\00\00\00)\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\09\00\00s.\00\00\00.\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\14\0a\00\0050\00\00\00\19\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00I\0a\00\00_2\00\00\00x\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00I\0a\00\00_2\00\00\00\9e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a8\0a\00\00]3\00\00\00\0f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\05\0b\00\0024\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\05\0b\00\0024\00\00\003\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\c6\00\00\00Y5\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\007\0b\00\00]6\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\003\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00\c5\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00\00\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00\18\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\000\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\003\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\009\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00w\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00|\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00\88\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\94\0b\00\00?7\00\00\00\99\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d3\0b\00\0038\00\00\00K\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d3\0b\00\0038\00\00\00T\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\06\0c\00\0099\00\00\00\1a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00?\0c\00\00.:\00\00\00\15\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00?\0c\00\00.:\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00?\0c\00\00.:\00\00\00b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00?\0c\00\00.:\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00m\0c\00\00B;\00\00\00\00\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00\0e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00\1c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\004\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00A\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00H\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00O\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00`\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00\8d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0c\00\00,<\00\00\00\95\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00'\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00;\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00E\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00Y\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00t\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00\ab\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00\b0\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\db\0c\00\00'=\00\00\00\c1\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00\08\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00\1a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00%\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\009\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00Z\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00m\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\02\0d\00\00T>\00\00\00z\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\0d\00\00U?\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\0d\00\00U?\00\00\00\c9\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\0d\00\00U?\00\00\00>\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\0d\00\00U?\00\00\00F\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00\14\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00\1d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00\1e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00n\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00\83\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab\0d\00\00;@\00\00\00\8b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\0d\00\00YA\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\0d\00\00YA\00\00\00G\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\0d\00\00YA\00\00\00R\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\0d\00\00YA\00\00\00\90\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\0d\00\00YA\00\00\00\9b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00?\0e\00\00vB\00\00\00\0f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b5\0e\00\00\80C\00\00\00v\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b5\0e\00\00\80C\00\00\00\94\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\005\0f\00\00zE\00\00\00\0f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0f\00\00\84F\00\00\00r\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\af\0f\00\00\84F\00\00\00\90\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\003\10\00\008H\00\00\00M\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\003\10\00\008H\00\00\00\be\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\003\10\00\008H\00\00\00\e3\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\003\10\00\008H\00\00\00\fb\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\003\10\00\008H\00\00\00\03\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00k\10\00\00II\00\00\00\19\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00k\10\00\00II\00\00\00D\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00k\10\00\00II\00\00\00\5c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b4\10\00\000J\00\00\00b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b4\10\00\000J\00\00\00{\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b4\10\00\000J\00\00\00\8c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e4\10\00\00`K\00\00\00\08\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e4\10\00\00`K\00\00\00\1e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e4\10\00\00`K\00\00\00;\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e4\10\00\00`K\00\00\00k\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e4\10\00\00`K\00\00\00p\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00D\11\00\00EL\00\00\003\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00D\11\00\00EL\00\00\00_\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00D\11\00\00EL\00\00\00d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00D\11\00\00EL\00\00\00\87\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00D\11\00\00EL\00\00\00\aa\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\89\11\00\00.M\00\00\00\15\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\89\11\00\00.M\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\89\11\00\00.M\00\00\00b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\89\11\00\00.M\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b7\11\00\00+N\00\00\00\10\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b7\11\00\00+N\00\00\00\1a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b7\11\00\00+N\00\00\00-\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b7\11\00\00+N\00\00\00m\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e2\11\00\00tO\00\00\00\0c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e2\11\00\00tO\00\00\00B\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e2\11\00\00tO\00\00\00G\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\12\00\00tP\00\00\00q\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00V\12\00\00tP\00\00\00v\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ca\12\00\00tQ\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00>\13\00\00.R\00\00\00\18\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00>\13\00\00.R\00\00\00Y\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00l\13\00\00.S\00\00\00\15\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00l\13\00\00.S\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00l\13\00\00.S\00\00\00b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00l\13\00\00.S\00\00\00g\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9a\13\00\006T\00\00\00\1b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9a\13\00\006T\00\00\00Z\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\13\00\00)U\00\00\00\00\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f9\13\00\00IV\00\00\00#\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00B\14\00\00.W\00\00\00-\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00p\14\00\00\7fY\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00p\14\00\00\7fY\00\00\00A\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00p\14\00\00\7fY\00\00\00F\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00p\14\00\00\7fY\00\00\00z\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ef\14\00\00EZ\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\15\00\00X[\00\00\00:\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\15\00\00X[\00\00\00O\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\15\00\00;\5c\00\00\00.\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\15\00\00;\5c\00\00\00Z\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\15\00\00;\5c\00\00\00_\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\15\00\00;\5c\00\00\00\8a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\c7\15\00\00)]\00\00\00\1e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\c7\15\00\00)]\00\00\00)\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f0\15\00\00+^\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1b\16\00\00+_\00\00\00 \00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1b\16\00\00+_\00\00\007\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1b\16\00\00+_\00\00\00>\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00O\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00l\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00~\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00\83\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00\8d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00\92\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00F\16\00\00Qa\00\00\00\bb\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00h\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00\af\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00\b4\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00\d9\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00\de\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\007\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00<\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\97\16\00\00ob\00\00\00I\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\06\17\00\00.d\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\17\00\00\89e\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\17\00\00\89e\00\00\00q\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\17\00\00\89e\00\00\00v\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bd\17\00\00\84f\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00A\18\00\00|g\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00A\18\00\00|g\00\00\007\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bd\18\00\00\85h\00\00\00\0c\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bd\18\00\00\85h\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00B\19\00\00yk\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\13\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00i\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00n\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\90\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\95\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\cc\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\d1\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00\15\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00=\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00B\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00G\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\bb\19\00\00;l\00\00\00J\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\19\00\00'm\00\00\00\08\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\19\00\00'm\00\00\00.\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\19\00\00'm\00\00\003\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f6\19\00\00'm\00\00\004\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1d\1a\00\00\0cn\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1d\1a\00\00\0cn\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1d\1a\00\00\0cn\00\00\00\1b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00)\1a\00\00_o\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\88\1a\00\00ip\00\00\00\0b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f1\1a\00\00*s\00\00\00\0f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1b\1b\00\00%t\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00@\1b\00\004v\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00t\1b\00\00\06w\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00%\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00a\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00r\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00\9f\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00\ae\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00\d5\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00\d8\04\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00z\1b\00\00\08x\00\00\00\8a\07\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\82\1b\00\00\04z\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\86\1b\00\00\06|\00\00\00#\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\86\1b\00\00\06|\00\00\002\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00\04\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00n\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00\f3\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00\f9\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00\13\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8c\1b\00\00\07}\00\00\00\16\02\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\93\1b\00\00\0e\7f\00\00\00\08\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\93\1b\00\00\0e\7f\00\00\000\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\1b\00\00\11\80\00\00\00=\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\1b\00\00\11\80\00\00\00\ae\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a1\1b\00\00\11\80\00\00\00\e0\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b2\1b\00\00\05\81\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b7\1b\00\00\19\82\00\00\00\06\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00(\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00,\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00/\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\004\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\008\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00:\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d0\1b\00\00\1d\83\00\00\00=\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ed\1b\00\00\12\84\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ff\1b\00\00\18\85\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\17\1c\00\00\10\86\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00'\1c\00\00\06\88\00\00\00\08\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00'\1c\00\00\06\88\00\00\00\13\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00'\1c\00\00\06\88\00\00\00!\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00-\1c\00\00\04\89\00\00\00%\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\001\1c\00\00\06\8a\00\00\00\00\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\001\1c\00\00\06\8a\00\00\00\04\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\001\1c\00\00\06\8a\00\00\00+\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\007\1c\00\00\11\8c\00\00\00\00\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\007\1c\00\00\11\8c\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00H\1c\00\00\09\8e\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00Q\1c\00\00\0c\90\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00]\1c\00\00\0b\93\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00h\1c\00\00\06\95\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00n\1c\00\00\06\96\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00n\1c\00\00\06\96\00\00\00\06\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00n\1c\00\00\06\96\00\00\00\0d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00t\1c\00\00\08\97\00\00\00\1a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00|\1c\00\00\0a\98\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00|\1c\00\00\0a\98\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00|\1c\00\00\0a\98\00\00\00\15\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00|\1c\00\00\0a\98\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\86\1c\00\008\9b\00\00\00\17\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\be\1c\00\00C\9c\00\00\00\18\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\be\1c\00\00C\9c\00\00\006\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\be\1c\00\00C\9c\00\00\00B\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\01\1d\00\003\9d\00\00\00\06\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\01\1d\00\003\9d\00\00\00\0a\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\1d\00\002\9e\00\00\00\22\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\004\1d\00\002\9e\00\00\003\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00f\1d\00\007\9f\00\00\00\02\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9d\1d\00\00p\a0\00\00\00\1d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9d\1d\00\00p\a0\00\00\00\7f\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9d\1d\00\00p\a0\00\00\00\9d\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9d\1d\00\00p\a0\00\00\00\a3\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\9d\1d\00\00p\a0\00\00\00\b3\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\0d\1e\00\00D\a1\00\00\007\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\0d\1e\00\00D\a1\00\00\00I\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\0d\1e\00\00D\a1\00\00\00u\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\0d\1e\00\00D\a1\00\00\00|\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\0d\1e\00\00D\a1\00\00\00\81\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00Q\1e\00\00D\a2\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\95\1e\00\00B\a3\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\d7\1e\00\00.\a4\00\00\00\16\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\05\1f\00\00,\a5\00\00\00\1b\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\001\1f\00\000\a6\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00a\1f\00\00D\a8\00\00\00\a2\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a5\1f\00\008\a9\00\00\001\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a5\1f\00\008\a9\00\00\00\05\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a5\1f\00\008\a9\00\00\00-\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a5\1f\00\008\a9\00\00\00r\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\dd\1f\00\009\ac\00\00\00$\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\16 \00\00<\af\00\00\00\80\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\16 \00\00<\af\00\00\00\87\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00R \00\00a\b0\00\00\00Q\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\b3 \00\002\b1\00\00\00@\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e5 \00\00b\b2\00\00\00\04\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00G!\00\002\b3\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00y!\00\002\b4\00\00\00,\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\ab!\00\00I\b5\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\f4!\00\00_\b6\00\00\00;\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00S\22\00\00;\b7\00\00\00\87\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00S\22\00\00;\b7\00\00\00\c2\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\8e\22\00\00X\b9\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\e6\22\00\000\bc\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\16#\00\00F\bf\00\00\00\1e\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\16#\00\00F\bf\00\00\00#\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\5c#\00\00H\c1\00\00\00\09\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\5c#\00\00H\c1\00\00\00\11\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\a4#\00\00G\c2\00\00\00\03\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\eb#\00\001\c4\00\00\00(\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\eb#\00\001\c4\00\00\000\01\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\1c$\00\00U\c5\00\00\001\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00q$\00\00S\c6\00\00\001\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\c4$\00\00^\c7\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\22%\00\00^\c8\00\00\00<\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00 \00\00\00\80%\00\00_\c9\00\00\00;\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00\ff\ff\ff\ff\df%\00\00b\ca\00\00\00\10\00\00\00\00\00\00\00\05\00\05\00\00\00\19\00\00\00\00")
    (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
    (@producers
      (language "C11" "")
      (language "Rust" "")
      (processed-by "clang" "19.1.5-wasi-sdk (https://github.com/llvm/llvm-project ab4b5a2db582958af1ee308a790cfdb42bd24720)")
      (processed-by "rustc" "1.87.0 (17067e9ac 2025-05-09)")
      (processed-by "wit-component" "0.20.1")
      (processed-by "wit-bindgen-c" "0.17.0")
    )
  )
  (core module (;1;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i64 i32)))
    (type (;4;) (func (param i32 i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32 i32 i32)))
    (type (;6;) (func (param i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32)))
    (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;9;) (func (result i32)))
    (type (;10;) (func (param i32 i32 i32) (result i32)))
    (type (;11;) (func (param i32 i32) (result i32)))
    (type (;12;) (func))
    (import "env" "memory" (memory (;0;) 0))
    (import "__main_module__" "_start" (func $_ZN22wasi_snapshot_preview13run6_start17h6125b5756c6a2506E (;0;) (type 0)))
    (import "wasi:cli/environment@0.2.3" "get-environment" (func $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E (;1;) (type 1)))
    (import "wasi:filesystem/types@0.2.3" "[resource-drop]descriptor" (func $_ZN141_$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..Descriptor$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h34b1c3918a0eebe8E (;2;) (type 1)))
    (import "wasi:io/streams@0.2.3" "[resource-drop]output-stream" (func $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E (;3;) (type 1)))
    (import "wasi:filesystem/types@0.2.3" "filesystem-error-code" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types21filesystem_error_code10wit_import17h2b5d8e99a65d0583E (;4;) (type 2)))
    (import "wasi:io/error@0.2.3" "[resource-drop]error" (func $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E (;5;) (type 1)))
    (import "wasi:io/streams@0.2.3" "[resource-drop]input-stream" (func $_ZN136_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..InputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h5547e0ecd980db5bE (;6;) (type 1)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.check-write" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream11check_write10wit_import17hdb0f94e542b67356E (;7;) (type 2)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.write" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream5write10wit_import17h1ec4b5645a67d72bE (;8;) (type 4)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.blocking-flush" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream14blocking_flush10wit_import17h7bbaef6ef9f5f783E (;9;) (type 2)))
    (import "__main_module__" "cabi_realloc" (func $_ZN22wasi_snapshot_preview15State3new12cabi_realloc17h88297338052a23f0E (;10;) (type 8)))
    (import "wasi:filesystem/preopens@0.2.2" "get-directories" (func $_ZN22wasi_snapshot_preview111descriptors31wasi_filesystem_get_directories17h9e1fe8bbb8a7f366E (;11;) (type 1)))
    (import "wasi:cli/stderr@0.2.3" "get-stderr" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E (;12;) (type 9)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.blocking-write-and-flush" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush10wit_import17hd0e6bb9bc2c2ef7aE (;13;) (type 4)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.write-via-stream" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor16write_via_stream10wit_import17h863d15eb6a8c37dcE (;14;) (type 3)))
    (import "wasi:cli/stdin@0.2.3" "get-stdin" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli5stdin9get_stdin10wit_import17he5bdc61203ca72fdE (;15;) (type 9)))
    (import "wasi:cli/stdout@0.2.3" "get-stdout" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stdout10get_stdout10wit_import17hfdc059c6457e3d59E (;16;) (type 9)))
    (import "wasi:cli/exit@0.2.3" "exit" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit10wit_import17hefacdcba211d0d02E (;17;) (type 1)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.append-via-stream" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream10wit_import17h2638911ab3ccf3ddE (;18;) (type 2)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.get-type" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type10wit_import17h708de2cfd599ed76E (;19;) (type 2)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.stat" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat10wit_import17hd8a47ee046c71cb9E (;20;) (type 2)))
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $internal_state_ptr (;1;) (mut i32) i32.const 0)
    (global $allocation_state (;2;) (mut i32) i32.const 0)
    (export "wasi:cli/run@0.2.3#run" (func $wasi:cli/run@0.2.3#run))
    (export "fd_write" (func $fd_write))
    (export "environ_get" (func $environ_get))
    (export "environ_sizes_get" (func $environ_sizes_get))
    (export "cabi_import_realloc" (func $cabi_import_realloc))
    (export "proc_exit" (func $proc_exit))
    (func $wasi:cli/run@0.2.3#run (;21;) (type 9) (result i32)
      call $allocate_stack
      call $_ZN22wasi_snapshot_preview13run6_start17h6125b5756c6a2506E
      i32.const 0
    )
    (func $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE (;22;) (type 9) (result i32)
      (local i32)
      block ;; label = @1
        call $get_state_ptr
        local.tee 0
        br_if 0 (;@1;)
        call $_ZN22wasi_snapshot_preview15State3new17h27fd4b5328bc4eeeE
        local.tee 0
        call $set_state_ptr
      end
      local.get 0
    )
    (func $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE (;23;) (type 1) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 32
      i32.store8 offset=47
      local.get 1
      i64.const 7308895158390646132
      i64.store offset=39 align=1
      local.get 1
      i64.const 8097863973307965728
      i64.store offset=31 align=1
      local.get 1
      i64.const 7234307576302018670
      i64.store offset=23 align=1
      local.get 1
      i64.const 8028075845441778529
      i64.store offset=15 align=1
      local.get 1
      i32.const 15
      i32.add
      i32.const 33
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3217h80b476442303f1eeE
      unreachable
    )
    (func $cabi_import_realloc (;24;) (type 8) (param i32 i32 i32 i32) (result i32)
      (local i32 i32 i64)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                          local.tee 5
                          i32.load
                          i32.const 560490357
                          i32.ne
                          br_if 0 (;@10;)
                          local.get 5
                          i32.load offset=65532
                          i32.const 560490357
                          i32.ne
                          br_if 1 (;@9;)
                          local.get 5
                          i64.load offset=4 align=4
                          local.set 6
                          local.get 5
                          i32.const 4
                          i32.store offset=4
                          local.get 4
                          i32.const 16
                          i32.add
                          local.get 5
                          i32.const 20
                          i32.add
                          i32.load
                          i32.store
                          local.get 4
                          i32.const 8
                          i32.add
                          local.get 5
                          i32.const 12
                          i32.add
                          i64.load align=4
                          i64.store
                          local.get 4
                          local.get 6
                          i64.store
                          local.get 0
                          i32.eqz
                          br_if 2 (;@8;)
                          local.get 1
                          local.get 3
                          i32.le_u
                          br_if 3 (;@7;)
                          local.get 2
                          i32.const 1
                          i32.eq
                          br_if 9 (;@1;)
                          i32.const 377
                          call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
                          unreachable
                        end
                        i32.const 2745
                        call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
                        unreachable
                      end
                      i32.const 2746
                      call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
                      unreachable
                    end
                    local.get 4
                    i32.load
                    br_table 5 (;@2;) 3 (;@4;) 2 (;@5;) 1 (;@6;) 4 (;@3;) 5 (;@2;)
                  end
                  i32.const 376
                  call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
                  unreachable
                end
                local.get 4
                i32.const 12
                i32.add
                local.set 0
                block ;; label = @6
                  local.get 2
                  i32.const 1
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 0
                  local.get 2
                  local.get 3
                  call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                  local.set 0
                  br 5 (;@1;)
                end
                local.get 4
                local.get 4
                i32.load offset=4
                local.tee 2
                i32.const 1
                i32.add
                i32.store offset=4
                block ;; label = @6
                  local.get 2
                  local.get 4
                  i32.load offset=8
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 4
                  local.get 4
                  i64.load offset=12 align=4
                  i64.store offset=24 align=4
                  local.get 4
                  i32.const 24
                  i32.add
                  i32.const 1
                  local.get 3
                  call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                  local.set 0
                  br 5 (;@1;)
                end
                local.get 0
                i32.const 1
                local.get 3
                call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                local.set 0
                br 4 (;@1;)
              end
              block ;; label = @5
                local.get 2
                i32.const 1
                i32.eq
                br_if 0 (;@5;)
                local.get 4
                i32.const 12
                i32.add
                local.get 2
                local.get 3
                call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                local.set 0
                br 4 (;@1;)
              end
              local.get 4
              i32.const 4
              i32.or
              i32.const 1
              local.get 3
              i32.const 1
              i32.add
              call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
              local.set 0
              br 3 (;@1;)
            end
            block ;; label = @4
              local.get 2
              i32.const 1
              i32.eq
              br_if 0 (;@4;)
              local.get 4
              i32.const 8
              i32.add
              local.get 2
              local.get 3
              call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
              local.set 0
              br 3 (;@1;)
            end
            local.get 4
            local.get 4
            i32.load offset=4
            local.get 3
            i32.add
            i32.store offset=4
            local.get 4
            local.get 4
            i64.load offset=8
            i64.store offset=24 align=4
            local.get 4
            i32.const 24
            i32.add
            i32.const 1
            local.get 3
            call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
            local.set 0
            br 2 (;@1;)
          end
          i32.const 418
          call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
          local.get 4
          i32.const 8250
          i32.store16 offset=24 align=1
          local.get 4
          i32.const 24
          i32.add
          i32.const 2
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get 4
          i64.const 748834980320733542
          i64.store offset=40 align=1
          local.get 4
          i64.const 7957688057596965985
          i64.store offset=32 align=1
          local.get 4
          i64.const 7165064744911531886
          i64.store offset=24 align=1
          local.get 4
          i32.const 24
          i32.add
          i32.const 24
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get 4
          i32.const 10
          i32.store8 offset=24
          local.get 4
          i32.const 24
          i32.add
          i32.const 1
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          unreachable
        end
        local.get 4
        i32.const 4
        i32.or
        local.get 2
        local.get 3
        call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
        local.set 0
        local.get 4
        i32.const 4
        i32.store
      end
      local.get 5
      i32.const 4
      i32.add
      local.tee 5
      local.get 4
      i64.load
      i64.store align=4
      local.get 5
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i32.load
      i32.store
      local.get 5
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load
      i64.store align=4
      local.get 4
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E (;25;) (type 10) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.popcnt
            i32.const 1
            i32.ne
            br_if 0 (;@3;)
            local.get 0
            i32.load offset=4
            local.tee 4
            local.get 1
            local.get 0
            i32.load
            local.tee 5
            i32.add
            i32.const -1
            i32.add
            i32.const 0
            local.get 1
            i32.sub
            i32.and
            local.get 5
            i32.sub
            local.tee 1
            i32.lt_u
            br_if 1 (;@2;)
            local.get 4
            local.get 1
            i32.sub
            local.tee 4
            local.get 2
            i32.ge_u
            br_if 2 (;@1;)
            i32.const 438
            call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
            local.get 3
            i32.const 8250
            i32.store16 offset=3 align=1
            local.get 3
            i32.const 3
            i32.add
            i32.const 2
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            local.get 3
            i32.const 10
            i32.store8 offset=31
            local.get 3
            i32.const 1701278305
            i32.store offset=27 align=1
            local.get 3
            i64.const 7791349879831294825
            i64.store offset=19 align=1
            local.get 3
            i64.const 2334406575183130223
            i64.store offset=11 align=1
            local.get 3
            i64.const 7598805550979902561
            i64.store offset=3 align=1
            local.get 3
            i32.const 3
            i32.add
            i32.const 29
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            local.get 3
            i32.const 10
            i32.store8 offset=3
            local.get 3
            i32.const 3
            i32.add
            i32.const 1
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            unreachable
          end
          i32.const 448
          call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
          local.get 3
          i32.const 8250
          i32.store16 offset=3 align=1
          local.get 3
          i32.const 3
          i32.add
          i32.const 2
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get 3
          i32.const 2676
          i32.store16 offset=19 align=1
          local.get 3
          i64.const 7954884637768641633
          i64.store offset=11 align=1
          local.get 3
          i64.const 2334106421097295465
          i64.store offset=3 align=1
          local.get 3
          i32.const 3
          i32.add
          i32.const 18
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get 3
          i32.const 10
          i32.store8 offset=3
          local.get 3
          i32.const 3
          i32.add
          i32.const 1
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          unreachable
        end
        i32.const 452
        call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
        local.get 3
        i32.const 8250
        i32.store16 offset=3 align=1
        local.get 3
        i32.const 3
        i32.add
        i32.const 2
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get 3
        i32.const 10
        i32.store8 offset=21
        local.get 3
        i32.const 25972
        i32.store16 offset=19 align=1
        local.get 3
        i64.const 7017575155838820463
        i64.store offset=11 align=1
        local.get 3
        i64.const 8367798494427701606
        i64.store offset=3 align=1
        local.get 3
        i32.const 3
        i32.add
        i32.const 19
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get 3
        i32.const 10
        i32.store8 offset=3
        local.get 3
        i32.const 3
        i32.add
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        unreachable
      end
      local.get 0
      local.get 4
      local.get 2
      i32.sub
      i32.store offset=4
      local.get 0
      local.get 5
      local.get 1
      i32.add
      local.tee 1
      local.get 2
      i32.add
      i32.store
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE (;26;) (type 1) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 32
      i32.store8 offset=47
      local.get 1
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 1
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 1
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 1
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 1
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 1
      i32.const 11
      i32.add
      i32.const 37
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
      local.get 1
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E (;27;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E
      i32.store offset=12
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 0
      local.get 1
      call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E
      block ;; label = @1
        local.get 2
        i32.load offset=4
        local.tee 1
        i32.const 2
        i32.eq
        br_if 0 (;@1;)
        local.get 1
        br_if 0 (;@1;)
        local.get 2
        i32.load offset=8
        local.tee 1
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 1
        call $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E
      end
      block ;; label = @1
        local.get 2
        i32.load offset=12
        local.tee 1
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 1
        call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE (;28;) (type 1) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get 1
      i32.const 10
      i32.store8 offset=15
      local.get 1
      i32.const 15
      i32.add
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $environ_get (;29;) (type 11) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
            local.tee 3
            i32.load
            i32.const 560490357
            i32.ne
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=65532
            i32.const 560490357
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 59032
            i32.store offset=20
            local.get 3
            i32.const -1
            i32.store offset=12
            local.get 3
            local.get 1
            i32.store offset=8
            local.get 3
            local.get 3
            i32.const 6192
            i32.add
            i32.store offset=16
            local.get 3
            i32.load offset=4
            local.set 1
            local.get 3
            i32.const 2
            i32.store offset=4
            local.get 1
            i32.const 4
            i32.ne
            br_if 2 (;@1;)
            local.get 2
            i64.const 0
            i64.store align=4
            local.get 2
            call $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E
            local.get 2
            i32.load offset=4
            local.set 4
            local.get 2
            i32.load
            local.set 1
            local.get 3
            i32.const 4
            i32.store offset=4
            block ;; label = @4
              local.get 4
              i32.eqz
              br_if 0 (;@4;)
              loop ;; label = @5
                local.get 1
                i32.const 12
                i32.add
                i32.load
                local.set 3
                local.get 1
                i32.const 8
                i32.add
                i32.load
                local.set 5
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.set 6
                local.get 0
                local.get 1
                i32.load
                local.tee 7
                i32.store
                local.get 7
                local.get 6
                i32.add
                i32.const 61
                i32.store8
                local.get 5
                local.get 3
                i32.add
                i32.const 0
                i32.store8
                local.get 1
                i32.const 16
                i32.add
                local.set 1
                local.get 0
                i32.const 4
                i32.add
                local.set 0
                local.get 4
                i32.const -1
                i32.add
                local.tee 4
                br_if 0 (;@5;)
              end
            end
            local.get 2
            i32.const 32
            i32.add
            global.set $__stack_pointer
            i32.const 0
            return
          end
          i32.const 2745
          call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
          unreachable
        end
        i32.const 2746
        call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
        unreachable
      end
      i32.const 2884
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get 2
      i32.const 8250
      i32.store16 align=1
      local.get 2
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 2
      i32.const 10
      i32.store8 offset=28
      local.get 2
      i32.const 1952805664
      i32.store offset=24 align=1
      local.get 2
      i64.const 8747223464599642400
      i64.store offset=16 align=1
      local.get 2
      i64.const 8245937404367563884
      i64.store offset=8 align=1
      local.get 2
      i64.const 6998721855778483561
      i64.store align=1
      local.get 2
      i32.const 29
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 2
      i32.const 10
      i32.store8
      local.get 2
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $environ_sizes_get (;30;) (type 11) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    call $get_allocation_state
                    i32.const -2
                    i32.add
                    br_table 1 (;@6;) 0 (;@7;) 1 (;@6;) 0 (;@7;)
                  end
                  i32.const 0
                  local.set 3
                  local.get 0
                  i32.const 0
                  i32.store
                  br 1 (;@5;)
                end
                call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                local.tee 3
                i32.load
                i32.const 560490357
                i32.ne
                br_if 1 (;@4;)
                local.get 3
                i32.load offset=65532
                i32.const 560490357
                i32.ne
                br_if 2 (;@3;)
                local.get 3
                i32.const 59032
                i32.store offset=16
                local.get 3
                local.get 3
                i32.const 6192
                i32.add
                i32.store offset=12
                local.get 3
                i32.load offset=4
                local.set 4
                local.get 3
                i64.const 1
                i64.store offset=4 align=4
                local.get 4
                i32.const 4
                i32.ne
                br_if 3 (;@2;)
                local.get 2
                i64.const 0
                i64.store align=4
                local.get 2
                call $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E
                local.get 2
                i32.load offset=4
                local.set 4
                local.get 3
                i32.load offset=4
                local.set 5
                local.get 3
                i32.const 4
                i32.store offset=4
                local.get 5
                i32.const 1
                i32.ne
                br_if 4 (;@1;)
                local.get 3
                i32.load offset=8
                local.set 3
                local.get 0
                local.get 4
                i32.store
                local.get 3
                local.get 4
                i32.const 1
                i32.shl
                i32.add
                local.set 3
              end
              local.get 1
              local.get 3
              i32.store
              local.get 2
              i32.const 32
              i32.add
              global.set $__stack_pointer
              i32.const 0
              return
            end
            i32.const 2745
            call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
            unreachable
          end
          i32.const 2746
          call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
          unreachable
        end
        i32.const 2884
        call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
        local.get 2
        i32.const 8250
        i32.store16 align=1
        local.get 2
        i32.const 2
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get 2
        i32.const 10
        i32.store8 offset=28
        local.get 2
        i32.const 1952805664
        i32.store offset=24 align=1
        local.get 2
        i64.const 8747223464599642400
        i64.store offset=16 align=1
        local.get 2
        i64.const 8245937404367563884
        i64.store offset=8 align=1
        local.get 2
        i64.const 6998721855778483561
        i64.store align=1
        local.get 2
        i32.const 29
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get 2
        i32.const 10
        i32.store8
        local.get 2
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        unreachable
      end
      i32.const 628
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview15State11descriptors17hfc2c9cda66bef96dE (;31;) (type 2) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 6160
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load offset=24
          br_if 0 (;@2;)
          local.get 1
          i32.const -1
          i32.store offset=24
          local.get 1
          i32.const 32
          i32.add
          local.set 3
          block ;; label = @3
            local.get 1
            i32.load offset=6180
            i32.const 2
            i32.ne
            br_if 0 (;@3;)
            local.get 2
            local.get 1
            call $_ZN22wasi_snapshot_preview111descriptors11Descriptors3new17h23d25887e02c514fE
            local.get 3
            local.get 2
            i32.const 6160
            call $memcpy
            drop
            local.get 1
            i32.load offset=6180
            i32.const 2
            i32.eq
            br_if 2 (;@1;)
          end
          local.get 0
          local.get 1
          i32.const 24
          i32.add
          i32.store offset=4
          local.get 0
          local.get 3
          i32.store
          local.get 2
          i32.const 6160
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 2833
        call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
        unreachable
      end
      i32.const 2837
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E (;32;) (type 6) (param i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.set 1
      i32.const 6
      local.set 2
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      block ;; label = @17
                                        block ;; label = @18
                                          block ;; label = @19
                                            block ;; label = @20
                                              block ;; label = @21
                                                block ;; label = @22
                                                  block ;; label = @23
                                                    block ;; label = @24
                                                      block ;; label = @25
                                                        block ;; label = @26
                                                          block ;; label = @27
                                                            block ;; label = @28
                                                              block ;; label = @29
                                                                block ;; label = @30
                                                                  block ;; label = @31
                                                                    block ;; label = @32
                                                                      block ;; label = @33
                                                                        block ;; label = @34
                                                                          block ;; label = @35
                                                                            block ;; label = @36
                                                                              block ;; label = @37
                                                                                local.get 0
                                                                                i32.const 255
                                                                                i32.and
                                                                                br_table 0 (;@37;) 36 (;@1;) 1 (;@36;) 2 (;@35;) 3 (;@34;) 4 (;@33;) 5 (;@32;) 6 (;@31;) 7 (;@30;) 8 (;@29;) 9 (;@28;) 10 (;@27;) 11 (;@26;) 12 (;@25;) 13 (;@24;) 14 (;@23;) 15 (;@22;) 16 (;@21;) 17 (;@20;) 18 (;@19;) 19 (;@18;) 20 (;@17;) 21 (;@16;) 22 (;@15;) 23 (;@14;) 24 (;@13;) 25 (;@12;) 26 (;@11;) 27 (;@10;) 28 (;@9;) 29 (;@8;) 30 (;@7;) 31 (;@6;) 32 (;@5;) 33 (;@4;) 34 (;@3;) 35 (;@2;) 0 (;@37;)
                                                                              end
                                                                              local.get 1
                                                                              i32.const 2
                                                                              i32.store16 offset=14
                                                                              local.get 1
                                                                              i32.const 14
                                                                              i32.add
                                                                              local.set 0
                                                                              local.get 1
                                                                              i32.load16_u offset=14
                                                                              return
                                                                            end
                                                                            i32.const 7
                                                                            return
                                                                          end
                                                                          i32.const 8
                                                                          return
                                                                        end
                                                                        i32.const 10
                                                                        return
                                                                      end
                                                                      i32.const 16
                                                                      return
                                                                    end
                                                                    i32.const 19
                                                                    return
                                                                  end
                                                                  i32.const 20
                                                                  return
                                                                end
                                                                i32.const 22
                                                                return
                                                              end
                                                              i32.const 25
                                                              return
                                                            end
                                                            i32.const 26
                                                            return
                                                          end
                                                          i32.const 27
                                                          return
                                                        end
                                                        i32.const 28
                                                        return
                                                      end
                                                      i32.const 29
                                                      return
                                                    end
                                                    i32.const 31
                                                    return
                                                  end
                                                  i32.const 32
                                                  return
                                                end
                                                i32.const 34
                                                return
                                              end
                                              i32.const 35
                                              return
                                            end
                                            i32.const 37
                                            return
                                          end
                                          i32.const 43
                                          return
                                        end
                                        i32.const 44
                                        return
                                      end
                                      i32.const 46
                                      return
                                    end
                                    i32.const 48
                                    return
                                  end
                                  i32.const 51
                                  return
                                end
                                i32.const 54
                                return
                              end
                              i32.const 55
                              return
                            end
                            i32.const 56
                            return
                          end
                          i32.const 58
                          return
                        end
                        i32.const 59
                        return
                      end
                      i32.const 60
                      return
                    end
                    i32.const 61
                    return
                  end
                  i32.const 63
                  return
                end
                i32.const 64
                return
              end
              i32.const 69
              return
            end
            i32.const 70
            return
          end
          i32.const 74
          return
        end
        i32.const 75
        local.set 2
      end
      local.get 2
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type17h38d62f575f54468dE (;33;) (type 2) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      i32.const 14
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type10wit_import17h708de2cfd599ed76E
      i32.const 0
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.load8_u offset=14
            br_table 1 (;@2;) 0 (;@3;) 2 (;@1;)
          end
          i32.const 1
          local.set 3
        end
        local.get 2
        i32.load8_u offset=15
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 0
      local.get 3
      i32.const 1
      i32.and
      i32.store8
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat17hbce2ab26bb1887cbE (;34;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      i32.const 8
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat10wit_import17hd8a47ee046c71cb9E
      local.get 2
      i32.load8_u offset=16
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.load8_u offset=8
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.load offset=104
          i32.store offset=88
          local.get 0
          local.get 2
          i64.load offset=96
          i64.store offset=80
          local.get 0
          local.get 2
          i32.load offset=80
          i32.store offset=64
          local.get 0
          local.get 2
          i64.load offset=72
          i64.store offset=56
          local.get 0
          local.get 2
          i32.load offset=56
          i32.store offset=40
          local.get 0
          local.get 2
          i64.load offset=48
          i64.store offset=32
          local.get 0
          local.get 2
          i64.load offset=32
          i64.store offset=16
          local.get 0
          local.get 2
          i64.load offset=24
          i64.store offset=8
          local.get 0
          local.get 1
          i32.store8
          local.get 0
          local.get 2
          i32.load8_u offset=88
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=72
          local.get 0
          local.get 2
          i32.load8_u offset=64
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=48
          local.get 0
          local.get 2
          i32.load8_u offset=40
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=24
          br 1 (;@1;)
        end
        local.get 0
        i64.const 2
        i64.store offset=72
        local.get 0
        local.get 1
        i32.store8
      end
      local.get 2
      i32.const 112
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview15State17with_import_alloc17h8e53b3b47ee99160E (;35;) (type 7) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 1
      i32.load offset=4
      local.set 4
      local.get 1
      local.get 2
      i64.load align=4
      i64.store offset=4 align=4
      local.get 1
      i32.const 12
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i64.load align=4
      i64.store align=4
      local.get 1
      i32.const 20
      i32.add
      local.get 2
      i32.const 16
      i32.add
      i32.load
      i32.store
      block ;; label = @1
        local.get 4
        i32.const 4
        i32.ne
        br_if 0 (;@1;)
        local.get 3
        i64.const 0
        i64.store align=4
        local.get 3
        call $_ZN22wasi_snapshot_preview111descriptors31wasi_filesystem_get_directories17h9e1fe8bbb8a7f366E
        local.get 0
        local.get 3
        i64.load align=4
        i64.store align=4
        local.get 0
        i32.const 24
        i32.add
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 16
        i32.add
        i32.load
        i32.store
        local.get 0
        i32.const 16
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i64.load align=4
        i64.store align=4
        local.get 0
        local.get 1
        i64.load align=4
        i64.store offset=8 align=4
        local.get 1
        i32.const 4
        i32.store
        local.get 3
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 2884
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get 3
      i32.const 8250
      i32.store16 align=1
      local.get 3
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 3
      i32.const 10
      i32.store8 offset=28
      local.get 3
      i32.const 1952805664
      i32.store offset=24 align=1
      local.get 3
      i64.const 8747223464599642400
      i64.store offset=16 align=1
      local.get 3
      i64.const 8245937404367563884
      i64.store offset=8 align=1
      local.get 3
      i64.const 6998721855778483561
      i64.store align=1
      local.get 3
      i32.const 29
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 3
      i32.const 10
      i32.store8
      local.get 3
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream17h9367b14712760239E (;36;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i64.const 0
      i64.store offset=8
      local.get 1
      i32.load
      local.get 2
      i32.const 8
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream10wit_import17h2638911ab3ccf3ddE
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.load8_u offset=8
          local.tee 1
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.load offset=12
          i32.store offset=4
          br 1 (;@1;)
        end
        local.get 0
        local.get 2
        i32.load8_u offset=12
        i32.store8 offset=1
      end
      local.get 0
      local.get 1
      i32.store8
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E (;37;) (type 4) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      local.get 3
      local.get 4
      i32.const 4
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush10wit_import17hd0e6bb9bc2c2ef7aE
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load8_u offset=4
          br_if 0 (;@2;)
          local.get 0
          i32.const 2
          i32.store
          br 1 (;@1;)
        end
        local.get 0
        i64.const 1
        local.get 4
        i64.load32_u offset=12
        i64.const 32
        i64.shl
        local.get 4
        i32.load8_u offset=8
        select
        i64.store align=4
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E (;38;) (type 6) (param i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      local.get 1
      i32.const 14
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types21filesystem_error_code10wit_import17h2b5d8e99a65d0583E
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load8_u offset=14
          br_if 0 (;@2;)
          i32.const 29
          local.set 2
          br 1 (;@1;)
        end
        local.get 1
        i32.load8_u offset=15
        call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
        local.set 2
      end
      block ;; label = @1
        local.get 0
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        call $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
    )
    (func $_ZN4core3ptr68drop_in_place$LT$wasi_snapshot_preview1..descriptors..Descriptor$GT$17h4b6e45103ecd4053E (;39;) (type 1) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=8
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=12
          local.tee 1
          i32.const -1
          i32.eq
          br_if 0 (;@2;)
          local.get 1
          call $_ZN136_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..InputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h5547e0ecd980db5bE
        end
        block ;; label = @2
          local.get 0
          i32.load offset=16
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=20
          local.tee 1
          i32.const -1
          i32.eq
          br_if 0 (;@2;)
          local.get 1
          call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
        end
        local.get 0
        i32.load8_u offset=41
        i32.const 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.tee 0
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        call $_ZN141_$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..Descriptor$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h34b1c3918a0eebe8E
      end
    )
    (func $fd_write (;40;) (type 8) (param i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            call $get_allocation_state
            i32.const -2
            i32.add
            br_table 1 (;@2;) 0 (;@3;) 1 (;@2;) 0 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.store
          i32.const 29
          local.set 1
          br 1 (;@1;)
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        local.get 2
                        i32.const 2
                        i32.lt_u
                        br_if 0 (;@9;)
                        local.get 1
                        local.get 2
                        i32.const 3
                        i32.shl
                        i32.add
                        i32.const -8
                        i32.add
                        local.set 5
                        loop ;; label = @10
                          local.get 1
                          i32.load offset=4
                          local.tee 6
                          br_if 3 (;@7;)
                          local.get 1
                          i32.const 8
                          i32.add
                          local.set 1
                          local.get 2
                          i32.const -1
                          i32.add
                          local.tee 2
                          i32.const 1
                          i32.gt_u
                          br_if 0 (;@10;)
                        end
                        local.get 5
                        local.set 1
                        br 1 (;@8;)
                      end
                      local.get 2
                      i32.eqz
                      br_if 2 (;@6;)
                    end
                    local.get 1
                    i32.load offset=4
                    local.set 6
                  end
                  local.get 1
                  i32.load
                  local.set 7
                  call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                  local.tee 1
                  i32.load
                  i32.const 560490357
                  i32.ne
                  br_if 1 (;@5;)
                  local.get 1
                  i32.load offset=65532
                  i32.const 560490357
                  i32.ne
                  br_if 2 (;@4;)
                  local.get 4
                  i32.const 8
                  i32.add
                  local.get 1
                  call $_ZN22wasi_snapshot_preview15State11descriptors17hfc2c9cda66bef96dE
                  i32.const 8
                  local.set 1
                  local.get 4
                  i32.load offset=12
                  local.set 2
                  local.get 4
                  i32.load offset=8
                  local.tee 5
                  i32.load16_u offset=6144
                  local.get 0
                  i32.le_u
                  br_if 4 (;@2;)
                  local.get 5
                  local.get 0
                  i32.const 48
                  i32.mul
                  i32.add
                  local.tee 0
                  i32.load
                  i32.const 1
                  i32.ne
                  br_if 4 (;@2;)
                  local.get 4
                  i32.const 16
                  i32.add
                  local.get 0
                  i32.const 8
                  i32.add
                  call $_ZN22wasi_snapshot_preview111descriptors7Streams16get_write_stream17h51434d5cfb257d25E
                  block ;; label = @7
                    local.get 4
                    i32.load16_u offset=16
                    br_if 0 (;@7;)
                    local.get 4
                    i32.load offset=20
                    local.set 1
                    block ;; label = @8
                      local.get 0
                      i32.load8_u offset=41
                      local.tee 5
                      i32.const 2
                      i32.eq
                      br_if 0 (;@8;)
                      local.get 4
                      i32.const 16
                      i32.add
                      local.get 5
                      i32.const 1
                      i32.and
                      local.get 1
                      local.get 7
                      local.get 6
                      call $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E
                      local.get 4
                      i32.load16_u offset=16
                      br_if 1 (;@7;)
                      br 5 (;@3;)
                    end
                    local.get 4
                    i32.const 16
                    i32.add
                    i32.const 1
                    local.get 1
                    local.get 7
                    local.get 6
                    call $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E
                    local.get 4
                    i32.load16_u offset=16
                    i32.eqz
                    br_if 4 (;@3;)
                  end
                  local.get 4
                  i32.load16_u offset=18
                  local.set 1
                  br 4 (;@2;)
                end
                i32.const 0
                local.set 1
                local.get 3
                i32.const 0
                i32.store
                br 4 (;@1;)
              end
              i32.const 2745
              call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
              unreachable
            end
            i32.const 2746
            call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
            unreachable
          end
          local.get 4
          i32.load offset=20
          local.set 1
          block ;; label = @3
            local.get 0
            i32.load8_u offset=41
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 0
              i32.load8_u offset=40
              br_if 0 (;@4;)
              local.get 0
              local.get 0
              i64.load offset=32
              local.get 1
              i64.extend_i32_u
              i64.add
              i64.store offset=32
              br 1 (;@3;)
            end
            local.get 4
            i32.const 16
            i32.add
            local.get 0
            i32.const 24
            i32.add
            call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat17hbce2ab26bb1887cbE
            block ;; label = @4
              local.get 4
              i64.load offset=88
              i64.const 2
              i64.eq
              br_if 0 (;@4;)
              local.get 0
              local.get 4
              i64.load offset=32
              i64.store offset=32
              br 1 (;@3;)
            end
            local.get 4
            i32.load8_u offset=16
            call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
            local.set 1
            br 1 (;@2;)
          end
          local.get 3
          local.get 1
          i32.store
          i32.const 0
          local.set 1
        end
        local.get 2
        local.get 2
        i32.load
        i32.const 1
        i32.add
        i32.store
      end
      local.get 4
      i32.const 112
      i32.add
      global.set $__stack_pointer
      local.get 1
      i32.const 65535
      i32.and
    )
    (func $_ZN22wasi_snapshot_preview111descriptors7Streams16get_write_stream17h51434d5cfb257d25E (;41;) (type 2) (param i32 i32)
      (local i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.const 12
      i32.add
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.load offset=8
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.load8_u offset=33
                    i32.const 2
                    i32.eq
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 1
                      i32.load8_u offset=20
                      i32.const 3
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 8
                      i32.store16 offset=2
                      br 3 (;@5;)
                    end
                    block ;; label = @8
                      local.get 1
                      i32.load8_u offset=32
                      br_if 0 (;@8;)
                      local.get 1
                      i64.load offset=24
                      local.set 4
                      local.get 2
                      i64.const 0
                      i64.store offset=8
                      local.get 1
                      i32.load offset=16
                      local.get 4
                      local.get 2
                      i32.const 8
                      i32.add
                      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor16write_via_stream10wit_import17h863d15eb6a8c37dcE
                      local.get 2
                      i32.load8_u offset=8
                      br_if 2 (;@6;)
                      br 4 (;@4;)
                    end
                    local.get 2
                    i32.const 8
                    i32.add
                    local.get 1
                    i32.const 16
                    i32.add
                    call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream17h9367b14712760239E
                    local.get 2
                    i32.load8_u offset=8
                    i32.eqz
                    br_if 3 (;@4;)
                    local.get 0
                    local.get 2
                    i32.load8_u offset=9
                    call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
                    i32.store16 offset=2
                    br 2 (;@5;)
                  end
                  local.get 0
                  i32.const 8
                  i32.store16 offset=2
                  br 1 (;@5;)
                end
                local.get 0
                local.get 2
                i32.load8_u offset=12
                call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
                i32.store16 offset=2
              end
              i32.const 1
              local.set 1
              br 2 (;@2;)
            end
            local.get 2
            i32.load offset=12
            local.set 5
            local.get 1
            i32.load offset=8
            local.tee 6
            br_if 2 (;@1;)
            block ;; label = @4
              local.get 6
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              i32.load
              local.tee 6
              i32.const -1
              i32.eq
              br_if 0 (;@4;)
              local.get 6
              call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
            end
            local.get 1
            local.get 5
            i32.store offset=12
            local.get 1
            i32.const 1
            i32.store offset=8
          end
          local.get 0
          local.get 3
          i32.store offset=4
          i32.const 0
          local.set 1
        end
        local.get 0
        local.get 1
        i32.store16
        local.get 2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 159
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E (;42;) (type 5) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.eqz
          br_if 0 (;@2;)
          local.get 4
          local.set 1
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                loop ;; label = @6
                  local.get 5
                  local.get 2
                  local.get 3
                  local.get 1
                  i32.const 4096
                  local.get 1
                  i32.const 4096
                  i32.lt_u
                  select
                  local.tee 6
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E
                  block ;; label = @7
                    local.get 5
                    i32.load
                    local.tee 7
                    i32.const 2
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 7
                    br_table 2 (;@5;) 3 (;@4;) 2 (;@5;)
                  end
                  local.get 3
                  local.get 6
                  i32.add
                  local.set 3
                  local.get 1
                  local.get 6
                  i32.sub
                  local.tee 1
                  br_if 0 (;@6;)
                end
                local.get 0
                i32.const 0
                i32.store16
                local.get 0
                local.get 4
                i32.store offset=4
                br 4 (;@1;)
              end
              local.get 5
              i32.load offset=4
              call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
              local.set 1
              br 1 (;@3;)
            end
            i32.const 29
            local.set 1
          end
          local.get 0
          i32.const 1
          i32.store16
          local.get 0
          local.get 1
          i32.store16 offset=2
          br 1 (;@1;)
        end
        local.get 2
        i32.load
        local.get 5
        call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream11check_write10wit_import17hdb0f94e542b67356E
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 5
                      i32.load8_u
                      br_if 0 (;@8;)
                      local.get 5
                      i32.load offset=8
                      local.set 1
                      br 1 (;@7;)
                    end
                    i32.const 0
                    local.set 1
                    i64.const 1
                    local.get 5
                    i64.load32_u offset=12
                    i64.const 32
                    i64.shl
                    local.get 5
                    i32.load8_u offset=8
                    select
                    local.tee 8
                    i32.wrap_i64
                    i32.const 1
                    i32.ne
                    br_if 1 (;@6;)
                  end
                  local.get 2
                  i32.load
                  local.get 3
                  local.get 4
                  local.get 1
                  local.get 4
                  local.get 1
                  i32.lt_u
                  select
                  local.tee 1
                  local.get 5
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream5write10wit_import17h1ec4b5645a67d72bE
                  local.get 5
                  i32.load8_u
                  br_if 2 (;@4;)
                  local.get 2
                  i32.load
                  local.get 5
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream14blocking_flush10wit_import17h7bbaef6ef9f5f783E
                  local.get 5
                  i32.load8_u
                  br_if 1 (;@5;)
                  local.get 0
                  i32.const 0
                  i32.store16
                  local.get 0
                  local.get 1
                  i32.store offset=4
                  br 5 (;@1;)
                end
                local.get 8
                i64.const 32
                i64.shr_u
                i32.wrap_i64
                call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
                local.set 1
                local.get 0
                i32.const 1
                i32.store16
                local.get 0
                local.get 1
                i32.store16 offset=2
                br 4 (;@1;)
              end
              i64.const 1
              local.get 5
              i64.load32_u offset=8
              i64.const 32
              i64.shl
              local.get 5
              i32.load8_u offset=4
              select
              local.tee 8
              i64.const 1
              i64.and
              i64.eqz
              br_if 1 (;@3;)
              i32.const 0
              local.set 1
              local.get 0
              i32.const 0
              i32.store offset=4
              br 2 (;@2;)
            end
            block ;; label = @4
              block ;; label = @5
                i64.const 1
                local.get 5
                i64.load32_u offset=8
                i64.const 32
                i64.shl
                local.get 5
                i32.load8_u offset=4
                select
                local.tee 8
                i64.const 1
                i64.and
                i64.eqz
                br_if 0 (;@5;)
                i32.const 0
                local.set 1
                local.get 0
                i32.const 0
                i32.store offset=4
                br 1 (;@4;)
              end
              local.get 0
              local.get 8
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
              i32.store16 offset=2
              i32.const 1
              local.set 1
            end
            local.get 0
            local.get 1
            i32.store16
            br 2 (;@1;)
          end
          local.get 0
          local.get 8
          i64.const 32
          i64.shr_u
          i32.wrap_i64
          call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
          i32.store16 offset=2
          i32.const 1
          local.set 1
        end
        local.get 0
        local.get 1
        i32.store16
      end
      local.get 5
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $proc_exit (;43;) (type 1) (param i32)
      (local i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.const 0
      i32.ne
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit17h506fb275a8b6a599E
      i32.const 2280
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get 1
      i32.const 8250
      i32.store16 offset=10 align=1
      local.get 1
      i32.const 10
      i32.add
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 1
      i32.const 2593
      i32.store16 offset=46 align=1
      local.get 1
      i32.const 1953069157
      i32.store offset=42 align=1
      local.get 1
      i64.const 2338537461596644384
      i64.store offset=34 align=1
      local.get 1
      i64.const 7957695015159098981
      i64.store offset=26 align=1
      local.get 1
      i64.const 7882825952909664372
      i64.store offset=18 align=1
      local.get 1
      i64.const 7599935561254793064
      i64.store offset=10 align=1
      local.get 1
      i32.const 10
      i32.add
      i32.const 38
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get 1
      i32.const 10
      i32.store8 offset=10
      local.get 1
      i32.const 10
      i32.add
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit17h506fb275a8b6a599E (;44;) (type 1) (param i32)
      local.get 0
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit10wit_import17hefacdcba211d0d02E
    )
    (func $_ZN22wasi_snapshot_preview15State3new17h27fd4b5328bc4eeeE (;45;) (type 9) (result i32)
      (local i32)
      block ;; label = @1
        call $get_allocation_state
        i32.const 2
        i32.ne
        br_if 0 (;@1;)
        i32.const 3
        call $set_allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $_ZN22wasi_snapshot_preview15State3new12cabi_realloc17h88297338052a23f0E
        local.set 0
        i32.const 4
        call $set_allocation_state
        local.get 0
        i32.const 2
        i32.store offset=6180
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        i64.const 17740359541
        i64.store
        local.get 0
        i32.const 65480
        i32.add
        i32.const 0
        i32.const 37
        call $memset
        drop
        local.get 0
        i32.const 560490357
        i32.store offset=65532
        local.get 0
        i32.const 11822
        i32.store16 offset=65528
        local.get 0
        i32.const 0
        i32.store offset=65520
        local.get 0
        return
      end
      i32.const 2777
      call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview111descriptors11Descriptors3new17h23d25887e02c514fE (;46;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 6256
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 0
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=6156
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli5stdin9get_stdin10wit_import17he5bdc61203ca72fdE
      local.set 4
      local.get 2
      i32.const 2
      i32.store8 offset=49
      local.get 2
      i32.const 0
      i32.store8 offset=32
      local.get 2
      i64.const 0
      i64.store offset=24
      local.get 2
      i32.const 1
      i32.store offset=8
      local.get 2
      local.get 4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=16
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stdout10get_stdout10wit_import17hfdc059c6457e3d59E
      local.set 4
      local.get 2
      i32.const 2
      i32.store8 offset=97
      local.get 2
      i32.const 1
      i32.store8 offset=80
      local.get 2
      i64.const 0
      i64.store offset=64
      local.get 2
      i32.const 1
      i32.store offset=56
      local.get 2
      local.get 4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=72
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E
      local.set 4
      local.get 2
      i32.const 3
      i32.store16 offset=6152
      local.get 2
      i32.const 2
      i32.store8 offset=145
      local.get 2
      i32.const 2
      i32.store8 offset=128
      local.get 2
      i64.const 0
      i64.store offset=112
      local.get 2
      i32.const 1
      i32.store offset=104
      local.get 2
      local.get 4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=120
      local.get 2
      i32.const 59032
      i32.store offset=6184
      local.get 2
      local.get 1
      i32.const 6192
      i32.add
      i32.store offset=6180
      local.get 2
      i64.const 1
      i64.store offset=6172 align=4
      local.get 2
      i32.const 6208
      i32.add
      local.get 1
      local.get 2
      i32.const 6172
      i32.add
      call $_ZN22wasi_snapshot_preview15State17with_import_alloc17h8e53b3b47ee99160E
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.load offset=6212
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=6208
            local.set 1
            local.get 2
            i32.const 152
            i32.add
            local.set 6
            local.get 2
            i32.const 6208
            i32.add
            i32.const 8
            i32.add
            local.set 4
            loop ;; label = @4
              local.get 2
              i32.const 6192
              i32.add
              i32.const 8
              i32.add
              local.get 1
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get 2
              local.get 1
              i64.load align=4
              i64.store offset=6192
              local.get 2
              local.get 2
              i32.const 6192
              i32.add
              call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type17h38d62f575f54468dE
              local.get 2
              i32.load8_u
              br_if 2 (;@2;)
              local.get 2
              i32.load8_u offset=1
              local.set 7
              local.get 4
              i64.const 0
              i64.store
              local.get 4
              i32.const 8
              i32.add
              i64.const 0
              i64.store
              local.get 2
              local.get 2
              i32.load offset=6200
              i32.store offset=6252
              local.get 2
              i32.const 256
              i32.store16 offset=6248
              local.get 2
              i64.const 0
              i64.store offset=6240
              local.get 2
              local.get 2
              i32.load offset=6192
              i32.store offset=6232
              local.get 2
              i32.const 1
              i32.store offset=6208
              local.get 2
              local.get 7
              i32.store8 offset=6236
              local.get 3
              i32.const 125
              i32.eq
              br_if 3 (;@1;)
              local.get 6
              local.get 2
              i32.const 6208
              i32.add
              i32.const 48
              call $memcpy
              local.set 6
              local.get 2
              local.get 3
              i32.const 4
              i32.add
              i32.store16 offset=6152
              local.get 1
              i32.const 12
              i32.add
              local.set 1
              local.get 6
              i32.const 48
              i32.add
              local.set 6
              local.get 3
              i32.const 1
              i32.add
              local.tee 7
              local.set 3
              local.get 5
              local.get 7
              i32.ne
              br_if 0 (;@4;)
            end
          end
          local.get 0
          local.get 2
          i32.const 8
          i32.add
          i32.const 6160
          call $memcpy
          drop
          local.get 2
          i32.const 6256
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 159
        call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
        unreachable
      end
      local.get 2
      i32.const 6208
      i32.add
      call $_ZN4core3ptr68drop_in_place$LT$wasi_snapshot_preview1..descriptors..Descriptor$GT$17h4b6e45103ecd4053E
      i32.const 159
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE (;47;) (type 1) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 10
        i32.div_u
        local.tee 2
        call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
        local.get 1
        local.get 2
        i32.const 246
        i32.mul
        local.get 0
        i32.add
        i32.const 48
        i32.or
        i32.store8 offset=15
        local.get 1
        i32.const 15
        i32.add
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros10eprint_u3217h80b476442303f1eeE (;48;) (type 1) (param i32)
      local.get 0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
    )
    (func $get_state_ptr (;49;) (type 9) (result i32)
      global.get $internal_state_ptr
    )
    (func $set_state_ptr (;50;) (type 1) (param i32)
      local.get 0
      global.set $internal_state_ptr
    )
    (func $get_allocation_state (;51;) (type 9) (result i32)
      global.get $allocation_state
    )
    (func $set_allocation_state (;52;) (type 1) (param i32)
      local.get 0
      global.set $allocation_state
    )
    (func $memset (;53;) (type 10) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 16
          i32.ge_u
          br_if 0 (;@2;)
          local.get 0
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.set 5
        block ;; label = @2
          local.get 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.set 3
          loop ;; label = @3
            local.get 3
            local.get 1
            i32.store8
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            local.get 5
            i32.lt_u
            br_if 0 (;@3;)
          end
        end
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        local.tee 4
        i32.const -4
        i32.and
        local.tee 2
        i32.add
        local.set 3
        block ;; label = @2
          local.get 2
          i32.const 1
          i32.lt_s
          br_if 0 (;@2;)
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 2
          loop ;; label = @3
            local.get 5
            local.get 2
            i32.store
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@3;)
          end
        end
        local.get 4
        i32.const 3
        i32.and
        local.set 2
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        local.get 2
        i32.add
        local.set 5
        loop ;; label = @2
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
    )
    (func $memcpy (;54;) (type 10) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 16
          i32.ge_u
          br_if 0 (;@2;)
          local.get 0
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.set 5
        block ;; label = @2
          local.get 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.set 3
          local.get 1
          local.set 6
          loop ;; label = @3
            local.get 3
            local.get 6
            i32.load8_u
            i32.store8
            local.get 6
            i32.const 1
            i32.add
            local.set 6
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            local.get 5
            i32.lt_u
            br_if 0 (;@3;)
          end
        end
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        local.tee 7
        i32.const -4
        i32.and
        local.tee 8
        i32.add
        local.set 3
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 4
            i32.add
            local.tee 9
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            i32.const 1
            i32.lt_s
            br_if 1 (;@2;)
            local.get 9
            i32.const 3
            i32.shl
            local.tee 6
            i32.const 24
            i32.and
            local.set 2
            local.get 9
            i32.const -4
            i32.and
            local.tee 10
            i32.const 4
            i32.add
            local.set 1
            i32.const 0
            local.get 6
            i32.sub
            i32.const 24
            i32.and
            local.set 4
            local.get 10
            i32.load
            local.set 6
            loop ;; label = @4
              local.get 5
              local.get 6
              local.get 2
              i32.shr_u
              local.get 1
              i32.load
              local.tee 6
              local.get 4
              i32.shl
              i32.or
              i32.store
              local.get 1
              i32.const 4
              i32.add
              local.set 1
              local.get 5
              i32.const 4
              i32.add
              local.tee 5
              local.get 3
              i32.lt_u
              br_if 0 (;@4;)
              br 2 (;@2;)
            end
          end
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 0 (;@2;)
          local.get 9
          local.set 1
          loop ;; label = @3
            local.get 5
            local.get 1
            i32.load
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@3;)
          end
        end
        local.get 7
        i32.const 3
        i32.and
        local.set 2
        local.get 9
        local.get 8
        i32.add
        local.set 1
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        local.get 2
        i32.add
        local.set 5
        loop ;; label = @2
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
    )
    (func $allocate_stack (;55;) (type 12)
      global.get $allocation_state
      i32.const 0
      i32.eq
      if ;; label = @1
        i32.const 1
        global.set $allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $_ZN22wasi_snapshot_preview15State3new12cabi_realloc17h88297338052a23f0E
        i32.const 65536
        i32.add
        global.set $__stack_pointer
        i32.const 2
        global.set $allocation_state
      end
    )
    (@producers
      (language "Rust" "")
      (processed-by "rustc" "1.83.0 (90b35a623 2024-11-26)")
    )
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (type (;2;) (func (param i32)))
    (type (;3;) (func (param i32)))
    (type (;4;) (func (param i32 i32)))
    (type (;5;) (func (param i32 i64 i32)))
    (type (;6;) (func (param i32 i32 i32 i32)))
    (table (;0;) 15 15 funcref)
    (export "0" (func $adapt-wasi_snapshot_preview1-fd_write))
    (export "1" (func $adapt-wasi_snapshot_preview1-environ_get))
    (export "2" (func $adapt-wasi_snapshot_preview1-environ_sizes_get))
    (export "3" (func $adapt-wasi_snapshot_preview1-proc_exit))
    (export "4" (func $indirect-wasi:cli/environment@0.2.3-get-environment))
    (export "5" (func $indirect-wasi:filesystem/types@0.2.3-filesystem-error-code))
    (export "6" (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.write-via-stream"))
    (export "7" (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.append-via-stream"))
    (export "8" (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.get-type"))
    (export "9" (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.stat"))
    (export "10" (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.check-write"))
    (export "11" (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.write"))
    (export "12" (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-flush"))
    (export "13" (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-write-and-flush"))
    (export "14" (func $indirect-wasi:filesystem/preopens@0.2.2-get-directories))
    (export "$imports" (table 0))
    (func $adapt-wasi_snapshot_preview1-fd_write (;0;) (type 0) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 0
      call_indirect (type 0)
    )
    (func $adapt-wasi_snapshot_preview1-environ_get (;1;) (type 1) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 1
      call_indirect (type 1)
    )
    (func $adapt-wasi_snapshot_preview1-environ_sizes_get (;2;) (type 1) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 2
      call_indirect (type 1)
    )
    (func $adapt-wasi_snapshot_preview1-proc_exit (;3;) (type 2) (param i32)
      local.get 0
      i32.const 3
      call_indirect (type 2)
    )
    (func $indirect-wasi:cli/environment@0.2.3-get-environment (;4;) (type 3) (param i32)
      local.get 0
      i32.const 4
      call_indirect (type 3)
    )
    (func $indirect-wasi:filesystem/types@0.2.3-filesystem-error-code (;5;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 5
      call_indirect (type 4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.write-via-stream" (;6;) (type 5) (param i32 i64 i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 6
      call_indirect (type 5)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.append-via-stream" (;7;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 7
      call_indirect (type 4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.get-type" (;8;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 8
      call_indirect (type 4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.stat" (;9;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 9
      call_indirect (type 4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.check-write" (;10;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 10
      call_indirect (type 4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.write" (;11;) (type 6) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 11
      call_indirect (type 6)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-flush" (;12;) (type 4) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 12
      call_indirect (type 4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-write-and-flush" (;13;) (type 6) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 13
      call_indirect (type 6)
    )
    (func $indirect-wasi:filesystem/preopens@0.2.2-get-directories (;14;) (type 3) (param i32)
      local.get 0
      i32.const 14
      call_indirect (type 3)
    )
    (@producers
      (processed-by "wit-component" "0.223.0")
    )
  )
  (core module (;3;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (type (;2;) (func (param i32)))
    (type (;3;) (func (param i32)))
    (type (;4;) (func (param i32 i32)))
    (type (;5;) (func (param i32 i64 i32)))
    (type (;6;) (func (param i32 i32 i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 1)))
    (import "" "2" (func (;2;) (type 1)))
    (import "" "3" (func (;3;) (type 2)))
    (import "" "4" (func (;4;) (type 3)))
    (import "" "5" (func (;5;) (type 4)))
    (import "" "6" (func (;6;) (type 5)))
    (import "" "7" (func (;7;) (type 4)))
    (import "" "8" (func (;8;) (type 4)))
    (import "" "9" (func (;9;) (type 4)))
    (import "" "10" (func (;10;) (type 4)))
    (import "" "11" (func (;11;) (type 6)))
    (import "" "12" (func (;12;) (type 4)))
    (import "" "13" (func (;13;) (type 6)))
    (import "" "14" (func (;14;) (type 3)))
    (import "" "$imports" (table (;0;) 15 15 funcref))
    (elem (;0;) (i32.const 0) func 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14)
    (@producers
      (processed-by "wit-component" "0.223.0")
    )
  )
  (core instance (;0;) (instantiate 2))
  (alias core export 0 "0" (core func (;0;)))
  (alias core export 0 "1" (core func (;1;)))
  (alias core export 0 "2" (core func (;2;)))
  (alias core export 0 "3" (core func (;3;)))
  (core instance (;1;)
    (export "fd_write" (func 0))
    (export "environ_get" (func 1))
    (export "environ_sizes_get" (func 2))
    (export "proc_exit" (func 3))
  )
  (type (;18;)
    (instance
      (type (;0;) (func (param "c" u8)))
      (export (;0;) "putc" (func (type 0)))
      (type (;1;) (func (param "a" u32) (param "l" u32)))
      (export (;1;) "puts" (func (type 1)))
      (type (;2;) (func (param "u" u8)))
      (export (;2;) "putu8" (func (type 2)))
      (type (;3;) (func (param "i" s8)))
      (export (;3;) "puti8" (func (type 3)))
      (type (;4;) (func (param "u" u16)))
      (export (;4;) "putu16" (func (type 4)))
      (type (;5;) (func (param "i" s16)))
      (export (;5;) "puti16" (func (type 5)))
      (type (;6;) (func (param "u" u32)))
      (export (;6;) "putu32" (func (type 6)))
      (type (;7;) (func (param "i" s32)))
      (export (;7;) "puti32" (func (type 7)))
      (type (;8;) (func (param "u" u64)))
      (export (;8;) "putu64" (func (type 8)))
      (type (;9;) (func (param "i" s64)))
      (export (;9;) "puti64" (func (type 9)))
      (type (;10;) (func (param "f" f32)))
      (export (;10;) "putf32" (func (type 10)))
      (type (;11;) (func (param "f" f64)))
      (export (;11;) "putf64" (func (type 11)))
      (type (;12;) (func (param "u" u8)))
      (export (;12;) "putbool" (func (type 12)))
    )
  )
  (import "whamm-core" (instance $whamm-core (;10;) (type 18)))
  (alias export $whamm-core "putc" (func (;0;)))
  (core func $putc' (;4;) (canon lower (func 0)))
  (alias export $whamm-core "puts" (func (;1;)))
  (core func $puts' (;5;) (canon lower (func 1)))
  (alias export $whamm-core "putu8" (func (;2;)))
  (core func $putu8' (;6;) (canon lower (func 2)))
  (alias export $whamm-core "puti8" (func (;3;)))
  (core func $puti8' (;7;) (canon lower (func 3)))
  (alias export $whamm-core "putu16" (func (;4;)))
  (core func $putu16' (;8;) (canon lower (func 4)))
  (alias export $whamm-core "puti16" (func (;5;)))
  (core func $puti16' (;9;) (canon lower (func 5)))
  (alias export $whamm-core "putu32" (func (;6;)))
  (core func $putu32' (;10;) (canon lower (func 6)))
  (alias export $whamm-core "puti32" (func (;7;)))
  (core func $puti32' (;11;) (canon lower (func 7)))
  (alias export $whamm-core "putu64" (func (;8;)))
  (core func $putu64' (;12;) (canon lower (func 8)))
  (alias export $whamm-core "puti64" (func (;9;)))
  (core func $puti64' (;13;) (canon lower (func 9)))
  (alias export $whamm-core "putf32" (func (;10;)))
  (core func $putf32' (;14;) (canon lower (func 10)))
  (alias export $whamm-core "putf64" (func (;11;)))
  (core func $putf64' (;15;) (canon lower (func 11)))
  (alias export $whamm-core "putbool" (func (;12;)))
  (core func $putbool' (;16;) (canon lower (func 12)))
  (core instance (;2;)
    (export "putc" (func $putc'))
    (export "puts" (func $puts'))
    (export "putu8" (func $putu8'))
    (export "puti8" (func $puti8'))
    (export "putu16" (func $putu16'))
    (export "puti16" (func $puti16'))
    (export "putu32" (func $putu32'))
    (export "puti32" (func $puti32'))
    (export "putu64" (func $putu64'))
    (export "puti64" (func $puti64'))
    (export "putf32" (func $putf32'))
    (export "putf64" (func $putf64'))
    (export "putbool" (func $putbool'))
  )
  (core instance (;3;) (instantiate 0
      (with "wasi_snapshot_preview1" (instance 1))
      (with "whamm_core" (instance 2))
    )
  )
  (alias core export 3 "memory" (core memory (;0;)))
  (core instance (;4;)
    (export "memory" (memory 0))
  )
  (alias core export 3 "_start" (core func (;17;)))
  (alias core export 3 "cabi_realloc" (core func (;18;)))
  (core instance (;5;)
    (export "_start" (func 17))
    (export "cabi_realloc" (func 18))
  )
  (alias core export 0 "4" (core func (;19;)))
  (core instance (;6;)
    (export "get-environment" (func $putu8'))
  )
  (alias export 8 "descriptor" (type (;19;)))
  (core func (;20;) (canon resource.drop 19))
  (alias core export 0 "5" (core func (;21;)))
  (alias core export 0 "6" (core func (;22;)))
  (alias core export 0 "7" (core func (;23;)))
  (alias core export 0 "8" (core func (;24;)))
  (alias core export 0 "9" (core func (;25;)))
  (core instance (;7;)
    (export "[resource-drop]descriptor" (func 20))
    (export "filesystem-error-code" (func 21))
    (export "[method]descriptor.write-via-stream" (func 22))
    (export "[method]descriptor.append-via-stream" (func 23))
    (export "[method]descriptor.get-type" (func 24))
    (export "[method]descriptor.stat" (func 25))
  )
  (alias export 3 "output-stream" (type (;20;)))
  (core func (;26;) (canon resource.drop 19))
  (alias export 3 "input-stream" (type (;21;)))
  (core func (;27;) (canon resource.drop 20))
  (alias core export 0 "10" (core func (;28;)))
  (alias core export 0 "11" (core func (;29;)))
  (alias core export 0 "12" (core func (;30;)))
  (alias core export 0 "13" (core func (;31;)))
  (core instance (;8;)
    (export "[resource-drop]output-stream" (func 26))
    (export "[resource-drop]input-stream" (func 27))
    (export "[method]output-stream.check-write" (func 28))
    (export "[method]output-stream.write" (func 29))
    (export "[method]output-stream.blocking-flush" (func 30))
    (export "[method]output-stream.blocking-write-and-flush" (func 31))
  )
  (alias export 3 "error" (type (;22;)))
  (core func (;32;) (canon resource.drop 21))
  (core instance (;9;)
    (export "[resource-drop]error" (func 19))
  )
  (alias core export 0 "14" (core func (;33;)))
  (core instance (;10;)
    (export "get-directories" (func 33))
  )
  (alias export 6 "get-stderr" (func (;13;)))
  (core func (;34;) (canon lower (func 13)))
  (core instance (;11;)
    (export "get-stderr" (func 34))
  )
  (alias export 4 "get-stdin" (func (;14;)))
  (core func (;35;) (canon lower (func 14)))
  (core instance (;12;)
    (export "get-stdin" (func 35))
  )
  (alias export 5 "get-stdout" (func (;15;)))
  (core func (;36;) (canon lower (func 15)))
  (core instance (;13;)
    (export "get-stdout" (func 36))
  )
  (alias export 1 "exit" (func (;16;)))
  (core func (;37;) (canon lower (func 16)))
  (core instance (;14;)
    (export "exit" (func 37))
  )
  (core instance (;15;) (instantiate 1
      (with "env" (instance 3))
      (with "__main_module__" (instance 5))
      (with "wasi:cli/environment@0.2.3" (instance 6))
      (with "wasi:filesystem/types@0.2.3" (instance 7))
      (with "wasi:io/streams@0.2.3" (instance 8))
      (with "wasi:io/error@0.2.3" (instance 9))
      (with "wasi:filesystem/preopens@0.2.2" (instance 10))
      (with "wasi:cli/stderr@0.2.3" (instance 11))
      (with "wasi:cli/stdin@0.2.3" (instance 12))
      (with "wasi:cli/stdout@0.2.3" (instance 13))
      (with "wasi:cli/exit@0.2.3" (instance 14))
    )
  )
  (alias core export 0 "$imports" (core table (;0;)))
  (alias core export 15 "fd_write" (core func (;38;)))
  (alias core export 15 "environ_get" (core func (;39;)))
  (alias core export 15 "environ_sizes_get" (core func (;40;)))
  (alias core export 15 "proc_exit" (core func (;41;)))
  (alias export 0 "get-environment" (func (;17;)))
  (alias core export 15 "cabi_import_realloc" (core func (;42;)))
  (core func (;43;) (canon lower (func 4) (memory 0) (realloc 42) string-encoding=utf8))
  (alias export 8 "filesystem-error-code" (func (;18;)))
  (core func (;44;) (canon lower (func 18) (memory 0)))
  (alias export 8 "[method]descriptor.write-via-stream" (func (;19;)))
  (core func (;45;) (canon lower (func 19) (memory 0)))
  (alias export 8 "[method]descriptor.append-via-stream" (func (;20;)))
  (core func (;46;) (canon lower (func 20) (memory 0)))
  (alias export 8 "[method]descriptor.get-type" (func (;21;)))
  (core func (;47;) (canon lower (func 21) (memory 0)))
  (alias export 8 "[method]descriptor.stat" (func (;22;)))
  (core func (;48;) (canon lower (func 22) (memory 0)))
  (alias export 3 "[method]output-stream.check-write" (func (;23;)))
  (core func (;49;) (canon lower (func 23) (memory 0)))
  (alias export 3 "[method]output-stream.write" (func (;24;)))
  (core func (;50;) (canon lower (func 24) (memory 0)))
  (alias export 3 "[method]output-stream.blocking-flush" (func (;25;)))
  (core func (;51;) (canon lower (func 25) (memory 0)))
  (alias export 3 "[method]output-stream.blocking-write-and-flush" (func (;26;)))
  (core func (;52;) (canon lower (func 26) (memory 0)))
  (alias export 9 "get-directories" (func (;27;)))
  (core func (;53;) (canon lower (func 27) (memory 0) (realloc 42) string-encoding=utf8))
  (core instance (;16;)
    (export "$imports" (table 0))
    (export "0" (func 38))
    (export "1" (func 39))
    (export "2" (func 40))
    (export "3" (func 41))
    (export "4" (func 43))
    (export "5" (func 44))
    (export "6" (func 45))
    (export "7" (func 46))
    (export "8" (func 47))
    (export "9" (func 48))
    (export "10" (func 49))
    (export "11" (func 50))
    (export "12" (func 51))
    (export "13" (func 52))
    (export "14" (func 53))
  )
  (core instance (;17;) (instantiate 3
      (with "" (instance 16))
    )
  )
  (type (;23;) (result))
  (type (;24;) (func (result 23)))
  (alias core export 15 "wasi:cli/run@0.2.3#run" (core func (;54;)))
  (func (;28;) (type 24) (canon lift (core func 54)))
  (component (;0;)
    (type (;0;) (result))
    (type (;1;) (func (result 0)))
    (import "import-func-run" (func (;0;) (type 1)))
    (type (;2;) (result))
    (type (;3;) (func (result 2)))
    (export (;1;) "run" (func 0) (func (type 3)))
  )
  (instance (;11;) (instantiate 0
      (with "import-func-run" (func 28))
    )
  )
  (export (;12;) "wasi:cli/run@0.2.3" (instance 11))
  (@producers
    (processed-by "wit-component" "0.223.0")
  )
)
