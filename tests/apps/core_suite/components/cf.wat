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
    (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h5858df6e6eba6e92E (;0;) (type 9)))
    (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;1;) (type 2)))
    (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;2;) (type 2)))
    (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;3;) (type 1)))
    (table (;0;) 70 70 funcref)
    (memory (;0;) 17)
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
    (export "memory" (memory 0))
    (export "_start" (func $_start))
    (export "__main_void" (func $__main_void))
    (export "calc" (func $calc))
    (export "print_x" (func $print_x))
    (export "opt_str" (func $opt_str))
    (export "main" (func $main))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE $main $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E $"#func53 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E $cabi_realloc $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E $_ZN4core3fmt5Write10write_char17ha202ea95069de918E $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E)
    (func $__wasm_call_ctors (;4;) (type 0))
    (func $_start (;5;) (type 0)
      (local i32)
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
          call $__wasm_call_ctors
          call $__main_void
          local.set 0
          call $__wasm_call_dtors
          local.get 0
          br_if 1 (;@1;)
          return
        end
        unreachable
      end
      local.get 0
      call $__wasi_proc_exit
      unreachable
    )
    (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE (;6;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E (;7;) (type 1) (param i32)
      local.get 0
      call_indirect (type 0)
    )
    (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E (;8;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE (;9;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.get 1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $calc (;10;) (type 2) (param i32 i32) (result i32)
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
    (func $print_x (;11;) (type 6) (param i32 i32)
      (local i32 i64)
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
    (func $opt_str (;12;) (type 6) (param i32 i32)
      (local i32 i32)
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
          call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
          unreachable
        end
        i32.const 1
        i32.const 5
        global.get $GOT.data.internal.__memory_base
        i32.const 1054136
        i32.add
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 1
      i32.const 4
      global.get $GOT.data.internal.__memory_base
      i32.const 1054136
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $main (;13;) (type 0)
      (local i32 i32 i32 i64 i64 i64)
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 0
      i32.const 47
      i32.add
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
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block ;; label = @1
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 80
      i32.add
      global.set $__stack_pointer
    )
    (func $__main_void (;14;) (type 10) (result i32)
      (local i32 i32)
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
      call $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E
      local.set 1
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc (;15;) (type 2) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      call $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc (;16;) (type 4) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc (;17;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc
      local.set 4
      local.get 4
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler (;18;) (type 6) (param i32 i32)
      local.get 0
      local.get 1
      call $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom
      return
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E (;19;) (type 6) (param i32 i32)
      local.get 0
      i64.const -245993367077761921
      i64.store offset=8
      local.get 0
      i64.const 6756087622182587336
      i64.store
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E (;20;) (type 6) (param i32 i32)
      local.get 0
      i64.const 7199936582794304877
      i64.store offset=8
      local.get 0
      i64.const -5076933981314334344
      i64.store
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E (;21;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
          call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E
          return
        end
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E (;22;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E (;23;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
          call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E
          local.set 0
          br 1 (;@1;)
        end
        local.get 1
        global.get $GOT.data.internal.__memory_base
        i32.const 1049034
        i32.add
        i32.const 4
        call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
        local.set 0
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E (;24;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      local.get 1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE (;25;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i64)
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
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
      local.set 1
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE (;26;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
          call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
          return
        end
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE
    )
    (func $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE (;27;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i64 i32 i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
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
    (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E (;28;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
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
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17ha202ea95069de918E (;29;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i64 i32 i32 i64)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
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
    (func $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE (;30;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
      call $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE (;31;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i64 i32)
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
                    call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  end
                  local.get 1
                  i32.const 12
                  i32.const 4
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
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E (;32;) (type 2) (param i32 i32) (result i32)
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
    (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E (;33;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i64 i32)
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
    (func $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E (;34;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054380
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E (;35;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054428
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E (;36;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054404
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE (;37;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054452
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE (;38;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054356
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN3std9panicking12default_hook17h8deeaf2f2b300de3E (;39;) (type 1) (param i32)
      (local i32 i32 i32)
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
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E (;40;) (type 1) (param i32)
      (local i32 i32 i32)
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
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE (;41;) (type 1) (param i32)
      (local i32 i32)
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
        call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
      end
    )
    (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E (;42;) (type 1) (param i32)
      (local i32)
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
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E (;43;) (type 1) (param i32)
      (local i32)
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
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E (;44;) (type 1) (param i32)
      (local i32)
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
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE (;45;) (type 1) (param i32)
      (local i32)
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
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E (;46;) (type 6) (param i32 i32)
      (local i32 i32)
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
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE (;47;) (type 6) (param i32 i32)
      local.get 0
      i32.const 0
      i32.store
    )
    (func $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E (;48;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32)
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
      call $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E
      unreachable
    )
    (func $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE (;49;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 2)
    )
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E (;50;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
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
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E (;51;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
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
    (func $_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E (;52;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $"#func53 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" (@name "_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE") (;53;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.get 1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E (;54;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
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
      call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E (;55;) (type 1) (param i32)
      (local i32 i32 i64 i64 i64 i32 i32 i32 i32 i32)
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
                    call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                    unreachable
                  end
                  call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
                  unreachable
                end
                global.get $GOT.data.internal.__memory_base
                i32.const 1054632
                i32.add
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
                call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              end
              local.get 0
              i32.const 12
              i32.const 4
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
    (func $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E (;56;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i64 i64 i32 i64)
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
          call $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E
        end
        local.get 5
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E (;57;) (type 0)
      (local i32 i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E (;58;) (type 4) (param i32 i32 i32)
      (local i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
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
    (func $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE (;59;) (type 0)
      call $abort
      unreachable
    )
    (func $_ZN3std3env11current_dir17h890335e8528685e2E (;60;) (type 1) (param i32)
      (local i32 i32 i32 i32 i32)
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
                call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
                local.get 1
                i32.load offset=8
                local.tee 3
                local.get 1
                i32.load offset=4
                local.tee 2
                call $getcwd
                i32.eqz
                br_if 0 (;@5;)
              end
            end
            local.get 1
            local.get 3
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
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  br 1 (;@5;)
                end
                local.get 3
                local.get 2
                i32.const 1
                local.get 4
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
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 1
      local.get 4
      global.get $GOT.data.internal.__memory_base
      i32.const 1055316
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3env7_var_os17hecfa64e4c3898426E (;61;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
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
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 2
            i32.const 12
            i32.const 4
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
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE (;62;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      local.get 2
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
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E (;63;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
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
        call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
        unreachable
      end
      local.get 8
      local.get 3
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E (;64;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32)
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
            call $strerror_r
            i32.const 0
            i32.lt_s
            br_if 0 (;@3;)
            local.get 2
            i32.const 1024
            i32.add
            local.get 2
            local.get 2
            call $strlen
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
        call $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE
        unreachable
      end
      local.get 3
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E (;65;) (type 2) (param i32 i32) (result i32)
      (local i32 i32)
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
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E (;66;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E (;67;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
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
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE (;68;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E (;69;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E (;70;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
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
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E (;71;) (type 6) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE (;72;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i64 i32)
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
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get 8
          local.get 3
          global.get $GOT.data.internal.__memory_base
          i32.const 1054720
          i32.add
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
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE (;73;) (type 0)
      (local i32 i32 i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E (;74;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i64 i64 i64)
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
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E (;75;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.load
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
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
    (func $_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE (;76;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32)
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
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 0
            i32.const 12
            i32.const 4
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
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E (;77;) (type 4) (param i32 i32 i32)
      (local i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
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
    (func $_ZN3std2io5stdio6_print17h68847dc224af8aecE (;78;) (type 1) (param i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE (;79;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i64 i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
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
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E (;80;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i64 i32 i32 i64)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
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
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E (;81;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
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
    (func $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE (;82;) (type 7) (param i32 i32 i32 i32)
      (local i32 i64 i32)
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
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h1459a55272857e0fE (;83;) (type 4) (param i32 i32 i32)
      (local i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
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
    (func $_ZN3std5panic19get_backtrace_style17h54380a7febe96116E (;84;) (type 10) (result i32)
      (local i32 i32 i32 i32 i32)
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
    (func $_ZN3std7process5abort17had5be8ae244d01ebE (;85;) (type 0)
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E (;86;) (type 1) (param i32)
      (local i32)
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
    (func $_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E (;87;) (type 10) (result i32)
      (local i32 i32 i32)
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
    (func $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E (;88;) (type 7) (param i32 i32 i32 i32)
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
    (func $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE (;89;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i64 i32 i32 i32 i32)
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
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get 6
        i32.const 12
        i32.const 4
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
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE (;90;) (type 1) (param i32)
      local.get 0
      call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE
      unreachable
    )
    (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE (;91;) (type 1) (param i32)
      (local i32 i32 i32)
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
      call $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E
      unreachable
    )
    (func $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E (;92;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32)
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc (;93;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
        call $malloc
        local.set 1
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc (;94;) (type 4) (param i32 i32 i32)
      local.get 0
      call $free
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc (;95;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32 i32)
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
          call $free
          local.get 2
          local.set 5
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        call $realloc
        local.set 5
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 5
    )
    (func $_ZN3std9panicking14payload_as_str17h6b6acf98476ffb56E (;96;) (type 4) (param i32 i32 i32)
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
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE (;97;) (type 4) (param i32 i32 i32)
      (local i32 i32 i64 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
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
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br 1 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        i32.const 0
        local.get 3
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
            call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
            local.get 3
            i32.load8_u offset=8
            local.get 3
            i32.load offset=12
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
          call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
          local.get 3
          i32.load8_u offset=8
          local.get 3
          i32.load offset=12
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
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E (;98;) (type 4) (param i32 i32 i32)
      (local i32 i64 i64 i64 i32 i32)
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
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 1
            i32.const 12
            i32.const 4
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
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get 1
            i32.const 12
            i32.const 4
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
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get 1
          i32.const 12
          i32.const 4
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
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN3std9panicking11panic_count8increase17hbcaad6b35138193cE (;99;) (type 3) (param i32) (result i32)
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
    (func $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind (;100;) (type 1) (param i32)
      (local i32 i64)
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
      call $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE
      unreachable
    )
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E (;101;) (type 6) (param i32 i32)
      (local i32 i32 i32 i64)
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
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 12
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
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE (;102;) (type 6) (param i32 i32)
      (local i32 i32 i32 i64)
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
    (func $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE (;103;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        local.set 0
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E (;104;) (type 6) (param i32 i32)
      (local i32 i32)
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
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 8
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
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E (;105;) (type 6) (param i32 i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055100
      i32.add
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE (;106;) (type 6) (param i32 i32)
      local.get 0
      local.get 1
      i64.load align=4
      i64.store
    )
    (func $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E (;107;) (type 2) (param i32 i32) (result i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
    )
    (func $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E (;108;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32)
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
              call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
              local.get 5
              i32.load8_u offset=48
              local.get 5
              i32.load offset=52
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
            call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
            local.get 5
            i32.load8_u offset=56
            local.get 5
            i32.load offset=60
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
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get 5
          i32.load8_u offset=48
          local.get 5
          i32.load offset=52
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
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get 5
          i32.load8_u offset=56
          local.get 5
          i32.load offset=60
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br 1 (;@1;)
        end
        local.get 0
        local.get 1
        call $_RNvCscSpY9Juk0HT_7___rustc10rust_panic
        unreachable
      end
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc10rust_panic (;109;) (type 6) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      local.get 1
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
      call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
      local.get 2
      i32.load8_u offset=16
      local.get 2
      i32.load offset=20
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $cabi_realloc (;110;) (type 9) (param i32 i32 i32 i32) (result i32)
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
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
          local.tee 2
          br_if 1 (;@1;)
        end
        call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
        unreachable
      end
      local.get 2
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE (;111;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E (;112;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE (;113;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E (;114;) (type 6) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E (;115;) (type 6) (param i32 i32)
      (local i32 i32)
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
      call $_ZN3std7process5abort17had5be8ae244d01ebE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom (;116;) (type 6) (param i32 i32)
      local.get 1
      local.get 0
      call $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc18___rust_start_panic (;117;) (type 2) (param i32 i32) (result i32)
      unreachable
    )
    (func $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE (;118;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $malloc (;119;) (type 3) (param i32) (result i32)
      local.get 0
      call $dlmalloc
    )
    (func $dlmalloc (;120;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
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
                  call $sbrk
                  local.set 8
                  i32.const 0
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
    (func $prepend_alloc (;121;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $free (;122;) (type 1) (param i32)
      local.get 0
      call $dlfree
    )
    (func $dlfree (;123;) (type 1) (param i32)
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
    (func $calloc (;124;) (type 2) (param i32 i32) (result i32)
      (local i32 i64)
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
        call $memset
        drop
      end
      local.get 0
    )
    (func $realloc (;125;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 1
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
          call $dispose_chunk
          local.get 0
          return
        end
        block ;; label = @2
          local.get 1
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
        call $memcpy
        local.set 1
        local.get 0
        call $dlfree
        local.get 1
        local.set 0
      end
      local.get 0
    )
    (func $dispose_chunk (;126;) (type 6) (param i32 i32)
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
    (func $posix_memalign (;127;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 16
            i32.ne
            br_if 0 (;@3;)
            local.get 2
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
    (func $internal_memalign (;128;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
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
        call $dispose_chunk
      end
      local.get 0
      i32.const 8
      i32.add
    )
    (func $_Exit (;129;) (type 1) (param i32)
      local.get 0
      call $__wasi_proc_exit
      unreachable
    )
    (func $__wasilibc_ensure_environ (;130;) (type 0)
      block ;; label = @1
        i32.const 0
        i32.load offset=1055524
        i32.const -1
        i32.ne
        br_if 0 (;@1;)
        call $__wasilibc_initialize_environ
      end
    )
    (func $__wasilibc_initialize_environ (;131;) (type 0)
      (local i32 i32 i32)
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
              call $malloc
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 4
              call $calloc
              local.tee 1
              br_if 1 (;@3;)
              local.get 2
              call $free
            end
            i32.const 70
            call $_Exit
            unreachable
          end
          local.get 1
          local.get 2
          call $__wasi_environ_get
          i32.eqz
          br_if 1 (;@1;)
          local.get 2
          call $free
          local.get 1
          call $free
        end
        i32.const 71
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
    (func $__wasi_environ_get (;132;) (type 2) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__imported_wasi_snapshot_preview1_environ_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_environ_sizes_get (;133;) (type 2) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__imported_wasi_snapshot_preview1_environ_sizes_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_proc_exit (;134;) (type 1) (param i32)
      local.get 0
      call $__imported_wasi_snapshot_preview1_proc_exit
      unreachable
    )
    (func $abort (;135;) (type 0)
      unreachable
    )
    (func $getcwd (;136;) (type 2) (param i32 i32) (result i32)
      (local i32)
      i32.const 0
      i32.load offset=1055528
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          local.get 2
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
        call $strcpy
        local.set 0
      end
      local.get 0
    )
    (func $sbrk (;137;) (type 3) (param i32) (result i32)
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
      call $abort
      unreachable
    )
    (func $getenv (;138;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32)
      call $__wasilibc_ensure_environ
      block ;; label = @1
        local.get 0
        i32.const 61
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
    (func $dummy (;139;) (type 0))
    (func $__wasm_call_dtors (;140;) (type 0)
      call $dummy
      call $__stdio_exit
    )
    (func $"#func141 dummy" (@name "dummy") (;141;) (type 2) (param i32 i32) (result i32)
      local.get 0
    )
    (func $__lctrans (;142;) (type 2) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $"#func141 dummy"
    )
    (func $__ofl_lock (;143;) (type 10) (result i32)
      i32.const 1056420
    )
    (func $__stdio_exit (;144;) (type 0)
      (local i32 i32 i32)
      block ;; label = @1
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
    (func $memcpy (;145;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $memset (;146;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $__strchrnul (;147;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
    (func $__stpcpy (;148;) (type 2) (param i32 i32) (result i32)
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
    (func $strcpy (;149;) (type 2) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__stpcpy
      drop
      local.get 0
    )
    (func $strdup (;150;) (type 3) (param i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        call $strlen
        i32.const 1
        i32.add
        local.tee 1
        call $malloc
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 0
        local.get 1
        call $memcpy
        drop
      end
      local.get 2
    )
    (func $strerror (;151;) (type 3) (param i32) (result i32)
      (local i32)
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
      call $__lctrans
    )
    (func $strerror_r (;152;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          call $strerror
          local.tee 0
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
        call $memcpy
        drop
        i32.const 0
        local.set 3
      end
      local.get 3
    )
    (func $strlen (;153;) (type 3) (param i32) (result i32)
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
    (func $strncmp (;154;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE (;155;) (type 1) (param i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E (;156;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32)
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
    (func $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE (;157;) (type 4) (param i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get 2
      call $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE
      unreachable
    )
    (func $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE (;158;) (type 7) (param i32 i32 i32 i32)
      (local i32)
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
    (func $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E (;159;) (type 6) (param i32 i32)
      local.get 1
      local.get 0
      call $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler
      unreachable
    )
    (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE (;160;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32)
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
            call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
            unreachable
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055556
          i32.add
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
    (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE (;161;) (type 6) (param i32 i32)
      (local i32 i32 i32 i32)
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
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br 1 (;@1;)
        end
        local.get 1
        local.get 3
        i32.const 1
        local.get 5
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
        local.tee 4
        br_if 0 (;@1;)
        i32.const 1
        local.get 5
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
    (func $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E (;162;) (type 4) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      call $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE (;163;) (type 4) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      call $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE
      unreachable
    )
    (func $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E (;164;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
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
    (func $_ZN4core9panicking5panic17hd836709591dfc35fE (;165;) (type 4) (param i32 i32 i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE (;166;) (type 6) (param i32 i32)
      (local i32)
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
      call $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind
      unreachable
    )
    (func $_ZN4core3fmt5write17h19dbf2ffaf30f068E (;167;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE (;168;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 6
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 6
    )
    (func $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE (;169;) (type 13) (param i32 i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
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
    (func $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E (;170;) (type 2) (param i32 i32) (result i32)
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
    (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E (;171;) (type 2) (param i32 i32) (result i32)
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
    (func $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE (;172;) (type 1) (param i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h04ddcd8be7687b8aE (;173;) (type 4) (param i32 i32 i32)
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
    (func $_ZN4core3str8converts9from_utf817hc11b0c33b11310b8E (;174;) (type 4) (param i32 i32 i32)
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
    (func $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE (;175;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
        call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E
        br_if 0 (;@1;)
        local.get 5
        global.get $GOT.data.internal.__memory_base
        i32.const 1053507
        i32.add
        i32.const 2
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
    (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E (;176;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE (;177;) (type 11) (param i32 i32 i32 i32 i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E (;178;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      i32.const 1
      local.get 1
      call $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE
    )
    (func $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE (;179;) (type 1) (param i32)
      global.get $GOT.data.internal.__memory_base
      i32.const 1053380
      i32.add
      i32.const 43
      local.get 0
      call $_ZN4core9panicking5panic17hd836709591dfc35fE
      unreachable
    )
    (func $_ZN4core6option13expect_failed17he15179d1cacc214eE (;180;) (type 4) (param i32 i32 i32)
      (local i32)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E (;181;) (type 2) (param i32 i32) (result i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE (;182;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E (;183;) (type 14) (param i32 i32 i32 i32 i32 i32 i32)
      (local i32 i64)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E (;184;) (type 2) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 2)
    )
    (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE (;185;) (type 2) (param i32 i32) (result i32)
      local.get 1
      i32.load
      local.get 1
      i32.load offset=4
      local.get 0
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E (;186;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE (;187;) (type 2) (param i32 i32) (result i32)
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
    (func $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E (;188;) (type 2) (param i32 i32) (result i32)
      local.get 0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055668
      i32.add
      local.get 1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E (;189;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
    (func $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E (;190;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E (;191;) (type 15) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
      (local i32)
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
      call $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE
      local.get 7
      local.get 8
      local.get 9
      local.get 10
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
    (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E (;192;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
    (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E (;193;) (type 2) (param i32 i32) (result i32)
      block ;; label = @1
        local.get 0
        i32.load8_u
        br_if 0 (;@1;)
        local.get 1
        global.get $GOT.data.internal.__memory_base
        i32.const 1053732
        i32.add
        i32.const 5
        call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
        return
      end
      local.get 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053737
      i32.add
      i32.const 4
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE (;194;) (type 5) (param i32 i32 i32) (result i32)
      local.get 2
      local.get 0
      local.get 1
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core5slice6memchr14memchr_aligned17hf4db372f52bc45e4E (;195;) (type 7) (param i32 i32 i32 i32)
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
    (func $_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E (;196;) (type 7) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
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
            call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
            unreachable
          end
          local.get 5
          local.get 3
          global.get $GOT.data.internal.__memory_base
          i32.const 1055708
          i32.add
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
    (func $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE (;197;) (type 4) (param i32 i32 i32)
      (local i32 i64)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE (;198;) (type 4) (param i32 i32 i32)
      (local i32 i64)
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
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E (;199;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E (;200;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE (;201;) (type 2) (param i32 i32) (result i32)
      (local i32 i32 i32)
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
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE (;202;) (type 2) (param i32 i32) (result i32)
      (local i32)
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
      call $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE
    )
    (data $.rodata (;0;) (i32.const 1048576) "/Users/evgilber/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs world!\0ahello==== CALC ====\0a\0a==== PRINT ====\0acalc(, ) -> \0alibrary/std/src/panicking.rs: /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/vec/mod.rs/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/string.rs/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/raw_vec/mod.rsUtf8Errorvalid_up_toerror_lenNoneSome:/rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/alloc/src/slice.rslibrary/std/src/rt.rslibrary/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhaustedmainRUST_BACKTRACEcalled `Result::unwrap()` on an `Err` valuefailed to write the buffered datalibrary/std/src/io/buffered/bufwriter.rs\01\00\00\00\00\00\00\00library/std/src/io/buffered/linewritershim.rsmid > lenfailed to write whole bufferentity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filequota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryin progressother erroruncategorized error (os error )library/std/src/io/stdio.rsfailed printing to stdoutlibrary/std/src/io/mod.rsa formatting trait implementation returned an error when the underlying stream did notadvancing io slices beyond their lengthadvancing IoSlice beyond its lengthlibrary/std/src/sys/io/io_slice/wasi.rspanicked at :\0acannot recursively acquire mutexlibrary/std/src/sys/sync/mutex/no_threads.rslibrary/std/src/sync/poison/once.rslock count overflow in reentrant mutexlibrary/std/src/sync/reentrant_lock.rsfile name contained an unexpected NUL bytestack backtrace:\0anote: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0amemory allocation of  bytes failed\0a bytes failedlibrary/std/src/alloc.rsnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a<unnamed>\0athread '' panicked at \0aBox<dyn Any>aborting due to panic at \0athread panicked while processing panic. aborting.\0athread caused non-unwinding panic. aborting.\0afatal runtime error: failed to initiate panic, error library/std/src/sys/pal/wasip2/../wasi/os.rsstrerror_r failureOnce instance has previously been poisonedone-time initialization may not be performed recursivelyfatal runtime error: rwlock locked for writing\0a\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\0e\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\0b\00\00\00\13\00\00\00/\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05capacity overflowlibrary/alloc/src/ffi/c_str.rs)\00\00\01\00\00\00\00\00\00\00BorrowMutErroralready borrowed: called `Option::unwrap()` on a `None` value==!=matchesassertion `left  right` failed\0a  left: \0a right:  right` failed: \0a  left: :      { ,  {\0a,\0a} }((\0a,0x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetruelibrary/core/src/slice/memchr.rs\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00range start index  out of range for slice of length range end index \00\00\00\02\00\00\00\02\00\00\00\07\00\00\00")
    (data $.data (;1;) (i32.const 1054112) "\00\00\00\00\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\05\00\00\00\00\00\10\00n\00\00\00\be\01\00\00\1d\00\00\00\01\00\00\00\00\00\00\00n\00\10\00\08\00\00\00{\00\10\00\0f\00\00\00\8a\00\10\00\11\00\00\00\9b\00\10\00\05\00\00\00\a0\00\10\00\02\00\00\00\a2\00\10\00\05\00\00\00\a7\00\10\00\01\00\00\00\0e\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\0f\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\c6\00\10\00L\00\00\00V\0a\00\00$\00\00\00\12\01\10\00K\00\00\00}\05\00\00\1b\00\00\00]\01\10\00P\00\00\00.\02\00\00\11\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\11\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\12\00\00\00\01\00\00\00\00\00\00\00\d2\01\10\00\01\00\00\00\d2\01\10\00\01\00\00\00\13\00\00\00\0c\00\00\00\04\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00\1e\00\00\00\1f\00\00\00 \00\00\00\17\00\00\00\0c\00\00\00\04\00\00\00!\00\00\00\22\00\00\00#\00\00\00\d3\01\10\00J\00\00\00\be\01\00\00\1d\00\00\00\1d\02\10\00\15\00\00\00\86\00\00\00\0d\00\00\00O\02\10\007\00\00\002\02\10\00\1d\00\00\00\a9\04\00\00\0d\00\00\00\c3\02\10\00!\00\00\00\17\00\00\00A\03\10\00\09\00\00\00\14\03\10\00-\00\00\00\16\01\00\00)\00\00\00J\03\10\00\1c\00\00\00\17\00\00\00\00\00\00\00\02\00\00\00h\17\10\00\01\00\00\00\00\00\00\00S\06\10\00\0b\00\00\00^\06\10\00\01\00\00\00\e4\02\10\00(\00\00\00z\00\00\00!\00\00\00_\06\10\00\1b\00\00\00\e3\02\00\00\13\00\00\00_\06\10\00\1b\00\00\00\5c\03\00\00\14\00\00\00z\06\10\00\13\00\00\00\c4\00\10\00\02\00\00\00_\06\10\00\1b\00\00\00\8d\04\00\00\09\00\00\00\ac\06\10\00V\00\00\00\93\06\10\00\19\00\00\00\88\02\00\00\11\00\00\00\93\06\10\00\19\00\00\00\08\06\00\00 \00\00\00\02\07\10\00'\00\00\00\93\06\10\00\19\00\00\00\0a\06\00\00\0d\00\00\00)\07\10\00#\00\00\00L\07\10\00'\00\00\00\14\00\00\00\0d\00\00\00\93\06\10\00\19\00\00\00\09\07\00\00$\00\00\00\81\07\10\00 \00\00\00\a1\07\10\00,\00\00\00\13\00\00\00\09\00\00\00\cd\07\10\00#\00\00\00\9b\00\00\002\00\00\00\cd\07\10\00#\00\00\00\d6\00\00\00\14\00\00\00\16\08\10\00&\00\00\00\1f\01\00\00-\00\00\00<\08\10\00*\00\00\00\14\00\00\00\00\00\00\00\02\00\00\00\98\18\10\00\cf\08\10\00\15\00\00\00\e4\08\10\00\0e\00\00\00\cf\08\10\00\15\00\00\00\f2\08\10\00\0d\00\00\00\ff\08\10\00\18\00\00\00d\01\00\00\09\00\00\00$\00\00\00\0c\00\00\00\04\00\00\00%\00\00\00&\00\00\00'\00\00\00(\00\00\00)\00\00\00*\00\00\00+\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00,\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\001\00\00\002\00\00\00\17\09\10\00N\00\00\00\a8\00\10\00\1c\00\00\00\1d\01\00\00.\00\00\00n\09\10\00\09\00\00\00w\09\10\00\0e\00\00\00\7f\07\10\00\02\00\00\00\85\09\10\00\01\00\00\00\01\00\00\00\13\00\00\00\0c\00\00\00\04\00\00\003\00\00\00\00\00\00\00\08\00\00\00\04\00\00\004\00\00\00\00\00\00\00\08\00\00\00\04\00\00\005\00\00\006\00\00\007\00\00\008\00\00\009\00\00\00\10\00\00\00\04\00\00\00:\00\00\00;\00\00\00<\00\00\00=\00\00\00\92\09\10\00\19\00\00\00\7f\07\10\00\02\00\00\00\85\09\10\00\01\00\00\00s\07\10\00\0c\00\00\00\7f\07\10\00\02\00\00\00\ab\09\10\003\00\00\00\de\09\10\00-\00\00\00\0b\0a\10\005\00\00\00\85\09\10\00\01\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00>\00\00\00@\0a\10\00,\00\00\00A\00\00\006\00\00\00l\0a\10\00\12\00\00\00@\0a\10\00,\00\00\00?\00\00\00\0d\00\00\00@\0a\10\00,\00\00\00F\00\00\00\13\00\00\00@\0a\10\00,\00\00\00M\00\00\00\15\00\00\00~\0a\10\00*\00\00\00\a8\0a\10\008\00\00\00\e0\0a\10\00/\00\00\00f\03\10\00v\03\10\00\87\03\10\00\99\03\10\00\a9\03\10\00\b9\03\10\00\cc\03\10\00\de\03\10\00\eb\03\10\00\f9\03\10\00\0e\04\10\00\1a\04\10\00%\04\10\00:\04\10\00O\04\10\00^\04\10\00l\04\10\00\7f\04\10\00\a5\04\10\00\dd\04\10\00\f6\04\10\00\0d\05\10\00\19\05\10\00\22\05\10\00,\05\10\00<\05\10\00S\05\10\00a\05\10\00o\05\10\00|\05\10\00\90\05\10\00\98\05\10\00\b3\05\10\00\c1\05\10\00\d1\05\10\00\e7\05\10\00\fc\05\10\00\07\06\10\00\1d\06\10\00*\06\10\005\06\10\00@\06\10\00\ff\ff\ff\ff\b8\0b\10\00j\12\10\00\11\00\00\00{\12\10\00\1e\00\00\00\1a\01\00\00\1e\00\00\00{\12\10\00\1e\00\00\00\16\01\00\007\00\00\00{\12\10\00\1e\00\00\00U\01\00\00\0b\00\00\00\b2\12\10\00\12\00\00\00\fa\12\10\00\10\00\00\00\0a\13\10\00\17\00\00\00!\13\10\00\09\00\00\00\fa\12\10\00\10\00\00\00*\13\10\00\10\00\00\00:\13\10\00\09\00\00\00!\13\10\00\09\00\00\00\01\00\00\00\00\00\00\00C\13\10\00\02\00\00\00\00\00\00\00\0c\00\00\00\04\00\00\00C\00\00\00D\00\00\00E\00\00\00-\14\10\00 \00\00\00\84\00\00\00\1e\00\00\00-\14\10\00 \00\00\00\a0\00\00\00\09\00\00\00M\15\10\00\12\00\00\00_\15\10\00\22\00\00\00\81\15\10\00\10\00\00\00_\15\10\00\22\00\00\00\ef\12\10\00\f1\12\10\00\f3\12\10\00")
    (@producers
      (language "C11" "")
      (language "Rust" "")
      (processed-by "clang" "19.1.5-wasi-sdk (https://github.com/llvm/llvm-project ab4b5a2db582958af1ee308a790cfdb42bd24720)")
      (processed-by "rustc" "1.87.0 (17067e9ac 2025-05-09)")
      (processed-by "wit-component" "0.20.1")
      (processed-by "wit-bindgen-c" "0.17.0")
    )
    (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
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
  (core instance (;2;) (instantiate 0
      (with "wasi_snapshot_preview1" (instance 1))
    )
  )
  (alias core export 2 "memory" (core memory (;0;)))
  (core instance (;3;)
    (export "memory" (memory 0))
  )
  (alias core export 2 "_start" (core func (;4;)))
  (alias core export 2 "cabi_realloc" (core func (;5;)))
  (core instance (;4;)
    (export "_start" (func 4))
    (export "cabi_realloc" (func 5))
  )
  (alias core export 0 "4" (core func (;6;)))
  (core instance (;5;)
    (export "get-environment" (func 6))
  )
  (alias export 8 "descriptor" (type (;18;)))
  (core func (;7;) (canon resource.drop 18))
  (alias core export 0 "5" (core func (;8;)))
  (alias core export 0 "6" (core func (;9;)))
  (alias core export 0 "7" (core func (;10;)))
  (alias core export 0 "8" (core func (;11;)))
  (alias core export 0 "9" (core func (;12;)))
  (core instance (;6;)
    (export "[resource-drop]descriptor" (func 7))
    (export "filesystem-error-code" (func 8))
    (export "[method]descriptor.write-via-stream" (func 9))
    (export "[method]descriptor.append-via-stream" (func 10))
    (export "[method]descriptor.get-type" (func 11))
    (export "[method]descriptor.stat" (func 12))
  )
  (alias export 3 "output-stream" (type (;19;)))
  (core func (;13;) (canon resource.drop 19))
  (alias export 3 "input-stream" (type (;20;)))
  (core func (;14;) (canon resource.drop 20))
  (alias core export 0 "10" (core func (;15;)))
  (alias core export 0 "11" (core func (;16;)))
  (alias core export 0 "12" (core func (;17;)))
  (alias core export 0 "13" (core func (;18;)))
  (core instance (;7;)
    (export "[resource-drop]output-stream" (func 13))
    (export "[resource-drop]input-stream" (func 14))
    (export "[method]output-stream.check-write" (func 15))
    (export "[method]output-stream.write" (func 16))
    (export "[method]output-stream.blocking-flush" (func 17))
    (export "[method]output-stream.blocking-write-and-flush" (func 18))
  )
  (alias export 2 "error" (type (;21;)))
  (core func (;19;) (canon resource.drop 21))
  (core instance (;8;)
    (export "[resource-drop]error" (func 19))
  )
  (alias core export 0 "14" (core func (;20;)))
  (core instance (;9;)
    (export "get-directories" (func 20))
  )
  (alias export 6 "get-stderr" (func (;0;)))
  (core func (;21;) (canon lower (func 0)))
  (core instance (;10;)
    (export "get-stderr" (func 21))
  )
  (alias export 4 "get-stdin" (func (;1;)))
  (core func (;22;) (canon lower (func 1)))
  (core instance (;11;)
    (export "get-stdin" (func 22))
  )
  (alias export 5 "get-stdout" (func (;2;)))
  (core func (;23;) (canon lower (func 2)))
  (core instance (;12;)
    (export "get-stdout" (func 23))
  )
  (alias export 1 "exit" (func (;3;)))
  (core func (;24;) (canon lower (func 3)))
  (core instance (;13;)
    (export "exit" (func 24))
  )
  (core instance (;14;) (instantiate 1
      (with "env" (instance 3))
      (with "__main_module__" (instance 4))
      (with "wasi:cli/environment@0.2.3" (instance 5))
      (with "wasi:filesystem/types@0.2.3" (instance 6))
      (with "wasi:io/streams@0.2.3" (instance 7))
      (with "wasi:io/error@0.2.3" (instance 8))
      (with "wasi:filesystem/preopens@0.2.2" (instance 9))
      (with "wasi:cli/stderr@0.2.3" (instance 10))
      (with "wasi:cli/stdin@0.2.3" (instance 11))
      (with "wasi:cli/stdout@0.2.3" (instance 12))
      (with "wasi:cli/exit@0.2.3" (instance 13))
    )
  )
  (alias core export 0 "$imports" (core table (;0;)))
  (alias core export 14 "fd_write" (core func (;25;)))
  (alias core export 14 "environ_get" (core func (;26;)))
  (alias core export 14 "environ_sizes_get" (core func (;27;)))
  (alias core export 14 "proc_exit" (core func (;28;)))
  (alias export 0 "get-environment" (func (;4;)))
  (alias core export 14 "cabi_import_realloc" (core func (;29;)))
  (core func (;30;) (canon lower (func 4) (memory 0) (realloc 29) string-encoding=utf8))
  (alias export 8 "filesystem-error-code" (func (;5;)))
  (core func (;31;) (canon lower (func 5) (memory 0)))
  (alias export 8 "[method]descriptor.write-via-stream" (func (;6;)))
  (core func (;32;) (canon lower (func 6) (memory 0)))
  (alias export 8 "[method]descriptor.append-via-stream" (func (;7;)))
  (core func (;33;) (canon lower (func 7) (memory 0)))
  (alias export 8 "[method]descriptor.get-type" (func (;8;)))
  (core func (;34;) (canon lower (func 8) (memory 0)))
  (alias export 8 "[method]descriptor.stat" (func (;9;)))
  (core func (;35;) (canon lower (func 9) (memory 0)))
  (alias export 3 "[method]output-stream.check-write" (func (;10;)))
  (core func (;36;) (canon lower (func 10) (memory 0)))
  (alias export 3 "[method]output-stream.write" (func (;11;)))
  (core func (;37;) (canon lower (func 11) (memory 0)))
  (alias export 3 "[method]output-stream.blocking-flush" (func (;12;)))
  (core func (;38;) (canon lower (func 12) (memory 0)))
  (alias export 3 "[method]output-stream.blocking-write-and-flush" (func (;13;)))
  (core func (;39;) (canon lower (func 13) (memory 0)))
  (alias export 9 "get-directories" (func (;14;)))
  (core func (;40;) (canon lower (func 14) (memory 0) (realloc 29) string-encoding=utf8))
  (core instance (;15;)
    (export "$imports" (table 0))
    (export "0" (func 25))
    (export "1" (func 26))
    (export "2" (func 27))
    (export "3" (func 28))
    (export "4" (func 30))
    (export "5" (func 31))
    (export "6" (func 32))
    (export "7" (func 33))
    (export "8" (func 34))
    (export "9" (func 35))
    (export "10" (func 36))
    (export "11" (func 37))
    (export "12" (func 38))
    (export "13" (func 39))
    (export "14" (func 40))
  )
  (core instance (;16;) (instantiate 3
      (with "" (instance 15))
    )
  )
  (type (;22;) (result))
  (type (;23;) (func (result 22)))
  (alias core export 14 "wasi:cli/run@0.2.3#run" (core func (;41;)))
  (func (;15;) (type 23) (canon lift (core func 41)))
  (component (;0;)
    (type (;0;) (result))
    (type (;1;) (func (result 0)))
    (import "import-func-run" (func (;0;) (type 1)))
    (type (;2;) (result))
    (type (;3;) (func (result 2)))
    (export (;1;) "run" (func 0) (func (type 3)))
  )
  (instance (;10;) (instantiate 0
      (with "import-func-run" (func 15))
    )
  )
  (export (;11;) "wasi:cli/run@0.2.3" (instance 10))
  (@producers
    (processed-by "wit-component" "0.223.0")
  )
)
