(component
  (type $#type0 (;0;)
    (instance
      (type $#type0 (;0;) (tuple string string))
      (type $#type1 (;1;) (list $#type0))
      (type $#type2 (;2;) (func (result $#type1)))
      (export $#func0 (;0;) "get-environment" (func (type $#type2)))
    )
  )
  (import "wasi:cli/environment@0.2.3" (instance $#instance0 (;0;) (type $#type0)))
  (type $#type1 (;1;)
    (instance
      (type $#type0 (;0;) (result))
      (type $#type1 (;1;) (func (param "status" $#type0)))
      (export $#func0 (;0;) "exit" (func (type $#type1)))
    )
  )
  (import "wasi:cli/exit@0.2.3" (instance $#instance1 (;1;) (type $#type1)))
  (type $#type2 (;2;)
    (instance
      (export $#type0 (;0;) "error" (type (sub resource)))
    )
  )
  (import "wasi:io/error@0.2.3" (instance $#instance2 (;2;) (type $#type2)))
  (alias export $#instance2 "error" (type $#type3 (;3;)))
  (type $#type4 (;4;)
    (instance
      (export $#type0 (;0;) "output-stream" (type (sub resource)))
      (alias outer 1 $#type3 (type $#type1 (;1;)))
      (export $#type2 (;2;) "error" (type (eq $#type1)))
      (type $#type3 (;3;) (own $#type2))
      (type $#type4 (;4;) (variant (case "last-operation-failed" $#type3) (case "closed")))
      (export $#type5 (;5;) "stream-error" (type (eq $#type4)))
      (export $#type6 (;6;) "input-stream" (type (sub resource)))
      (type $#type7 (;7;) (borrow $#type0))
      (type $#type8 (;8;) (result u64 (error $#type5)))
      (type $#type9 (;9;) (func (param "self" $#type7) (result $#type8)))
      (export $#func0 (;0;) "[method]output-stream.check-write" (func (type $#type9)))
      (type $#type10 (;10;) (list u8))
      (type $#type11 (;11;) (result (error $#type5)))
      (type $#type12 (;12;) (func (param "self" $#type7) (param "contents" $#type10) (result $#type11)))
      (export $#func1 (;1;) "[method]output-stream.write" (func (type $#type12)))
      (export $#func2 (;2;) "[method]output-stream.blocking-write-and-flush" (func (type $#type12)))
      (type $#type13 (;13;) (func (param "self" $#type7) (result $#type11)))
      (export $#func3 (;3;) "[method]output-stream.blocking-flush" (func (type $#type13)))
    )
  )
  (import "wasi:io/streams@0.2.3" (instance $#instance3 (;3;) (type $#type4)))
  (alias export $#instance3 "input-stream" (type $#type5 (;5;)))
  (type $#type6 (;6;)
    (instance
      (alias outer 1 $#type5 (type $#type0 (;0;)))
      (export $#type1 (;1;) "input-stream" (type (eq $#type0)))
      (type $#type2 (;2;) (own $#type1))
      (type $#type3 (;3;) (func (result $#type2)))
      (export $#func0 (;0;) "get-stdin" (func (type $#type3)))
    )
  )
  (import "wasi:cli/stdin@0.2.3" (instance $#instance4 (;4;) (type $#type6)))
  (alias export $#instance3 "output-stream" (type $#type7 (;7;)))
  (type $#type8 (;8;)
    (instance
      (alias outer 1 $#type7 (type $#type0 (;0;)))
      (export $#type1 (;1;) "output-stream" (type (eq $#type0)))
      (type $#type2 (;2;) (own $#type1))
      (type $#type3 (;3;) (func (result $#type2)))
      (export $#func0 (;0;) "get-stdout" (func (type $#type3)))
    )
  )
  (import "wasi:cli/stdout@0.2.3" (instance $#instance5 (;5;) (type $#type8)))
  (alias export $#instance3 "output-stream" (type $#type9 (;9;)))
  (type $#type10 (;10;)
    (instance
      (alias outer 1 $#type9 (type $#type0 (;0;)))
      (export $#type1 (;1;) "output-stream" (type (eq $#type0)))
      (type $#type2 (;2;) (own $#type1))
      (type $#type3 (;3;) (func (result $#type2)))
      (export $#func0 (;0;) "get-stderr" (func (type $#type3)))
    )
  )
  (import "wasi:cli/stderr@0.2.3" (instance $#instance6 (;6;) (type $#type10)))
  (type $#type11 (;11;)
    (instance
      (type $#type0 (;0;) (record (field "seconds" u64) (field "nanoseconds" u32)))
      (export $#type1 (;1;) "datetime" (type (eq $#type0)))
    )
  )
  (import "wasi:clocks/wall-clock@0.2.3" (instance $#instance7 (;7;) (type $#type11)))
  (alias export $#instance3 "output-stream" (type $#type12 (;12;)))
  (alias export $#instance7 "datetime" (type $#type13 (;13;)))
  (alias export $#instance3 "error" (type $#type14 (;14;)))
  (type $#type15 (;15;)
    (instance
      (export $#type0 (;0;) "descriptor" (type (sub resource)))
      (type $#type1 (;1;) u64)
      (export $#type2 (;2;) "filesize" (type (eq $#type1)))
      (alias outer 1 $#type12 (type $#type3 (;3;)))
      (export $#type4 (;4;) "output-stream" (type (eq $#type3)))
      (type $#type5 (;5;) (enum "access" "would-block" "already" "bad-descriptor" "busy" "deadlock" "quota" "exist" "file-too-large" "illegal-byte-sequence" "in-progress" "interrupted" "invalid" "io" "is-directory" "loop" "too-many-links" "message-size" "name-too-long" "no-device" "no-entry" "no-lock" "insufficient-memory" "insufficient-space" "not-directory" "not-empty" "not-recoverable" "unsupported" "no-tty" "no-such-device" "overflow" "not-permitted" "pipe" "read-only" "invalid-seek" "text-file-busy" "cross-device"))
      (export $#type6 (;6;) "error-code" (type (eq $#type5)))
      (type $#type7 (;7;) (enum "unknown" "block-device" "character-device" "directory" "fifo" "symbolic-link" "regular-file" "socket"))
      (export $#type8 (;8;) "descriptor-type" (type (eq $#type7)))
      (type $#type9 (;9;) u64)
      (export $#type10 (;10;) "link-count" (type (eq $#type9)))
      (alias outer 1 $#type13 (type $#type11 (;11;)))
      (export $#type12 (;12;) "datetime" (type (eq $#type11)))
      (type $#type13 (;13;) (option $#type12))
      (type $#type14 (;14;) (record (field "type" $#type8) (field "link-count" $#type10) (field "size" $#type2) (field "data-access-timestamp" $#type13) (field "data-modification-timestamp" $#type13) (field "status-change-timestamp" $#type13)))
      (export $#type15 (;15;) "descriptor-stat" (type (eq $#type14)))
      (alias outer 1 $#type14 (type $#type16 (;16;)))
      (export $#type17 (;17;) "error" (type (eq $#type16)))
      (type $#type18 (;18;) (borrow $#type0))
      (type $#type19 (;19;) (own $#type4))
      (type $#type20 (;20;) (result $#type19 (error $#type6)))
      (type $#type21 (;21;) (func (param "self" $#type18) (param "offset" $#type2) (result $#type20)))
      (export $#func0 (;0;) "[method]descriptor.write-via-stream" (func (type $#type21)))
      (type $#type22 (;22;) (func (param "self" $#type18) (result $#type20)))
      (export $#func1 (;1;) "[method]descriptor.append-via-stream" (func (type $#type22)))
      (type $#type23 (;23;) (result $#type8 (error $#type6)))
      (type $#type24 (;24;) (func (param "self" $#type18) (result $#type23)))
      (export $#func2 (;2;) "[method]descriptor.get-type" (func (type $#type24)))
      (type $#type25 (;25;) (result $#type15 (error $#type6)))
      (type $#type26 (;26;) (func (param "self" $#type18) (result $#type25)))
      (export $#func3 (;3;) "[method]descriptor.stat" (func (type $#type26)))
      (type $#type27 (;27;) (borrow $#type17))
      (type $#type28 (;28;) (option $#type6))
      (type $#type29 (;29;) (func (param "err" $#type27) (result $#type28)))
      (export $#func4 (;4;) "filesystem-error-code" (func (type $#type29)))
    )
  )
  (import "wasi:filesystem/types@0.2.3" (instance $#instance8 (;8;) (type $#type15)))
  (alias export $#instance8 "descriptor" (type $#type16 (;16;)))
  (type $#type17 (;17;)
    (instance
      (alias outer 1 $#type16 (type $#type0 (;0;)))
      (export $#type1 (;1;) "descriptor" (type (eq $#type0)))
      (type $#type2 (;2;) (own $#type1))
      (type $#type3 (;3;) (tuple $#type2 string))
      (type $#type4 (;4;) (list $#type3))
      (type $#type5 (;5;) (func (result $#type4)))
      (export $#func0 (;0;) "get-directories" (func (type $#type5)))
    )
  )
  (import "wasi:filesystem/preopens@0.2.3" (instance $#instance9 (;9;) (type $#type17)))
  (core module $#module0 (;0;)
    (type $#type0 (;0;) (func))
    (type $#type1 (;1;) (func (param i32)))
    (type $#type2 (;2;) (func (param i32 i32) (result i32)))
    (type $#type3 (;3;) (func (param i32) (result i32)))
    (type $#type4 (;4;) (func (param i32 i32 i32)))
    (type $#type5 (;5;) (func (param i32 i32 i32) (result i32)))
    (type $#type6 (;6;) (func (param i32 i32)))
    (type $#type7 (;7;) (func (param i32 i32 i32 i32)))
    (type $#type8 (;8;) (func (param i32 i64 i32) (result i64)))
    (type $#type9 (;9;) (func (param i32 i32 i32 i32) (result i32)))
    (type $#type10 (;10;) (func (result i32)))
    (type $#type11 (;11;) (func (param i32 i32 i32 i32 i32)))
    (type $#type12 (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type $#type13 (;13;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type $#type14 (;14;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type $#type15 (;15;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h5858df6e6eba6e92E (;0;) (type $#type9)))
    (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;1;) (type $#type2)))
    (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;2;) (type $#type2)))
    (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;3;) (type $#type1)))
    (table $#table0 (;0;) 70 70 funcref)
    (memory $#memory0 (;0;) 17)
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
    (export "memory" (memory $#memory0))
    (export "_start" (func $_start))
    (export "__main_void" (func $__main_void))
    (export "calc" (func $calc))
    (export "print_x" (func $print_x))
    (export "opt_str" (func $opt_str))
    (export "main" (func $main))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem $#elem0 (;0;) (i32.const 1) func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE $main $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E $"#func53 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E $cabi_realloc $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E $_ZN4core3fmt5Write10write_char17ha202ea95069de918E $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E)
    (func $__wasm_call_ctors (;4;) (type $#type0))
    (func $_start (;5;) (type $#type0)
      (local $#local0 i32)
      block $#label0
        block $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055768
          i32.add
          i32.load
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055768
          i32.add
          i32.const 1
          i32.store
          call $__wasm_call_ctors
          call $__main_void
          local.set $#local0
          call $__wasm_call_dtors
          local.get $#local0
          br_if $#label0
          return
        end
        unreachable
      end
      local.get $#local0
      call $__wasi_proc_exit
      unreachable
    )
    (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h08ccfe76ed0e264cE (;6;) (type $#type3) (param $#local0 i32) (result i32)
      local.get $#local0
      i32.load
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E (;7;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call_indirect (type $#type0)
    )
    (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h21f0882c0e328ac4E (;8;) (type $#type3) (param $#local0 i32) (result i32)
      local.get $#local0
      i32.load
      call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6257031b949724e3E
      i32.const 0
    )
    (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE (;9;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load offset=4
      local.get $#local0
      i32.load offset=8
      local.get $#local1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $calc (;10;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local1
      local.get $#local0
      i32.add
      local.get $#local1
      local.get $#local0
      i32.mul
      local.get $#local0
      i32.const 5
      i32.gt_s
      select
    )
    (func $print_x (;11;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      local.get $#local0
      i32.store8 offset=15
      block $#label0
        local.get $#local1
        i32.eqz
        br_if $#label0
        global.get $GOT.data.internal.__table_base
        i32.const 0
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get $#local2
        i32.const 52
        i32.add
        i64.extend_i32_u
        i64.or
        local.set $#local3
        loop $#label1
          local.get $#local2
          i32.const 52
          i32.add
          local.get $#local2
          i32.const 15
          i32.add
          call $opt_str
          local.get $#local2
          i32.const 2
          i32.store offset=20
          local.get $#local2
          global.get $GOT.data.internal.__memory_base
          i32.const 1054152
          i32.add
          i32.store offset=16
          local.get $#local2
          local.get $#local3
          i64.store offset=40
          local.get $#local2
          i64.const 1
          i64.store offset=28 align=4
          local.get $#local2
          local.get $#local2
          i32.const 40
          i32.add
          i32.store offset=24
          local.get $#local2
          i32.const 16
          i32.add
          call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
          block $#label2
            local.get $#local2
            i32.load offset=52
            local.tee $#local0
            i32.eqz
            br_if $#label2
            local.get $#local2
            i32.load offset=56
            local.get $#local0
            i32.const 1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local1
          i32.const -1
          i32.add
          local.tee $#local1
          br_if $#label1
        end
      end
      local.get $#local2
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $opt_str (;12;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local1
                    i32.load8_u
                    br_table $#label6 $#label5 $#label4 $#label6
                  end
                  i32.const 2
                  local.set $#local2
                  i32.const 2
                  i32.const 1
                  call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                  local.tee $#local1
                  i32.eqz
                  br_if $#label2
                  local.get $#local1
                  i32.const 26984
                  i32.store16 align=1
                  br $#label3
                end
                i32.const 5
                local.set $#local2
                i32.const 5
                i32.const 1
                call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                local.tee $#local1
                i32.eqz
                br_if $#label1
                local.get $#local1
                global.get $GOT.data.internal.__memory_base
                i32.const 1048694
                i32.add
                local.tee $#local3
                i32.load align=1
                i32.store align=1
                local.get $#local1
                i32.const 4
                i32.add
                local.get $#local3
                i32.const 4
                i32.add
                i32.load8_u
                i32.store8
                br $#label3
              end
              i32.const 4
              local.set $#local2
              i32.const 4
              i32.const 1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee $#local1
              i32.eqz
              br_if $#label0
              local.get $#local1
              i32.const 1886745383
              i32.store align=1
            end
            local.get $#local0
            local.get $#local2
            i32.store offset=8
            local.get $#local0
            local.get $#local1
            i32.store offset=4
            local.get $#local0
            local.get $#local2
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
    (func $main (;13;) (type $#type0)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32) (local $#local3 i64) (local $#local4 i64) (local $#local5 i64)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      local.get $#local0
      i32.const 0
      i32.store offset=64
      local.get $#local0
      i32.const 1
      i32.store offset=52
      local.get $#local0
      i64.const 4
      i64.store offset=56 align=4
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      local.tee $#local1
      i32.const 1054168
      i32.add
      i32.store offset=48
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 9
      i32.store offset=12
      local.get $#local0
      i32.const 0
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local1
      i32.const 1054184
      i32.add
      local.tee $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local3
      local.get $#local0
      i32.const 72
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local3
      local.get $#local0
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      local.get $#local0
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 9
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 8
      i32.store offset=12
      local.get $#local0
      i32.const 1
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 9
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 7
      i32.store offset=12
      local.get $#local0
      i32.const 2
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 9
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 6
      i32.store offset=12
      local.get $#local0
      i32.const 3
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 9
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 5
      i32.store offset=12
      local.get $#local0
      i32.const 4
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 20
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 4
      i32.store offset=12
      local.get $#local0
      i32.const 5
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 20
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 3
      i32.store offset=12
      local.get $#local0
      i32.const 6
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 18
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 2
      i32.store offset=12
      local.get $#local0
      i32.const 7
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 14
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 1
      i32.store offset=12
      local.get $#local0
      i32.const 8
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 8
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 0
      i32.store offset=12
      local.get $#local0
      i32.const 9
      i32.store offset=16
      local.get $#local0
      i32.const 4
      i32.store offset=24
      local.get $#local0
      local.get $#local2
      i32.store offset=20
      local.get $#local0
      i64.const 3
      i64.store offset=32 align=4
      local.get $#local0
      local.get $#local4
      i64.store offset=64
      local.get $#local0
      local.get $#local5
      i64.store offset=56
      local.get $#local0
      local.get $#local3
      i64.store offset=48
      local.get $#local0
      i32.const 0
      i32.store offset=72
      local.get $#local0
      local.get $#local0
      i32.const 48
      i32.add
      i32.store offset=28
      local.get $#local0
      i32.const 20
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 0
      i32.store offset=64
      local.get $#local0
      i32.const 1
      i32.store offset=52
      local.get $#local0
      local.get $#local1
      i32.const 1054176
      i32.add
      i32.store offset=48
      local.get $#local0
      i64.const 4
      i64.store offset=56 align=4
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      local.get $#local0
      i32.const 0
      i32.store8 offset=47
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      local.get $#local1
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      global.get $GOT.data.internal.__table_base
      i32.const 0
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local0
      i32.const 20
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 1
      i32.store8 offset=47
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 2
      i32.store8 offset=47
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 20
      i32.add
      local.get $#local0
      i32.const 47
      i32.add
      call $opt_str
      local.get $#local0
      i32.const 2
      i32.store offset=52
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054152
      i32.add
      i32.store offset=48
      local.get $#local0
      local.get $#local4
      i64.store offset=72
      local.get $#local0
      i64.const 1
      i64.store offset=60 align=4
      local.get $#local0
      local.get $#local0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get $#local0
      i32.const 48
      i32.add
      call $_ZN3std2io5stdio6_print17h68847dc224af8aecE
      block $#label0
        local.get $#local0
        i32.load offset=20
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.get $#local2
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local0
      i32.const 80
      i32.add
      global.set $__stack_pointer
    )
    (func $__main_void (;14;) (type $#type10) (result i32)
      (local $#local0 i32) (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      local.get $#local0
      global.get $GOT.data.internal.__table_base
      i32.const 2
      i32.add
      i32.store offset=12
      local.get $#local0
      i32.const 12
      i32.add
      global.get $GOT.data.internal.__memory_base
      i32.const 1054112
      i32.add
      i32.const 0
      i32.const 0
      i32.const 0
      call $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E
      local.set $#local1
      local.get $#local0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc (;15;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      local.get $#local0
      local.get $#local1
      call $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc
      local.set $#local2
      local.get $#local2
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc (;16;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc (;17;) (type $#type9) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      (local $#local4 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      local.get $#local3
      call $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc
      local.set $#local4
      local.get $#local4
      return
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler (;18;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      call $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom
      return
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3133017c71586385E (;19;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      i64.const -245993367077761921
      i64.store offset=8
      local.get $#local0
      i64.const 6756087622182587336
      i64.store
    )
    (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5bb0dfafb9ae49c1E (;20;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      i64.const 7199936582794304877
      i64.store offset=8
      local.get $#local0
      i64.const -5076933981314334344
      i64.store
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb3ae6bcfea8481c9E (;21;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      local.get $#local0
      i32.load
      local.set $#local0
      block $#label0
        local.get $#local1
        i32.load offset=8
        local.tee $#local2
        i32.const 33554432
        i32.and
        br_if $#label0
        block $#label1
          local.get $#local2
          i32.const 67108864
          i32.and
          br_if $#label1
          local.get $#local0
          local.get $#local1
          call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E
          return
        end
        local.get $#local0
        local.get $#local1
        call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E
        return
      end
      local.get $#local0
      local.get $#local1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd0146943339132b1E (;22;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load
      local.get $#local1
      call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf74014a6b3fa1f23E (;23;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local0
          i32.load
          local.tee $#local0
          i32.load8_u
          i32.const 1
          i32.ne
          br_if $#label1
          local.get $#local2
          local.get $#local0
          i32.const 1
          i32.add
          i32.store offset=12
          local.get $#local1
          global.get $GOT.data.internal.__memory_base
          local.tee $#local0
          i32.const 1049038
          i32.add
          i32.const 4
          local.get $#local2
          i32.const 12
          i32.add
          local.get $#local0
          i32.const 1054220
          i32.add
          call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E
          local.set $#local0
          br $#label0
        end
        local.get $#local1
        global.get $GOT.data.internal.__memory_base
        i32.const 1049034
        i32.add
        i32.const 4
        call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
        local.set $#local0
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb2426bb1f86971d3E (;24;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load
      local.get $#local0
      i32.load offset=4
      local.get $#local1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfb53e53618ee070fE (;25;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local1
      i32.load offset=4
      local.set $#local3
      local.get $#local1
      i32.load
      local.set $#local4
      local.get $#local0
      i32.load
      local.set $#local1
      local.get $#local2
      i32.const 3
      i32.store offset=4
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054332
      i32.add
      i32.store
      local.get $#local2
      i64.const 3
      i64.store offset=12 align=4
      local.get $#local2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local5
      local.get $#local1
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get $#local2
      local.get $#local5
      local.get $#local1
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local2
      global.get $GOT.data.internal.__table_base
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local1
      i64.extend_i32_u
      i64.or
      i64.store offset=24
      local.get $#local2
      local.get $#local2
      i32.const 24
      i32.add
      i32.store offset=8
      local.get $#local4
      local.get $#local3
      local.get $#local2
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
      local.set $#local1
      local.get $#local2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hb492cb4d51a1fd6aE (;26;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      block $#label0
        local.get $#local1
        i32.load offset=8
        local.tee $#local2
        i32.const 33554432
        i32.and
        br_if $#label0
        block $#label1
          local.get $#local2
          i32.const 67108864
          i32.and
          br_if $#label1
          local.get $#local0
          local.get $#local1
          call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
          return
        end
        local.get $#local0
        local.get $#local1
        call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE
        return
      end
      local.get $#local0
      local.get $#local1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE
    )
    (func $_ZN4core3fmt5Write10write_char17h8b7930802b161d9cE (;27;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i64) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i32.const 0
      i32.store offset=4
      block $#label0
        block $#label1
          local.get $#local1
          i32.const 128
          i32.lt_u
          br_if $#label1
          block $#label2
            local.get $#local1
            i32.const 2048
            i32.lt_u
            br_if $#label2
            block $#label3
              local.get $#local1
              i32.const 65536
              i32.lt_u
              br_if $#label3
              local.get $#local2
              local.get $#local1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=7
              local.get $#local2
              local.get $#local1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=4
              local.get $#local2
              local.get $#local1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=6
              local.get $#local2
              local.get $#local1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=5
              i32.const 4
              local.set $#local1
              br $#label0
            end
            local.get $#local2
            local.get $#local1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get $#local2
            local.get $#local1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=4
            local.get $#local2
            local.get $#local1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            i32.const 3
            local.set $#local1
            br $#label0
          end
          local.get $#local2
          local.get $#local1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=5
          local.get $#local2
          local.get $#local1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=4
          i32.const 2
          local.set $#local1
          br $#label0
        end
        local.get $#local2
        local.get $#local1
        i32.store8 offset=4
        i32.const 1
        local.set $#local1
      end
      local.get $#local2
      i32.const 8
      i32.add
      local.get $#local0
      i32.load offset=8
      local.get $#local2
      i32.const 4
      i32.add
      local.get $#local1
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E
      block $#label0
        local.get $#local2
        i32.load8_u offset=8
        local.tee $#local1
        i32.const 4
        i32.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.set $#local3
        local.get $#local2
        i64.load offset=8
        local.set $#local4
        block $#label1
          block $#label2
            local.get $#local0
            i32.load8_u
            local.tee $#local5
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local5
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local3
          i32.load
          local.set $#local6
          block $#label2
            local.get $#local3
            i32.const 4
            i32.add
            i32.load
            local.tee $#local5
            i32.load
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local6
            local.get $#local7
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local5
            i32.load offset=4
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local6
            local.get $#local7
            local.get $#local5
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local3
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local0
        local.get $#local4
        i64.store align=4
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local1
      i32.const 4
      i32.ne
    )
    (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E (;28;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32) (local $#local12 i64) (local $#local13 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        local.get $#local1
                        i32.load
                        local.tee $#local1
                        i32.load offset=16
                        br_if $#label8
                        local.get $#local1
                        i32.const -1
                        i32.store offset=16
                        local.get $#local4
                        i32.const 8
                        i32.add
                        i32.const 10
                        local.get $#local2
                        local.get $#local3
                        call $_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E
                        block $#label9
                          block $#label10
                            local.get $#local4
                            i32.load offset=8
                            i32.const 1
                            i32.and
                            i32.eqz
                            br_if $#label10
                            local.get $#local3
                            local.get $#local4
                            i32.load offset=12
                            i32.const 1
                            i32.add
                            local.tee $#local5
                            i32.ge_u
                            br_if $#label9
                            local.get $#local4
                            i32.const 0
                            i32.store offset=32
                            local.get $#local4
                            i32.const 1
                            i32.store offset=20
                            local.get $#local4
                            i64.const 4
                            i64.store offset=24 align=4
                            local.get $#local4
                            global.get $GOT.data.internal.__memory_base
                            local.tee $#local1
                            i32.const 1054544
                            i32.add
                            i32.store offset=16
                            local.get $#local4
                            i32.const 16
                            i32.add
                            local.get $#local1
                            i32.const 1054552
                            i32.add
                            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                            unreachable
                          end
                          block $#label10
                            local.get $#local1
                            i32.load offset=28
                            local.tee $#local6
                            br_if $#label10
                            i32.const 0
                            local.set $#local6
                            br $#label2
                          end
                          local.get $#local1
                          i32.load offset=24
                          local.tee $#local7
                          local.get $#local6
                          i32.add
                          i32.const -1
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.ne
                          br_if $#label2
                          i32.const 0
                          local.set $#local8
                          loop $#label10
                            local.get $#local4
                            local.get $#local6
                            local.get $#local8
                            i32.sub
                            local.tee $#local9
                            i32.store offset=44
                            local.get $#local4
                            local.get $#local7
                            local.get $#local8
                            i32.add
                            local.tee $#local10
                            i32.store offset=40
                            local.get $#local4
                            i32.const 16
                            i32.add
                            i32.const 1
                            local.get $#local4
                            i32.const 40
                            i32.add
                            i32.const 1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    block $#label15
                                      local.get $#local4
                                      i32.load16_u offset=16
                                      i32.const 1
                                      i32.ne
                                      br_if $#label15
                                      local.get $#local9
                                      local.set $#local11
                                      local.get $#local4
                                      i32.load16_u offset=18
                                      local.tee $#local5
                                      i32.const 8
                                      i32.eq
                                      br_if $#label14
                                      local.get $#local1
                                      i32.const 0
                                      i32.store8 offset=32
                                      local.get $#local5
                                      i32.const 27
                                      i32.eq
                                      br_if $#label11
                                      local.get $#local5
                                      i64.extend_i32_u
                                      i64.const 32
                                      i64.shl
                                      local.set $#local12
                                      br $#label13
                                    end
                                    local.get $#local4
                                    i32.load offset=20
                                    local.set $#local11
                                  end
                                  local.get $#local1
                                  i32.const 0
                                  i32.store8 offset=32
                                  local.get $#local11
                                  br_if $#label12
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054532
                                  i32.add
                                  i64.extend_i32_u
                                  i64.const 32
                                  i64.shl
                                  i64.const 2
                                  i64.or
                                  local.set $#local12
                                end
                                block $#label13
                                  local.get $#local8
                                  i32.eqz
                                  br_if $#label13
                                  block $#label14
                                    local.get $#local9
                                    i32.eqz
                                    br_if $#label14
                                    local.get $#local7
                                    local.get $#local10
                                    local.get $#local9
                                    memory.copy
                                  end
                                  local.get $#local1
                                  local.get $#local9
                                  i32.store offset=28
                                end
                                local.get $#local12
                                i64.const 255
                                i64.and
                                i64.const 4
                                i64.ne
                                br_if $#label7
                                local.get $#local1
                                i32.load offset=28
                                local.set $#local6
                                br $#label2
                              end
                              local.get $#local11
                              local.get $#local8
                              i32.add
                              local.set $#local8
                            end
                            local.get $#local8
                            local.get $#local6
                            i32.ge_u
                            br_if $#label3
                            br $#label10
                          end
                        end
                        block $#label9
                          local.get $#local1
                          i32.load offset=28
                          local.tee $#local8
                          br_if $#label9
                          local.get $#local5
                          i32.eqz
                          br_if $#label4
                          local.get $#local2
                          local.set $#local6
                          local.get $#local5
                          local.set $#local8
                          loop $#label10
                            local.get $#local4
                            local.get $#local8
                            i32.store offset=44
                            local.get $#local4
                            local.get $#local6
                            i32.store offset=40
                            local.get $#local4
                            i32.const 16
                            i32.add
                            i32.const 1
                            local.get $#local4
                            i32.const 40
                            i32.add
                            i32.const 1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    local.get $#local4
                                    i32.load16_u offset=16
                                    i32.const 1
                                    i32.ne
                                    br_if $#label14
                                    local.get $#local4
                                    i64.load16_u offset=18
                                    local.tee $#local12
                                    i64.const 27
                                    i64.eq
                                    br_if $#label11
                                    local.get $#local12
                                    i64.const 32
                                    i64.shl
                                    local.set $#local12
                                    br $#label13
                                  end
                                  local.get $#local4
                                  i32.load offset=20
                                  local.tee $#local11
                                  br_if $#label12
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054584
                                  i32.add
                                  i64.load
                                  local.set $#local12
                                end
                                local.get $#local12
                                i64.const 255
                                i64.and
                                i64.const 4
                                i64.eq
                                br_if $#label4
                                local.get $#local12
                                i64.const -4294967041
                                i64.and
                                i64.const 34359738368
                                i64.eq
                                br_if $#label4
                                local.get $#local0
                                local.get $#local12
                                i64.store align=4
                                br $#label1
                              end
                              local.get $#local8
                              local.get $#local11
                              i32.lt_u
                              br_if $#label6
                              local.get $#local6
                              local.get $#local11
                              i32.add
                              local.set $#local6
                              local.get $#local8
                              local.get $#local11
                              i32.sub
                              local.set $#local8
                            end
                            local.get $#local8
                            br_if $#label10
                            br $#label4
                          end
                        end
                        block $#label9
                          block $#label10
                            block $#label11
                              local.get $#local5
                              local.get $#local1
                              i32.load offset=20
                              local.get $#local8
                              i32.sub
                              i32.lt_u
                              br_if $#label11
                              local.get $#local4
                              i32.const 16
                              i32.add
                              local.get $#local1
                              i32.const 20
                              i32.add
                              local.get $#local2
                              local.get $#local5
                              call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
                              local.get $#local4
                              i32.load8_u offset=16
                              i32.const 4
                              i32.eq
                              br_if $#label10
                              local.get $#local0
                              local.get $#local4
                              i64.load offset=16
                              i64.store align=4
                              br $#label1
                            end
                            block $#label11
                              local.get $#local5
                              i32.eqz
                              br_if $#label11
                              local.get $#local1
                              i32.load offset=24
                              local.get $#local8
                              i32.add
                              local.get $#local2
                              local.get $#local5
                              memory.copy
                            end
                            local.get $#local1
                            local.get $#local8
                            local.get $#local5
                            i32.add
                            local.tee $#local11
                            i32.store offset=28
                            br $#label9
                          end
                          local.get $#local1
                          i32.load offset=28
                          local.set $#local11
                        end
                        local.get $#local11
                        i32.eqz
                        br_if $#label4
                        local.get $#local1
                        i32.load offset=24
                        local.set $#local7
                        i32.const 0
                        local.set $#local8
                        loop $#label9
                          local.get $#local4
                          local.get $#local11
                          local.get $#local8
                          i32.sub
                          local.tee $#local9
                          i32.store offset=44
                          local.get $#local4
                          local.get $#local7
                          local.get $#local8
                          i32.add
                          local.tee $#local10
                          i32.store offset=40
                          local.get $#local4
                          i32.const 16
                          i32.add
                          i32.const 1
                          local.get $#local4
                          i32.const 40
                          i32.add
                          i32.const 1
                          call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                          block $#label10
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    local.get $#local4
                                    i32.load16_u offset=16
                                    i32.const 1
                                    i32.ne
                                    br_if $#label14
                                    local.get $#local9
                                    local.set $#local6
                                    local.get $#local4
                                    i32.load16_u offset=18
                                    local.tee $#local13
                                    i32.const 8
                                    i32.eq
                                    br_if $#label13
                                    local.get $#local1
                                    i32.const 0
                                    i32.store8 offset=32
                                    local.get $#local13
                                    i32.const 27
                                    i32.eq
                                    br_if $#label10
                                    local.get $#local13
                                    i64.extend_i32_u
                                    i64.const 32
                                    i64.shl
                                    local.set $#local12
                                    br $#label12
                                  end
                                  local.get $#local4
                                  i32.load offset=20
                                  local.set $#local6
                                end
                                local.get $#local1
                                i32.const 0
                                i32.store8 offset=32
                                local.get $#local6
                                br_if $#label11
                                global.get $GOT.data.internal.__memory_base
                                i32.const 1054532
                                i32.add
                                i64.extend_i32_u
                                i64.const 32
                                i64.shl
                                i64.const 2
                                i64.or
                                local.set $#local12
                              end
                              block $#label12
                                local.get $#local8
                                i32.eqz
                                br_if $#label12
                                block $#label13
                                  local.get $#local9
                                  i32.eqz
                                  br_if $#label13
                                  local.get $#local7
                                  local.get $#local10
                                  local.get $#local9
                                  memory.copy
                                end
                                local.get $#local1
                                local.get $#local9
                                i32.store offset=28
                              end
                              local.get $#local12
                              i64.const 255
                              i64.and
                              i64.const 4
                              i64.eq
                              br_if $#label4
                              local.get $#local0
                              local.get $#local12
                              i64.store align=4
                              br $#label1
                            end
                            local.get $#local6
                            local.get $#local8
                            i32.add
                            local.set $#local8
                          end
                          local.get $#local8
                          local.get $#local11
                          i32.ge_u
                          br_if $#label5
                          br $#label9
                        end
                      end
                      global.get $GOT.data.internal.__memory_base
                      i32.const 1054648
                      i32.add
                      call $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE
                      unreachable
                    end
                    local.get $#local0
                    local.get $#local12
                    i64.store align=4
                    br $#label1
                  end
                  local.get $#local11
                  local.get $#local8
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054784
                  i32.add
                  call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
                  unreachable
                end
                block $#label5
                  local.get $#local8
                  local.get $#local11
                  i32.gt_u
                  br_if $#label5
                  local.get $#local1
                  i32.const 0
                  i32.store offset=28
                  br $#label4
                end
                local.get $#local8
                local.get $#local11
                global.get $GOT.data.internal.__memory_base
                i32.const 1054252
                i32.add
                call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
                unreachable
              end
              local.get $#local2
              local.get $#local5
              i32.add
              local.set $#local11
              block $#label4
                local.get $#local3
                local.get $#local5
                i32.sub
                local.tee $#local8
                local.get $#local1
                i32.load offset=20
                local.get $#local1
                i32.load offset=28
                local.tee $#local6
                i32.sub
                i32.lt_u
                br_if $#label4
                local.get $#local0
                local.get $#local1
                i32.const 20
                i32.add
                local.get $#local11
                local.get $#local8
                call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
                br $#label1
              end
              block $#label4
                local.get $#local8
                i32.eqz
                br_if $#label4
                local.get $#local1
                i32.load offset=24
                local.get $#local6
                i32.add
                local.get $#local11
                local.get $#local8
                memory.copy
              end
              local.get $#local0
              i32.const 4
              i32.store8
              local.get $#local1
              local.get $#local6
              local.get $#local8
              i32.add
              i32.store offset=28
              br $#label1
            end
            local.get $#local8
            local.get $#local6
            i32.gt_u
            br_if $#label0
            i32.const 0
            local.set $#local6
            local.get $#local1
            i32.const 0
            i32.store offset=28
          end
          block $#label2
            local.get $#local3
            local.get $#local1
            i32.load offset=20
            local.get $#local6
            i32.sub
            i32.lt_u
            br_if $#label2
            local.get $#local0
            local.get $#local1
            i32.const 20
            i32.add
            local.get $#local2
            local.get $#local3
            call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E
            br $#label1
          end
          block $#label2
            local.get $#local3
            i32.eqz
            br_if $#label2
            local.get $#local1
            i32.load offset=24
            local.get $#local6
            i32.add
            local.get $#local2
            local.get $#local3
            memory.copy
          end
          local.get $#local0
          i32.const 4
          i32.store8
          local.get $#local1
          local.get $#local6
          local.get $#local3
          i32.add
          i32.store offset=28
        end
        local.get $#local1
        local.get $#local1
        i32.load offset=16
        i32.const 1
        i32.add
        i32.store offset=16
        local.get $#local4
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local8
      local.get $#local6
      global.get $GOT.data.internal.__memory_base
      i32.const 1054252
      i32.add
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17ha202ea95069de918E (;29;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i64) (local $#local7 i32) (local $#local8 i32) (local $#local9 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i32.const 0
      i32.store offset=12
      block $#label0
        block $#label1
          local.get $#local1
          i32.const 128
          i32.lt_u
          br_if $#label1
          block $#label2
            local.get $#local1
            i32.const 2048
            i32.lt_u
            br_if $#label2
            block $#label3
              local.get $#local1
              i32.const 65536
              i32.lt_u
              br_if $#label3
              local.get $#local2
              local.get $#local1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get $#local2
              local.get $#local1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get $#local2
              local.get $#local1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get $#local2
              local.get $#local1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set $#local1
              br $#label0
            end
            local.get $#local2
            local.get $#local1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $#local2
            local.get $#local1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get $#local2
            local.get $#local1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set $#local1
            br $#label0
          end
          local.get $#local2
          local.get $#local1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get $#local2
          local.get $#local1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set $#local1
          br $#label0
        end
        local.get $#local2
        local.get $#local1
        i32.store8 offset=12
        i32.const 1
        local.set $#local1
      end
      i32.const 0
      local.set $#local3
      block $#label0
        i32.const 0
        local.get $#local0
        i32.load offset=8
        local.tee $#local4
        i32.load offset=4
        local.tee $#local5
        local.get $#local4
        i64.load offset=8
        local.tee $#local6
        i64.const 4294967295
        local.get $#local6
        i64.const 4294967295
        i64.lt_u
        select
        i32.wrap_i64
        i32.sub
        local.tee $#local7
        local.get $#local7
        local.get $#local5
        i32.gt_u
        select
        local.tee $#local7
        local.get $#local1
        local.get $#local7
        local.get $#local1
        i32.lt_u
        select
        local.tee $#local8
        i32.eqz
        br_if $#label0
        local.get $#local4
        i32.load
        local.get $#local6
        local.get $#local5
        i64.extend_i32_u
        local.tee $#local9
        local.get $#local6
        local.get $#local9
        i64.lt_u
        select
        i32.wrap_i64
        i32.add
        local.get $#local2
        i32.const 12
        i32.add
        local.get $#local8
        memory.copy
      end
      local.get $#local4
      local.get $#local6
      local.get $#local8
      i64.extend_i32_u
      i64.add
      i64.store offset=8
      block $#label0
        local.get $#local7
        local.get $#local1
        i32.ge_u
        br_if $#label0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054584
        i32.add
        i64.load
        local.tee $#local6
        i64.const 255
        i64.and
        i64.const 4
        i64.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.set $#local4
        block $#label1
          block $#label2
            local.get $#local0
            i32.load8_u
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local4
          i32.load
          local.set $#local7
          block $#label2
            local.get $#local4
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local7
            local.get $#local5
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local7
            local.get $#local5
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local4
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local0
        local.get $#local6
        i64.store align=4
        i32.const 1
        local.set $#local3
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local3
    )
    (func $_ZN4core3fmt5Write10write_char17ha824fcc3f54c220bE (;30;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i32.const 0
      i32.store offset=12
      block $#label0
        block $#label1
          local.get $#local1
          i32.const 128
          i32.lt_u
          br_if $#label1
          block $#label2
            local.get $#local1
            i32.const 2048
            i32.lt_u
            br_if $#label2
            block $#label3
              local.get $#local1
              i32.const 65536
              i32.lt_u
              br_if $#label3
              local.get $#local2
              local.get $#local1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get $#local2
              local.get $#local1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get $#local2
              local.get $#local1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get $#local2
              local.get $#local1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set $#local1
              br $#label0
            end
            local.get $#local2
            local.get $#local1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $#local2
            local.get $#local1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get $#local2
            local.get $#local1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set $#local1
            br $#label0
          end
          local.get $#local2
          local.get $#local1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get $#local2
          local.get $#local1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set $#local1
          br $#label0
        end
        local.get $#local2
        local.get $#local1
        i32.store8 offset=12
        i32.const 1
        local.set $#local1
      end
      local.get $#local0
      local.get $#local2
      i32.const 12
      i32.add
      local.get $#local1
      call $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE
      local.set $#local1
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68e160dc1acd4cebE (;31;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i64) (local $#local6 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      i32.const 0
      local.set $#local4
      block $#label0
        block $#label1
          local.get $#local2
          i32.eqz
          br_if $#label1
          loop $#label2
            local.get $#local3
            local.get $#local2
            i32.store offset=4
            local.get $#local3
            local.get $#local1
            i32.store
            local.get $#local3
            i32.const 8
            i32.add
            i32.const 2
            local.get $#local3
            i32.const 1
            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local3
                    i32.load16_u offset=8
                    i32.const 1
                    i32.ne
                    br_if $#label6
                    local.get $#local3
                    i64.load16_u offset=10
                    local.tee $#local5
                    i64.const 27
                    i64.eq
                    br_if $#label3
                    local.get $#local5
                    i64.const 32
                    i64.shl
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local3
                  i32.load offset=12
                  local.tee $#local6
                  br_if $#label4
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  local.set $#local5
                end
                local.get $#local5
                i64.const 255
                i64.and
                i64.const 4
                i64.eq
                br_if $#label1
                local.get $#local0
                i32.load offset=4
                local.set $#local1
                block $#label5
                  block $#label6
                    local.get $#local0
                    i32.load8_u
                    local.tee $#local2
                    i32.const 4
                    i32.gt_u
                    br_if $#label6
                    local.get $#local2
                    i32.const 3
                    i32.ne
                    br_if $#label5
                  end
                  local.get $#local1
                  i32.load
                  local.set $#local6
                  block $#label6
                    local.get $#local1
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee $#local2
                    i32.load
                    local.tee $#local4
                    i32.eqz
                    br_if $#label6
                    local.get $#local6
                    local.get $#local4
                    call_indirect (type $#type1)
                  end
                  block $#label6
                    local.get $#local2
                    i32.load offset=4
                    local.tee $#local4
                    i32.eqz
                    br_if $#label6
                    local.get $#local6
                    local.get $#local4
                    local.get $#local2
                    i32.load offset=8
                    call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  end
                  local.get $#local1
                  i32.const 12
                  i32.const 4
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                end
                local.get $#local0
                local.get $#local5
                i64.store align=4
                i32.const 1
                local.set $#local4
                br $#label1
              end
              local.get $#local2
              local.get $#local6
              i32.lt_u
              br_if $#label0
              local.get $#local1
              local.get $#local6
              i32.add
              local.set $#local1
              local.get $#local2
              local.get $#local6
              i32.sub
              local.set $#local2
            end
            local.get $#local2
            br_if $#label2
          end
        end
        local.get $#local3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get $#local4
        return
      end
      local.get $#local6
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN4core3fmt5Write10write_char17hcc1e03d97af61dd3E (;32;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i32.const 0
      i32.store offset=12
      block $#label0
        block $#label1
          local.get $#local1
          i32.const 128
          i32.lt_u
          br_if $#label1
          block $#label2
            local.get $#local1
            i32.const 2048
            i32.lt_u
            br_if $#label2
            block $#label3
              local.get $#local1
              i32.const 65536
              i32.lt_u
              br_if $#label3
              local.get $#local2
              local.get $#local1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get $#local2
              local.get $#local1
              i32.const 18
              i32.shr_u
              i32.const 240
              i32.or
              i32.store8 offset=12
              local.get $#local2
              local.get $#local1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get $#local2
              local.get $#local1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 4
              local.set $#local1
              br $#label0
            end
            local.get $#local2
            local.get $#local1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get $#local2
            local.get $#local1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get $#local2
            local.get $#local1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set $#local1
            br $#label0
          end
          local.get $#local2
          local.get $#local1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get $#local2
          local.get $#local1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set $#local1
          br $#label0
        end
        local.get $#local2
        local.get $#local1
        i32.store8 offset=12
        i32.const 1
        local.set $#local1
      end
      block $#label0
        local.get $#local1
        local.get $#local0
        i32.load offset=8
        local.tee $#local0
        i32.load
        local.get $#local0
        i32.load offset=8
        local.tee $#local3
        i32.sub
        i32.le_u
        br_if $#label0
        local.get $#local0
        local.get $#local3
        local.get $#local1
        i32.const 1
        i32.const 1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get $#local0
        i32.load offset=8
        local.set $#local3
      end
      block $#label0
        local.get $#local1
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local3
        i32.add
        local.get $#local2
        i32.const 12
        i32.add
        local.get $#local1
        memory.copy
      end
      local.get $#local0
      local.get $#local3
      local.get $#local1
      i32.add
      i32.store offset=8
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E (;33;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32)
      (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i64) (local $#local9 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            local.get $#local2
            i32.add
            local.tee $#local2
            local.get $#local1
            i32.ge_u
            br_if $#label2
            i32.const 0
            local.set $#local6
            br $#label1
          end
          i32.const 0
          local.set $#local6
          block $#label2
            local.get $#local3
            local.get $#local4
            i32.add
            i32.const -1
            i32.add
            i32.const 0
            local.get $#local3
            i32.sub
            i32.and
            i64.extend_i32_u
            local.get $#local2
            local.get $#local0
            i32.load
            local.tee $#local1
            i32.const 1
            i32.shl
            local.tee $#local7
            local.get $#local2
            local.get $#local7
            i32.gt_u
            select
            local.tee $#local2
            i32.const 8
            i32.const 4
            local.get $#local4
            i32.const 1
            i32.eq
            select
            local.tee $#local7
            local.get $#local2
            local.get $#local7
            i32.gt_u
            select
            local.tee $#local7
            i64.extend_i32_u
            i64.mul
            local.tee $#local8
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            i32.eqz
            br_if $#label2
            br $#label1
          end
          local.get $#local8
          i32.wrap_i64
          local.tee $#local9
          i32.const -2147483648
          local.get $#local3
          i32.sub
          i32.gt_u
          br_if $#label1
          i32.const 0
          local.set $#local2
          block $#label2
            local.get $#local1
            i32.eqz
            br_if $#label2
            local.get $#local5
            local.get $#local1
            local.get $#local4
            i32.mul
            i32.store offset=28
            local.get $#local5
            local.get $#local0
            i32.load offset=4
            i32.store offset=20
            local.get $#local3
            local.set $#local2
          end
          local.get $#local5
          local.get $#local2
          i32.store offset=24
          local.get $#local5
          i32.const 8
          i32.add
          local.get $#local3
          local.get $#local9
          local.get $#local5
          i32.const 20
          i32.add
          call $_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E
          local.get $#local5
          i32.load offset=8
          i32.const 1
          i32.ne
          br_if $#label0
          local.get $#local5
          i32.load offset=16
          local.set $#local2
          local.get $#local5
          i32.load offset=12
          local.set $#local6
        end
        local.get $#local6
        local.get $#local2
        global.get $GOT.data.internal.__memory_base
        i32.const 1054284
        i32.add
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      local.get $#local5
      i32.load offset=12
      local.set $#local3
      local.get $#local0
      local.get $#local7
      i32.store
      local.get $#local0
      local.get $#local3
      i32.store offset=4
      local.get $#local5
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN4core3fmt5Write9write_fmt17h55598fc5c0cb65b6E (;34;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054380
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h626effaba0392c72E (;35;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054428
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17h833e28c405f61703E (;36;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054404
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hb7fc36774a22570cE (;37;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054452
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt5Write9write_fmt17hbdd1e816e40e27eaE (;38;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1054356
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN3std9panicking12default_hook17h8deeaf2f2b300de3E (;39;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      i32.const 3
      local.set $#local2
      block $#label0
        local.get $#local0
        i32.load8_u offset=13
        br_if $#label0
        i32.const 1
        local.set $#local2
        global.get $GOT.data.internal.__memory_base
        i32.const 1055872
        i32.add
        i32.load
        i32.const 1
        i32.gt_u
        br_if $#label0
        call $_ZN3std5panic19get_backtrace_style17h54380a7febe96116E
        i32.const 255
        i32.and
        local.set $#local2
      end
      local.get $#local1
      local.get $#local2
      i32.store8 offset=15
      local.get $#local1
      local.get $#local0
      i32.load offset=8
      i32.store offset=16
      local.get $#local1
      local.get $#local0
      i32.load
      local.get $#local0
      i32.load offset=4
      call $_ZN3std9panicking14payload_as_str17h6b6acf98476ffb56E
      local.get $#local1
      local.get $#local1
      i64.load
      i64.store offset=20 align=4
      global.get $GOT.data.internal.__memory_base
      i32.const 1055775
      i32.add
      i32.load8_u
      local.set $#local0
      local.get $#local1
      local.get $#local1
      i32.const 15
      i32.add
      i32.store offset=36
      local.get $#local1
      local.get $#local1
      i32.const 20
      i32.add
      i32.store offset=32
      local.get $#local1
      local.get $#local1
      i32.const 16
      i32.add
      i32.store offset=28
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local0
                br_if $#label4
                local.get $#local1
                i64.const 0
                i64.store offset=40 align=4
                br $#label3
              end
              global.get $GOT.data.internal.__memory_base
              local.tee $#local0
              i32.const 1055775
              i32.add
              i32.const 1
              i32.store8
              local.get $#local0
              i32.const 1055880
              i32.add
              local.tee $#local2
              i32.load
              local.set $#local0
              local.get $#local2
              i32.const 0
              i32.store
              local.get $#local1
              i32.const 0
              i32.store offset=40
              local.get $#local1
              local.get $#local0
              i32.store offset=44
              local.get $#local0
              br_if $#label2
            end
            global.get $GOT.data.internal.__memory_base
            local.set $#local0
            local.get $#local1
            i32.const 40
            i32.add
            call $_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE
            local.get $#local1
            i32.const 28
            i32.add
            local.get $#local1
            i32.const 79
            i32.add
            local.get $#local0
            i32.const 1054984
            i32.add
            call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE
            br $#label1
          end
          local.get $#local0
          i32.load8_u offset=8
          local.set $#local2
          local.get $#local0
          i32.const 1
          i32.store8 offset=8
          local.get $#local1
          local.get $#local2
          i32.store8 offset=51
          local.get $#local2
          i32.const 1
          i32.eq
          br_if $#label0
          local.get $#local1
          i32.const 28
          i32.add
          local.get $#local0
          i32.const 12
          i32.add
          global.get $GOT.data.internal.__memory_base
          local.tee $#local2
          i32.const 1054944
          i32.add
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE
          local.get $#local0
          i32.const 0
          i32.store8 offset=8
          local.get $#local2
          i32.const 1055775
          i32.add
          i32.const 1
          i32.store8
          local.get $#local2
          i32.const 1055880
          i32.add
          local.tee $#local3
          i32.load
          local.set $#local2
          local.get $#local3
          local.get $#local0
          i32.store
          local.get $#local1
          local.get $#local2
          i32.store offset=56
          local.get $#local1
          i32.const 1
          i32.store offset=52
          local.get $#local2
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local2
          i32.load
          local.tee $#local0
          i32.const -1
          i32.add
          i32.store
          local.get $#local0
          i32.const 1
          i32.ne
          br_if $#label1
          local.get $#local1
          i32.const 56
          i32.add
          call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
        end
        local.get $#local1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local1
      i64.const 0
      i64.store offset=64 align=4
      local.get $#local1
      i64.const 17179869185
      i64.store offset=56 align=4
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      local.tee $#local0
      i32.const 1054800
      i32.add
      i32.store offset=52
      i32.const 0
      local.get $#local1
      i32.const 51
      i32.add
      global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
      local.get $#local1
      i32.const 52
      i32.add
      local.get $#local0
      i32.const 1054808
      i32.add
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN4core3ptr119drop_in_place$LT$std..io..default_write_fmt..Adapter$LT$std..io..cursor..Cursor$LT$$RF$mut$u20$$u5b$u8$u5d$$GT$$GT$$GT$17h16ee2cfc92135802E (;40;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32)
      local.get $#local0
      i32.load offset=4
      local.set $#local1
      block $#label0
        block $#label1
          local.get $#local0
          i32.load8_u
          local.tee $#local0
          i32.const 4
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const 3
          i32.ne
          br_if $#label0
        end
        local.get $#local1
        i32.load
        local.set $#local2
        block $#label1
          local.get $#local1
          i32.const 4
          i32.add
          i32.load
          local.tee $#local0
          i32.load
          local.tee $#local3
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local3
          call_indirect (type $#type1)
        end
        block $#label1
          local.get $#local0
          i32.load offset=4
          local.tee $#local3
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local3
          local.get $#local0
          i32.load offset=8
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local1
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr199drop_in_place$LT$core..result..Result$LT$core..option..Option$LT$alloc..sync..Arc$LT$std..sync..poison..mutex..Mutex$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$$C$std..thread..local..AccessError$GT$$GT$17h57ce350d8f97d7bdE (;41;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32)
      block $#label0
        local.get $#local0
        i32.load
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.tee $#local1
        i32.eqz
        br_if $#label0
        local.get $#local1
        local.get $#local1
        i32.load
        local.tee $#local2
        i32.const -1
        i32.add
        i32.store
        local.get $#local2
        i32.const 1
        i32.ne
        br_if $#label0
        local.get $#local0
        i32.const 4
        i32.add
        call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
      end
    )
    (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E (;42;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local0
        i32.const 12
        i32.add
        i32.load
        local.tee $#local1
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.const 16
        i32.add
        i32.load
        local.get $#local1
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      block $#label0
        local.get $#local0
        i32.const -1
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local0
        i32.load offset=4
        local.tee $#local1
        i32.const -1
        i32.add
        i32.store offset=4
        local.get $#local1
        i32.const 1
        i32.ne
        br_if $#label0
        local.get $#local0
        i32.const 24
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4b180a35608f6a06E (;43;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local1
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local1
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h6e1c45cbfa22c4b0E (;44;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local1
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local1
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h022ec53c9c32dafeE (;45;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local1
        i32.const -2147483648
        i32.or
        i32.const -2147483648
        i32.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local1
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E (;46;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      block $#label0
        block $#label1
          local.get $#local0
          i32.const 255
          i32.and
          local.tee $#local0
          i32.const 4
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const 3
          i32.ne
          br_if $#label0
        end
        local.get $#local1
        i32.load
        local.set $#local2
        block $#label1
          local.get $#local1
          i32.const 4
          i32.add
          i32.load
          local.tee $#local0
          i32.load
          local.tee $#local3
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local3
          call_indirect (type $#type1)
        end
        block $#label1
          local.get $#local0
          i32.load offset=4
          local.tee $#local3
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local3
          local.get $#local0
          i32.load offset=8
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local1
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
    )
    (func $_ZN4core5panic12PanicPayload6as_str17h20d8f31a3d632eefE (;47;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      i32.const 0
      i32.store
    )
    (func $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E (;48;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32)
      (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      local.get $#local5
      local.get $#local2
      i32.store offset=12
      local.get $#local5
      local.get $#local1
      i32.store offset=8
      local.get $#local0
      local.get $#local5
      i32.const 8
      i32.add
      global.get $GOT.data.internal.__memory_base
      i32.const 1054236
      i32.add
      local.tee $#local2
      local.get $#local5
      i32.const 12
      i32.add
      local.get $#local2
      local.get $#local3
      local.get $#local4
      call $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E
      unreachable
    )
    (func $_ZN52_$LT$$RF$mut$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hfa61e2b14c461e6cE (;49;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load
      local.get $#local1
      local.get $#local0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type $#type2)
    )
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h68b670546d4b4325E (;50;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local1
          i32.const 128
          i32.lt_u
          br_if $#label1
          local.get $#local2
          i32.const 0
          i32.store offset=12
          block $#label2
            block $#label3
              local.get $#local1
              i32.const 2048
              i32.lt_u
              br_if $#label3
              block $#label4
                local.get $#local1
                i32.const 65536
                i32.lt_u
                br_if $#label4
                local.get $#local2
                local.get $#local1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=15
                local.get $#local2
                local.get $#local1
                i32.const 18
                i32.shr_u
                i32.const 240
                i32.or
                i32.store8 offset=12
                local.get $#local2
                local.get $#local1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get $#local2
                local.get $#local1
                i32.const 12
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 4
                local.set $#local1
                br $#label2
              end
              local.get $#local2
              local.get $#local1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get $#local2
              local.get $#local1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get $#local2
              local.get $#local1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set $#local1
              br $#label2
            end
            local.get $#local2
            local.get $#local1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get $#local2
            local.get $#local1
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            i32.store8 offset=12
            i32.const 2
            local.set $#local1
          end
          block $#label2
            local.get $#local1
            local.get $#local0
            i32.load
            local.get $#local0
            i32.load offset=8
            local.tee $#local3
            i32.sub
            i32.le_u
            br_if $#label2
            local.get $#local0
            local.get $#local3
            local.get $#local1
            i32.const 1
            i32.const 1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get $#local0
            i32.load offset=8
            local.set $#local3
          end
          block $#label2
            local.get $#local1
            i32.eqz
            br_if $#label2
            local.get $#local0
            i32.load offset=4
            local.get $#local3
            i32.add
            local.get $#local2
            i32.const 12
            i32.add
            local.get $#local1
            memory.copy
          end
          local.get $#local0
          local.get $#local3
          local.get $#local1
          i32.add
          i32.store offset=8
          br $#label0
        end
        block $#label1
          local.get $#local0
          i32.load offset=8
          local.tee $#local3
          local.get $#local0
          i32.load
          i32.ne
          br_if $#label1
          local.get $#local0
          global.get $GOT.data.internal.__memory_base
          i32.const 1054268
          i32.add
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E
        end
        local.get $#local0
        i32.load offset=4
        local.get $#local3
        i32.add
        local.get $#local1
        i32.store8
        local.get $#local0
        local.get $#local3
        i32.const 1
        i32.add
        i32.store offset=8
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h486a897459032d71E (;51;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32)
      block $#label0
        local.get $#local2
        local.get $#local0
        i32.load
        local.get $#local0
        i32.load offset=8
        local.tee $#local3
        i32.sub
        i32.le_u
        br_if $#label0
        local.get $#local0
        local.get $#local3
        local.get $#local2
        i32.const 1
        i32.const 1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get $#local0
        i32.load offset=8
        local.set $#local3
      end
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local3
        i32.add
        local.get $#local1
        local.get $#local2
        memory.copy
      end
      local.get $#local0
      local.get $#local3
      local.get $#local2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $_ZN5alloc7raw_vec11finish_grow17h7ebb6dc860794861E (;52;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      block $#label0
        local.get $#local2
        i32.const 0
        i32.lt_s
        br_if $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local3
              i32.load offset=4
              i32.eqz
              br_if $#label3
              block $#label4
                local.get $#local3
                i32.load offset=8
                local.tee $#local4
                br_if $#label4
                block $#label5
                  local.get $#local2
                  br_if $#label5
                  local.get $#local1
                  local.set $#local3
                  br $#label1
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                br $#label2
              end
              local.get $#local3
              i32.load
              local.get $#local4
              local.get $#local1
              local.get $#local2
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
              local.set $#local3
              br $#label1
            end
            block $#label3
              local.get $#local2
              br_if $#label3
              local.get $#local1
              local.set $#local3
              br $#label1
            end
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
          end
          local.get $#local2
          local.get $#local1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.set $#local3
        end
        block $#label1
          local.get $#local3
          br_if $#label1
          local.get $#local0
          local.get $#local2
          i32.store offset=8
          local.get $#local0
          local.get $#local1
          i32.store offset=4
          local.get $#local0
          i32.const 1
          i32.store
          return
        end
        local.get $#local0
        local.get $#local2
        i32.store offset=8
        local.get $#local0
        local.get $#local3
        i32.store offset=4
        local.get $#local0
        i32.const 0
        i32.store
        return
      end
      local.get $#local0
      i32.const 0
      i32.store offset=4
      local.get $#local0
      i32.const 1
      i32.store
    )
    (func $"#func53 _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE" (@name "_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h27e60e158b73f3abE") (;53;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load offset=4
      local.get $#local0
      i32.load offset=8
      local.get $#local1
      call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
    )
    (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fe2f91d1a557a12E (;54;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      local.get $#local0
      i32.const 4
      i32.add
      i32.store offset=12
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      local.tee $#local3
      i32.const 1049005
      i32.add
      i32.const 9
      local.get $#local3
      i32.const 1049014
      i32.add
      i32.const 11
      local.get $#local0
      local.get $#local3
      i32.const 1054300
      i32.add
      local.get $#local3
      i32.const 1049025
      i32.add
      i32.const 9
      local.get $#local2
      i32.const 12
      i32.add
      local.get $#local3
      i32.const 1054316
      i32.add
      call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E
      local.set $#local3
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local3
    )
    (func $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E (;55;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i64) (local $#local4 i64) (local $#local5 i64) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055774
                          i32.add
                          i32.load8_u
                          br_table $#label9 $#label7 $#label8 $#label0 $#label9
                        end
                        global.get $GOT.data.internal.__memory_base
                        i32.const 1055774
                        i32.add
                        i32.const 2
                        i32.store8
                        local.get $#local0
                        i32.load8_u
                        local.set $#local2
                        local.get $#local0
                        i32.const 0
                        i32.store8
                        block $#label9
                          local.get $#local2
                          i32.const 1
                          i32.ne
                          br_if $#label9
                          local.get $#local1
                          i32.const 0
                          i32.store8 offset=39
                          block $#label10
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055776
                            i32.add
                            i32.load8_u offset=40
                            i32.const 3
                            i32.eq
                            br_if $#label10
                            local.get $#local1
                            i32.const 39
                            i32.add
                            call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E
                            local.get $#local1
                            i32.load8_u offset=39
                            i32.const 1
                            i32.and
                            br_if $#label1
                          end
                          block $#label10
                            global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
                            i64.load
                            local.tee $#local3
                            i64.const 0
                            i64.ne
                            br_if $#label10
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055856
                            i32.add
                            i64.load
                            local.set $#local4
                            loop $#label11
                              local.get $#local4
                              i64.const -1
                              i64.eq
                              br_if $#label6
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055856
                              i32.add
                              local.tee $#local0
                              local.get $#local4
                              i64.const 1
                              i64.add
                              local.tee $#local3
                              local.get $#local0
                              i64.load
                              local.tee $#local5
                              local.get $#local5
                              local.get $#local4
                              i64.eq
                              local.tee $#local0
                              select
                              i64.store
                              local.get $#local5
                              local.set $#local4
                              local.get $#local0
                              i32.eqz
                              br_if $#label11
                            end
                            global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
                            local.get $#local3
                            i64.store
                          end
                          block $#label10
                            block $#label11
                              local.get $#local3
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              i64.load
                              i64.eq
                              br_if $#label11
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              local.tee $#local2
                              i32.load8_u offset=12
                              local.set $#local6
                              i32.const 1
                              local.set $#local0
                              local.get $#local2
                              i32.const 1
                              i32.store8 offset=12
                              local.get $#local6
                              br_if $#label1
                              global.get $GOT.data.internal.__memory_base
                              i32.const 1055776
                              i32.add
                              local.get $#local3
                              i64.store
                              br $#label10
                            end
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1055776
                            i32.add
                            i32.load offset=8
                            local.tee $#local0
                            i32.const -1
                            i32.eq
                            br_if $#label1
                            local.get $#local0
                            i32.const 1
                            i32.add
                            local.set $#local0
                          end
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          local.tee $#local2
                          local.get $#local0
                          i32.store offset=8
                          local.get $#local2
                          i32.load offset=16
                          br_if $#label5
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          local.tee $#local0
                          i32.const -1
                          i32.store offset=16
                          local.get $#local0
                          i32.load8_u offset=32
                          br_if $#label2
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          i32.load offset=28
                          local.tee $#local6
                          i32.eqz
                          br_if $#label2
                          global.get $GOT.data.internal.__memory_base
                          i32.const 1055776
                          i32.add
                          i32.load offset=24
                          local.set $#local7
                          i32.const 0
                          local.set $#local0
                          loop $#label10
                            local.get $#local1
                            local.get $#local6
                            local.get $#local0
                            i32.sub
                            local.tee $#local8
                            i32.store offset=44
                            local.get $#local1
                            local.get $#local7
                            local.get $#local0
                            i32.add
                            local.tee $#local9
                            i32.store offset=40
                            local.get $#local1
                            i32.const 12
                            i32.add
                            i32.const 1
                            local.get $#local1
                            i32.const 40
                            i32.add
                            i32.const 1
                            call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    block $#label15
                                      local.get $#local1
                                      i32.load16_u offset=12
                                      i32.const 1
                                      i32.ne
                                      br_if $#label15
                                      local.get $#local8
                                      local.set $#local2
                                      local.get $#local1
                                      i32.load16_u offset=14
                                      local.tee $#local10
                                      i32.const -8
                                      i32.add
                                      br_table $#label14 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label13 $#label11 $#label13
                                    end
                                    local.get $#local1
                                    i32.load offset=16
                                    local.set $#local2
                                  end
                                  local.get $#local2
                                  br_if $#label12
                                  global.get $GOT.data.internal.__memory_base
                                  i32.const 1054532
                                  i32.add
                                  i64.extend_i32_u
                                  i64.const 32
                                  i64.shl
                                  i64.const 2
                                  i64.or
                                  local.set $#local4
                                  br $#label4
                                end
                                local.get $#local10
                                i64.extend_i32_u
                                i64.const 32
                                i64.shl
                                local.set $#local4
                                br $#label4
                              end
                              local.get $#local2
                              local.get $#local0
                              i32.add
                              local.set $#local0
                            end
                            local.get $#local0
                            local.get $#local6
                            i32.ge_u
                            br_if $#label3
                            br $#label10
                          end
                        end
                        global.get $GOT.data.internal.__memory_base
                        i32.const 1054824
                        i32.add
                        call $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE
                        unreachable
                      end
                      local.get $#local1
                      i32.const 0
                      i32.store offset=28
                      local.get $#local1
                      i32.const 1
                      i32.store offset=16
                      local.get $#local1
                      i64.const 4
                      i64.store offset=20 align=4
                      local.get $#local1
                      global.get $GOT.data.internal.__memory_base
                      local.tee $#local0
                      i32.const 1055340
                      i32.add
                      i32.store offset=12
                      local.get $#local1
                      i32.const 12
                      i32.add
                      local.get $#local0
                      i32.const 1054492
                      i32.add
                      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                      unreachable
                    end
                    local.get $#local1
                    i32.const 0
                    i32.store offset=28
                    local.get $#local1
                    i32.const 1
                    i32.store offset=16
                    local.get $#local1
                    i64.const 4
                    i64.store offset=20 align=4
                    local.get $#local1
                    global.get $GOT.data.internal.__memory_base
                    local.tee $#local0
                    i32.const 1055332
                    i32.add
                    i32.store offset=12
                    local.get $#local1
                    i32.const 12
                    i32.add
                    local.get $#local0
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
              local.get $#local4
              i32.wrap_i64
              local.set $#local2
              block $#label4
                local.get $#local0
                i32.eqz
                br_if $#label4
                local.get $#local8
                i32.eqz
                br_if $#label4
                local.get $#local7
                local.get $#local9
                local.get $#local8
                memory.copy
              end
              block $#label4
                local.get $#local2
                i32.const 255
                i32.and
                local.tee $#local0
                i32.const 4
                i32.gt_u
                br_if $#label4
                local.get $#local0
                i32.const 3
                i32.ne
                br_if $#label2
              end
              local.get $#local4
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              local.tee $#local0
              i32.load
              local.set $#local6
              block $#label4
                local.get $#local0
                i32.const 4
                i32.add
                i32.load
                local.tee $#local2
                i32.load
                local.tee $#local8
                i32.eqz
                br_if $#label4
                local.get $#local6
                local.get $#local8
                call_indirect (type $#type1)
              end
              block $#label4
                local.get $#local2
                i32.load offset=4
                local.tee $#local8
                i32.eqz
                br_if $#label4
                local.get $#local6
                local.get $#local8
                local.get $#local2
                i32.load offset=8
                call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              end
              local.get $#local0
              i32.const 12
              i32.const 4
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              br $#label2
            end
            local.get $#local0
            local.get $#local6
            i32.le_u
            br_if $#label2
            local.get $#local0
            local.get $#local6
            global.get $GOT.data.internal.__memory_base
            i32.const 1054252
            i32.add
            call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
            unreachable
          end
          block $#label2
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load offset=20
            local.tee $#local0
            i32.eqz
            br_if $#label2
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load offset=24
            local.get $#local0
            i32.const 1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          local.tee $#local0
          i32.const 0
          i32.store8 offset=32
          local.get $#local0
          i32.const 0
          i32.store offset=28
          local.get $#local0
          i64.const 4294967296
          i64.store offset=20 align=4
          local.get $#local0
          local.get $#local0
          i32.load offset=16
          i32.const 1
          i32.add
          i32.store offset=16
          local.get $#local0
          local.get $#local0
          i32.load offset=8
          i32.const -1
          i32.add
          local.tee $#local2
          i32.store offset=8
          local.get $#local2
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          local.tee $#local0
          i32.const 0
          i32.store8 offset=12
          local.get $#local0
          i64.const 0
          i64.store
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055774
        i32.add
        i32.const 3
        i32.store8
      end
      local.get $#local1
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2rt19lang_start_internal17hd7e4828cbdcf6ef2E (;56;) (type $#type12) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (result i32)
      (local $#local5 i32) (local $#local6 i64) (local $#local7 i64) (local $#local8 i32) (local $#local9 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      block $#label0
        block $#label1
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          i64.load
          local.tee $#local6
          i64.const 0
          i64.ne
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055856
          i32.add
          i64.load
          local.set $#local7
          loop $#label2
            local.get $#local7
            i64.const -1
            i64.eq
            br_if $#label0
            global.get $GOT.data.internal.__memory_base
            i32.const 1055856
            i32.add
            local.tee $#local8
            local.get $#local7
            i64.const 1
            i64.add
            local.tee $#local6
            local.get $#local8
            i64.load
            local.tee $#local9
            local.get $#local9
            local.get $#local7
            i64.eq
            local.tee $#local8
            select
            i64.store
            local.get $#local9
            local.set $#local7
            local.get $#local8
            i32.eqz
            br_if $#label2
          end
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.get $#local6
          i64.store
        end
        global.get $GOT.data.internal.__memory_base
        local.tee $#local8
        i32.const 1055864
        i32.add
        local.get $#local6
        i64.store
        local.get $#local0
        local.get $#local1
        i32.load offset=20
        call_indirect (type $#type3)
        local.set $#local1
        block $#label1
          local.get $#local8
          i32.const 1055774
          i32.add
          i32.load8_u
          i32.const 3
          i32.eq
          br_if $#label1
          local.get $#local5
          i32.const 1
          i32.store8 offset=15
          local.get $#local5
          i32.const 15
          i32.add
          call $_ZN3std3sys4sync4once10no_threads4Once4call17h452a412a50c0c1e5E
        end
        local.get $#local5
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get $#local1
        return
      end
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E (;57;) (type $#type0)
      (local $#local0 i32) (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      local.get $#local0
      i32.const 0
      i32.store offset=24
      local.get $#local0
      i32.const 1
      i32.store offset=12
      local.get $#local0
      i64.const 4
      i64.store offset=16 align=4
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      local.tee $#local1
      i32.const 1054508
      i32.add
      i32.store offset=8
      local.get $#local0
      i32.const 8
      i32.add
      local.get $#local1
      i32.const 1054516
      i32.add
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E (;58;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      i32.const 4
      i32.store8 offset=8
      local.get $#local3
      local.get $#local1
      i32.store offset=16
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054404
            i32.add
            local.get $#local2
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if $#label2
            local.get $#local3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if $#label1
            local.get $#local3
            i32.const 0
            i32.store offset=40
            local.get $#local3
            i32.const 1
            i32.store offset=28
            local.get $#local3
            i64.const 4
            i64.store offset=32 align=4
            local.get $#local3
            global.get $GOT.data.internal.__memory_base
            local.tee $#local1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get $#local3
            i32.const 24
            i32.add
            local.get $#local1
            i32.const 1054704
            i32.add
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get $#local0
          i32.const 4
          i32.store8
          local.get $#local3
          i32.load offset=12
          local.set $#local2
          block $#label2
            local.get $#local3
            i32.load8_u offset=8
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label0
          end
          local.get $#local2
          i32.load
          local.set $#local0
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local2
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        i64.load offset=8
        i64.store align=4
      end
      local.get $#local3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE (;59;) (type $#type0)
      call $abort
      unreachable
    )
    (func $_ZN3std3env11current_dir17h890335e8528685e2E (;60;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      i32.const 512
      local.set $#local2
      block $#label0
        block $#label1
          i32.const 512
          i32.const 1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.tee $#local3
          i32.eqz
          br_if $#label1
          local.get $#local1
          local.get $#local3
          i32.store offset=8
          local.get $#local1
          i32.const 512
          i32.store offset=4
          block $#label2
            block $#label3
              local.get $#local3
              i32.const 512
              call $getcwd
              br_if $#label3
              i32.const 512
              local.set $#local2
              loop $#label4
                block $#label5
                  global.get $GOT.data.internal.errno
                  i32.load
                  local.tee $#local4
                  i32.const 68
                  i32.eq
                  br_if $#label5
                  local.get $#local0
                  local.get $#local4
                  i32.store offset=8
                  local.get $#local0
                  i64.const 2147483648
                  i64.store align=4
                  local.get $#local2
                  i32.eqz
                  br_if $#label2
                  local.get $#local3
                  local.get $#local2
                  i32.const 1
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  br $#label2
                end
                local.get $#local1
                local.get $#local2
                i32.store offset=12
                local.get $#local1
                i32.const 4
                i32.add
                local.get $#local2
                i32.const 1
                i32.const 1
                i32.const 1
                call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
                local.get $#local1
                i32.load offset=8
                local.tee $#local3
                local.get $#local1
                i32.load offset=4
                local.tee $#local2
                call $getcwd
                i32.eqz
                br_if $#label4
              end
            end
            local.get $#local1
            local.get $#local3
            call $strlen
            local.tee $#local4
            i32.store offset=12
            block $#label3
              local.get $#local2
              local.get $#local4
              i32.le_u
              br_if $#label3
              block $#label4
                block $#label5
                  local.get $#local4
                  br_if $#label5
                  i32.const 1
                  local.set $#local5
                  local.get $#local3
                  local.get $#local2
                  i32.const 1
                  call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
                  br $#label4
                end
                local.get $#local3
                local.get $#local2
                i32.const 1
                local.get $#local4
                call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
                local.tee $#local5
                i32.eqz
                br_if $#label0
              end
              local.get $#local1
              local.get $#local4
              i32.store offset=4
              local.get $#local1
              local.get $#local5
              i32.store offset=8
            end
            local.get $#local0
            local.get $#local1
            i64.load offset=4 align=4
            i64.store align=4
            local.get $#local0
            i32.const 8
            i32.add
            local.get $#local1
            i32.const 4
            i32.add
            i32.const 8
            i32.add
            i32.load
            i32.store
          end
          local.get $#local1
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
      local.get $#local4
      global.get $GOT.data.internal.__memory_base
      i32.const 1055316
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3env7_var_os17hecfa64e4c3898426E (;61;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 416
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.const 383
            i32.gt_u
            br_if $#label2
            block $#label3
              local.get $#local2
              i32.eqz
              br_if $#label3
              local.get $#local3
              i32.const 20
              i32.add
              local.get $#local1
              local.get $#local2
              memory.copy
            end
            local.get $#local3
            i32.const 20
            i32.add
            local.get $#local2
            i32.add
            i32.const 0
            i32.store8
            local.get $#local3
            i32.const 404
            i32.add
            local.get $#local3
            i32.const 20
            i32.add
            local.get $#local2
            i32.const 1
            i32.add
            call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h04ddcd8be7687b8aE
            block $#label3
              local.get $#local3
              i32.load offset=404
              i32.const 1
              i32.ne
              br_if $#label3
              local.get $#local3
              global.get $GOT.data.internal.__memory_base
              i32.const 1054888
              i32.add
              i64.load
              i64.store offset=12 align=4
              i32.const -2147483647
              local.set $#local2
              br $#label1
            end
            block $#label3
              local.get $#local3
              i32.load offset=408
              call $getenv
              local.tee $#local1
              br_if $#label3
              i32.const -2147483648
              local.set $#local2
              br $#label1
            end
            i32.const 0
            local.set $#local4
            local.get $#local1
            call $strlen
            local.tee $#local2
            i32.const 0
            i32.lt_s
            br_if $#label0
            block $#label3
              block $#label4
                local.get $#local2
                br_if $#label4
                i32.const 1
                local.set $#local5
                br $#label3
              end
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set $#local4
              local.get $#local2
              i32.const 1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee $#local5
              i32.eqz
              br_if $#label0
            end
            block $#label3
              local.get $#local2
              i32.eqz
              br_if $#label3
              local.get $#local5
              local.get $#local1
              local.get $#local2
              memory.copy
            end
            local.get $#local3
            local.get $#local2
            i32.store offset=16
            local.get $#local3
            local.get $#local5
            i32.store offset=12
            br $#label1
          end
          local.get $#local3
          i32.const 8
          i32.add
          local.get $#local1
          local.get $#local2
          call $_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE
          local.get $#local3
          i32.load offset=8
          local.set $#local2
        end
        block $#label1
          block $#label2
            local.get $#local2
            i32.const -2147483647
            i32.eq
            br_if $#label2
            local.get $#local0
            local.get $#local3
            i64.load offset=12 align=4
            i64.store offset=4 align=4
            local.get $#local0
            local.get $#local2
            i32.store
            br $#label1
          end
          block $#label2
            local.get $#local3
            i32.load8_u offset=12
            i32.const 3
            i32.ne
            br_if $#label2
            local.get $#local3
            i32.load offset=16
            local.tee $#local2
            i32.load
            local.set $#local5
            block $#label3
              local.get $#local2
              i32.const 4
              i32.add
              i32.load
              local.tee $#local1
              i32.load
              local.tee $#local4
              i32.eqz
              br_if $#label3
              local.get $#local5
              local.get $#local4
              call_indirect (type $#type1)
            end
            block $#label3
              local.get $#local1
              i32.load offset=4
              local.tee $#local4
              i32.eqz
              br_if $#label3
              local.get $#local5
              local.get $#local4
              local.get $#local1
              i32.load offset=8
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get $#local2
            i32.const 12
            i32.const 4
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local0
          i32.const -2147483648
          i32.store
        end
        local.get $#local3
        i32.const 416
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local4
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std3sys3pal6common14small_c_string24run_with_cstr_allocating17h829449bc9b745a5cE (;62;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      local.get $#local2
      call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.load
            local.tee $#local2
            i32.const -2147483648
            i32.ne
            br_if $#label2
            local.get $#local3
            i32.load offset=8
            local.set $#local1
            block $#label3
              block $#label4
                local.get $#local3
                i32.load offset=4
                local.tee $#local4
                call $getenv
                local.tee $#local5
                br_if $#label4
                local.get $#local0
                i32.const -2147483648
                i32.store
                br $#label3
              end
              i32.const 0
              local.set $#local6
              local.get $#local5
              call $strlen
              local.tee $#local2
              i32.const 0
              i32.lt_s
              br_if $#label0
              block $#label4
                block $#label5
                  local.get $#local2
                  br_if $#label5
                  i32.const 1
                  local.set $#local7
                  br $#label4
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                i32.const 1
                local.set $#local6
                local.get $#local2
                i32.const 1
                call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
                local.tee $#local7
                i32.eqz
                br_if $#label0
              end
              block $#label4
                local.get $#local2
                i32.eqz
                br_if $#label4
                local.get $#local7
                local.get $#local5
                local.get $#local2
                memory.copy
              end
              local.get $#local0
              local.get $#local2
              i32.store offset=8
              local.get $#local0
              local.get $#local7
              i32.store offset=4
              local.get $#local0
              local.get $#local2
              i32.store
            end
            local.get $#local4
            i32.const 0
            i32.store8
            local.get $#local1
            i32.eqz
            br_if $#label1
            local.get $#local4
            local.get $#local1
            i32.const 1
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            br $#label1
          end
          local.get $#local0
          i32.const -2147483647
          i32.store
          local.get $#local0
          global.get $GOT.data.internal.__memory_base
          i32.const 1054888
          i32.add
          i64.load
          i64.store offset=4 align=4
          local.get $#local2
          i32.eqz
          br_if $#label1
          local.get $#local3
          i32.load offset=4
          local.get $#local2
          i32.const 1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local6
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h9b758179f08a1825E (;63;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32) (local $#local12 i32) (local $#local13 i64) (local $#local14 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local3
              local.get $#local1
              i32.load
              local.tee $#local5
              local.get $#local1
              i32.load offset=8
              local.tee $#local6
              i32.sub
              i32.le_u
              br_if $#label3
              block $#label4
                local.get $#local6
                br_if $#label4
                i32.const 0
                local.set $#local6
                br $#label3
              end
              local.get $#local1
              i32.load offset=4
              local.set $#local7
              i32.const 0
              local.set $#local8
              loop $#label4
                local.get $#local4
                local.get $#local6
                local.get $#local8
                i32.sub
                local.tee $#local9
                i32.store offset=4
                local.get $#local4
                local.get $#local7
                local.get $#local8
                i32.add
                local.tee $#local10
                i32.store
                local.get $#local4
                i32.const 8
                i32.add
                i32.const 1
                local.get $#local4
                i32.const 1
                call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          local.get $#local4
                          i32.load16_u offset=8
                          i32.const 1
                          i32.ne
                          br_if $#label9
                          local.get $#local9
                          local.set $#local11
                          local.get $#local4
                          i32.load16_u offset=10
                          local.tee $#local12
                          i32.const 8
                          i32.eq
                          br_if $#label8
                          local.get $#local1
                          i32.const 0
                          i32.store8 offset=12
                          local.get $#local12
                          i32.const 27
                          i32.eq
                          br_if $#label5
                          local.get $#local12
                          i64.extend_i32_u
                          i64.const 32
                          i64.shl
                          local.set $#local13
                          br $#label7
                        end
                        local.get $#local4
                        i32.load offset=12
                        local.set $#local11
                      end
                      local.get $#local1
                      i32.const 0
                      i32.store8 offset=12
                      local.get $#local11
                      br_if $#label6
                      global.get $GOT.data.internal.__memory_base
                      i32.const 1054532
                      i32.add
                      i64.extend_i32_u
                      i64.const 32
                      i64.shl
                      i64.const 2
                      i64.or
                      local.set $#local13
                    end
                    block $#label7
                      local.get $#local8
                      i32.eqz
                      br_if $#label7
                      block $#label8
                        local.get $#local9
                        i32.eqz
                        br_if $#label8
                        local.get $#local7
                        local.get $#local10
                        local.get $#local9
                        memory.copy
                      end
                      local.get $#local1
                      local.get $#local9
                      i32.store offset=8
                      local.get $#local9
                      local.set $#local6
                    end
                    local.get $#local13
                    i64.const 255
                    i64.and
                    i64.const 4
                    i64.eq
                    br_if $#label3
                    local.get $#local0
                    local.get $#local13
                    i64.store align=4
                    br $#label2
                  end
                  local.get $#local11
                  local.get $#local8
                  i32.add
                  local.set $#local8
                end
                local.get $#local8
                local.get $#local6
                i32.lt_u
                br_if $#label4
              end
              local.get $#local8
              local.get $#local6
              i32.gt_u
              br_if $#label1
              i32.const 0
              local.set $#local6
              local.get $#local1
              i32.const 0
              i32.store offset=8
            end
            block $#label3
              local.get $#local3
              local.get $#local5
              i32.ge_u
              br_if $#label3
              block $#label4
                local.get $#local3
                i32.eqz
                br_if $#label4
                local.get $#local1
                i32.load offset=4
                local.get $#local6
                i32.add
                local.get $#local2
                local.get $#local3
                memory.copy
              end
              local.get $#local0
              i32.const 4
              i32.store8
              local.get $#local1
              local.get $#local6
              local.get $#local3
              i32.add
              i32.store offset=8
              br $#label2
            end
            i64.const 0
            local.set $#local14
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local3
                    i32.eqz
                    br_if $#label6
                    loop $#label7
                      local.get $#local4
                      local.get $#local3
                      i32.store offset=4
                      local.get $#local4
                      local.get $#local2
                      i32.store
                      local.get $#local4
                      i32.const 8
                      i32.add
                      i32.const 1
                      local.get $#local4
                      i32.const 1
                      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                      block $#label8
                        block $#label9
                          block $#label10
                            block $#label11
                              local.get $#local4
                              i32.load16_u offset=8
                              i32.const 1
                              i32.ne
                              br_if $#label11
                              local.get $#local4
                              i64.load16_u offset=10
                              local.tee $#local13
                              i64.const 27
                              i64.eq
                              br_if $#label8
                              local.get $#local13
                              i64.const 32
                              i64.shl
                              local.set $#local13
                              br $#label10
                            end
                            local.get $#local4
                            i32.load offset=12
                            local.tee $#local8
                            br_if $#label9
                            global.get $GOT.data.internal.__memory_base
                            i32.const 1054584
                            i32.add
                            i64.load
                            local.set $#local13
                          end
                          local.get $#local13
                          i64.const 32
                          i64.shr_u
                          local.set $#local14
                          local.get $#local13
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          local.tee $#local8
                          i32.const 4
                          i32.eq
                          br_if $#label5
                          local.get $#local8
                          br_if $#label4
                          local.get $#local14
                          i64.const 8
                          i64.ne
                          br_if $#label4
                          i64.const 4
                          local.set $#local14
                          i64.const 0
                          local.set $#local13
                          br $#label3
                        end
                        local.get $#local3
                        local.get $#local8
                        i32.lt_u
                        br_if $#label0
                        local.get $#local2
                        local.get $#local8
                        i32.add
                        local.set $#local2
                        local.get $#local3
                        local.get $#local8
                        i32.sub
                        local.set $#local3
                      end
                      local.get $#local3
                      br_if $#label7
                    end
                  end
                  i64.const 0
                  local.set $#local13
                end
                local.get $#local13
                i64.const 4294967040
                i64.and
                local.get $#local14
                i64.const 32
                i64.shl
                i64.or
                local.set $#local13
                i64.const 4
                local.set $#local14
                br $#label3
              end
              local.get $#local13
              i64.const 255
              i64.and
              local.set $#local14
              local.get $#local13
              i64.const -256
              i64.and
              local.set $#local13
            end
            local.get $#local1
            i32.const 0
            i32.store8 offset=12
            local.get $#local0
            local.get $#local14
            local.get $#local13
            i64.or
            i64.store align=4
          end
          local.get $#local4
          i32.const 16
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get $#local8
        local.get $#local6
        global.get $GOT.data.internal.__memory_base
        i32.const 1054252
        i32.add
        call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
        unreachable
      end
      local.get $#local8
      local.get $#local3
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E (;64;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 1056
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        i32.const 1024
        i32.eqz
        br_if $#label0
        local.get $#local2
        i32.const 0
        i32.const 1024
        memory.fill
      end
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            local.get $#local2
            i32.const 1024
            call $strerror_r
            i32.const 0
            i32.lt_s
            br_if $#label2
            local.get $#local2
            i32.const 1024
            i32.add
            local.get $#local2
            local.get $#local2
            call $strlen
            call $_ZN4core3str8converts9from_utf817hc11b0c33b11310b8E
            local.get $#local2
            i32.load offset=1024
            br_if $#label1
            i32.const 0
            local.set $#local3
            local.get $#local2
            i32.load offset=1032
            local.tee $#local1
            i32.const 0
            i32.lt_s
            br_if $#label0
            local.get $#local2
            i32.load offset=1028
            local.set $#local4
            block $#label3
              block $#label4
                local.get $#local1
                br_if $#label4
                i32.const 1
                local.set $#local5
                br $#label3
              end
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set $#local3
              local.get $#local1
              i32.const 1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee $#local5
              i32.eqz
              br_if $#label0
            end
            block $#label3
              local.get $#local1
              i32.eqz
              br_if $#label3
              local.get $#local5
              local.get $#local4
              local.get $#local1
              memory.copy
            end
            local.get $#local0
            local.get $#local1
            i32.store offset=8
            local.get $#local0
            local.get $#local5
            i32.store offset=4
            local.get $#local0
            local.get $#local1
            i32.store
            local.get $#local2
            i32.const 1056
            i32.add
            global.set $__stack_pointer
            return
          end
          local.get $#local2
          i32.const 0
          i32.store offset=1040
          local.get $#local2
          i32.const 1
          i32.store offset=1028
          local.get $#local2
          i64.const 4
          i64.store offset=1032 align=4
          local.get $#local2
          global.get $GOT.data.internal.__memory_base
          local.tee $#local1
          i32.const 1055276
          i32.add
          i32.store offset=1024
          local.get $#local2
          i32.const 1024
          i32.add
          local.get $#local1
          i32.const 1055284
          i32.add
          call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
          unreachable
        end
        local.get $#local2
        local.get $#local2
        i64.load offset=1028 align=4
        i64.store offset=1048
        global.get $GOT.data.internal.__memory_base
        local.tee $#local1
        i32.const 1049240
        i32.add
        i32.const 43
        local.get $#local2
        i32.const 1048
        i32.add
        local.get $#local1
        i32.const 1055244
        i32.add
        local.get $#local1
        i32.const 1055260
        i32.add
        call $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE
        unreachable
      end
      local.get $#local3
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1054476
      i32.add
      call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
      unreachable
    )
    (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E (;65;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local0
                i32.load8_u
                br_table $#label4 $#label3 $#label2 $#label1 $#label4
              end
              local.get $#local2
              local.get $#local0
              i32.load offset=4
              local.tee $#local0
              i32.store offset=4
              local.get $#local2
              i32.const 8
              i32.add
              local.get $#local0
              call $_ZN3std3sys3pal6wasip22os12error_string17h1c15a2cf4164b5e8E
              local.get $#local2
              i32.const 3
              i32.store offset=44
              local.get $#local2
              global.get $GOT.data.internal.__memory_base
              i32.const 1054592
              i32.add
              i32.store offset=40
              local.get $#local2
              i64.const 2
              i64.store offset=52 align=4
              local.get $#local2
              global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get $#local2
              i32.const 4
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=32
              local.get $#local2
              global.get $GOT.data.internal.__table_base
              i32.const 7
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get $#local2
              i32.const 8
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=24
              local.get $#local2
              local.get $#local2
              i32.const 24
              i32.add
              i32.store offset=48
              local.get $#local1
              i32.load
              local.get $#local1
              i32.load offset=4
              local.get $#local2
              i32.const 40
              i32.add
              call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
              local.set $#local0
              local.get $#local2
              i32.load offset=8
              local.tee $#local1
              i32.eqz
              br_if $#label0
              local.get $#local2
              i32.load offset=12
              local.get $#local1
              i32.const 1
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
              br $#label0
            end
            local.get $#local0
            i32.load8_u offset=1
            local.set $#local3
            local.get $#local2
            i32.const 1
            i32.store offset=44
            local.get $#local2
            global.get $GOT.data.internal.__memory_base
            local.tee $#local0
            i32.const 1049356
            i32.add
            i32.store offset=40
            local.get $#local2
            i64.const 1
            i64.store offset=52 align=4
            local.get $#local2
            local.get $#local0
            i32.const 1051408
            i32.add
            local.get $#local3
            i32.const 2
            i32.shl
            local.tee $#local3
            i32.add
            i32.load
            i32.store offset=28
            local.get $#local2
            local.get $#local0
            i32.const 1055356
            i32.add
            local.get $#local3
            i32.add
            i32.load
            i32.store offset=24
            local.get $#local2
            global.get $GOT.data.internal.__table_base
            i32.const 6
            i32.add
            i64.extend_i32_u
            i64.const 32
            i64.shl
            local.get $#local2
            i32.const 24
            i32.add
            i64.extend_i32_u
            i64.or
            i64.store offset=8
            local.get $#local2
            local.get $#local2
            i32.const 8
            i32.add
            i32.store offset=48
            local.get $#local1
            i32.load
            local.get $#local1
            i32.load offset=4
            local.get $#local2
            i32.const 40
            i32.add
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            local.set $#local0
            br $#label0
          end
          local.get $#local0
          i32.load offset=4
          local.tee $#local0
          i32.load
          local.get $#local0
          i32.load offset=4
          local.get $#local1
          call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE
          local.set $#local0
          br $#label0
        end
        local.get $#local0
        i32.load offset=4
        local.tee $#local0
        i32.load
        local.get $#local1
        local.get $#local0
        i32.load offset=4
        i32.load offset=16
        call_indirect (type $#type2)
        local.set $#local0
      end
      local.get $#local2
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h9a03eaad6ba158f3E (;66;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      block $#label0
        local.get $#local3
        local.get $#local1
        i32.load
        local.get $#local1
        i32.load offset=8
        local.tee $#local4
        i32.sub
        i32.le_u
        br_if $#label0
        local.get $#local1
        local.get $#local4
        local.get $#local3
        i32.const 1
        i32.const 1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get $#local1
        i32.load offset=8
        local.set $#local4
      end
      block $#label0
        local.get $#local3
        i32.eqz
        br_if $#label0
        local.get $#local1
        i32.load offset=4
        local.get $#local4
        i32.add
        local.get $#local2
        local.get $#local3
        memory.copy
      end
      local.get $#local0
      local.get $#local3
      i32.store offset=4
      local.get $#local1
      local.get $#local4
      local.get $#local3
      i32.add
      i32.store offset=8
      local.get $#local0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h110872ad791321b0E (;67;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32)
      block $#label0
        block $#label1
          local.get $#local3
          br_if $#label1
          i32.const 0
          local.set $#local4
          br $#label0
        end
        local.get $#local3
        i32.const 3
        i32.and
        local.set $#local5
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 4
            i32.ge_u
            br_if $#label2
            i32.const 0
            local.set $#local4
            i32.const 0
            local.set $#local6
            br $#label1
          end
          local.get $#local2
          i32.const 28
          i32.add
          local.set $#local7
          local.get $#local3
          i32.const -4
          i32.and
          local.set $#local8
          i32.const 0
          local.set $#local4
          i32.const 0
          local.set $#local6
          loop $#label2
            local.get $#local7
            i32.load
            local.get $#local7
            i32.const -8
            i32.add
            i32.load
            local.get $#local7
            i32.const -16
            i32.add
            i32.load
            local.get $#local7
            i32.const -24
            i32.add
            i32.load
            local.get $#local4
            i32.add
            i32.add
            i32.add
            i32.add
            local.set $#local4
            local.get $#local7
            i32.const 32
            i32.add
            local.set $#local7
            local.get $#local8
            local.get $#local6
            i32.const 4
            i32.add
            local.tee $#local6
            i32.ne
            br_if $#label2
          end
        end
        block $#label1
          local.get $#local5
          i32.eqz
          br_if $#label1
          local.get $#local6
          i32.const 3
          i32.shl
          local.get $#local2
          i32.add
          i32.const 4
          i32.add
          local.set $#local7
          loop $#label2
            local.get $#local7
            i32.load
            local.get $#local4
            i32.add
            local.set $#local4
            local.get $#local7
            i32.const 8
            i32.add
            local.set $#local7
            local.get $#local5
            i32.const -1
            i32.add
            local.tee $#local5
            br_if $#label2
          end
        end
        local.get $#local3
        i32.const 3
        i32.shl
        local.set $#local7
        block $#label1
          local.get $#local4
          local.get $#local1
          i32.load
          local.get $#local1
          i32.load offset=8
          local.tee $#local5
          i32.sub
          i32.le_u
          br_if $#label1
          local.get $#local1
          local.get $#local5
          local.get $#local4
          i32.const 1
          i32.const 1
          call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        end
        local.get $#local2
        local.get $#local7
        i32.add
        local.set $#local8
        local.get $#local1
        i32.load offset=8
        local.set $#local7
        loop $#label1
          local.get $#local2
          i32.load
          local.set $#local6
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local5
            local.get $#local1
            i32.load
            local.get $#local7
            i32.sub
            i32.le_u
            br_if $#label2
            local.get $#local1
            local.get $#local7
            local.get $#local5
            i32.const 1
            i32.const 1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get $#local1
            i32.load offset=8
            local.set $#local7
          end
          block $#label2
            local.get $#local5
            i32.eqz
            br_if $#label2
            local.get $#local1
            i32.load offset=4
            local.get $#local7
            i32.add
            local.get $#local6
            local.get $#local5
            memory.copy
          end
          local.get $#local1
          local.get $#local7
          local.get $#local5
          i32.add
          local.tee $#local7
          i32.store offset=8
          local.get $#local2
          i32.const 8
          i32.add
          local.tee $#local2
          local.get $#local8
          i32.ne
          br_if $#label1
        end
      end
      local.get $#local0
      i32.const 4
      i32.store8
      local.get $#local0
      local.get $#local4
      i32.store offset=4
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h6111fbf394a5d3beE (;68;) (type $#type3) (param $#local0 i32) (result i32)
      i32.const 1
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17ha32072b6ce4d89d9E (;69;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      block $#label0
        local.get $#local3
        local.get $#local1
        i32.load
        local.get $#local1
        i32.load offset=8
        local.tee $#local4
        i32.sub
        i32.le_u
        br_if $#label0
        local.get $#local1
        local.get $#local4
        local.get $#local3
        i32.const 1
        i32.const 1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get $#local1
        i32.load offset=8
        local.set $#local4
      end
      block $#label0
        local.get $#local3
        i32.eqz
        br_if $#label0
        local.get $#local1
        i32.load offset=4
        local.get $#local4
        i32.add
        local.get $#local2
        local.get $#local3
        memory.copy
      end
      local.get $#local0
      i32.const 4
      i32.store8
      local.get $#local1
      local.get $#local4
      local.get $#local3
      i32.add
      i32.store offset=8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$18write_all_vectored17ha176c49c553bc085E (;70;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32)
      block $#label0
        local.get $#local3
        i32.eqz
        br_if $#label0
        local.get $#local3
        i32.const 3
        i32.and
        local.set $#local4
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 4
            i32.ge_u
            br_if $#label2
            i32.const 0
            local.set $#local5
            i32.const 0
            local.set $#local6
            br $#label1
          end
          local.get $#local2
          i32.const 28
          i32.add
          local.set $#local7
          local.get $#local3
          i32.const -4
          i32.and
          local.set $#local8
          i32.const 0
          local.set $#local5
          i32.const 0
          local.set $#local6
          loop $#label2
            local.get $#local7
            i32.load
            local.get $#local7
            i32.const -8
            i32.add
            i32.load
            local.get $#local7
            i32.const -16
            i32.add
            i32.load
            local.get $#local7
            i32.const -24
            i32.add
            i32.load
            local.get $#local5
            i32.add
            i32.add
            i32.add
            i32.add
            local.set $#local5
            local.get $#local7
            i32.const 32
            i32.add
            local.set $#local7
            local.get $#local8
            local.get $#local6
            i32.const 4
            i32.add
            local.tee $#local6
            i32.ne
            br_if $#label2
          end
        end
        block $#label1
          local.get $#local4
          i32.eqz
          br_if $#label1
          local.get $#local6
          i32.const 3
          i32.shl
          local.get $#local2
          i32.add
          i32.const 4
          i32.add
          local.set $#local7
          loop $#label2
            local.get $#local7
            i32.load
            local.get $#local5
            i32.add
            local.set $#local5
            local.get $#local7
            i32.const 8
            i32.add
            local.set $#local7
            local.get $#local4
            i32.const -1
            i32.add
            local.tee $#local4
            br_if $#label2
          end
        end
        local.get $#local3
        i32.const 3
        i32.shl
        local.set $#local4
        block $#label1
          local.get $#local5
          local.get $#local1
          i32.load
          local.get $#local1
          i32.load offset=8
          local.tee $#local7
          i32.sub
          i32.le_u
          br_if $#label1
          local.get $#local1
          local.get $#local7
          local.get $#local5
          i32.const 1
          i32.const 1
          call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
          local.get $#local1
          i32.load offset=8
          local.set $#local7
        end
        local.get $#local2
        local.get $#local4
        i32.add
        local.set $#local6
        loop $#label1
          local.get $#local2
          i32.load
          local.set $#local4
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local5
            local.get $#local1
            i32.load
            local.get $#local7
            i32.sub
            i32.le_u
            br_if $#label2
            local.get $#local1
            local.get $#local7
            local.get $#local5
            i32.const 1
            i32.const 1
            call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
            local.get $#local1
            i32.load offset=8
            local.set $#local7
          end
          block $#label2
            local.get $#local5
            i32.eqz
            br_if $#label2
            local.get $#local1
            i32.load offset=4
            local.get $#local7
            i32.add
            local.get $#local4
            local.get $#local5
            memory.copy
          end
          local.get $#local1
          local.get $#local7
          local.get $#local5
          i32.add
          local.tee $#local7
          i32.store offset=8
          local.get $#local2
          i32.const 8
          i32.add
          local.tee $#local2
          local.get $#local6
          i32.ne
          br_if $#label1
        end
      end
      local.get $#local0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hd00fe73690d72cc0E (;71;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std2io5Write18write_all_vectored17h084a6fa04d8a14dfE (;72;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i64) (local $#local11 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local3
          i32.eqz
          br_if $#label1
          local.get $#local2
          i32.const 4
          i32.add
          local.set $#local5
          local.get $#local3
          i32.const 3
          i32.shl
          local.set $#local6
          local.get $#local3
          i32.const -1
          i32.add
          i32.const 536870911
          i32.and
          i32.const 1
          i32.add
          local.set $#local7
          i32.const 0
          local.set $#local8
          block $#label2
            loop $#label3
              local.get $#local5
              i32.load
              br_if $#label2
              local.get $#local5
              i32.const 8
              i32.add
              local.set $#local5
              local.get $#local8
              i32.const 1
              i32.add
              local.set $#local8
              local.get $#local6
              i32.const -8
              i32.add
              local.tee $#local6
              br_if $#label3
            end
            local.get $#local7
            local.set $#local8
          end
          block $#label2
            local.get $#local3
            local.get $#local8
            i32.lt_u
            br_if $#label2
            local.get $#local3
            local.get $#local8
            i32.eq
            br_if $#label1
            local.get $#local3
            local.get $#local8
            i32.sub
            local.set $#local7
            local.get $#local2
            local.get $#local8
            i32.const 3
            i32.shl
            i32.add
            local.set $#local9
            block $#label3
              loop $#label4
                local.get $#local4
                i32.const 8
                i32.add
                i32.const 2
                local.get $#local9
                local.get $#local7
                call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
                block $#label5
                  local.get $#local4
                  i32.load16_u offset=8
                  i32.eqz
                  br_if $#label5
                  local.get $#local4
                  i64.load16_u offset=10
                  local.tee $#local10
                  i64.const 27
                  i64.eq
                  br_if $#label4
                  local.get $#local0
                  local.get $#local10
                  i64.const 32
                  i64.shl
                  i64.store align=4
                  br $#label0
                end
                block $#label5
                  local.get $#local4
                  i32.load offset=12
                  local.tee $#local5
                  br_if $#label5
                  local.get $#local0
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  i64.store align=4
                  br $#label0
                end
                local.get $#local9
                i32.const 4
                i32.add
                local.set $#local8
                local.get $#local7
                i32.const 3
                i32.shl
                local.set $#local3
                local.get $#local7
                i32.const -1
                i32.add
                i32.const 536870911
                i32.and
                i32.const 1
                i32.add
                local.set $#local11
                i32.const 0
                local.set $#local6
                block $#label5
                  loop $#label6
                    local.get $#local5
                    local.get $#local8
                    i32.load
                    local.tee $#local2
                    i32.lt_u
                    br_if $#label5
                    local.get $#local8
                    i32.const 8
                    i32.add
                    local.set $#local8
                    local.get $#local6
                    i32.const 1
                    i32.add
                    local.set $#local6
                    local.get $#local5
                    local.get $#local2
                    i32.sub
                    local.set $#local5
                    local.get $#local3
                    i32.const -8
                    i32.add
                    local.tee $#local3
                    br_if $#label6
                  end
                  local.get $#local11
                  local.set $#local6
                end
                block $#label5
                  local.get $#local7
                  local.get $#local6
                  i32.lt_u
                  br_if $#label5
                  block $#label6
                    local.get $#local7
                    local.get $#local6
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.eqz
                    br_if $#label1
                    local.get $#local4
                    i32.const 0
                    i32.store offset=24
                    local.get $#local4
                    i32.const 1
                    i32.store offset=12
                    local.get $#local4
                    i64.const 4
                    i64.store offset=16 align=4
                    local.get $#local4
                    global.get $GOT.data.internal.__memory_base
                    local.tee $#local5
                    i32.const 1054736
                    i32.add
                    i32.store offset=8
                    local.get $#local4
                    i32.const 8
                    i32.add
                    local.get $#local5
                    i32.const 1054744
                    i32.add
                    call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
                    unreachable
                  end
                  local.get $#local9
                  local.get $#local6
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee $#local9
                  i32.load offset=4
                  local.tee $#local8
                  local.get $#local5
                  i32.lt_u
                  br_if $#label3
                  local.get $#local7
                  local.get $#local6
                  i32.sub
                  local.set $#local7
                  local.get $#local9
                  local.get $#local8
                  local.get $#local5
                  i32.sub
                  i32.store offset=4
                  local.get $#local9
                  local.get $#local9
                  i32.load
                  local.get $#local5
                  i32.add
                  i32.store
                  br $#label4
                end
              end
              local.get $#local6
              local.get $#local7
              global.get $GOT.data.internal.__memory_base
              i32.const 1054720
              i32.add
              call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
              unreachable
            end
            local.get $#local4
            i32.const 0
            i32.store offset=24
            local.get $#local4
            i32.const 1
            i32.store offset=12
            local.get $#local4
            i64.const 4
            i64.store offset=16 align=4
            local.get $#local4
            global.get $GOT.data.internal.__memory_base
            local.tee $#local5
            i32.const 1054760
            i32.add
            i32.store offset=8
            local.get $#local4
            i32.const 8
            i32.add
            local.get $#local5
            i32.const 1054768
            i32.add
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get $#local8
          local.get $#local3
          global.get $GOT.data.internal.__memory_base
          i32.const 1054720
          i32.add
          call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
          unreachable
        end
        local.get $#local0
        i32.const 4
        i32.store8
      end
      local.get $#local4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE (;73;) (type $#type0)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              global.get $GOT.data.internal.__memory_base
              i32.const 1055776
              i32.add
              i32.load8_u offset=40
              br_table $#label3 $#label3 $#label0 $#label2 $#label3
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
            local.tee $#local1
            i32.eqz
            br_if $#label1
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            local.tee $#local2
            i32.const 3
            i32.store8 offset=40
            local.get $#local2
            i32.const 0
            i32.store8 offset=32
            local.get $#local2
            i32.const 0
            i32.store offset=28
            local.get $#local2
            local.get $#local1
            i32.store offset=24
            local.get $#local2
            i64.const 4398046511104
            i64.store offset=16
            local.get $#local2
            i32.const 0
            i32.store8 offset=12
            local.get $#local2
            i32.const 0
            i32.store offset=8
            local.get $#local2
            i64.const 0
            i64.store
          end
          local.get $#local0
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
      local.get $#local0
      i32.const 0
      i32.store offset=24
      local.get $#local0
      i32.const 1
      i32.store offset=12
      local.get $#local0
      i64.const 4
      i64.store offset=16 align=4
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      local.tee $#local2
      i32.const 1055340
      i32.add
      i32.store offset=8
      local.get $#local0
      i32.const 8
      i32.add
      local.get $#local2
      i32.const 1054840
      i32.add
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E (;74;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i64) (local $#local5 i64) (local $#local6 i64)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
      local.set $#local2
      local.get $#local0
      i32.load
      local.set $#local3
      block $#label0
        block $#label1
          local.get $#local2
          i64.load
          local.tee $#local4
          i64.const 0
          i64.ne
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055856
          i32.add
          i64.load
          local.set $#local5
          loop $#label2
            local.get $#local5
            i64.const -1
            i64.eq
            br_if $#label0
            global.get $GOT.data.internal.__memory_base
            i32.const 1055856
            i32.add
            local.tee $#local0
            local.get $#local5
            i64.const 1
            i64.add
            local.tee $#local4
            local.get $#local0
            i64.load
            local.tee $#local6
            local.get $#local6
            local.get $#local5
            i64.eq
            local.tee $#local0
            select
            i64.store
            local.get $#local6
            local.set $#local5
            local.get $#local0
            i32.eqz
            br_if $#label2
          end
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.get $#local4
          i64.store
        end
        block $#label1
          block $#label2
            block $#label3
              local.get $#local4
              local.get $#local3
              i64.load
              i64.eq
              br_if $#label3
              local.get $#local3
              i32.load8_u offset=12
              local.set $#local0
              local.get $#local3
              i32.const 1
              i32.store8 offset=12
              local.get $#local1
              local.get $#local0
              i32.store8 offset=7
              local.get $#local0
              i32.eqz
              br_if $#label2
              local.get $#local1
              i64.const 0
              i64.store offset=20 align=4
              local.get $#local1
              i64.const 17179869185
              i64.store offset=12 align=4
              local.get $#local1
              global.get $GOT.data.internal.__memory_base
              local.tee $#local0
              i32.const 1054800
              i32.add
              i32.store offset=8
              i32.const 0
              local.get $#local1
              i32.const 7
              i32.add
              global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
              local.get $#local1
              i32.const 8
              i32.add
              local.get $#local0
              i32.const 1054808
              i32.add
              call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
              unreachable
            end
            block $#label3
              local.get $#local3
              i32.load offset=8
              local.tee $#local0
              i32.const -1
              i32.eq
              br_if $#label3
              local.get $#local3
              local.get $#local0
              i32.const 1
              i32.add
              i32.store offset=8
              br $#label1
            end
            global.get $GOT.data.internal.__memory_base
            local.tee $#local0
            i32.const 1050608
            i32.add
            i32.const 38
            local.get $#local0
            i32.const 1054856
            i32.add
            call $_ZN4core6option13expect_failed17he15179d1cacc214eE
            unreachable
          end
          local.get $#local3
          i32.const 1
          i32.store offset=8
          local.get $#local3
          local.get $#local4
          i64.store
        end
        local.get $#local1
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get $#local3
        return
      end
      call $_ZN3std6thread8ThreadId3new9exhausted17h9a714e6a52b10ff2E
      unreachable
    )
    (func $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E (;75;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      i32.load
      call $_ZN3std2io5stdio6Stderr4lock17h3698e32654bacba9E
      i32.store offset=4
      local.get $#local3
      i32.const 4
      i32.store8 offset=8
      global.get $GOT.data.internal.__memory_base
      local.set $#local1
      local.get $#local3
      local.get $#local3
      i32.const 4
      i32.add
      i32.store offset=16
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 8
            i32.add
            local.get $#local1
            i32.const 1054452
            i32.add
            local.get $#local2
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if $#label2
            local.get $#local3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if $#label1
            local.get $#local3
            i32.const 0
            i32.store offset=40
            local.get $#local3
            i32.const 1
            i32.store offset=28
            local.get $#local3
            i64.const 4
            i64.store offset=32 align=4
            local.get $#local3
            global.get $GOT.data.internal.__memory_base
            local.tee $#local1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get $#local3
            i32.const 24
            i32.add
            local.get $#local1
            i32.const 1054704
            i32.add
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get $#local0
          i32.const 4
          i32.store8
          local.get $#local3
          i32.load offset=12
          local.set $#local2
          block $#label2
            local.get $#local3
            i32.load8_u offset=8
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label0
          end
          local.get $#local2
          i32.load
          local.set $#local0
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local2
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        i64.load offset=8
        i64.store align=4
      end
      local.get $#local3
      i32.load offset=4
      local.tee $#local1
      local.get $#local1
      i32.load offset=8
      i32.const -1
      i32.add
      local.tee $#local2
      i32.store offset=8
      block $#label0
        local.get $#local2
        br_if $#label0
        local.get $#local1
        i32.const 0
        i32.store8 offset=12
        local.get $#local1
        i64.const 0
        i64.store
      end
      local.get $#local3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE (;76;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            global.get $GOT.data.internal.__memory_base
            i32.const 1055775
            i32.add
            i32.load8_u
            br_if $#label2
            i32.const 0
            local.set $#local2
            br $#label1
          end
          global.get $GOT.data.internal.__memory_base
          i32.const 1055880
          i32.add
          local.tee $#local3
          i32.load
          local.set $#local4
          i32.const 0
          local.set $#local2
          local.get $#local3
          i32.const 0
          i32.store
          local.get $#local4
          i32.eqz
          br_if $#label1
          local.get $#local4
          i32.load8_u offset=8
          local.set $#local2
          local.get $#local4
          i32.const 1
          i32.store8 offset=8
          local.get $#local1
          local.get $#local2
          i32.store8 offset=7
          local.get $#local2
          i32.const 1
          i32.eq
          br_if $#label0
          local.get $#local1
          i32.const 8
          i32.add
          local.get $#local4
          i32.const 12
          i32.add
          local.get $#local0
          call $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E
          local.get $#local1
          i32.load offset=12
          local.set $#local0
          block $#label2
            block $#label3
              local.get $#local1
              i32.load8_u offset=8
              local.tee $#local2
              i32.const 4
              i32.gt_u
              br_if $#label3
              local.get $#local2
              i32.const 3
              i32.ne
              br_if $#label2
            end
            local.get $#local0
            i32.load
            local.set $#local3
            block $#label3
              local.get $#local0
              i32.const 4
              i32.add
              i32.load
              local.tee $#local2
              i32.load
              local.tee $#local5
              i32.eqz
              br_if $#label3
              local.get $#local3
              local.get $#local5
              call_indirect (type $#type1)
            end
            block $#label3
              local.get $#local2
              i32.load offset=4
              local.tee $#local5
              i32.eqz
              br_if $#label3
              local.get $#local3
              local.get $#local5
              local.get $#local2
              i32.load offset=8
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get $#local0
            i32.const 12
            i32.const 4
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local4
          i32.const 0
          i32.store8 offset=8
          global.get $GOT.data.internal.__memory_base
          i32.const 1055880
          i32.add
          local.tee $#local0
          i32.load
          local.set $#local2
          local.get $#local0
          local.get $#local4
          i32.store
          local.get $#local1
          local.get $#local2
          i32.store offset=8
          block $#label2
            local.get $#local2
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local2
            i32.load
            local.tee $#local4
            i32.const -1
            i32.add
            i32.store
            local.get $#local4
            i32.const 1
            i32.ne
            br_if $#label2
            local.get $#local1
            i32.const 8
            i32.add
            call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha457a181b33cef13E
          end
          i32.const 1
          local.set $#local2
        end
        local.get $#local1
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get $#local2
        return
      end
      local.get $#local1
      i64.const 0
      i64.store offset=20 align=4
      local.get $#local1
      i64.const 17179869185
      i64.store offset=12 align=4
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      local.tee $#local4
      i32.const 1054800
      i32.add
      i32.store offset=8
      i32.const 0
      local.get $#local1
      i32.const 7
      i32.add
      global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
      local.get $#local1
      i32.const 8
      i32.add
      local.get $#local4
      i32.const 1054808
      i32.add
      call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17hbdc31f8d31db6f33E (;77;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      i32.const 4
      i32.store8 offset=8
      local.get $#local3
      local.get $#local1
      i32.store offset=16
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054380
            i32.add
            local.get $#local2
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if $#label2
            local.get $#local3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if $#label1
            local.get $#local3
            i32.const 0
            i32.store offset=40
            local.get $#local3
            i32.const 1
            i32.store offset=28
            local.get $#local3
            i64.const 4
            i64.store offset=32 align=4
            local.get $#local3
            global.get $GOT.data.internal.__memory_base
            local.tee $#local1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get $#local3
            i32.const 24
            i32.add
            local.get $#local1
            i32.const 1054704
            i32.add
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get $#local0
          i32.const 4
          i32.store8
          local.get $#local3
          i32.load offset=12
          local.set $#local2
          block $#label2
            local.get $#local3
            i32.load8_u offset=8
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label0
          end
          local.get $#local2
          i32.load
          local.set $#local0
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local2
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        i64.load offset=8
        i64.store align=4
      end
      local.get $#local3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std2io5stdio6_print17h68847dc224af8aecE (;78;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local1
      i32.const 6
      i32.store offset=12
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1050253
      i32.add
      i32.store offset=8
      block $#label0
        block $#label1
          local.get $#local0
          call $_ZN3std2io5stdio31print_to_buffer_if_capture_used17hd9f5137dfd1b3bfaE
          br_if $#label1
          block $#label2
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load8_u offset=40
            i32.const 3
            i32.eq
            br_if $#label2
            call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h8bc05d055e169caeE
          end
          local.get $#local1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055776
          i32.add
          i32.store offset=28
          local.get $#local1
          local.get $#local1
          i32.const 28
          i32.add
          i32.store offset=40
          local.get $#local1
          i32.const 16
          i32.add
          local.get $#local1
          i32.const 40
          i32.add
          local.get $#local0
          call $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h664d27df34e63de5E
          local.get $#local1
          i32.load8_u offset=16
          i32.const 4
          i32.ne
          br_if $#label0
        end
        local.get $#local1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local1
      local.get $#local1
      i64.load offset=16
      i64.store offset=32
      local.get $#local1
      i32.const 2
      i32.store offset=44
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      local.tee $#local0
      i32.const 1054664
      i32.add
      i32.store offset=40
      local.get $#local1
      i64.const 2
      i64.store offset=52 align=4
      local.get $#local1
      global.get $GOT.func.internal._ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17hfdcafeb05abc56c5E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local1
      i32.const 32
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=72
      local.get $#local1
      global.get $GOT.data.internal.__table_base
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local1
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=64
      local.get $#local1
      local.get $#local1
      i32.const 64
      i32.add
      i32.store offset=48
      local.get $#local1
      i32.const 40
      i32.add
      local.get $#local0
      i32.const 1054680
      i32.add
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3496f4efac5e3bcaE (;79;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i64) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      i32.const 8
      i32.add
      local.get $#local0
      i32.load offset=8
      local.get $#local1
      local.get $#local2
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h76fe596eb9db9d56E
      block $#label0
        local.get $#local3
        i32.load8_u offset=8
        local.tee $#local2
        i32.const 4
        i32.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.set $#local4
        local.get $#local3
        i64.load offset=8
        local.set $#local5
        block $#label1
          block $#label2
            local.get $#local0
            i32.load8_u
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local4
          i32.load
          local.set $#local6
          block $#label2
            local.get $#local4
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local6
            local.get $#local7
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local6
            local.get $#local7
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local4
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local0
        local.get $#local5
        i64.store align=4
      end
      local.get $#local3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local2
      i32.const 4
      i32.ne
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h68cfbbfee7a49e96E (;80;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i64) (local $#local7 i32) (local $#local8 i32) (local $#local9 i64)
      i32.const 0
      local.set $#local3
      block $#label0
        i32.const 0
        local.get $#local0
        i32.load offset=8
        local.tee $#local4
        i32.load offset=4
        local.tee $#local5
        local.get $#local4
        i64.load offset=8
        local.tee $#local6
        i64.const 4294967295
        local.get $#local6
        i64.const 4294967295
        i64.lt_u
        select
        i32.wrap_i64
        i32.sub
        local.tee $#local7
        local.get $#local7
        local.get $#local5
        i32.gt_u
        select
        local.tee $#local7
        local.get $#local2
        local.get $#local7
        local.get $#local2
        i32.lt_u
        select
        local.tee $#local8
        i32.eqz
        br_if $#label0
        local.get $#local4
        i32.load
        local.get $#local6
        local.get $#local5
        i64.extend_i32_u
        local.tee $#local9
        local.get $#local6
        local.get $#local9
        i64.lt_u
        select
        i32.wrap_i64
        i32.add
        local.get $#local1
        local.get $#local8
        memory.copy
      end
      local.get $#local4
      local.get $#local6
      local.get $#local8
      i64.extend_i32_u
      i64.add
      i64.store offset=8
      block $#label0
        local.get $#local7
        local.get $#local2
        i32.ge_u
        br_if $#label0
        global.get $GOT.data.internal.__memory_base
        i32.const 1054584
        i32.add
        i64.load
        local.tee $#local6
        i64.const 255
        i64.and
        i64.const 4
        i64.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.set $#local4
        block $#label1
          block $#label2
            local.get $#local0
            i32.load8_u
            local.tee $#local2
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local2
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local4
          i32.load
          local.set $#local7
          block $#label2
            local.get $#local4
            i32.const 4
            i32.add
            i32.load
            local.tee $#local2
            i32.load
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local7
            local.get $#local5
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local2
            i32.load offset=4
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local7
            local.get $#local5
            local.get $#local2
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local4
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local0
        local.get $#local6
        i64.store align=4
        i32.const 1
        local.set $#local3
      end
      local.get $#local3
    )
    (func $_ZN81_$LT$std..io..default_write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hd47061e07350c8a7E (;81;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32)
      block $#label0
        local.get $#local2
        local.get $#local0
        i32.load offset=8
        local.tee $#local0
        i32.load
        local.get $#local0
        i32.load offset=8
        local.tee $#local3
        i32.sub
        i32.le_u
        br_if $#label0
        local.get $#local0
        local.get $#local3
        local.get $#local2
        i32.const 1
        i32.const 1
        call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h4d5dd5efb0ae2592E
        local.get $#local0
        i32.load offset=8
        local.set $#local3
      end
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.load offset=4
        local.get $#local3
        i32.add
        local.get $#local1
        local.get $#local2
        memory.copy
      end
      local.get $#local0
      local.get $#local3
      local.get $#local2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $_ZN3std2io5Write9write_all17h25f1158500ddcd1aE (;82;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i64) (local $#local6 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.eqz
            br_if $#label2
            loop $#label3
              local.get $#local4
              local.get $#local3
              i32.store offset=4
              local.get $#local4
              local.get $#local2
              i32.store
              local.get $#local4
              i32.const 8
              i32.add
              i32.const 2
              local.get $#local4
              i32.const 1
              call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
              block $#label4
                block $#label5
                  local.get $#local4
                  i32.load16_u offset=8
                  i32.eqz
                  br_if $#label5
                  local.get $#local4
                  i64.load16_u offset=10
                  local.tee $#local5
                  i64.const 27
                  i64.eq
                  br_if $#label4
                  local.get $#local0
                  local.get $#local5
                  i64.const 32
                  i64.shl
                  i64.store align=4
                  br $#label1
                end
                block $#label5
                  local.get $#local4
                  i32.load offset=12
                  local.tee $#local6
                  br_if $#label5
                  local.get $#local0
                  global.get $GOT.data.internal.__memory_base
                  i32.const 1054584
                  i32.add
                  i64.load
                  i64.store align=4
                  br $#label1
                end
                local.get $#local3
                local.get $#local6
                i32.lt_u
                br_if $#label0
                local.get $#local2
                local.get $#local6
                i32.add
                local.set $#local2
                local.get $#local3
                local.get $#local6
                i32.sub
                local.set $#local3
              end
              local.get $#local3
              br_if $#label3
            end
          end
          local.get $#local0
          i32.const 4
          i32.store8
        end
        local.get $#local4
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local6
      local.get $#local3
      global.get $GOT.data.internal.__memory_base
      i32.const 1054784
      i32.add
      call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
      unreachable
    )
    (func $_ZN3std2io5Write9write_fmt17h1459a55272857e0fE (;83;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      i32.const 4
      i32.store8 offset=8
      local.get $#local3
      local.get $#local1
      i32.store offset=16
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 8
            i32.add
            global.get $GOT.data.internal.__memory_base
            i32.const 1054428
            i32.add
            local.get $#local2
            call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
            i32.eqz
            br_if $#label2
            local.get $#local3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if $#label1
            local.get $#local3
            i32.const 0
            i32.store offset=40
            local.get $#local3
            i32.const 1
            i32.store offset=28
            local.get $#local3
            i64.const 4
            i64.store offset=32 align=4
            local.get $#local3
            global.get $GOT.data.internal.__memory_base
            local.tee $#local1
            i32.const 1054696
            i32.add
            i32.store offset=24
            local.get $#local3
            i32.const 24
            i32.add
            local.get $#local1
            i32.const 1054704
            i32.add
            call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
            unreachable
          end
          local.get $#local0
          i32.const 4
          i32.store8
          local.get $#local3
          i32.load offset=12
          local.set $#local2
          block $#label2
            local.get $#local3
            i32.load8_u offset=8
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label0
          end
          local.get $#local2
          i32.load
          local.set $#local0
          block $#label2
            local.get $#local2
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local2
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        i64.load offset=8
        i64.store align=4
      end
      local.get $#local3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std5panic19get_backtrace_style17h54380a7febe96116E (;84;) (type $#type10) (result i32)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      i32.const 3
      local.set $#local1
      block $#label0
        global.get $GOT.data.internal.__memory_base
        i32.const 1055824
        i32.add
        i32.load8_u
        i32.const -1
        i32.add
        local.tee $#local2
        i32.const 255
        i32.and
        i32.const 3
        i32.lt_u
        br_if $#label0
        local.get $#local0
        i32.const 4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1049226
        i32.add
        i32.const 14
        call $_ZN3std3env7_var_os17hecfa64e4c3898426E
        i32.const 2
        local.set $#local2
        block $#label1
          local.get $#local0
          i32.load offset=4
          local.tee $#local3
          i32.const -2147483648
          i32.eq
          br_if $#label1
          local.get $#local0
          i32.load offset=8
          local.set $#local4
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local0
                    i32.load offset=12
                    i32.const -1
                    i32.add
                    br_table $#label5 $#label4 $#label4 $#label6 $#label4
                  end
                  local.get $#local4
                  i32.load align=1
                  i32.const 1819047270
                  i32.ne
                  br_if $#label4
                  i32.const 1
                  local.set $#local2
                  i32.const 2
                  local.set $#local1
                  local.get $#local3
                  br_if $#label2
                  br $#label1
                end
                local.get $#local4
                i32.load8_u
                i32.const 48
                i32.eq
                br_if $#label3
              end
              i32.const 0
              local.set $#local2
              i32.const 1
              local.set $#local1
              local.get $#local3
              i32.eqz
              br_if $#label1
              br $#label2
            end
            i32.const 2
            local.set $#local2
            i32.const 3
            local.set $#local1
            local.get $#local3
            i32.eqz
            br_if $#label1
          end
          local.get $#local4
          local.get $#local3
          i32.const 1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055824
        i32.add
        local.tee $#local3
        local.get $#local3
        i32.load8_u
        local.tee $#local3
        local.get $#local1
        local.get $#local3
        select
        i32.store8
        local.get $#local3
        i32.eqz
        br_if $#label0
        i32.const 3
        local.set $#local2
        local.get $#local3
        i32.const 4
        i32.ge_u
        br_if $#label0
        i32.const 33619971
        local.get $#local3
        i32.const 3
        i32.shl
        i32.const 248
        i32.and
        i32.shr_u
        local.set $#local2
      end
      local.get $#local0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local2
    )
    (func $_ZN3std7process5abort17had5be8ae244d01ebE (;85;) (type $#type0)
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17h01579792467e4581E (;86;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            global.get $GOT.data.internal.__memory_base
            i32.const 1055776
            i32.add
            i32.load8_u offset=40
            br_table $#label1 $#label1 $#label2 $#label0 $#label1
          end
          local.get $#local1
          i32.const 0
          i32.store offset=24
          local.get $#local1
          i32.const 1
          i32.store offset=12
          local.get $#local1
          i64.const 4
          i64.store offset=16 align=4
          local.get $#local1
          global.get $GOT.data.internal.__memory_base
          local.tee $#local0
          i32.const 1055340
          i32.add
          i32.store offset=8
          local.get $#local1
          i32.const 8
          i32.add
          local.get $#local0
          i32.const 1054840
          i32.add
          call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
          unreachable
        end
        local.get $#local0
        i32.const 1
        i32.store8
        global.get $GOT.data.internal.__memory_base
        i32.const 1055776
        i32.add
        local.tee $#local0
        i32.const 3
        i32.store8 offset=40
        local.get $#local0
        i32.const 0
        i32.store8 offset=32
        local.get $#local0
        i64.const 1
        i64.store offset=24
        local.get $#local0
        i64.const 0
        i64.store offset=16
        local.get $#local0
        i32.const 0
        i32.store8 offset=12
        local.get $#local0
        i32.const 0
        i32.store offset=8
        local.get $#local0
        i64.const 0
        i64.store
      end
      local.get $#local1
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E (;87;) (type $#type10) (result i32)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      global.get $GOT.data.internal.__memory_base
      i32.const 1055825
      i32.add
      local.tee $#local1
      i32.load8_u
      local.set $#local2
      local.get $#local1
      i32.const 1
      i32.store8
      local.get $#local0
      local.get $#local2
      i32.store8 offset=7
      block $#label0
        local.get $#local2
        i32.const 1
        i32.ne
        br_if $#label0
        local.get $#local0
        i64.const 0
        i64.store offset=20 align=4
        local.get $#local0
        i64.const 17179869185
        i64.store offset=12 align=4
        local.get $#local0
        global.get $GOT.data.internal.__memory_base
        local.tee $#local2
        i32.const 1054800
        i32.add
        i32.store offset=8
        i32.const 0
        local.get $#local0
        i32.const 7
        i32.add
        global.get $GOT.data.internal._ZN3std4sync4mpmc5waker17current_thread_id5DUMMY28_$u7b$$u7b$closure$u7d$$u7d$3VAL17h915cf42b11aa1aa4E
        local.get $#local0
        i32.const 8
        i32.add
        local.get $#local2
        i32.const 1054808
        i32.add
        call $_ZN4core9panicking13assert_failed17heafbb113157aa4f0E
        unreachable
      end
      global.get $GOT.data.internal.__memory_base
      local.set $#local2
      local.get $#local0
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get $#local2
      i32.const 1055825
      i32.add
    )
    (func $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E (;88;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      local.get $#local4
      i32.const 1
      i32.store offset=12
      local.get $#local4
      i64.const 1
      i64.store offset=20 align=4
      local.get $#local4
      global.get $GOT.data.internal.__memory_base
      i32.const 1049356
      i32.add
      i32.store offset=8
      local.get $#local4
      local.get $#local3
      i32.store8 offset=47
      local.get $#local4
      global.get $GOT.func.internal._ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local4
      i32.const 47
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local4
      local.get $#local4
      i32.const 32
      i32.add
      i32.store offset=16
      local.get $#local0
      local.get $#local1
      local.get $#local4
      i32.const 8
      i32.add
      local.get $#local2
      call_indirect (type $#type4)
      local.get $#local4
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN98_$LT$std..sys..backtrace..BacktraceLock..print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h6e926483d10e272bE (;89;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i64) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local1
      i32.load offset=4
      local.set $#local3
      local.get $#local1
      i32.load
      local.set $#local4
      local.get $#local0
      i32.load8_u
      local.set $#local0
      local.get $#local2
      i32.const 4
      i32.add
      call $_ZN3std3env11current_dir17h890335e8528685e2E
      local.get $#local2
      i64.load offset=8 align=4
      local.set $#local5
      block $#label0
        local.get $#local2
        i32.load offset=4
        local.tee $#local1
        i32.const -2147483648
        i32.ne
        br_if $#label0
        local.get $#local5
        i64.const 255
        i64.and
        i64.const 3
        i64.ne
        br_if $#label0
        local.get $#local5
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee $#local6
        i32.load
        local.set $#local7
        block $#label1
          local.get $#local6
          i32.const 4
          i32.add
          i32.load
          local.tee $#local8
          i32.load
          local.tee $#local9
          i32.eqz
          br_if $#label1
          local.get $#local7
          local.get $#local9
          call_indirect (type $#type1)
        end
        block $#label1
          local.get $#local8
          i32.load offset=4
          local.tee $#local9
          i32.eqz
          br_if $#label1
          local.get $#local7
          local.get $#local9
          local.get $#local8
          i32.load offset=8
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local6
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      block $#label0
        block $#label1
          block $#label2
            local.get $#local4
            global.get $GOT.data.internal.__memory_base
            i32.const 1050726
            i32.add
            i32.const 17
            local.get $#local3
            i32.load offset=12
            local.tee $#local3
            call_indirect (type $#type5)
            br_if $#label2
            block $#label3
              local.get $#local0
              i32.const 1
              i32.and
              br_if $#label3
              local.get $#local4
              global.get $GOT.data.internal.__memory_base
              i32.const 1050743
              i32.add
              i32.const 88
              local.get $#local3
              call_indirect (type $#type5)
              br_if $#label2
            end
            i32.const 0
            local.set $#local4
            local.get $#local1
            i32.const -2147483648
            i32.or
            i32.const -2147483648
            i32.eq
            br_if $#label0
            br $#label1
          end
          i32.const 1
          local.set $#local4
          local.get $#local1
          i32.const -2147483648
          i32.or
          i32.const -2147483648
          i32.eq
          br_if $#label0
        end
        local.get $#local5
        i32.wrap_i64
        local.get $#local1
        i32.const 1
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local4
    )
    (func $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE (;90;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE
      unreachable
    )
    (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0eaf9acd3a3c0f2dE (;91;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local0
      i32.load
      local.tee $#local2
      i32.load offset=12
      local.set $#local3
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local2
              i32.load offset=4
              br_table $#label3 $#label2 $#label1
            end
            local.get $#local3
            br_if $#label1
            i32.const 1
            local.set $#local2
            i32.const 0
            local.set $#local3
            br $#label0
          end
          local.get $#local3
          br_if $#label1
          local.get $#local2
          i32.load
          local.tee $#local2
          i32.load offset=4
          local.set $#local3
          local.get $#local2
          i32.load
          local.set $#local2
          br $#label0
        end
        local.get $#local1
        i32.const -2147483648
        i32.store
        global.get $GOT.data.internal.__memory_base
        local.set $#local2
        local.get $#local1
        local.get $#local0
        i32.store offset=12
        local.get $#local1
        local.get $#local2
        i32.const 1055144
        i32.add
        local.get $#local0
        i32.load offset=4
        local.get $#local0
        i32.load offset=8
        local.tee $#local0
        i32.load8_u offset=8
        local.get $#local0
        i32.load8_u offset=9
        call $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E
        unreachable
      end
      local.get $#local1
      local.get $#local3
      i32.store offset=4
      local.get $#local1
      local.get $#local2
      i32.store
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055116
      i32.add
      local.get $#local0
      i32.load offset=4
      local.get $#local0
      i32.load offset=8
      local.tee $#local0
      i32.load8_u offset=8
      local.get $#local0
      i32.load8_u offset=9
      call $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E
      unreachable
    )
    (func $_ZN3std5alloc24default_alloc_error_hook17hafb1c9a509edc766E (;92;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        global.get $GOT.data.internal._RNvCscSpY9Juk0HT_7___rustc39___rust_alloc_error_handler_should_panic
        i32.load8_u
        br_if $#label0
        local.get $#local2
        i32.const 2
        i32.store offset=12
        local.get $#local2
        global.get $GOT.data.internal.__memory_base
        i32.const 1054896
        i32.add
        i32.store offset=8
        local.get $#local2
        i64.const 1
        i64.store offset=20 align=4
        local.get $#local2
        global.get $GOT.func.internal._ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h0c6ce12b01068af4E
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get $#local2
        i32.const 40
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=32
        local.get $#local2
        local.get $#local1
        i32.store offset=40
        local.get $#local2
        local.get $#local2
        i32.const 32
        i32.add
        i32.store offset=16
        local.get $#local2
        local.get $#local2
        i32.const 47
        i32.add
        local.get $#local2
        i32.const 8
        i32.add
        call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
        local.get $#local2
        i32.load offset=4
        local.set $#local3
        block $#label1
          block $#label2
            local.get $#local2
            i32.load8_u
            local.tee $#local1
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local3
          i32.load
          local.set $#local4
          block $#label2
            local.get $#local3
            i32.const 4
            i32.add
            i32.load
            local.tee $#local1
            i32.load
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local5
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local1
            i32.load offset=4
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local5
            local.get $#local1
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local3
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local2
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local2
      i32.const 2
      i32.store offset=12
      local.get $#local2
      i64.const 1
      i64.store offset=20 align=4
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      local.tee $#local3
      i32.const 1054912
      i32.add
      i32.store offset=8
      local.get $#local2
      local.get $#local1
      i32.store
      local.get $#local2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h0c6ce12b01068af4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local2
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local2
      local.get $#local2
      i32.const 32
      i32.add
      i32.store offset=16
      local.get $#local2
      i32.const 8
      i32.add
      local.get $#local3
      i32.const 1054928
      i32.add
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc11___rdl_alloc (;93;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            i32.const 8
            i32.gt_u
            br_if $#label2
            local.get $#local1
            local.get $#local0
            i32.le_u
            br_if $#label1
          end
          local.get $#local2
          i32.const 0
          i32.store offset=12
          local.get $#local2
          i32.const 12
          i32.add
          local.get $#local1
          i32.const 4
          local.get $#local1
          i32.const 4
          i32.gt_u
          select
          local.get $#local0
          call $posix_memalign
          local.set $#local1
          i32.const 0
          local.get $#local2
          i32.load offset=12
          local.get $#local1
          select
          local.set $#local1
          br $#label0
        end
        local.get $#local0
        call $malloc
        local.set $#local1
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_dealloc (;94;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      local.get $#local0
      call $free
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc13___rdl_realloc (;95;) (type $#type9) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.const 8
            i32.gt_u
            br_if $#label2
            local.get $#local2
            local.get $#local3
            i32.le_u
            br_if $#label1
          end
          i32.const 0
          local.set $#local5
          local.get $#local4
          i32.const 0
          i32.store offset=12
          local.get $#local4
          i32.const 12
          i32.add
          local.get $#local2
          i32.const 4
          local.get $#local2
          i32.const 4
          i32.gt_u
          select
          local.get $#local3
          call $posix_memalign
          br_if $#label0
          local.get $#local4
          i32.load offset=12
          local.tee $#local2
          i32.eqz
          br_if $#label0
          block $#label2
            local.get $#local3
            local.get $#local1
            local.get $#local3
            local.get $#local1
            i32.lt_u
            select
            local.tee $#local3
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local0
            local.get $#local3
            memory.copy
          end
          local.get $#local0
          call $free
          local.get $#local2
          local.set $#local5
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        call $realloc
        local.set $#local5
      end
      local.get $#local4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local5
    )
    (func $_ZN3std9panicking14payload_as_str17h6b6acf98476ffb56E (;96;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      local.get $#local2
      i32.load offset=12
      local.tee $#local4
      call_indirect (type $#type6)
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i64.load
            i64.const -5076933981314334344
            i64.ne
            br_if $#label2
            i32.const 4
            local.set $#local2
            local.get $#local1
            local.set $#local5
            local.get $#local3
            i64.load offset=8
            i64.const 7199936582794304877
            i64.eq
            br_if $#label1
          end
          local.get $#local3
          local.get $#local1
          local.get $#local4
          call_indirect (type $#type6)
          global.get $GOT.data.internal.__memory_base
          i32.const 1051014
          i32.add
          local.set $#local2
          i32.const 12
          local.set $#local4
          local.get $#local3
          i64.load
          i64.const 6756087622182587336
          i64.ne
          br_if $#label0
          local.get $#local3
          i64.load offset=8
          i64.const -245993367077761921
          i64.ne
          br_if $#label0
          local.get $#local1
          i32.const 4
          i32.add
          local.set $#local5
          i32.const 8
          local.set $#local2
        end
        local.get $#local1
        local.get $#local2
        i32.add
        i32.load
        local.set $#local4
        local.get $#local5
        i32.load
        local.set $#local2
      end
      local.get $#local0
      local.get $#local4
      i32.store offset=4
      local.get $#local0
      local.get $#local2
      i32.store
      local.get $#local3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h4c0831588a617affE (;97;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i64) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      call $_ZN3std3sys9backtrace4lock17hdf48be0f9c8878d9E
      local.set $#local4
      local.get $#local0
      i64.load align=4
      local.set $#local5
      local.get $#local3
      local.get $#local2
      i32.store offset=20
      local.get $#local3
      local.get $#local1
      i32.store offset=16
      local.get $#local3
      local.get $#local5
      i64.store offset=8 align=4
      block $#label0
        block $#label1
          global.get $GOT.data.internal._ZN3std6thread7current7CURRENT17h17ed3b660549d676E
          i32.load
          local.tee $#local6
          i32.const 2
          i32.gt_u
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055864
          i32.add
          i64.load
          local.set $#local5
          global.get $GOT.data.internal._ZN3std6thread7current2id2ID17h2449a857c48c194bE
          local.set $#local6
          block $#label2
            block $#label3
              local.get $#local5
              i64.eqz
              br_if $#label3
              local.get $#local6
              i64.load
              local.get $#local5
              i64.eq
              br_if $#label2
            end
            local.get $#local3
            i32.const 8
            i32.add
            i32.const 0
            local.get $#local3
            call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
            br $#label0
          end
          local.get $#local3
          i32.const 8
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1049222
          i32.add
          i32.const 4
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br $#label0
        end
        block $#label1
          local.get $#local6
          i32.load offset=8
          local.tee $#local7
          i32.eqz
          br_if $#label1
          local.get $#local3
          i32.const 8
          i32.add
          local.get $#local7
          local.get $#local6
          i32.const 12
          i32.add
          i32.load
          i32.const -1
          i32.add
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br $#label0
        end
        global.get $GOT.data.internal.__memory_base
        local.set $#local7
        block $#label1
          local.get $#local6
          i64.load
          local.get $#local7
          i32.const 1055864
          i32.add
          i64.load
          i64.ne
          br_if $#label1
          local.get $#local3
          i32.const 8
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1049222
          i32.add
          i32.const 4
          call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
          br $#label0
        end
        local.get $#local3
        i32.const 8
        i32.add
        i32.const 0
        local.get $#local3
        call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E
      end
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local0
              i32.load offset=8
              i32.load8_u
              br_table $#label3 $#label2 $#label1 $#label0 $#label3
            end
            local.get $#local3
            i32.const 8
            i32.add
            local.get $#local1
            local.get $#local2
            i32.load offset=36
            i32.const 0
            call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
            local.get $#local3
            i32.load8_u offset=8
            local.get $#local3
            i32.load offset=12
            call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
            br $#label0
          end
          local.get $#local3
          i32.const 8
          i32.add
          local.get $#local1
          local.get $#local2
          i32.load offset=36
          i32.const 1
          call $_ZN3std3sys9backtrace13BacktraceLock5print17hd4309a196a1d7f77E
          local.get $#local3
          i32.load8_u offset=8
          local.get $#local3
          i32.load offset=12
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br $#label0
        end
        global.get $GOT.data.internal.__memory_base
        i32.const 1055080
        i32.add
        local.tee $#local0
        i32.load8_u
        local.set $#local6
        local.get $#local0
        i32.const 0
        i32.store8
        local.get $#local6
        i32.eqz
        br_if $#label0
        local.get $#local3
        i32.const 0
        i32.store offset=24
        local.get $#local3
        i32.const 1
        i32.store offset=12
        local.get $#local3
        global.get $GOT.data.internal.__memory_base
        i32.const 1055024
        i32.add
        i32.store offset=8
        local.get $#local3
        i64.const 4
        i64.store offset=16 align=4
        local.get $#local3
        local.get $#local1
        local.get $#local3
        i32.const 8
        i32.add
        local.get $#local2
        i32.load offset=36
        call_indirect (type $#type4)
        local.get $#local3
        i32.load8_u
        local.get $#local3
        i32.load offset=4
        call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
      end
      local.get $#local4
      i32.const 0
      i32.store8
      local.get $#local3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h5207d826e8556c40E (;98;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i64) (local $#local5 i64) (local $#local6 i64) (local $#local7 i32) (local $#local8 i32)
      global.get $__stack_pointer
      i32.const 592
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local2
      i32.const 9
      local.get $#local1
      select
      i32.store offset=4
      local.get $#local3
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      local.tee $#local2
      i32.const 1050981
      i32.add
      local.get $#local1
      select
      i32.store
      block $#label0
        i32.const 512
        i32.eqz
        br_if $#label0
        local.get $#local3
        i32.const 8
        i32.add
        i32.const 0
        i32.const 512
        memory.fill
      end
      local.get $#local3
      i64.const 0
      i64.store offset=528
      local.get $#local3
      i32.const 512
      i32.store offset=524
      local.get $#local3
      local.get $#local3
      i32.const 8
      i32.add
      i32.store offset=520
      local.get $#local0
      i64.load32_u
      local.set $#local4
      local.get $#local0
      i64.load32_u offset=4
      local.set $#local5
      local.get $#local3
      local.get $#local2
      i32.const 1055048
      i32.add
      i32.store offset=544
      local.get $#local3
      i64.const 3
      i64.store offset=556 align=4
      local.get $#local3
      local.get $#local5
      global.get $GOT.data.internal.__table_base
      local.tee $#local1
      i32.const 6
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local6
      i64.or
      local.tee $#local5
      i64.store offset=584
      local.get $#local3
      local.get $#local4
      local.get $#local1
      i32.const 10
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.or
      local.tee $#local4
      i64.store offset=576
      local.get $#local3
      local.get $#local6
      local.get $#local3
      i64.extend_i32_u
      i64.or
      local.tee $#local6
      i64.store offset=568
      local.get $#local3
      local.get $#local3
      i32.const 568
      i32.add
      i32.store offset=552
      local.get $#local3
      i32.const 4
      i32.store offset=548
      local.get $#local3
      i32.const 536
      i32.add
      local.get $#local3
      i32.const 520
      i32.add
      local.get $#local3
      i32.const 544
      i32.add
      call $_ZN3std2io5Write9write_fmt17h1459a55272857e0fE
      block $#label0
        block $#label1
          block $#label2
            local.get $#local3
            i32.load8_u offset=536
            local.tee $#local1
            i32.const 4
            i32.ne
            br_if $#label2
            local.get $#local3
            i32.load offset=528
            local.tee $#local1
            i32.const 513
            i32.ge_u
            br_if $#label0
            local.get $#local3
            i32.const 568
            i32.add
            local.get $#local0
            i32.load offset=8
            local.get $#local3
            i32.const 8
            i32.add
            local.get $#local1
            local.get $#local0
            i32.load offset=12
            i32.load offset=28
            call_indirect (type $#type7)
            local.get $#local3
            i32.load offset=572
            local.set $#local1
            block $#label3
              local.get $#local3
              i32.load8_u offset=568
              local.tee $#local0
              i32.const 4
              i32.gt_u
              br_if $#label3
              local.get $#local0
              i32.const 3
              i32.ne
              br_if $#label1
            end
            local.get $#local1
            i32.load
            local.set $#local2
            block $#label3
              local.get $#local1
              i32.const 4
              i32.add
              i32.load
              local.tee $#local0
              i32.load
              local.tee $#local7
              i32.eqz
              br_if $#label3
              local.get $#local2
              local.get $#local7
              call_indirect (type $#type1)
            end
            block $#label3
              local.get $#local0
              i32.load offset=4
              local.tee $#local7
              i32.eqz
              br_if $#label3
              local.get $#local2
              local.get $#local7
              local.get $#local0
              i32.load offset=8
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get $#local1
            i32.const 12
            i32.const 4
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            br $#label1
          end
          block $#label2
            local.get $#local1
            i32.const 3
            i32.lt_u
            br_if $#label2
            local.get $#local3
            i32.load offset=540
            local.tee $#local1
            i32.load
            local.set $#local7
            block $#label3
              local.get $#local1
              i32.const 4
              i32.add
              i32.load
              local.tee $#local2
              i32.load
              local.tee $#local8
              i32.eqz
              br_if $#label3
              local.get $#local7
              local.get $#local8
              call_indirect (type $#type1)
            end
            block $#label3
              local.get $#local2
              i32.load offset=4
              local.tee $#local8
              i32.eqz
              br_if $#label3
              local.get $#local7
              local.get $#local8
              local.get $#local2
              i32.load offset=8
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
            end
            local.get $#local1
            i32.const 12
            i32.const 4
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local0
          i32.load offset=12
          i32.const 36
          i32.add
          i32.load
          local.set $#local1
          global.get $GOT.data.internal.__memory_base
          local.set $#local2
          local.get $#local0
          i32.load offset=8
          local.set $#local0
          local.get $#local3
          local.get $#local2
          i32.const 1055048
          i32.add
          i32.store offset=544
          local.get $#local3
          i64.const 3
          i64.store offset=556 align=4
          local.get $#local3
          local.get $#local5
          i64.store offset=584
          local.get $#local3
          local.get $#local4
          i64.store offset=576
          local.get $#local3
          local.get $#local6
          i64.store offset=568
          local.get $#local3
          local.get $#local3
          i32.const 568
          i32.add
          i32.store offset=552
          local.get $#local3
          i32.const 4
          i32.store offset=548
          local.get $#local3
          i32.const 536
          i32.add
          local.get $#local0
          local.get $#local3
          i32.const 544
          i32.add
          local.get $#local1
          call_indirect (type $#type4)
          local.get $#local3
          i32.load offset=540
          local.set $#local1
          block $#label2
            local.get $#local3
            i32.load8_u offset=536
            local.tee $#local0
            i32.const 4
            i32.gt_u
            br_if $#label2
            local.get $#local0
            i32.const 3
            i32.ne
            br_if $#label1
          end
          local.get $#local1
          i32.load
          local.set $#local2
          block $#label2
            local.get $#local1
            i32.const 4
            i32.add
            i32.load
            local.tee $#local0
            i32.load
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local7
            call_indirect (type $#type1)
          end
          block $#label2
            local.get $#local0
            i32.load offset=4
            local.tee $#local7
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local7
            local.get $#local0
            i32.load offset=8
            call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          end
          local.get $#local1
          i32.const 12
          i32.const 4
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
        end
        local.get $#local3
        i32.const 592
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get $#local1
      i32.const 512
      global.get $GOT.data.internal.__memory_base
      i32.const 1055032
      i32.add
      call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
      unreachable
    )
    (func $_ZN3std9panicking11panic_count8increase17hbcaad6b35138193cE (;99;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32)
      global.get $GOT.data.internal._ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h79e1014250ce3477E
      local.tee $#local1
      local.get $#local1
      i32.load
      local.tee $#local2
      i32.const 1
      i32.add
      i32.store
      i32.const 0
      local.set $#local1
      block $#label0
        local.get $#local2
        i32.const 0
        i32.lt_s
        br_if $#label0
        i32.const 1
        local.set $#local1
        global.get $GOT.data.internal.__memory_base
        i32.const 1055876
        i32.add
        i32.load8_u
        br_if $#label0
        global.get $GOT.data.internal.__memory_base
        local.tee $#local1
        i32.const 1055876
        i32.add
        local.get $#local0
        i32.store8
        local.get $#local1
        i32.const 1055872
        i32.add
        local.tee $#local1
        local.get $#local1
        i32.load
        i32.const 1
        i32.add
        i32.store
        i32.const 2
        local.set $#local1
      end
      local.get $#local1
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind (;100;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local0
      i64.load align=4
      local.set $#local2
      local.get $#local1
      local.get $#local0
      i32.store offset=12
      local.get $#local1
      local.get $#local2
      i64.store offset=4 align=4
      local.get $#local1
      i32.const 4
      i32.add
      call $_ZN3std3sys9backtrace26__rust_end_short_backtrace17h4ff0739ecd963f9bE
      unreachable
    )
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17heb32705b5eb94843E (;101;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        local.get $#local1
        i32.load
        i32.const -2147483648
        i32.ne
        br_if $#label0
        local.get $#local1
        i32.load offset=12
        local.set $#local3
        local.get $#local2
        i32.const 28
        i32.add
        i32.const 8
        i32.add
        local.tee $#local4
        i32.const 0
        i32.store
        local.get $#local2
        i64.const 4294967296
        i64.store offset=28 align=4
        local.get $#local2
        i32.const 40
        i32.add
        i32.const 8
        i32.add
        local.get $#local3
        i32.load
        local.tee $#local3
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        i32.const 40
        i32.add
        i32.const 16
        i32.add
        local.get $#local3
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        local.get $#local3
        i64.load align=4
        i64.store offset=40
        local.get $#local2
        i32.const 28
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1054356
        i32.add
        local.get $#local2
        i32.const 40
        i32.add
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        drop
        local.get $#local2
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        local.get $#local4
        i32.load
        local.tee $#local3
        i32.store
        local.get $#local2
        local.get $#local2
        i64.load offset=28 align=4
        local.tee $#local5
        i64.store offset=16
        local.get $#local1
        i32.const 8
        i32.add
        local.get $#local3
        i32.store
        local.get $#local1
        local.get $#local5
        i64.store align=4
      end
      local.get $#local1
      i64.load align=4
      local.set $#local5
      local.get $#local1
      i64.const 4294967296
      i64.store align=4
      local.get $#local2
      i32.const 8
      i32.add
      local.tee $#local3
      local.get $#local1
      i32.const 8
      i32.add
      local.tee $#local1
      i32.load
      i32.store
      local.get $#local1
      i32.const 0
      i32.store
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      local.get $#local2
      local.get $#local5
      i64.store
      block $#label0
        i32.const 12
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee $#local1
        br_if $#label0
        i32.const 4
        i32.const 12
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get $#local1
      local.get $#local2
      i64.load
      i64.store align=4
      local.get $#local1
      i32.const 8
      i32.add
      local.get $#local3
      i32.load
      i32.store
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055084
      i32.add
      i32.store offset=4
      local.get $#local0
      local.get $#local1
      i32.store
      local.get $#local2
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h71afbfcd7cf1bd1fE (;102;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        local.get $#local1
        i32.load
        i32.const -2147483648
        i32.ne
        br_if $#label0
        local.get $#local1
        i32.load offset=12
        local.set $#local3
        local.get $#local2
        i32.const 12
        i32.add
        i32.const 8
        i32.add
        local.tee $#local4
        i32.const 0
        i32.store
        local.get $#local2
        i64.const 4294967296
        i64.store offset=12 align=4
        local.get $#local2
        i32.const 24
        i32.add
        i32.const 8
        i32.add
        local.get $#local3
        i32.load
        local.tee $#local3
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        i32.const 24
        i32.add
        i32.const 16
        i32.add
        local.get $#local3
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        local.get $#local3
        i64.load align=4
        i64.store offset=24
        local.get $#local2
        i32.const 12
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1054356
        i32.add
        local.get $#local2
        i32.const 24
        i32.add
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        drop
        local.get $#local2
        i32.const 8
        i32.add
        local.get $#local4
        i32.load
        local.tee $#local3
        i32.store
        local.get $#local2
        local.get $#local2
        i64.load offset=12 align=4
        local.tee $#local5
        i64.store
        local.get $#local1
        i32.const 8
        i32.add
        local.get $#local3
        i32.store
        local.get $#local1
        local.get $#local5
        i64.store align=4
      end
      local.get $#local0
      local.get $#local1
      i32.store
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055084
      i32.add
      i32.store offset=4
      local.get $#local2
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN95_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hc90ae3d8f0698ecdE (;103;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local0
          i32.load
          i32.const -2147483648
          i32.eq
          br_if $#label1
          local.get $#local1
          local.get $#local0
          i32.load offset=4
          local.get $#local0
          i32.load offset=8
          call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
          local.set $#local0
          br $#label0
        end
        local.get $#local2
        i32.const 8
        i32.add
        i32.const 8
        i32.add
        local.get $#local0
        i32.load offset=12
        i32.load
        local.tee $#local0
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        i32.const 8
        i32.add
        i32.const 16
        i32.add
        local.get $#local0
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get $#local2
        local.get $#local0
        i64.load align=4
        i64.store offset=8
        local.get $#local1
        i32.load
        local.get $#local1
        i32.load offset=4
        local.get $#local2
        i32.const 8
        i32.add
        call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
        local.set $#local0
      end
      local.get $#local2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdbde2bd2b5782eb3E (;104;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
      i32.load8_u
      drop
      local.get $#local1
      i32.load offset=4
      local.set $#local2
      local.get $#local1
      i32.load
      local.set $#local3
      block $#label0
        i32.const 8
        i32.const 4
        call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
        local.tee $#local1
        br_if $#label0
        i32.const 4
        i32.const 8
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get $#local1
      local.get $#local2
      i32.store offset=4
      local.get $#local1
      local.get $#local3
      i32.store
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055100
      i32.add
      i32.store offset=4
      local.get $#local0
      local.get $#local1
      i32.store
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h11f35de1f0d82ba2E (;105;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055100
      i32.add
      i32.store offset=4
      local.get $#local0
      local.get $#local1
      i32.store
    )
    (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$6as_str17h4f48d65bdfe0a98fE (;106;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i64.load align=4
      i64.store
    )
    (func $_ZN92_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..fmt..Display$GT$3fmt17hf46dd676442c3a05E (;107;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local1
      local.get $#local0
      i32.load
      local.get $#local0
      i32.load offset=4
      call $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E
    )
    (func $_ZN3std9panicking20rust_panic_with_hook17hb97340d45d508331E (;108;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32)
      (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 96
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      local.get $#local5
      local.get $#local1
      i32.store offset=32
      local.get $#local5
      local.get $#local0
      i32.store offset=28
      local.get $#local5
      local.get $#local2
      i32.store offset=36
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              i32.const 1
              call $_ZN3std9panicking11panic_count8increase17hbcaad6b35138193cE
              i32.const 255
              i32.and
              local.tee $#local6
              i32.const 2
              i32.eq
              br_if $#label3
              local.get $#local6
              i32.const 1
              i32.and
              i32.eqz
              br_if $#label2
              local.get $#local5
              i32.const 16
              i32.add
              local.get $#local0
              local.get $#local1
              i32.load offset=24
              call_indirect (type $#type6)
              local.get $#local5
              local.get $#local5
              i32.load offset=20
              i32.const 0
              local.get $#local5
              i32.load offset=16
              local.tee $#local1
              select
              i32.store offset=44
              local.get $#local5
              local.get $#local1
              i32.const 1
              local.get $#local1
              select
              i32.store offset=40
              local.get $#local5
              i32.const 3
              i32.store offset=76
              local.get $#local5
              global.get $GOT.data.internal.__memory_base
              i32.const 1055196
              i32.add
              i32.store offset=72
              local.get $#local5
              i64.const 2
              i64.store offset=84 align=4
              local.get $#local5
              global.get $GOT.data.internal.__table_base
              local.tee $#local1
              i32.const 6
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get $#local5
              i32.const 40
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=64
              local.get $#local5
              local.get $#local1
              i32.const 10
              i32.add
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get $#local5
              i32.const 36
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=56
              local.get $#local5
              local.get $#local5
              i32.const 56
              i32.add
              i32.store offset=80
              local.get $#local5
              i32.const 48
              i32.add
              local.get $#local5
              i32.const 48
              i32.add
              local.get $#local5
              i32.const 72
              i32.add
              call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
              local.get $#local5
              i32.load8_u offset=48
              local.get $#local5
              i32.load offset=52
              call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
              br $#label0
            end
            global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
            i32.load
            local.tee $#local6
            i32.const -1
            i32.gt_s
            br_if $#label1
            local.get $#local5
            i32.const 1
            i32.store offset=76
            local.get $#local5
            global.get $GOT.data.internal.__memory_base
            i32.const 1055348
            i32.add
            i32.store offset=72
            local.get $#local5
            i64.const 0
            i64.store offset=84 align=4
            local.get $#local5
            local.get $#local5
            i32.const 48
            i32.add
            i32.store offset=80
            local.get $#local5
            i32.const 56
            i32.add
            local.get $#local5
            i32.const 48
            i32.add
            local.get $#local5
            i32.const 72
            i32.add
            call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
            local.get $#local5
            i32.load8_u offset=56
            local.get $#local5
            i32.load offset=60
            call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
            br $#label0
          end
          local.get $#local5
          i32.const 3
          i32.store offset=76
          local.get $#local5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055172
          i32.add
          i32.store offset=72
          local.get $#local5
          i64.const 2
          i64.store offset=84 align=4
          local.get $#local5
          global.get $GOT.data.internal.__table_base
          local.tee $#local1
          i32.const 11
          i32.add
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get $#local5
          i32.const 28
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=64
          local.get $#local5
          local.get $#local1
          i32.const 10
          i32.add
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get $#local5
          i32.const 36
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=56
          local.get $#local5
          local.get $#local5
          i32.const 56
          i32.add
          i32.store offset=80
          local.get $#local5
          i32.const 48
          i32.add
          local.get $#local5
          i32.const 48
          i32.add
          local.get $#local5
          i32.const 72
          i32.add
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get $#local5
          i32.load8_u offset=48
          local.get $#local5
          i32.load offset=52
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br $#label0
        end
        global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
        local.tee $#local7
        local.get $#local6
        i32.const 1
        i32.add
        i32.store
        block $#label1
          block $#label2
            local.get $#local7
            i32.load offset=4
            i32.eqz
            br_if $#label2
            local.get $#local5
            i32.const 8
            i32.add
            local.get $#local0
            local.get $#local1
            i32.load offset=20
            call_indirect (type $#type6)
            local.get $#local5
            local.get $#local4
            i32.store8 offset=85
            local.get $#local5
            local.get $#local3
            i32.store8 offset=84
            local.get $#local5
            local.get $#local2
            i32.store offset=80
            local.get $#local5
            local.get $#local5
            i64.load offset=8
            i64.store offset=72 align=4
            global.get $GOT.data.internal._ZN3std9panicking4HOOK17h16d1102fa369f342E
            local.tee $#local2
            i32.load offset=4
            local.get $#local5
            i32.const 72
            i32.add
            local.get $#local2
            i32.load offset=8
            i32.load offset=20
            call_indirect (type $#type6)
            br $#label1
          end
          local.get $#local5
          local.get $#local0
          local.get $#local1
          i32.load offset=20
          call_indirect (type $#type6)
          local.get $#local5
          local.get $#local4
          i32.store8 offset=85
          local.get $#local5
          local.get $#local3
          i32.store8 offset=84
          local.get $#local5
          local.get $#local2
          i32.store offset=80
          local.get $#local5
          local.get $#local5
          i64.load
          i64.store offset=72 align=4
          local.get $#local5
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
        local.tee $#local2
        local.get $#local2
        i32.load
        i32.const -1
        i32.add
        i32.store
        block $#label1
          local.get $#local3
          br_if $#label1
          local.get $#local5
          i32.const 0
          i32.store offset=88
          local.get $#local5
          i32.const 1
          i32.store offset=76
          local.get $#local5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055220
          i32.add
          i32.store offset=72
          local.get $#local5
          i64.const 4
          i64.store offset=80 align=4
          local.get $#local5
          i32.const 56
          i32.add
          local.get $#local5
          i32.const 48
          i32.add
          local.get $#local5
          i32.const 72
          i32.add
          call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
          local.get $#local5
          i32.load8_u offset=56
          local.get $#local5
          i32.load offset=60
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
          br $#label0
        end
        local.get $#local0
        local.get $#local1
        call $_RNvCscSpY9Juk0HT_7___rustc10rust_panic
        unreachable
      end
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc10rust_panic (;109;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      local.get $#local0
      local.get $#local1
      call $_RNvCscSpY9Juk0HT_7___rustc18___rust_start_panic
      i32.store offset=12
      local.get $#local2
      i32.const 2
      i32.store offset=28
      local.get $#local2
      global.get $GOT.data.internal.__memory_base
      i32.const 1055228
      i32.add
      i32.store offset=24
      local.get $#local2
      i64.const 1
      i64.store offset=36 align=4
      local.get $#local2
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local2
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=48
      local.get $#local2
      local.get $#local2
      i32.const 48
      i32.add
      i32.store offset=32
      local.get $#local2
      i32.const 16
      i32.add
      local.get $#local2
      i32.const 63
      i32.add
      local.get $#local2
      i32.const 24
      i32.add
      call $_ZN3std2io5Write9write_fmt17h67a7676457c7b3d8E
      local.get $#local2
      i32.load8_u offset=16
      local.get $#local2
      i32.load offset=20
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h945aafbb9c5e45d0E
      call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
      unreachable
    )
    (func $cabi_realloc (;110;) (type $#type9) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            br_if $#label2
            local.get $#local3
            i32.eqz
            br_if $#label0
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
            local.get $#local3
            local.get $#local2
            call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
            local.tee $#local2
            i32.eqz
            br_if $#label1
            br $#label0
          end
          local.get $#local0
          local.get $#local1
          local.get $#local2
          local.get $#local3
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
          local.tee $#local2
          br_if $#label0
        end
        call $_ZN3std3sys3pal6wasip27helpers14abort_internal17h4cdbbf2d76c51f7aE
        unreachable
      end
      local.get $#local2
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5write17h074fa39550ca3e6dE (;111;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      local.get $#local4
      local.get $#local3
      i32.store offset=4
      local.get $#local4
      local.get $#local2
      i32.store
      local.get $#local4
      i32.const 8
      i32.add
      i32.const 2
      local.get $#local4
      i32.const 1
      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
      block $#label0
        block $#label1
          local.get $#local4
          i32.load16_u offset=8
          i32.const 1
          i32.ne
          br_if $#label1
          local.get $#local0
          local.get $#local4
          i64.load16_u offset=10
          i64.const 32
          i64.shl
          i64.store align=4
          br $#label0
        end
        local.get $#local0
        local.get $#local4
        i32.load offset=12
        i32.store offset=4
        local.get $#local0
        i32.const 4
        i32.store8
      end
      local.get $#local4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h039c43fb7c645440E (;112;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      local.get $#local4
      i32.const 8
      i32.add
      i32.const 2
      local.get $#local2
      local.get $#local3
      call $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE
      block $#label0
        block $#label1
          local.get $#local4
          i32.load16_u offset=8
          i32.const 1
          i32.ne
          br_if $#label1
          local.get $#local0
          local.get $#local4
          i64.load16_u offset=10
          i64.const 32
          i64.shl
          i64.store align=4
          br $#label0
        end
        local.get $#local0
        local.get $#local4
        i32.load offset=12
        i32.store offset=4
        local.get $#local0
        i32.const 4
        i32.store8
      end
      local.get $#local4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h652587ea317ff22dE (;113;) (type $#type3) (param $#local0 i32) (result i32)
      i32.const 1
    )
    (func $_ZN64_$LT$std..sys..stdio..wasi..Stderr$u20$as$u20$std..io..Write$GT$5flush17he9abf091f2dacbb1E (;114;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      i32.const 4
      i32.store8
    )
    (func $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E (;115;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $GOT.data.internal.__table_base
      local.set $#local2
      local.get $#local0
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055828
      i32.add
      i32.load
      local.tee $#local3
      local.get $#local2
      i32.const 12
      i32.add
      local.get $#local3
      select
      call_indirect (type $#type6)
      call $_ZN3std7process5abort17had5be8ae244d01ebE
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc8___rg_oom (;116;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local1
      local.get $#local0
      call $_ZN3std5alloc8rust_oom17h583d0dc7a0d2cf42E
      unreachable
    )
    (func $_RNvCscSpY9Juk0HT_7___rustc18___rust_start_panic (;117;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      unreachable
    )
    (func $_ZN4wasi13lib_generated8fd_write17h3935b9905651faecE (;118;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local1
          local.get $#local2
          local.get $#local3
          local.get $#local4
          i32.const 12
          i32.add
          call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h5858df6e6eba6e92E
          local.tee $#local3
          br_if $#label1
          local.get $#local0
          local.get $#local4
          i32.load offset=12
          i32.store offset=4
          i32.const 0
          local.set $#local3
          br $#label0
        end
        local.get $#local0
        local.get $#local3
        i32.store16 offset=2
        i32.const 1
        local.set $#local3
      end
      local.get $#local0
      local.get $#local3
      i32.store16
      local.get $#local4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $malloc (;119;) (type $#type3) (param $#local0 i32) (result i32)
      local.get $#local0
      call $dlmalloc
    )
    (func $dlmalloc (;120;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          block $#label10
                            block $#label11
                              block $#label12
                                i32.const 0
                                i32.load offset=1055912
                                local.tee $#local2
                                br_if $#label12
                                block $#label13
                                  i32.const 0
                                  i32.load offset=1056360
                                  local.tee $#local3
                                  br_if $#label13
                                  i32.const 0
                                  i64.const -1
                                  i64.store offset=1056372 align=4
                                  i32.const 0
                                  i64.const 281474976776192
                                  i64.store offset=1056364 align=4
                                  i32.const 0
                                  local.get $#local1
                                  i32.const 8
                                  i32.add
                                  i32.const -16
                                  i32.and
                                  i32.const 1431655768
                                  i32.xor
                                  local.tee $#local3
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
                                br_if $#label11
                                i32.const 0
                                local.set $#local2
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                i32.const 89
                                i32.lt_u
                                br_if $#label12
                                i32.const 0
                                local.set $#local4
                                i32.const 0
                                i32.const 1056432
                                i32.store offset=1056336
                                i32.const 0
                                i32.const 1056432
                                i32.store offset=1055904
                                i32.const 0
                                local.get $#local3
                                i32.store offset=1055924
                                i32.const 0
                                i32.const -1
                                i32.store offset=1055920
                                i32.const 0
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                local.tee $#local3
                                i32.store offset=1056340
                                i32.const 0
                                local.get $#local3
                                i32.store offset=1056324
                                i32.const 0
                                local.get $#local3
                                i32.store offset=1056320
                                loop $#label13
                                  local.get $#local4
                                  i32.const 1055948
                                  i32.add
                                  local.get $#local4
                                  i32.const 1055936
                                  i32.add
                                  local.tee $#local3
                                  i32.store
                                  local.get $#local3
                                  local.get $#local4
                                  i32.const 1055928
                                  i32.add
                                  local.tee $#local5
                                  i32.store
                                  local.get $#local4
                                  i32.const 1055940
                                  i32.add
                                  local.get $#local5
                                  i32.store
                                  local.get $#local4
                                  i32.const 1055956
                                  i32.add
                                  local.get $#local4
                                  i32.const 1055944
                                  i32.add
                                  local.tee $#local5
                                  i32.store
                                  local.get $#local5
                                  local.get $#local3
                                  i32.store
                                  local.get $#local4
                                  i32.const 1055964
                                  i32.add
                                  local.get $#local4
                                  i32.const 1055952
                                  i32.add
                                  local.tee $#local3
                                  i32.store
                                  local.get $#local3
                                  local.get $#local5
                                  i32.store
                                  local.get $#local4
                                  i32.const 1055960
                                  i32.add
                                  local.get $#local3
                                  i32.store
                                  local.get $#local4
                                  i32.const 32
                                  i32.add
                                  local.tee $#local4
                                  i32.const 256
                                  i32.ne
                                  br_if $#label13
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
                                local.tee $#local4
                                i32.add
                                local.tee $#local2
                                i32.store offset=1055912
                                i32.const 0
                                i32.const 1114112
                                i32.const 1056432
                                i32.sub
                                local.get $#local4
                                i32.sub
                                i32.const -56
                                i32.add
                                local.tee $#local4
                                i32.store offset=1055900
                                local.get $#local2
                                local.get $#local4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                              end
                              block $#label12
                                block $#label13
                                  local.get $#local0
                                  i32.const 236
                                  i32.gt_u
                                  br_if $#label13
                                  block $#label14
                                    i32.const 0
                                    i32.load offset=1055888
                                    local.tee $#local6
                                    i32.const 16
                                    local.get $#local0
                                    i32.const 19
                                    i32.add
                                    i32.const 496
                                    i32.and
                                    local.get $#local0
                                    i32.const 11
                                    i32.lt_u
                                    select
                                    local.tee $#local5
                                    i32.const 3
                                    i32.shr_u
                                    local.tee $#local3
                                    i32.shr_u
                                    local.tee $#local4
                                    i32.const 3
                                    i32.and
                                    i32.eqz
                                    br_if $#label14
                                    block $#label15
                                      block $#label16
                                        local.get $#local4
                                        i32.const 1
                                        i32.and
                                        local.get $#local3
                                        i32.or
                                        i32.const 1
                                        i32.xor
                                        local.tee $#local5
                                        i32.const 3
                                        i32.shl
                                        local.tee $#local3
                                        i32.const 1055928
                                        i32.add
                                        local.tee $#local4
                                        local.get $#local3
                                        i32.const 1055936
                                        i32.add
                                        i32.load
                                        local.tee $#local3
                                        i32.load offset=8
                                        local.tee $#local0
                                        i32.ne
                                        br_if $#label16
                                        i32.const 0
                                        local.get $#local6
                                        i32.const -2
                                        local.get $#local5
                                        i32.rotl
                                        i32.and
                                        i32.store offset=1055888
                                        br $#label15
                                      end
                                      local.get $#local4
                                      local.get $#local0
                                      i32.store offset=8
                                      local.get $#local0
                                      local.get $#local4
                                      i32.store offset=12
                                    end
                                    local.get $#local3
                                    i32.const 8
                                    i32.add
                                    local.set $#local4
                                    local.get $#local3
                                    local.get $#local5
                                    i32.const 3
                                    i32.shl
                                    local.tee $#local5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    local.get $#local3
                                    local.get $#local5
                                    i32.add
                                    local.tee $#local3
                                    local.get $#local3
                                    i32.load offset=4
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    br $#label0
                                  end
                                  local.get $#local5
                                  i32.const 0
                                  i32.load offset=1055896
                                  local.tee $#local7
                                  i32.le_u
                                  br_if $#label12
                                  block $#label14
                                    local.get $#local4
                                    i32.eqz
                                    br_if $#label14
                                    block $#label15
                                      block $#label16
                                        local.get $#local4
                                        local.get $#local3
                                        i32.shl
                                        i32.const 2
                                        local.get $#local3
                                        i32.shl
                                        local.tee $#local4
                                        i32.const 0
                                        local.get $#local4
                                        i32.sub
                                        i32.or
                                        i32.and
                                        i32.ctz
                                        local.tee $#local3
                                        i32.const 3
                                        i32.shl
                                        local.tee $#local4
                                        i32.const 1055928
                                        i32.add
                                        local.tee $#local0
                                        local.get $#local4
                                        i32.const 1055936
                                        i32.add
                                        i32.load
                                        local.tee $#local4
                                        i32.load offset=8
                                        local.tee $#local8
                                        i32.ne
                                        br_if $#label16
                                        i32.const 0
                                        local.get $#local6
                                        i32.const -2
                                        local.get $#local3
                                        i32.rotl
                                        i32.and
                                        local.tee $#local6
                                        i32.store offset=1055888
                                        br $#label15
                                      end
                                      local.get $#local0
                                      local.get $#local8
                                      i32.store offset=8
                                      local.get $#local8
                                      local.get $#local0
                                      i32.store offset=12
                                    end
                                    local.get $#local4
                                    local.get $#local5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    local.get $#local4
                                    local.get $#local3
                                    i32.const 3
                                    i32.shl
                                    local.tee $#local3
                                    i32.add
                                    local.get $#local3
                                    local.get $#local5
                                    i32.sub
                                    local.tee $#local0
                                    i32.store
                                    local.get $#local4
                                    local.get $#local5
                                    i32.add
                                    local.tee $#local8
                                    local.get $#local0
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    block $#label15
                                      local.get $#local7
                                      i32.eqz
                                      br_if $#label15
                                      local.get $#local7
                                      i32.const -8
                                      i32.and
                                      i32.const 1055928
                                      i32.add
                                      local.set $#local5
                                      i32.const 0
                                      i32.load offset=1055908
                                      local.set $#local3
                                      block $#label16
                                        block $#label17
                                          local.get $#local6
                                          i32.const 1
                                          local.get $#local7
                                          i32.const 3
                                          i32.shr_u
                                          i32.shl
                                          local.tee $#local9
                                          i32.and
                                          br_if $#label17
                                          i32.const 0
                                          local.get $#local6
                                          local.get $#local9
                                          i32.or
                                          i32.store offset=1055888
                                          local.get $#local5
                                          local.set $#local9
                                          br $#label16
                                        end
                                        local.get $#local5
                                        i32.load offset=8
                                        local.set $#local9
                                      end
                                      local.get $#local9
                                      local.get $#local3
                                      i32.store offset=12
                                      local.get $#local5
                                      local.get $#local3
                                      i32.store offset=8
                                      local.get $#local3
                                      local.get $#local5
                                      i32.store offset=12
                                      local.get $#local3
                                      local.get $#local9
                                      i32.store offset=8
                                    end
                                    local.get $#local4
                                    i32.const 8
                                    i32.add
                                    local.set $#local4
                                    i32.const 0
                                    local.get $#local8
                                    i32.store offset=1055908
                                    i32.const 0
                                    local.get $#local0
                                    i32.store offset=1055896
                                    br $#label0
                                  end
                                  i32.const 0
                                  i32.load offset=1055892
                                  local.tee $#local10
                                  i32.eqz
                                  br_if $#label12
                                  local.get $#local10
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 1056192
                                  i32.add
                                  i32.load
                                  local.tee $#local8
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get $#local5
                                  i32.sub
                                  local.set $#local3
                                  local.get $#local8
                                  local.set $#local0
                                  block $#label14
                                    loop $#label15
                                      block $#label16
                                        local.get $#local0
                                        i32.load offset=16
                                        local.tee $#local4
                                        br_if $#label16
                                        local.get $#local0
                                        i32.load offset=20
                                        local.tee $#local4
                                        i32.eqz
                                        br_if $#label14
                                      end
                                      local.get $#local4
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get $#local5
                                      i32.sub
                                      local.tee $#local0
                                      local.get $#local3
                                      local.get $#local0
                                      local.get $#local3
                                      i32.lt_u
                                      local.tee $#local0
                                      select
                                      local.set $#local3
                                      local.get $#local4
                                      local.get $#local8
                                      local.get $#local0
                                      select
                                      local.set $#local8
                                      local.get $#local4
                                      local.set $#local0
                                      br $#label15
                                    end
                                  end
                                  local.get $#local8
                                  i32.load offset=24
                                  local.set $#local2
                                  block $#label14
                                    local.get $#local8
                                    i32.load offset=12
                                    local.tee $#local4
                                    local.get $#local8
                                    i32.eq
                                    br_if $#label14
                                    local.get $#local8
                                    i32.load offset=8
                                    local.tee $#local0
                                    local.get $#local4
                                    i32.store offset=12
                                    local.get $#local4
                                    local.get $#local0
                                    i32.store offset=8
                                    br $#label1
                                  end
                                  block $#label14
                                    block $#label15
                                      local.get $#local8
                                      i32.load offset=20
                                      local.tee $#local0
                                      i32.eqz
                                      br_if $#label15
                                      local.get $#local8
                                      i32.const 20
                                      i32.add
                                      local.set $#local9
                                      br $#label14
                                    end
                                    local.get $#local8
                                    i32.load offset=16
                                    local.tee $#local0
                                    i32.eqz
                                    br_if $#label10
                                    local.get $#local8
                                    i32.const 16
                                    i32.add
                                    local.set $#local9
                                  end
                                  loop $#label14
                                    local.get $#local9
                                    local.set $#local11
                                    local.get $#local0
                                    local.tee $#local4
                                    i32.const 20
                                    i32.add
                                    local.set $#local9
                                    local.get $#local4
                                    i32.load offset=20
                                    local.tee $#local0
                                    br_if $#label14
                                    local.get $#local4
                                    i32.const 16
                                    i32.add
                                    local.set $#local9
                                    local.get $#local4
                                    i32.load offset=16
                                    local.tee $#local0
                                    br_if $#label14
                                  end
                                  local.get $#local11
                                  i32.const 0
                                  i32.store
                                  br $#label1
                                end
                                i32.const -1
                                local.set $#local5
                                local.get $#local0
                                i32.const -65
                                i32.gt_u
                                br_if $#label12
                                local.get $#local0
                                i32.const 19
                                i32.add
                                local.tee $#local4
                                i32.const -16
                                i32.and
                                local.set $#local5
                                i32.const 0
                                i32.load offset=1055892
                                local.tee $#local10
                                i32.eqz
                                br_if $#label12
                                i32.const 31
                                local.set $#local7
                                block $#label13
                                  local.get $#local0
                                  i32.const 16777196
                                  i32.gt_u
                                  br_if $#label13
                                  local.get $#local5
                                  i32.const 38
                                  local.get $#local4
                                  i32.const 8
                                  i32.shr_u
                                  i32.clz
                                  local.tee $#local4
                                  i32.sub
                                  i32.shr_u
                                  i32.const 1
                                  i32.and
                                  local.get $#local4
                                  i32.const 1
                                  i32.shl
                                  i32.sub
                                  i32.const 62
                                  i32.add
                                  local.set $#local7
                                end
                                i32.const 0
                                local.get $#local5
                                i32.sub
                                local.set $#local3
                                block $#label13
                                  block $#label14
                                    block $#label15
                                      block $#label16
                                        local.get $#local7
                                        i32.const 2
                                        i32.shl
                                        i32.const 1056192
                                        i32.add
                                        i32.load
                                        local.tee $#local0
                                        br_if $#label16
                                        i32.const 0
                                        local.set $#local4
                                        i32.const 0
                                        local.set $#local9
                                        br $#label15
                                      end
                                      i32.const 0
                                      local.set $#local4
                                      local.get $#local5
                                      i32.const 0
                                      i32.const 25
                                      local.get $#local7
                                      i32.const 1
                                      i32.shr_u
                                      i32.sub
                                      local.get $#local7
                                      i32.const 31
                                      i32.eq
                                      select
                                      i32.shl
                                      local.set $#local8
                                      i32.const 0
                                      local.set $#local9
                                      loop $#label16
                                        block $#label17
                                          local.get $#local0
                                          i32.load offset=4
                                          i32.const -8
                                          i32.and
                                          local.get $#local5
                                          i32.sub
                                          local.tee $#local6
                                          local.get $#local3
                                          i32.ge_u
                                          br_if $#label17
                                          local.get $#local6
                                          local.set $#local3
                                          local.get $#local0
                                          local.set $#local9
                                          local.get $#local6
                                          br_if $#label17
                                          i32.const 0
                                          local.set $#local3
                                          local.get $#local0
                                          local.set $#local9
                                          local.get $#local0
                                          local.set $#local4
                                          br $#label14
                                        end
                                        local.get $#local4
                                        local.get $#local0
                                        i32.load offset=20
                                        local.tee $#local6
                                        local.get $#local6
                                        local.get $#local0
                                        local.get $#local8
                                        i32.const 29
                                        i32.shr_u
                                        i32.const 4
                                        i32.and
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i32.load
                                        local.tee $#local11
                                        i32.eq
                                        select
                                        local.get $#local4
                                        local.get $#local6
                                        select
                                        local.set $#local4
                                        local.get $#local8
                                        i32.const 1
                                        i32.shl
                                        local.set $#local8
                                        local.get $#local11
                                        local.set $#local0
                                        local.get $#local11
                                        br_if $#label16
                                      end
                                    end
                                    block $#label15
                                      local.get $#local4
                                      local.get $#local9
                                      i32.or
                                      br_if $#label15
                                      i32.const 0
                                      local.set $#local9
                                      i32.const 2
                                      local.get $#local7
                                      i32.shl
                                      local.tee $#local4
                                      i32.const 0
                                      local.get $#local4
                                      i32.sub
                                      i32.or
                                      local.get $#local10
                                      i32.and
                                      local.tee $#local4
                                      i32.eqz
                                      br_if $#label12
                                      local.get $#local4
                                      i32.ctz
                                      i32.const 2
                                      i32.shl
                                      i32.const 1056192
                                      i32.add
                                      i32.load
                                      local.set $#local4
                                    end
                                    local.get $#local4
                                    i32.eqz
                                    br_if $#label13
                                  end
                                  loop $#label14
                                    local.get $#local4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get $#local5
                                    i32.sub
                                    local.tee $#local6
                                    local.get $#local3
                                    i32.lt_u
                                    local.set $#local8
                                    block $#label15
                                      local.get $#local4
                                      i32.load offset=16
                                      local.tee $#local0
                                      br_if $#label15
                                      local.get $#local4
                                      i32.load offset=20
                                      local.set $#local0
                                    end
                                    local.get $#local6
                                    local.get $#local3
                                    local.get $#local8
                                    select
                                    local.set $#local3
                                    local.get $#local4
                                    local.get $#local9
                                    local.get $#local8
                                    select
                                    local.set $#local9
                                    local.get $#local0
                                    local.set $#local4
                                    local.get $#local0
                                    br_if $#label14
                                  end
                                end
                                local.get $#local9
                                i32.eqz
                                br_if $#label12
                                local.get $#local3
                                i32.const 0
                                i32.load offset=1055896
                                local.get $#local5
                                i32.sub
                                i32.ge_u
                                br_if $#label12
                                local.get $#local9
                                i32.load offset=24
                                local.set $#local11
                                block $#label13
                                  local.get $#local9
                                  i32.load offset=12
                                  local.tee $#local4
                                  local.get $#local9
                                  i32.eq
                                  br_if $#label13
                                  local.get $#local9
                                  i32.load offset=8
                                  local.tee $#local0
                                  local.get $#local4
                                  i32.store offset=12
                                  local.get $#local4
                                  local.get $#local0
                                  i32.store offset=8
                                  br $#label2
                                end
                                block $#label13
                                  block $#label14
                                    local.get $#local9
                                    i32.load offset=20
                                    local.tee $#local0
                                    i32.eqz
                                    br_if $#label14
                                    local.get $#local9
                                    i32.const 20
                                    i32.add
                                    local.set $#local8
                                    br $#label13
                                  end
                                  local.get $#local9
                                  i32.load offset=16
                                  local.tee $#local0
                                  i32.eqz
                                  br_if $#label9
                                  local.get $#local9
                                  i32.const 16
                                  i32.add
                                  local.set $#local8
                                end
                                loop $#label13
                                  local.get $#local8
                                  local.set $#local6
                                  local.get $#local0
                                  local.tee $#local4
                                  i32.const 20
                                  i32.add
                                  local.set $#local8
                                  local.get $#local4
                                  i32.load offset=20
                                  local.tee $#local0
                                  br_if $#label13
                                  local.get $#local4
                                  i32.const 16
                                  i32.add
                                  local.set $#local8
                                  local.get $#local4
                                  i32.load offset=16
                                  local.tee $#local0
                                  br_if $#label13
                                end
                                local.get $#local6
                                i32.const 0
                                i32.store
                                br $#label2
                              end
                              block $#label12
                                i32.const 0
                                i32.load offset=1055896
                                local.tee $#local4
                                local.get $#local5
                                i32.lt_u
                                br_if $#label12
                                i32.const 0
                                i32.load offset=1055908
                                local.set $#local3
                                block $#label13
                                  block $#label14
                                    local.get $#local4
                                    local.get $#local5
                                    i32.sub
                                    local.tee $#local0
                                    i32.const 16
                                    i32.lt_u
                                    br_if $#label14
                                    local.get $#local3
                                    local.get $#local5
                                    i32.add
                                    local.tee $#local8
                                    local.get $#local0
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    local.get $#local3
                                    local.get $#local4
                                    i32.add
                                    local.get $#local0
                                    i32.store
                                    local.get $#local3
                                    local.get $#local5
                                    i32.const 3
                                    i32.or
                                    i32.store offset=4
                                    br $#label13
                                  end
                                  local.get $#local3
                                  local.get $#local4
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get $#local3
                                  local.get $#local4
                                  i32.add
                                  local.tee $#local4
                                  local.get $#local4
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  i32.const 0
                                  local.set $#local8
                                  i32.const 0
                                  local.set $#local0
                                end
                                i32.const 0
                                local.get $#local0
                                i32.store offset=1055896
                                i32.const 0
                                local.get $#local8
                                i32.store offset=1055908
                                local.get $#local3
                                i32.const 8
                                i32.add
                                local.set $#local4
                                br $#label0
                              end
                              block $#label12
                                i32.const 0
                                i32.load offset=1055900
                                local.tee $#local0
                                local.get $#local5
                                i32.le_u
                                br_if $#label12
                                local.get $#local2
                                local.get $#local5
                                i32.add
                                local.tee $#local4
                                local.get $#local0
                                local.get $#local5
                                i32.sub
                                local.tee $#local3
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                i32.const 0
                                local.get $#local4
                                i32.store offset=1055912
                                i32.const 0
                                local.get $#local3
                                i32.store offset=1055900
                                local.get $#local2
                                local.get $#local5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get $#local2
                                i32.const 8
                                i32.add
                                local.set $#local4
                                br $#label0
                              end
                              block $#label12
                                block $#label13
                                  i32.const 0
                                  i32.load offset=1056360
                                  i32.eqz
                                  br_if $#label13
                                  i32.const 0
                                  i32.load offset=1056368
                                  local.set $#local3
                                  br $#label12
                                end
                                i32.const 0
                                i64.const -1
                                i64.store offset=1056372 align=4
                                i32.const 0
                                i64.const 281474976776192
                                i64.store offset=1056364 align=4
                                i32.const 0
                                local.get $#local1
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
                                local.set $#local3
                              end
                              i32.const 0
                              local.set $#local4
                              block $#label12
                                local.get $#local3
                                local.get $#local5
                                i32.const 71
                                i32.add
                                local.tee $#local11
                                i32.add
                                local.tee $#local8
                                i32.const 0
                                local.get $#local3
                                i32.sub
                                local.tee $#local6
                                i32.and
                                local.tee $#local9
                                local.get $#local5
                                i32.gt_u
                                br_if $#label12
                                i32.const 0
                                i32.const 48
                                i32.store offset=1056384
                                br $#label0
                              end
                              block $#label12
                                i32.const 0
                                i32.load offset=1056328
                                local.tee $#local4
                                i32.eqz
                                br_if $#label12
                                block $#label13
                                  i32.const 0
                                  i32.load offset=1056320
                                  local.tee $#local3
                                  local.get $#local9
                                  i32.add
                                  local.tee $#local7
                                  local.get $#local3
                                  i32.le_u
                                  br_if $#label13
                                  local.get $#local7
                                  local.get $#local4
                                  i32.le_u
                                  br_if $#label12
                                end
                                i32.const 0
                                local.set $#local4
                                i32.const 0
                                i32.const 48
                                i32.store offset=1056384
                                br $#label0
                              end
                              i32.const 0
                              i32.load8_u offset=1056332
                              i32.const 4
                              i32.and
                              br_if $#label6
                              block $#label12
                                block $#label13
                                  block $#label14
                                    local.get $#local2
                                    i32.eqz
                                    br_if $#label14
                                    i32.const 1056336
                                    local.set $#local4
                                    loop $#label15
                                      block $#label16
                                        local.get $#local4
                                        i32.load
                                        local.tee $#local3
                                        local.get $#local2
                                        i32.gt_u
                                        br_if $#label16
                                        local.get $#local3
                                        local.get $#local4
                                        i32.load offset=4
                                        i32.add
                                        local.get $#local2
                                        i32.gt_u
                                        br_if $#label13
                                      end
                                      local.get $#local4
                                      i32.load offset=8
                                      local.tee $#local4
                                      br_if $#label15
                                    end
                                  end
                                  i32.const 0
                                  call $sbrk
                                  local.tee $#local8
                                  i32.const -1
                                  i32.eq
                                  br_if $#label7
                                  local.get $#local9
                                  local.set $#local6
                                  block $#label14
                                    i32.const 0
                                    i32.load offset=1056364
                                    local.tee $#local4
                                    i32.const -1
                                    i32.add
                                    local.tee $#local3
                                    local.get $#local8
                                    i32.and
                                    i32.eqz
                                    br_if $#label14
                                    local.get $#local9
                                    local.get $#local8
                                    i32.sub
                                    local.get $#local3
                                    local.get $#local8
                                    i32.add
                                    i32.const 0
                                    local.get $#local4
                                    i32.sub
                                    i32.and
                                    i32.add
                                    local.set $#local6
                                  end
                                  local.get $#local6
                                  local.get $#local5
                                  i32.le_u
                                  br_if $#label7
                                  local.get $#local6
                                  i32.const 2147483646
                                  i32.gt_u
                                  br_if $#label7
                                  block $#label14
                                    i32.const 0
                                    i32.load offset=1056328
                                    local.tee $#local4
                                    i32.eqz
                                    br_if $#label14
                                    i32.const 0
                                    i32.load offset=1056320
                                    local.tee $#local3
                                    local.get $#local6
                                    i32.add
                                    local.tee $#local0
                                    local.get $#local3
                                    i32.le_u
                                    br_if $#label7
                                    local.get $#local0
                                    local.get $#local4
                                    i32.gt_u
                                    br_if $#label7
                                  end
                                  local.get $#local6
                                  call $sbrk
                                  local.tee $#local4
                                  local.get $#local8
                                  i32.ne
                                  br_if $#label12
                                  br $#label5
                                end
                                local.get $#local8
                                local.get $#local0
                                i32.sub
                                local.get $#local6
                                i32.and
                                local.tee $#local6
                                i32.const 2147483646
                                i32.gt_u
                                br_if $#label7
                                local.get $#local6
                                call $sbrk
                                local.tee $#local8
                                local.get $#local4
                                i32.load
                                local.get $#local4
                                i32.load offset=4
                                i32.add
                                i32.eq
                                br_if $#label8
                                local.get $#local8
                                local.set $#local4
                              end
                              block $#label12
                                local.get $#local6
                                local.get $#local5
                                i32.const 72
                                i32.add
                                i32.ge_u
                                br_if $#label12
                                local.get $#local4
                                i32.const -1
                                i32.eq
                                br_if $#label12
                                block $#label13
                                  local.get $#local11
                                  local.get $#local6
                                  i32.sub
                                  i32.const 0
                                  i32.load offset=1056368
                                  local.tee $#local3
                                  i32.add
                                  i32.const 0
                                  local.get $#local3
                                  i32.sub
                                  i32.and
                                  local.tee $#local3
                                  i32.const 2147483646
                                  i32.le_u
                                  br_if $#label13
                                  local.get $#local4
                                  local.set $#local8
                                  br $#label5
                                end
                                block $#label13
                                  local.get $#local3
                                  call $sbrk
                                  i32.const -1
                                  i32.eq
                                  br_if $#label13
                                  local.get $#local3
                                  local.get $#local6
                                  i32.add
                                  local.set $#local6
                                  local.get $#local4
                                  local.set $#local8
                                  br $#label5
                                end
                                i32.const 0
                                local.get $#local6
                                i32.sub
                                call $sbrk
                                drop
                                br $#label7
                              end
                              local.get $#local4
                              local.set $#local8
                              local.get $#local4
                              i32.const -1
                              i32.ne
                              br_if $#label5
                              br $#label7
                            end
                            unreachable
                          end
                          i32.const 0
                          local.set $#local4
                          br $#label1
                        end
                        i32.const 0
                        local.set $#local4
                        br $#label2
                      end
                      local.get $#local8
                      i32.const -1
                      i32.ne
                      br_if $#label5
                    end
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056332
                    i32.const 4
                    i32.or
                    i32.store offset=1056332
                  end
                  local.get $#local9
                  i32.const 2147483646
                  i32.gt_u
                  br_if $#label4
                  local.get $#local9
                  call $sbrk
                  local.set $#local8
                  i32.const 0
                  call $sbrk
                  local.set $#local4
                  local.get $#local8
                  i32.const -1
                  i32.eq
                  br_if $#label4
                  local.get $#local4
                  i32.const -1
                  i32.eq
                  br_if $#label4
                  local.get $#local8
                  local.get $#local4
                  i32.ge_u
                  br_if $#label4
                  local.get $#local4
                  local.get $#local8
                  i32.sub
                  local.tee $#local6
                  local.get $#local5
                  i32.const 56
                  i32.add
                  i32.le_u
                  br_if $#label4
                end
                i32.const 0
                i32.const 0
                i32.load offset=1056320
                local.get $#local6
                i32.add
                local.tee $#local4
                i32.store offset=1056320
                block $#label5
                  local.get $#local4
                  i32.const 0
                  i32.load offset=1056324
                  i32.le_u
                  br_if $#label5
                  i32.const 0
                  local.get $#local4
                  i32.store offset=1056324
                end
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        i32.const 0
                        i32.load offset=1055912
                        local.tee $#local3
                        i32.eqz
                        br_if $#label8
                        i32.const 1056336
                        local.set $#local4
                        loop $#label9
                          local.get $#local8
                          local.get $#local4
                          i32.load
                          local.tee $#local0
                          local.get $#local4
                          i32.load offset=4
                          local.tee $#local9
                          i32.add
                          i32.eq
                          br_if $#label7
                          local.get $#local4
                          i32.load offset=8
                          local.tee $#local4
                          br_if $#label9
                          br $#label6
                        end
                      end
                      block $#label8
                        block $#label9
                          i32.const 0
                          i32.load offset=1055904
                          local.tee $#local4
                          i32.eqz
                          br_if $#label9
                          local.get $#local8
                          local.get $#local4
                          i32.ge_u
                          br_if $#label8
                        end
                        i32.const 0
                        local.get $#local8
                        i32.store offset=1055904
                      end
                      i32.const 0
                      local.set $#local4
                      i32.const 0
                      local.get $#local6
                      i32.store offset=1056340
                      i32.const 0
                      local.get $#local8
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
                      loop $#label8
                        local.get $#local4
                        i32.const 1055948
                        i32.add
                        local.get $#local4
                        i32.const 1055936
                        i32.add
                        local.tee $#local3
                        i32.store
                        local.get $#local3
                        local.get $#local4
                        i32.const 1055928
                        i32.add
                        local.tee $#local0
                        i32.store
                        local.get $#local4
                        i32.const 1055940
                        i32.add
                        local.get $#local0
                        i32.store
                        local.get $#local4
                        i32.const 1055956
                        i32.add
                        local.get $#local4
                        i32.const 1055944
                        i32.add
                        local.tee $#local0
                        i32.store
                        local.get $#local0
                        local.get $#local3
                        i32.store
                        local.get $#local4
                        i32.const 1055964
                        i32.add
                        local.get $#local4
                        i32.const 1055952
                        i32.add
                        local.tee $#local3
                        i32.store
                        local.get $#local3
                        local.get $#local0
                        i32.store
                        local.get $#local4
                        i32.const 1055960
                        i32.add
                        local.get $#local3
                        i32.store
                        local.get $#local4
                        i32.const 32
                        i32.add
                        local.tee $#local4
                        i32.const 256
                        i32.ne
                        br_if $#label8
                      end
                      local.get $#local8
                      i32.const -8
                      local.get $#local8
                      i32.sub
                      i32.const 15
                      i32.and
                      local.tee $#local4
                      i32.add
                      local.tee $#local3
                      local.get $#local6
                      i32.const -56
                      i32.add
                      local.tee $#local0
                      local.get $#local4
                      i32.sub
                      local.tee $#local4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      i32.const 0
                      i32.const 0
                      i32.load offset=1056376
                      i32.store offset=1055916
                      i32.const 0
                      local.get $#local4
                      i32.store offset=1055900
                      i32.const 0
                      local.get $#local3
                      i32.store offset=1055912
                      local.get $#local8
                      local.get $#local0
                      i32.add
                      i32.const 56
                      i32.store offset=4
                      br $#label5
                    end
                    local.get $#local3
                    local.get $#local8
                    i32.ge_u
                    br_if $#label6
                    local.get $#local3
                    local.get $#local0
                    i32.lt_u
                    br_if $#label6
                    local.get $#local4
                    i32.load offset=12
                    i32.const 8
                    i32.and
                    br_if $#label6
                    local.get $#local3
                    i32.const -8
                    local.get $#local3
                    i32.sub
                    i32.const 15
                    i32.and
                    local.tee $#local0
                    i32.add
                    local.tee $#local8
                    i32.const 0
                    i32.load offset=1055900
                    local.get $#local6
                    i32.add
                    local.tee $#local11
                    local.get $#local0
                    i32.sub
                    local.tee $#local0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $#local4
                    local.get $#local9
                    local.get $#local6
                    i32.add
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056376
                    i32.store offset=1055916
                    i32.const 0
                    local.get $#local0
                    i32.store offset=1055900
                    i32.const 0
                    local.get $#local8
                    i32.store offset=1055912
                    local.get $#local3
                    local.get $#local11
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br $#label5
                  end
                  block $#label6
                    local.get $#local8
                    i32.const 0
                    i32.load offset=1055904
                    i32.ge_u
                    br_if $#label6
                    i32.const 0
                    local.get $#local8
                    i32.store offset=1055904
                  end
                  local.get $#local8
                  local.get $#local6
                  i32.add
                  local.set $#local0
                  i32.const 1056336
                  local.set $#local4
                  block $#label6
                    block $#label7
                      loop $#label8
                        local.get $#local4
                        i32.load
                        local.tee $#local9
                        local.get $#local0
                        i32.eq
                        br_if $#label7
                        local.get $#local4
                        i32.load offset=8
                        local.tee $#local4
                        br_if $#label8
                        br $#label6
                      end
                    end
                    local.get $#local4
                    i32.load8_u offset=12
                    i32.const 8
                    i32.and
                    i32.eqz
                    br_if $#label3
                  end
                  i32.const 1056336
                  local.set $#local4
                  block $#label6
                    loop $#label7
                      block $#label8
                        local.get $#local4
                        i32.load
                        local.tee $#local0
                        local.get $#local3
                        i32.gt_u
                        br_if $#label8
                        local.get $#local0
                        local.get $#local4
                        i32.load offset=4
                        i32.add
                        local.tee $#local0
                        local.get $#local3
                        i32.gt_u
                        br_if $#label6
                      end
                      local.get $#local4
                      i32.load offset=8
                      local.set $#local4
                      br $#label7
                    end
                  end
                  local.get $#local8
                  i32.const -8
                  local.get $#local8
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee $#local4
                  i32.add
                  local.tee $#local11
                  local.get $#local6
                  i32.const -56
                  i32.add
                  local.tee $#local9
                  local.get $#local4
                  i32.sub
                  local.tee $#local4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get $#local8
                  local.get $#local9
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  local.get $#local3
                  local.get $#local0
                  i32.const 55
                  local.get $#local0
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.add
                  i32.const -63
                  i32.add
                  local.tee $#local9
                  local.get $#local9
                  local.get $#local3
                  i32.const 16
                  i32.add
                  i32.lt_u
                  select
                  local.tee $#local9
                  i32.const 35
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056376
                  i32.store offset=1055916
                  i32.const 0
                  local.get $#local4
                  i32.store offset=1055900
                  i32.const 0
                  local.get $#local11
                  i32.store offset=1055912
                  local.get $#local9
                  i32.const 16
                  i32.add
                  i32.const 0
                  i64.load offset=1056344 align=4
                  i64.store align=4
                  local.get $#local9
                  i32.const 0
                  i64.load offset=1056336 align=4
                  i64.store offset=8 align=4
                  i32.const 0
                  local.get $#local9
                  i32.const 8
                  i32.add
                  i32.store offset=1056344
                  i32.const 0
                  local.get $#local6
                  i32.store offset=1056340
                  i32.const 0
                  local.get $#local8
                  i32.store offset=1056336
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056348
                  local.get $#local9
                  i32.const 36
                  i32.add
                  local.set $#local4
                  loop $#label6
                    local.get $#local4
                    i32.const 7
                    i32.store
                    local.get $#local4
                    i32.const 4
                    i32.add
                    local.tee $#local4
                    local.get $#local0
                    i32.lt_u
                    br_if $#label6
                  end
                  local.get $#local9
                  local.get $#local3
                  i32.eq
                  br_if $#label5
                  local.get $#local9
                  local.get $#local9
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get $#local9
                  local.get $#local9
                  local.get $#local3
                  i32.sub
                  local.tee $#local8
                  i32.store
                  local.get $#local3
                  local.get $#local8
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  block $#label6
                    block $#label7
                      local.get $#local8
                      i32.const 255
                      i32.gt_u
                      br_if $#label7
                      local.get $#local8
                      i32.const -8
                      i32.and
                      i32.const 1055928
                      i32.add
                      local.set $#local4
                      block $#label8
                        block $#label9
                          i32.const 0
                          i32.load offset=1055888
                          local.tee $#local0
                          i32.const 1
                          local.get $#local8
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee $#local8
                          i32.and
                          br_if $#label9
                          i32.const 0
                          local.get $#local0
                          local.get $#local8
                          i32.or
                          i32.store offset=1055888
                          local.get $#local4
                          local.set $#local0
                          br $#label8
                        end
                        local.get $#local4
                        i32.load offset=8
                        local.set $#local0
                      end
                      local.get $#local0
                      local.get $#local3
                      i32.store offset=12
                      local.get $#local4
                      local.get $#local3
                      i32.store offset=8
                      i32.const 12
                      local.set $#local8
                      i32.const 8
                      local.set $#local9
                      br $#label6
                    end
                    i32.const 31
                    local.set $#local4
                    block $#label7
                      local.get $#local8
                      i32.const 16777215
                      i32.gt_u
                      br_if $#label7
                      local.get $#local8
                      i32.const 38
                      local.get $#local8
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee $#local4
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get $#local4
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set $#local4
                    end
                    local.get $#local3
                    local.get $#local4
                    i32.store offset=28
                    local.get $#local3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get $#local4
                    i32.const 2
                    i32.shl
                    i32.const 1056192
                    i32.add
                    local.set $#local0
                    block $#label7
                      block $#label8
                        block $#label9
                          i32.const 0
                          i32.load offset=1055892
                          local.tee $#local9
                          i32.const 1
                          local.get $#local4
                          i32.shl
                          local.tee $#local6
                          i32.and
                          br_if $#label9
                          local.get $#local0
                          local.get $#local3
                          i32.store
                          i32.const 0
                          local.get $#local9
                          local.get $#local6
                          i32.or
                          i32.store offset=1055892
                          local.get $#local3
                          local.get $#local0
                          i32.store offset=24
                          br $#label8
                        end
                        local.get $#local8
                        i32.const 0
                        i32.const 25
                        local.get $#local4
                        i32.const 1
                        i32.shr_u
                        i32.sub
                        local.get $#local4
                        i32.const 31
                        i32.eq
                        select
                        i32.shl
                        local.set $#local4
                        local.get $#local0
                        i32.load
                        local.set $#local9
                        loop $#label9
                          local.get $#local9
                          local.tee $#local0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get $#local8
                          i32.eq
                          br_if $#label7
                          local.get $#local4
                          i32.const 29
                          i32.shr_u
                          local.set $#local9
                          local.get $#local4
                          i32.const 1
                          i32.shl
                          local.set $#local4
                          local.get $#local0
                          local.get $#local9
                          i32.const 4
                          i32.and
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee $#local6
                          i32.load
                          local.tee $#local9
                          br_if $#label9
                        end
                        local.get $#local6
                        local.get $#local3
                        i32.store
                        local.get $#local3
                        local.get $#local0
                        i32.store offset=24
                      end
                      i32.const 8
                      local.set $#local8
                      i32.const 12
                      local.set $#local9
                      local.get $#local3
                      local.set $#local0
                      local.get $#local3
                      local.set $#local4
                      br $#label6
                    end
                    local.get $#local0
                    i32.load offset=8
                    local.set $#local4
                    local.get $#local0
                    local.get $#local3
                    i32.store offset=8
                    local.get $#local4
                    local.get $#local3
                    i32.store offset=12
                    local.get $#local3
                    local.get $#local4
                    i32.store offset=8
                    i32.const 0
                    local.set $#local4
                    i32.const 24
                    local.set $#local8
                    i32.const 12
                    local.set $#local9
                  end
                  local.get $#local3
                  local.get $#local9
                  i32.add
                  local.get $#local0
                  i32.store
                  local.get $#local3
                  local.get $#local8
                  i32.add
                  local.get $#local4
                  i32.store
                end
                i32.const 0
                i32.load offset=1055900
                local.tee $#local4
                local.get $#local5
                i32.le_u
                br_if $#label4
                i32.const 0
                i32.load offset=1055912
                local.tee $#local3
                local.get $#local5
                i32.add
                local.tee $#local0
                local.get $#local4
                local.get $#local5
                i32.sub
                local.tee $#local4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.get $#local4
                i32.store offset=1055900
                i32.const 0
                local.get $#local0
                i32.store offset=1055912
                local.get $#local3
                local.get $#local5
                i32.const 3
                i32.or
                i32.store offset=4
                local.get $#local3
                i32.const 8
                i32.add
                local.set $#local4
                br $#label0
              end
              i32.const 0
              local.set $#local4
              i32.const 0
              i32.const 48
              i32.store offset=1056384
              br $#label0
            end
            local.get $#local4
            local.get $#local8
            i32.store
            local.get $#local4
            local.get $#local4
            i32.load offset=4
            local.get $#local6
            i32.add
            i32.store offset=4
            local.get $#local8
            local.get $#local9
            local.get $#local5
            call $prepend_alloc
            local.set $#local4
            br $#label0
          end
          block $#label2
            local.get $#local11
            i32.eqz
            br_if $#label2
            block $#label3
              block $#label4
                local.get $#local9
                local.get $#local9
                i32.load offset=28
                local.tee $#local8
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee $#local0
                i32.load
                i32.ne
                br_if $#label4
                local.get $#local0
                local.get $#local4
                i32.store
                local.get $#local4
                br_if $#label3
                i32.const 0
                local.get $#local10
                i32.const -2
                local.get $#local8
                i32.rotl
                i32.and
                local.tee $#local10
                i32.store offset=1055892
                br $#label2
              end
              local.get $#local11
              i32.const 16
              i32.const 20
              local.get $#local11
              i32.load offset=16
              local.get $#local9
              i32.eq
              select
              i32.add
              local.get $#local4
              i32.store
              local.get $#local4
              i32.eqz
              br_if $#label2
            end
            local.get $#local4
            local.get $#local11
            i32.store offset=24
            block $#label3
              local.get $#local9
              i32.load offset=16
              local.tee $#local0
              i32.eqz
              br_if $#label3
              local.get $#local4
              local.get $#local0
              i32.store offset=16
              local.get $#local0
              local.get $#local4
              i32.store offset=24
            end
            local.get $#local9
            i32.load offset=20
            local.tee $#local0
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local0
            i32.store offset=20
            local.get $#local0
            local.get $#local4
            i32.store offset=24
          end
          block $#label2
            block $#label3
              local.get $#local3
              i32.const 15
              i32.gt_u
              br_if $#label3
              local.get $#local9
              local.get $#local3
              local.get $#local5
              i32.or
              local.tee $#local4
              i32.const 3
              i32.or
              i32.store offset=4
              local.get $#local9
              local.get $#local4
              i32.add
              local.tee $#local4
              local.get $#local4
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br $#label2
            end
            local.get $#local9
            local.get $#local5
            i32.add
            local.tee $#local8
            local.get $#local3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $#local9
            local.get $#local5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $#local8
            local.get $#local3
            i32.add
            local.get $#local3
            i32.store
            block $#label3
              local.get $#local3
              i32.const 255
              i32.gt_u
              br_if $#label3
              local.get $#local3
              i32.const -8
              i32.and
              i32.const 1055928
              i32.add
              local.set $#local4
              block $#label4
                block $#label5
                  i32.const 0
                  i32.load offset=1055888
                  local.tee $#local5
                  i32.const 1
                  local.get $#local3
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee $#local3
                  i32.and
                  br_if $#label5
                  i32.const 0
                  local.get $#local5
                  local.get $#local3
                  i32.or
                  i32.store offset=1055888
                  local.get $#local4
                  local.set $#local3
                  br $#label4
                end
                local.get $#local4
                i32.load offset=8
                local.set $#local3
              end
              local.get $#local3
              local.get $#local8
              i32.store offset=12
              local.get $#local4
              local.get $#local8
              i32.store offset=8
              local.get $#local8
              local.get $#local4
              i32.store offset=12
              local.get $#local8
              local.get $#local3
              i32.store offset=8
              br $#label2
            end
            i32.const 31
            local.set $#local4
            block $#label3
              local.get $#local3
              i32.const 16777215
              i32.gt_u
              br_if $#label3
              local.get $#local3
              i32.const 38
              local.get $#local3
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee $#local4
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get $#local4
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set $#local4
            end
            local.get $#local8
            local.get $#local4
            i32.store offset=28
            local.get $#local8
            i64.const 0
            i64.store offset=16 align=4
            local.get $#local4
            i32.const 2
            i32.shl
            i32.const 1056192
            i32.add
            local.set $#local5
            block $#label3
              local.get $#local10
              i32.const 1
              local.get $#local4
              i32.shl
              local.tee $#local0
              i32.and
              br_if $#label3
              local.get $#local5
              local.get $#local8
              i32.store
              i32.const 0
              local.get $#local10
              local.get $#local0
              i32.or
              i32.store offset=1055892
              local.get $#local8
              local.get $#local5
              i32.store offset=24
              local.get $#local8
              local.get $#local8
              i32.store offset=8
              local.get $#local8
              local.get $#local8
              i32.store offset=12
              br $#label2
            end
            local.get $#local3
            i32.const 0
            i32.const 25
            local.get $#local4
            i32.const 1
            i32.shr_u
            i32.sub
            local.get $#local4
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set $#local4
            local.get $#local5
            i32.load
            local.set $#local0
            block $#label3
              loop $#label4
                local.get $#local0
                local.tee $#local5
                i32.load offset=4
                i32.const -8
                i32.and
                local.get $#local3
                i32.eq
                br_if $#label3
                local.get $#local4
                i32.const 29
                i32.shr_u
                local.set $#local0
                local.get $#local4
                i32.const 1
                i32.shl
                local.set $#local4
                local.get $#local5
                local.get $#local0
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee $#local6
                i32.load
                local.tee $#local0
                br_if $#label4
              end
              local.get $#local6
              local.get $#local8
              i32.store
              local.get $#local8
              local.get $#local5
              i32.store offset=24
              local.get $#local8
              local.get $#local8
              i32.store offset=12
              local.get $#local8
              local.get $#local8
              i32.store offset=8
              br $#label2
            end
            local.get $#local5
            i32.load offset=8
            local.tee $#local4
            local.get $#local8
            i32.store offset=12
            local.get $#local5
            local.get $#local8
            i32.store offset=8
            local.get $#local8
            i32.const 0
            i32.store offset=24
            local.get $#local8
            local.get $#local5
            i32.store offset=12
            local.get $#local8
            local.get $#local4
            i32.store offset=8
          end
          local.get $#local9
          i32.const 8
          i32.add
          local.set $#local4
          br $#label0
        end
        block $#label1
          local.get $#local2
          i32.eqz
          br_if $#label1
          block $#label2
            block $#label3
              local.get $#local8
              local.get $#local8
              i32.load offset=28
              local.tee $#local9
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee $#local0
              i32.load
              i32.ne
              br_if $#label3
              local.get $#local0
              local.get $#local4
              i32.store
              local.get $#local4
              br_if $#label2
              i32.const 0
              local.get $#local10
              i32.const -2
              local.get $#local9
              i32.rotl
              i32.and
              i32.store offset=1055892
              br $#label1
            end
            local.get $#local2
            i32.const 16
            i32.const 20
            local.get $#local2
            i32.load offset=16
            local.get $#local8
            i32.eq
            select
            i32.add
            local.get $#local4
            i32.store
            local.get $#local4
            i32.eqz
            br_if $#label1
          end
          local.get $#local4
          local.get $#local2
          i32.store offset=24
          block $#label2
            local.get $#local8
            i32.load offset=16
            local.tee $#local0
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local0
            i32.store offset=16
            local.get $#local0
            local.get $#local4
            i32.store offset=24
          end
          local.get $#local8
          i32.load offset=20
          local.tee $#local0
          i32.eqz
          br_if $#label1
          local.get $#local4
          local.get $#local0
          i32.store offset=20
          local.get $#local0
          local.get $#local4
          i32.store offset=24
        end
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 15
            i32.gt_u
            br_if $#label2
            local.get $#local8
            local.get $#local3
            local.get $#local5
            i32.or
            local.tee $#local4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $#local8
            local.get $#local4
            i32.add
            local.tee $#local4
            local.get $#local4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br $#label1
          end
          local.get $#local8
          local.get $#local5
          i32.add
          local.tee $#local0
          local.get $#local3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $#local8
          local.get $#local5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $#local0
          local.get $#local3
          i32.add
          local.get $#local3
          i32.store
          block $#label2
            local.get $#local7
            i32.eqz
            br_if $#label2
            local.get $#local7
            i32.const -8
            i32.and
            i32.const 1055928
            i32.add
            local.set $#local5
            i32.const 0
            i32.load offset=1055908
            local.set $#local4
            block $#label3
              block $#label4
                i32.const 1
                local.get $#local7
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee $#local9
                local.get $#local6
                i32.and
                br_if $#label4
                i32.const 0
                local.get $#local9
                local.get $#local6
                i32.or
                i32.store offset=1055888
                local.get $#local5
                local.set $#local9
                br $#label3
              end
              local.get $#local5
              i32.load offset=8
              local.set $#local9
            end
            local.get $#local9
            local.get $#local4
            i32.store offset=12
            local.get $#local5
            local.get $#local4
            i32.store offset=8
            local.get $#local4
            local.get $#local5
            i32.store offset=12
            local.get $#local4
            local.get $#local9
            i32.store offset=8
          end
          i32.const 0
          local.get $#local0
          i32.store offset=1055908
          i32.const 0
          local.get $#local3
          i32.store offset=1055896
        end
        local.get $#local8
        i32.const 8
        i32.add
        local.set $#local4
      end
      local.get $#local1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local4
    )
    (func $prepend_alloc (;121;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      local.get $#local0
      i32.const -8
      local.get $#local0
      i32.sub
      i32.const 15
      i32.and
      i32.add
      local.tee $#local3
      local.get $#local2
      i32.const 3
      i32.or
      i32.store offset=4
      local.get $#local1
      i32.const -8
      local.get $#local1
      i32.sub
      i32.const 15
      i32.and
      i32.add
      local.tee $#local4
      local.get $#local3
      local.get $#local2
      i32.add
      local.tee $#local5
      i32.sub
      local.set $#local0
      block $#label0
        block $#label1
          local.get $#local4
          i32.const 0
          i32.load offset=1055912
          i32.ne
          br_if $#label1
          i32.const 0
          local.get $#local5
          i32.store offset=1055912
          i32.const 0
          i32.const 0
          i32.load offset=1055900
          local.get $#local0
          i32.add
          local.tee $#local2
          i32.store offset=1055900
          local.get $#local5
          local.get $#local2
          i32.const 1
          i32.or
          i32.store offset=4
          br $#label0
        end
        block $#label1
          local.get $#local4
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if $#label1
          i32.const 0
          local.get $#local5
          i32.store offset=1055908
          i32.const 0
          i32.const 0
          i32.load offset=1055896
          local.get $#local0
          i32.add
          local.tee $#local2
          i32.store offset=1055896
          local.get $#local5
          local.get $#local2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $#local5
          local.get $#local2
          i32.add
          local.get $#local2
          i32.store
          br $#label0
        end
        block $#label1
          local.get $#local4
          i32.load offset=4
          local.tee $#local1
          i32.const 3
          i32.and
          i32.const 1
          i32.ne
          br_if $#label1
          local.get $#local1
          i32.const -8
          i32.and
          local.set $#local6
          local.get $#local4
          i32.load offset=12
          local.set $#local2
          block $#label2
            block $#label3
              local.get $#local1
              i32.const 255
              i32.gt_u
              br_if $#label3
              block $#label4
                local.get $#local2
                local.get $#local4
                i32.load offset=8
                local.tee $#local7
                i32.ne
                br_if $#label4
                i32.const 0
                i32.const 0
                i32.load offset=1055888
                i32.const -2
                local.get $#local1
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=1055888
                br $#label2
              end
              local.get $#local2
              local.get $#local7
              i32.store offset=8
              local.get $#local7
              local.get $#local2
              i32.store offset=12
              br $#label2
            end
            local.get $#local4
            i32.load offset=24
            local.set $#local8
            block $#label3
              block $#label4
                local.get $#local2
                local.get $#local4
                i32.eq
                br_if $#label4
                local.get $#local4
                i32.load offset=8
                local.tee $#local1
                local.get $#local2
                i32.store offset=12
                local.get $#local2
                local.get $#local1
                i32.store offset=8
                br $#label3
              end
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local4
                    i32.load offset=20
                    local.tee $#local1
                    i32.eqz
                    br_if $#label6
                    local.get $#local4
                    i32.const 20
                    i32.add
                    local.set $#local7
                    br $#label5
                  end
                  local.get $#local4
                  i32.load offset=16
                  local.tee $#local1
                  i32.eqz
                  br_if $#label4
                  local.get $#local4
                  i32.const 16
                  i32.add
                  local.set $#local7
                end
                loop $#label5
                  local.get $#local7
                  local.set $#local9
                  local.get $#local1
                  local.tee $#local2
                  i32.const 20
                  i32.add
                  local.set $#local7
                  local.get $#local2
                  i32.load offset=20
                  local.tee $#local1
                  br_if $#label5
                  local.get $#local2
                  i32.const 16
                  i32.add
                  local.set $#local7
                  local.get $#local2
                  i32.load offset=16
                  local.tee $#local1
                  br_if $#label5
                end
                local.get $#local9
                i32.const 0
                i32.store
                br $#label3
              end
              i32.const 0
              local.set $#local2
            end
            local.get $#local8
            i32.eqz
            br_if $#label2
            block $#label3
              block $#label4
                local.get $#local4
                local.get $#local4
                i32.load offset=28
                local.tee $#local7
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee $#local1
                i32.load
                i32.ne
                br_if $#label4
                local.get $#local1
                local.get $#local2
                i32.store
                local.get $#local2
                br_if $#label3
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get $#local7
                i32.rotl
                i32.and
                i32.store offset=1055892
                br $#label2
              end
              local.get $#local8
              i32.const 16
              i32.const 20
              local.get $#local8
              i32.load offset=16
              local.get $#local4
              i32.eq
              select
              i32.add
              local.get $#local2
              i32.store
              local.get $#local2
              i32.eqz
              br_if $#label2
            end
            local.get $#local2
            local.get $#local8
            i32.store offset=24
            block $#label3
              local.get $#local4
              i32.load offset=16
              local.tee $#local1
              i32.eqz
              br_if $#label3
              local.get $#local2
              local.get $#local1
              i32.store offset=16
              local.get $#local1
              local.get $#local2
              i32.store offset=24
            end
            local.get $#local4
            i32.load offset=20
            local.tee $#local1
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local1
            i32.store offset=20
            local.get $#local1
            local.get $#local2
            i32.store offset=24
          end
          local.get $#local6
          local.get $#local0
          i32.add
          local.set $#local0
          local.get $#local4
          local.get $#local6
          i32.add
          local.tee $#local4
          i32.load offset=4
          local.set $#local1
        end
        local.get $#local4
        local.get $#local1
        i32.const -2
        i32.and
        i32.store offset=4
        local.get $#local5
        local.get $#local0
        i32.add
        local.get $#local0
        i32.store
        local.get $#local5
        local.get $#local0
        i32.const 1
        i32.or
        i32.store offset=4
        block $#label1
          local.get $#local0
          i32.const 255
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set $#local2
          block $#label2
            block $#label3
              i32.const 0
              i32.load offset=1055888
              local.tee $#local1
              i32.const 1
              local.get $#local0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee $#local0
              i32.and
              br_if $#label3
              i32.const 0
              local.get $#local1
              local.get $#local0
              i32.or
              i32.store offset=1055888
              local.get $#local2
              local.set $#local0
              br $#label2
            end
            local.get $#local2
            i32.load offset=8
            local.set $#local0
          end
          local.get $#local0
          local.get $#local5
          i32.store offset=12
          local.get $#local2
          local.get $#local5
          i32.store offset=8
          local.get $#local5
          local.get $#local2
          i32.store offset=12
          local.get $#local5
          local.get $#local0
          i32.store offset=8
          br $#label0
        end
        i32.const 31
        local.set $#local2
        block $#label1
          local.get $#local0
          i32.const 16777215
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const 38
          local.get $#local0
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee $#local2
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get $#local2
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set $#local2
        end
        local.get $#local5
        local.get $#local2
        i32.store offset=28
        local.get $#local5
        i64.const 0
        i64.store offset=16 align=4
        local.get $#local2
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set $#local1
        block $#label1
          i32.const 0
          i32.load offset=1055892
          local.tee $#local7
          i32.const 1
          local.get $#local2
          i32.shl
          local.tee $#local4
          i32.and
          br_if $#label1
          local.get $#local1
          local.get $#local5
          i32.store
          i32.const 0
          local.get $#local7
          local.get $#local4
          i32.or
          i32.store offset=1055892
          local.get $#local5
          local.get $#local1
          i32.store offset=24
          local.get $#local5
          local.get $#local5
          i32.store offset=8
          local.get $#local5
          local.get $#local5
          i32.store offset=12
          br $#label0
        end
        local.get $#local0
        i32.const 0
        i32.const 25
        local.get $#local2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get $#local2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set $#local2
        local.get $#local1
        i32.load
        local.set $#local7
        block $#label1
          loop $#label2
            local.get $#local7
            local.tee $#local1
            i32.load offset=4
            i32.const -8
            i32.and
            local.get $#local0
            i32.eq
            br_if $#label1
            local.get $#local2
            i32.const 29
            i32.shr_u
            local.set $#local7
            local.get $#local2
            i32.const 1
            i32.shl
            local.set $#local2
            local.get $#local1
            local.get $#local7
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee $#local4
            i32.load
            local.tee $#local7
            br_if $#label2
          end
          local.get $#local4
          local.get $#local5
          i32.store
          local.get $#local5
          local.get $#local1
          i32.store offset=24
          local.get $#local5
          local.get $#local5
          i32.store offset=12
          local.get $#local5
          local.get $#local5
          i32.store offset=8
          br $#label0
        end
        local.get $#local1
        i32.load offset=8
        local.tee $#local2
        local.get $#local5
        i32.store offset=12
        local.get $#local1
        local.get $#local5
        i32.store offset=8
        local.get $#local5
        i32.const 0
        i32.store offset=24
        local.get $#local5
        local.get $#local1
        i32.store offset=12
        local.get $#local5
        local.get $#local2
        i32.store offset=8
      end
      local.get $#local3
      i32.const 8
      i32.add
    )
    (func $free (;122;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $dlfree
    )
    (func $dlfree (;123;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      block $#label0
        local.get $#local0
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.const -8
        i32.add
        local.tee $#local1
        local.get $#local0
        i32.const -4
        i32.add
        i32.load
        local.tee $#local2
        i32.const -8
        i32.and
        local.tee $#local0
        i32.add
        local.set $#local3
        block $#label1
          local.get $#local2
          i32.const 1
          i32.and
          br_if $#label1
          local.get $#local2
          i32.const 2
          i32.and
          i32.eqz
          br_if $#label0
          local.get $#local1
          local.get $#local1
          i32.load
          local.tee $#local4
          i32.sub
          local.tee $#local1
          i32.const 0
          i32.load offset=1055904
          i32.lt_u
          br_if $#label0
          local.get $#local4
          local.get $#local0
          i32.add
          local.set $#local0
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local1
                  i32.const 0
                  i32.load offset=1055908
                  i32.eq
                  br_if $#label5
                  local.get $#local1
                  i32.load offset=12
                  local.set $#local2
                  block $#label6
                    local.get $#local4
                    i32.const 255
                    i32.gt_u
                    br_if $#label6
                    local.get $#local2
                    local.get $#local1
                    i32.load offset=8
                    local.tee $#local5
                    i32.ne
                    br_if $#label4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055888
                    i32.const -2
                    local.get $#local4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1055888
                    br $#label1
                  end
                  local.get $#local1
                  i32.load offset=24
                  local.set $#local6
                  block $#label6
                    local.get $#local2
                    local.get $#local1
                    i32.eq
                    br_if $#label6
                    local.get $#local1
                    i32.load offset=8
                    local.tee $#local4
                    local.get $#local2
                    i32.store offset=12
                    local.get $#local2
                    local.get $#local4
                    i32.store offset=8
                    br $#label2
                  end
                  block $#label6
                    block $#label7
                      local.get $#local1
                      i32.load offset=20
                      local.tee $#local4
                      i32.eqz
                      br_if $#label7
                      local.get $#local1
                      i32.const 20
                      i32.add
                      local.set $#local5
                      br $#label6
                    end
                    local.get $#local1
                    i32.load offset=16
                    local.tee $#local4
                    i32.eqz
                    br_if $#label3
                    local.get $#local1
                    i32.const 16
                    i32.add
                    local.set $#local5
                  end
                  loop $#label6
                    local.get $#local5
                    local.set $#local7
                    local.get $#local4
                    local.tee $#local2
                    i32.const 20
                    i32.add
                    local.set $#local5
                    local.get $#local2
                    i32.load offset=20
                    local.tee $#local4
                    br_if $#label6
                    local.get $#local2
                    i32.const 16
                    i32.add
                    local.set $#local5
                    local.get $#local2
                    i32.load offset=16
                    local.tee $#local4
                    br_if $#label6
                  end
                  local.get $#local7
                  i32.const 0
                  i32.store
                  br $#label2
                end
                local.get $#local3
                i32.load offset=4
                local.tee $#local2
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if $#label1
                local.get $#local3
                local.get $#local2
                i32.const -2
                i32.and
                i32.store offset=4
                i32.const 0
                local.get $#local0
                i32.store offset=1055896
                local.get $#local3
                local.get $#local0
                i32.store
                local.get $#local1
                local.get $#local0
                i32.const 1
                i32.or
                i32.store offset=4
                return
              end
              local.get $#local2
              local.get $#local5
              i32.store offset=8
              local.get $#local5
              local.get $#local2
              i32.store offset=12
              br $#label1
            end
            i32.const 0
            local.set $#local2
          end
          local.get $#local6
          i32.eqz
          br_if $#label1
          block $#label2
            block $#label3
              local.get $#local1
              local.get $#local1
              i32.load offset=28
              local.tee $#local5
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee $#local4
              i32.load
              i32.ne
              br_if $#label3
              local.get $#local4
              local.get $#local2
              i32.store
              local.get $#local2
              br_if $#label2
              i32.const 0
              i32.const 0
              i32.load offset=1055892
              i32.const -2
              local.get $#local5
              i32.rotl
              i32.and
              i32.store offset=1055892
              br $#label1
            end
            local.get $#local6
            i32.const 16
            i32.const 20
            local.get $#local6
            i32.load offset=16
            local.get $#local1
            i32.eq
            select
            i32.add
            local.get $#local2
            i32.store
            local.get $#local2
            i32.eqz
            br_if $#label1
          end
          local.get $#local2
          local.get $#local6
          i32.store offset=24
          block $#label2
            local.get $#local1
            i32.load offset=16
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local4
            i32.store offset=16
            local.get $#local4
            local.get $#local2
            i32.store offset=24
          end
          local.get $#local1
          i32.load offset=20
          local.tee $#local4
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local4
          i32.store offset=20
          local.get $#local4
          local.get $#local2
          i32.store offset=24
        end
        local.get $#local1
        local.get $#local3
        i32.ge_u
        br_if $#label0
        local.get $#local3
        i32.load offset=4
        local.tee $#local4
        i32.const 1
        i32.and
        i32.eqz
        br_if $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local4
                  i32.const 2
                  i32.and
                  br_if $#label5
                  block $#label6
                    local.get $#local3
                    i32.const 0
                    i32.load offset=1055912
                    i32.ne
                    br_if $#label6
                    i32.const 0
                    local.get $#local1
                    i32.store offset=1055912
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055900
                    local.get $#local0
                    i32.add
                    local.tee $#local0
                    i32.store offset=1055900
                    local.get $#local1
                    local.get $#local0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $#local1
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if $#label0
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055896
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055908
                    return
                  end
                  block $#label6
                    local.get $#local3
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if $#label6
                    i32.const 0
                    local.get $#local1
                    i32.store offset=1055908
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055896
                    local.get $#local0
                    i32.add
                    local.tee $#local0
                    i32.store offset=1055896
                    local.get $#local1
                    local.get $#local0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $#local1
                    local.get $#local0
                    i32.add
                    local.get $#local0
                    i32.store
                    return
                  end
                  local.get $#local4
                  i32.const -8
                  i32.and
                  local.get $#local0
                  i32.add
                  local.set $#local0
                  local.get $#local3
                  i32.load offset=12
                  local.set $#local2
                  block $#label6
                    local.get $#local4
                    i32.const 255
                    i32.gt_u
                    br_if $#label6
                    block $#label7
                      local.get $#local2
                      local.get $#local3
                      i32.load offset=8
                      local.tee $#local5
                      i32.ne
                      br_if $#label7
                      i32.const 0
                      i32.const 0
                      i32.load offset=1055888
                      i32.const -2
                      local.get $#local4
                      i32.const 3
                      i32.shr_u
                      i32.rotl
                      i32.and
                      i32.store offset=1055888
                      br $#label2
                    end
                    local.get $#local2
                    local.get $#local5
                    i32.store offset=8
                    local.get $#local5
                    local.get $#local2
                    i32.store offset=12
                    br $#label2
                  end
                  local.get $#local3
                  i32.load offset=24
                  local.set $#local6
                  block $#label6
                    local.get $#local2
                    local.get $#local3
                    i32.eq
                    br_if $#label6
                    local.get $#local3
                    i32.load offset=8
                    local.tee $#local4
                    local.get $#local2
                    i32.store offset=12
                    local.get $#local2
                    local.get $#local4
                    i32.store offset=8
                    br $#label3
                  end
                  block $#label6
                    block $#label7
                      local.get $#local3
                      i32.load offset=20
                      local.tee $#local4
                      i32.eqz
                      br_if $#label7
                      local.get $#local3
                      i32.const 20
                      i32.add
                      local.set $#local5
                      br $#label6
                    end
                    local.get $#local3
                    i32.load offset=16
                    local.tee $#local4
                    i32.eqz
                    br_if $#label4
                    local.get $#local3
                    i32.const 16
                    i32.add
                    local.set $#local5
                  end
                  loop $#label6
                    local.get $#local5
                    local.set $#local7
                    local.get $#local4
                    local.tee $#local2
                    i32.const 20
                    i32.add
                    local.set $#local5
                    local.get $#local2
                    i32.load offset=20
                    local.tee $#local4
                    br_if $#label6
                    local.get $#local2
                    i32.const 16
                    i32.add
                    local.set $#local5
                    local.get $#local2
                    i32.load offset=16
                    local.tee $#local4
                    br_if $#label6
                  end
                  local.get $#local7
                  i32.const 0
                  i32.store
                  br $#label3
                end
                local.get $#local3
                local.get $#local4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get $#local1
                local.get $#local0
                i32.add
                local.get $#local0
                i32.store
                local.get $#local1
                local.get $#local0
                i32.const 1
                i32.or
                i32.store offset=4
                br $#label1
              end
              i32.const 0
              local.set $#local2
            end
            local.get $#local6
            i32.eqz
            br_if $#label2
            block $#label3
              block $#label4
                local.get $#local3
                local.get $#local3
                i32.load offset=28
                local.tee $#local5
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee $#local4
                i32.load
                i32.ne
                br_if $#label4
                local.get $#local4
                local.get $#local2
                i32.store
                local.get $#local2
                br_if $#label3
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get $#local5
                i32.rotl
                i32.and
                i32.store offset=1055892
                br $#label2
              end
              local.get $#local6
              i32.const 16
              i32.const 20
              local.get $#local6
              i32.load offset=16
              local.get $#local3
              i32.eq
              select
              i32.add
              local.get $#local2
              i32.store
              local.get $#local2
              i32.eqz
              br_if $#label2
            end
            local.get $#local2
            local.get $#local6
            i32.store offset=24
            block $#label3
              local.get $#local3
              i32.load offset=16
              local.tee $#local4
              i32.eqz
              br_if $#label3
              local.get $#local2
              local.get $#local4
              i32.store offset=16
              local.get $#local4
              local.get $#local2
              i32.store offset=24
            end
            local.get $#local3
            i32.load offset=20
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local2
            local.get $#local4
            i32.store offset=20
            local.get $#local4
            local.get $#local2
            i32.store offset=24
          end
          local.get $#local1
          local.get $#local0
          i32.add
          local.get $#local0
          i32.store
          local.get $#local1
          local.get $#local0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $#local1
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if $#label1
          i32.const 0
          local.get $#local0
          i32.store offset=1055896
          return
        end
        block $#label1
          local.get $#local0
          i32.const 255
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set $#local2
          block $#label2
            block $#label3
              i32.const 0
              i32.load offset=1055888
              local.tee $#local4
              i32.const 1
              local.get $#local0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee $#local0
              i32.and
              br_if $#label3
              i32.const 0
              local.get $#local4
              local.get $#local0
              i32.or
              i32.store offset=1055888
              local.get $#local2
              local.set $#local0
              br $#label2
            end
            local.get $#local2
            i32.load offset=8
            local.set $#local0
          end
          local.get $#local0
          local.get $#local1
          i32.store offset=12
          local.get $#local2
          local.get $#local1
          i32.store offset=8
          local.get $#local1
          local.get $#local2
          i32.store offset=12
          local.get $#local1
          local.get $#local0
          i32.store offset=8
          return
        end
        i32.const 31
        local.set $#local2
        block $#label1
          local.get $#local0
          i32.const 16777215
          i32.gt_u
          br_if $#label1
          local.get $#local0
          i32.const 38
          local.get $#local0
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee $#local2
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get $#local2
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set $#local2
        end
        local.get $#local1
        local.get $#local2
        i32.store offset=28
        local.get $#local1
        i64.const 0
        i64.store offset=16 align=4
        local.get $#local2
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set $#local3
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                i32.const 0
                i32.load offset=1055892
                local.tee $#local4
                i32.const 1
                local.get $#local2
                i32.shl
                local.tee $#local5
                i32.and
                br_if $#label4
                i32.const 0
                local.get $#local4
                local.get $#local5
                i32.or
                i32.store offset=1055892
                i32.const 8
                local.set $#local0
                i32.const 24
                local.set $#local2
                local.get $#local3
                local.set $#local5
                br $#label3
              end
              local.get $#local0
              i32.const 0
              i32.const 25
              local.get $#local2
              i32.const 1
              i32.shr_u
              i32.sub
              local.get $#local2
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set $#local2
              local.get $#local3
              i32.load
              local.set $#local5
              loop $#label4
                local.get $#local5
                local.tee $#local4
                i32.load offset=4
                i32.const -8
                i32.and
                local.get $#local0
                i32.eq
                br_if $#label2
                local.get $#local2
                i32.const 29
                i32.shr_u
                local.set $#local5
                local.get $#local2
                i32.const 1
                i32.shl
                local.set $#local2
                local.get $#local4
                local.get $#local5
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee $#local3
                i32.load
                local.tee $#local5
                br_if $#label4
              end
              i32.const 8
              local.set $#local0
              i32.const 24
              local.set $#local2
              local.get $#local4
              local.set $#local5
            end
            local.get $#local1
            local.set $#local4
            local.get $#local1
            local.set $#local7
            br $#label1
          end
          local.get $#local4
          i32.load offset=8
          local.tee $#local5
          local.get $#local1
          i32.store offset=12
          i32.const 8
          local.set $#local2
          local.get $#local4
          i32.const 8
          i32.add
          local.set $#local3
          i32.const 0
          local.set $#local7
          i32.const 24
          local.set $#local0
        end
        local.get $#local3
        local.get $#local1
        i32.store
        local.get $#local1
        local.get $#local2
        i32.add
        local.get $#local5
        i32.store
        local.get $#local1
        local.get $#local4
        i32.store offset=12
        local.get $#local1
        local.get $#local0
        i32.add
        local.get $#local7
        i32.store
        i32.const 0
        i32.const 0
        i32.load offset=1055920
        i32.const -1
        i32.add
        local.tee $#local1
        i32.const -1
        local.get $#local1
        select
        i32.store offset=1055920
      end
    )
    (func $calloc (;124;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i64)
      block $#label0
        block $#label1
          local.get $#local0
          br_if $#label1
          i32.const 0
          local.set $#local2
          br $#label0
        end
        local.get $#local0
        i64.extend_i32_u
        local.get $#local1
        i64.extend_i32_u
        i64.mul
        local.tee $#local3
        i32.wrap_i64
        local.set $#local2
        local.get $#local1
        local.get $#local0
        i32.or
        i32.const 65536
        i32.lt_u
        br_if $#label0
        i32.const -1
        local.get $#local2
        local.get $#local3
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.const 0
        i32.ne
        select
        local.set $#local2
      end
      block $#label0
        local.get $#local2
        call $dlmalloc
        local.tee $#local0
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.const -4
        i32.add
        i32.load8_u
        i32.const 3
        i32.and
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.const 0
        local.get $#local2
        call $memset
        drop
      end
      local.get $#local0
    )
    (func $realloc (;125;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32) (local $#local12 i32)
      block $#label0
        local.get $#local0
        br_if $#label0
        local.get $#local1
        call $dlmalloc
        return
      end
      block $#label0
        local.get $#local1
        i32.const -64
        i32.lt_u
        br_if $#label0
        i32.const 0
        i32.const 48
        i32.store offset=1056384
        i32.const 0
        return
      end
      i32.const 16
      local.get $#local1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get $#local1
      i32.const 11
      i32.lt_u
      select
      local.set $#local2
      local.get $#local0
      i32.const -4
      i32.add
      local.tee $#local3
      i32.load
      local.tee $#local4
      i32.const -8
      i32.and
      local.set $#local5
      block $#label0
        block $#label1
          block $#label2
            local.get $#local4
            i32.const 3
            i32.and
            br_if $#label2
            local.get $#local2
            i32.const 256
            i32.lt_u
            br_if $#label1
            local.get $#local5
            local.get $#local2
            i32.const 4
            i32.or
            i32.lt_u
            br_if $#label1
            local.get $#local5
            local.get $#local2
            i32.sub
            i32.const 0
            i32.load offset=1056368
            i32.const 1
            i32.shl
            i32.le_u
            br_if $#label0
            br $#label1
          end
          local.get $#local0
          i32.const -8
          i32.add
          local.tee $#local6
          local.get $#local5
          i32.add
          local.set $#local7
          block $#label2
            local.get $#local5
            local.get $#local2
            i32.lt_u
            br_if $#label2
            local.get $#local5
            local.get $#local2
            i32.sub
            local.tee $#local1
            i32.const 16
            i32.lt_u
            br_if $#label0
            local.get $#local3
            local.get $#local2
            local.get $#local4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get $#local6
            local.get $#local2
            i32.add
            local.tee $#local2
            local.get $#local1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get $#local7
            local.get $#local7
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $#local2
            local.get $#local1
            call $dispose_chunk
            local.get $#local0
            return
          end
          block $#label2
            local.get $#local7
            i32.const 0
            i32.load offset=1055912
            i32.ne
            br_if $#label2
            i32.const 0
            i32.load offset=1055900
            local.get $#local5
            i32.add
            local.tee $#local5
            local.get $#local2
            i32.le_u
            br_if $#label1
            local.get $#local3
            local.get $#local2
            local.get $#local4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            i32.const 0
            local.get $#local6
            local.get $#local2
            i32.add
            local.tee $#local1
            i32.store offset=1055912
            i32.const 0
            local.get $#local5
            local.get $#local2
            i32.sub
            local.tee $#local2
            i32.store offset=1055900
            local.get $#local1
            local.get $#local2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $#local0
            return
          end
          block $#label2
            local.get $#local7
            i32.const 0
            i32.load offset=1055908
            i32.ne
            br_if $#label2
            i32.const 0
            i32.load offset=1055896
            local.get $#local5
            i32.add
            local.tee $#local5
            local.get $#local2
            i32.lt_u
            br_if $#label1
            block $#label3
              block $#label4
                local.get $#local5
                local.get $#local2
                i32.sub
                local.tee $#local1
                i32.const 16
                i32.lt_u
                br_if $#label4
                local.get $#local3
                local.get $#local2
                local.get $#local4
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get $#local6
                local.get $#local2
                i32.add
                local.tee $#local2
                local.get $#local1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get $#local6
                local.get $#local5
                i32.add
                local.tee $#local5
                local.get $#local1
                i32.store
                local.get $#local5
                local.get $#local5
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                br $#label3
              end
              local.get $#local3
              local.get $#local4
              i32.const 1
              i32.and
              local.get $#local5
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get $#local6
              local.get $#local5
              i32.add
              local.tee $#local1
              local.get $#local1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.set $#local1
              i32.const 0
              local.set $#local2
            end
            i32.const 0
            local.get $#local2
            i32.store offset=1055908
            i32.const 0
            local.get $#local1
            i32.store offset=1055896
            local.get $#local0
            return
          end
          local.get $#local7
          i32.load offset=4
          local.tee $#local8
          i32.const 2
          i32.and
          br_if $#label1
          local.get $#local8
          i32.const -8
          i32.and
          local.get $#local5
          i32.add
          local.tee $#local9
          local.get $#local2
          i32.lt_u
          br_if $#label1
          local.get $#local9
          local.get $#local2
          i32.sub
          local.set $#local10
          local.get $#local7
          i32.load offset=12
          local.set $#local1
          block $#label2
            block $#label3
              local.get $#local8
              i32.const 255
              i32.gt_u
              br_if $#label3
              block $#label4
                local.get $#local1
                local.get $#local7
                i32.load offset=8
                local.tee $#local5
                i32.ne
                br_if $#label4
                i32.const 0
                i32.const 0
                i32.load offset=1055888
                i32.const -2
                local.get $#local8
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=1055888
                br $#label2
              end
              local.get $#local1
              local.get $#local5
              i32.store offset=8
              local.get $#local5
              local.get $#local1
              i32.store offset=12
              br $#label2
            end
            local.get $#local7
            i32.load offset=24
            local.set $#local11
            block $#label3
              block $#label4
                local.get $#local1
                local.get $#local7
                i32.eq
                br_if $#label4
                local.get $#local7
                i32.load offset=8
                local.tee $#local5
                local.get $#local1
                i32.store offset=12
                local.get $#local1
                local.get $#local5
                i32.store offset=8
                br $#label3
              end
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local7
                    i32.load offset=20
                    local.tee $#local5
                    i32.eqz
                    br_if $#label6
                    local.get $#local7
                    i32.const 20
                    i32.add
                    local.set $#local8
                    br $#label5
                  end
                  local.get $#local7
                  i32.load offset=16
                  local.tee $#local5
                  i32.eqz
                  br_if $#label4
                  local.get $#local7
                  i32.const 16
                  i32.add
                  local.set $#local8
                end
                loop $#label5
                  local.get $#local8
                  local.set $#local12
                  local.get $#local5
                  local.tee $#local1
                  i32.const 20
                  i32.add
                  local.set $#local8
                  local.get $#local1
                  i32.load offset=20
                  local.tee $#local5
                  br_if $#label5
                  local.get $#local1
                  i32.const 16
                  i32.add
                  local.set $#local8
                  local.get $#local1
                  i32.load offset=16
                  local.tee $#local5
                  br_if $#label5
                end
                local.get $#local12
                i32.const 0
                i32.store
                br $#label3
              end
              i32.const 0
              local.set $#local1
            end
            local.get $#local11
            i32.eqz
            br_if $#label2
            block $#label3
              block $#label4
                local.get $#local7
                local.get $#local7
                i32.load offset=28
                local.tee $#local8
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee $#local5
                i32.load
                i32.ne
                br_if $#label4
                local.get $#local5
                local.get $#local1
                i32.store
                local.get $#local1
                br_if $#label3
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get $#local8
                i32.rotl
                i32.and
                i32.store offset=1055892
                br $#label2
              end
              local.get $#local11
              i32.const 16
              i32.const 20
              local.get $#local11
              i32.load offset=16
              local.get $#local7
              i32.eq
              select
              i32.add
              local.get $#local1
              i32.store
              local.get $#local1
              i32.eqz
              br_if $#label2
            end
            local.get $#local1
            local.get $#local11
            i32.store offset=24
            block $#label3
              local.get $#local7
              i32.load offset=16
              local.tee $#local5
              i32.eqz
              br_if $#label3
              local.get $#local1
              local.get $#local5
              i32.store offset=16
              local.get $#local5
              local.get $#local1
              i32.store offset=24
            end
            local.get $#local7
            i32.load offset=20
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local1
            local.get $#local5
            i32.store offset=20
            local.get $#local5
            local.get $#local1
            i32.store offset=24
          end
          block $#label2
            local.get $#local10
            i32.const 15
            i32.gt_u
            br_if $#label2
            local.get $#local3
            local.get $#local4
            i32.const 1
            i32.and
            local.get $#local9
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get $#local6
            local.get $#local9
            i32.add
            local.tee $#local1
            local.get $#local1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get $#local0
            return
          end
          local.get $#local3
          local.get $#local2
          local.get $#local4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get $#local6
          local.get $#local2
          i32.add
          local.tee $#local1
          local.get $#local10
          i32.const 3
          i32.or
          i32.store offset=4
          local.get $#local6
          local.get $#local9
          i32.add
          local.tee $#local2
          local.get $#local2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $#local1
          local.get $#local10
          call $dispose_chunk
          local.get $#local0
          return
        end
        block $#label1
          local.get $#local1
          call $dlmalloc
          local.tee $#local2
          br_if $#label1
          i32.const 0
          return
        end
        local.get $#local2
        local.get $#local0
        i32.const -4
        i32.const -8
        local.get $#local3
        i32.load
        local.tee $#local5
        i32.const 3
        i32.and
        select
        local.get $#local5
        i32.const -8
        i32.and
        i32.add
        local.tee $#local5
        local.get $#local1
        local.get $#local5
        local.get $#local1
        i32.lt_u
        select
        call $memcpy
        local.set $#local1
        local.get $#local0
        call $dlfree
        local.get $#local1
        local.set $#local0
      end
      local.get $#local0
    )
    (func $dispose_chunk (;126;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      local.get $#local0
      local.get $#local1
      i32.add
      local.set $#local2
      block $#label0
        block $#label1
          local.get $#local0
          i32.load offset=4
          local.tee $#local3
          i32.const 1
          i32.and
          br_if $#label1
          local.get $#local3
          i32.const 2
          i32.and
          i32.eqz
          br_if $#label0
          local.get $#local0
          i32.load
          local.tee $#local4
          local.get $#local1
          i32.add
          local.set $#local1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local0
                  local.get $#local4
                  i32.sub
                  local.tee $#local0
                  i32.const 0
                  i32.load offset=1055908
                  i32.eq
                  br_if $#label5
                  local.get $#local0
                  i32.load offset=12
                  local.set $#local3
                  block $#label6
                    local.get $#local4
                    i32.const 255
                    i32.gt_u
                    br_if $#label6
                    local.get $#local3
                    local.get $#local0
                    i32.load offset=8
                    local.tee $#local5
                    i32.ne
                    br_if $#label4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055888
                    i32.const -2
                    local.get $#local4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1055888
                    br $#label1
                  end
                  local.get $#local0
                  i32.load offset=24
                  local.set $#local6
                  block $#label6
                    local.get $#local3
                    local.get $#local0
                    i32.eq
                    br_if $#label6
                    local.get $#local0
                    i32.load offset=8
                    local.tee $#local4
                    local.get $#local3
                    i32.store offset=12
                    local.get $#local3
                    local.get $#local4
                    i32.store offset=8
                    br $#label2
                  end
                  block $#label6
                    block $#label7
                      local.get $#local0
                      i32.load offset=20
                      local.tee $#local4
                      i32.eqz
                      br_if $#label7
                      local.get $#local0
                      i32.const 20
                      i32.add
                      local.set $#local5
                      br $#label6
                    end
                    local.get $#local0
                    i32.load offset=16
                    local.tee $#local4
                    i32.eqz
                    br_if $#label3
                    local.get $#local0
                    i32.const 16
                    i32.add
                    local.set $#local5
                  end
                  loop $#label6
                    local.get $#local5
                    local.set $#local7
                    local.get $#local4
                    local.tee $#local3
                    i32.const 20
                    i32.add
                    local.set $#local5
                    local.get $#local3
                    i32.load offset=20
                    local.tee $#local4
                    br_if $#label6
                    local.get $#local3
                    i32.const 16
                    i32.add
                    local.set $#local5
                    local.get $#local3
                    i32.load offset=16
                    local.tee $#local4
                    br_if $#label6
                  end
                  local.get $#local7
                  i32.const 0
                  i32.store
                  br $#label2
                end
                local.get $#local2
                i32.load offset=4
                local.tee $#local3
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if $#label1
                local.get $#local2
                local.get $#local3
                i32.const -2
                i32.and
                i32.store offset=4
                i32.const 0
                local.get $#local1
                i32.store offset=1055896
                local.get $#local2
                local.get $#local1
                i32.store
                local.get $#local0
                local.get $#local1
                i32.const 1
                i32.or
                i32.store offset=4
                return
              end
              local.get $#local3
              local.get $#local5
              i32.store offset=8
              local.get $#local5
              local.get $#local3
              i32.store offset=12
              br $#label1
            end
            i32.const 0
            local.set $#local3
          end
          local.get $#local6
          i32.eqz
          br_if $#label1
          block $#label2
            block $#label3
              local.get $#local0
              local.get $#local0
              i32.load offset=28
              local.tee $#local5
              i32.const 2
              i32.shl
              i32.const 1056192
              i32.add
              local.tee $#local4
              i32.load
              i32.ne
              br_if $#label3
              local.get $#local4
              local.get $#local3
              i32.store
              local.get $#local3
              br_if $#label2
              i32.const 0
              i32.const 0
              i32.load offset=1055892
              i32.const -2
              local.get $#local5
              i32.rotl
              i32.and
              i32.store offset=1055892
              br $#label1
            end
            local.get $#local6
            i32.const 16
            i32.const 20
            local.get $#local6
            i32.load offset=16
            local.get $#local0
            i32.eq
            select
            i32.add
            local.get $#local3
            i32.store
            local.get $#local3
            i32.eqz
            br_if $#label1
          end
          local.get $#local3
          local.get $#local6
          i32.store offset=24
          block $#label2
            local.get $#local0
            i32.load offset=16
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local3
            local.get $#local4
            i32.store offset=16
            local.get $#local4
            local.get $#local3
            i32.store offset=24
          end
          local.get $#local0
          i32.load offset=20
          local.tee $#local4
          i32.eqz
          br_if $#label1
          local.get $#local3
          local.get $#local4
          i32.store offset=20
          local.get $#local4
          local.get $#local3
          i32.store offset=24
        end
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local2
                  i32.load offset=4
                  local.tee $#local4
                  i32.const 2
                  i32.and
                  br_if $#label5
                  block $#label6
                    local.get $#local2
                    i32.const 0
                    i32.load offset=1055912
                    i32.ne
                    br_if $#label6
                    i32.const 0
                    local.get $#local0
                    i32.store offset=1055912
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055900
                    local.get $#local1
                    i32.add
                    local.tee $#local1
                    i32.store offset=1055900
                    local.get $#local0
                    local.get $#local1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $#local0
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if $#label0
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055896
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055908
                    return
                  end
                  block $#label6
                    local.get $#local2
                    i32.const 0
                    i32.load offset=1055908
                    i32.ne
                    br_if $#label6
                    i32.const 0
                    local.get $#local0
                    i32.store offset=1055908
                    i32.const 0
                    i32.const 0
                    i32.load offset=1055896
                    local.get $#local1
                    i32.add
                    local.tee $#local1
                    i32.store offset=1055896
                    local.get $#local0
                    local.get $#local1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get $#local0
                    local.get $#local1
                    i32.add
                    local.get $#local1
                    i32.store
                    return
                  end
                  local.get $#local4
                  i32.const -8
                  i32.and
                  local.get $#local1
                  i32.add
                  local.set $#local1
                  local.get $#local2
                  i32.load offset=12
                  local.set $#local3
                  block $#label6
                    local.get $#local4
                    i32.const 255
                    i32.gt_u
                    br_if $#label6
                    block $#label7
                      local.get $#local3
                      local.get $#local2
                      i32.load offset=8
                      local.tee $#local5
                      i32.ne
                      br_if $#label7
                      i32.const 0
                      i32.const 0
                      i32.load offset=1055888
                      i32.const -2
                      local.get $#local4
                      i32.const 3
                      i32.shr_u
                      i32.rotl
                      i32.and
                      i32.store offset=1055888
                      br $#label2
                    end
                    local.get $#local3
                    local.get $#local5
                    i32.store offset=8
                    local.get $#local5
                    local.get $#local3
                    i32.store offset=12
                    br $#label2
                  end
                  local.get $#local2
                  i32.load offset=24
                  local.set $#local6
                  block $#label6
                    local.get $#local3
                    local.get $#local2
                    i32.eq
                    br_if $#label6
                    local.get $#local2
                    i32.load offset=8
                    local.tee $#local4
                    local.get $#local3
                    i32.store offset=12
                    local.get $#local3
                    local.get $#local4
                    i32.store offset=8
                    br $#label3
                  end
                  block $#label6
                    block $#label7
                      local.get $#local2
                      i32.load offset=20
                      local.tee $#local4
                      i32.eqz
                      br_if $#label7
                      local.get $#local2
                      i32.const 20
                      i32.add
                      local.set $#local5
                      br $#label6
                    end
                    local.get $#local2
                    i32.load offset=16
                    local.tee $#local4
                    i32.eqz
                    br_if $#label4
                    local.get $#local2
                    i32.const 16
                    i32.add
                    local.set $#local5
                  end
                  loop $#label6
                    local.get $#local5
                    local.set $#local7
                    local.get $#local4
                    local.tee $#local3
                    i32.const 20
                    i32.add
                    local.set $#local5
                    local.get $#local3
                    i32.load offset=20
                    local.tee $#local4
                    br_if $#label6
                    local.get $#local3
                    i32.const 16
                    i32.add
                    local.set $#local5
                    local.get $#local3
                    i32.load offset=16
                    local.tee $#local4
                    br_if $#label6
                  end
                  local.get $#local7
                  i32.const 0
                  i32.store
                  br $#label3
                end
                local.get $#local2
                local.get $#local4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get $#local0
                local.get $#local1
                i32.add
                local.get $#local1
                i32.store
                local.get $#local0
                local.get $#local1
                i32.const 1
                i32.or
                i32.store offset=4
                br $#label1
              end
              i32.const 0
              local.set $#local3
            end
            local.get $#local6
            i32.eqz
            br_if $#label2
            block $#label3
              block $#label4
                local.get $#local2
                local.get $#local2
                i32.load offset=28
                local.tee $#local5
                i32.const 2
                i32.shl
                i32.const 1056192
                i32.add
                local.tee $#local4
                i32.load
                i32.ne
                br_if $#label4
                local.get $#local4
                local.get $#local3
                i32.store
                local.get $#local3
                br_if $#label3
                i32.const 0
                i32.const 0
                i32.load offset=1055892
                i32.const -2
                local.get $#local5
                i32.rotl
                i32.and
                i32.store offset=1055892
                br $#label2
              end
              local.get $#local6
              i32.const 16
              i32.const 20
              local.get $#local6
              i32.load offset=16
              local.get $#local2
              i32.eq
              select
              i32.add
              local.get $#local3
              i32.store
              local.get $#local3
              i32.eqz
              br_if $#label2
            end
            local.get $#local3
            local.get $#local6
            i32.store offset=24
            block $#label3
              local.get $#local2
              i32.load offset=16
              local.tee $#local4
              i32.eqz
              br_if $#label3
              local.get $#local3
              local.get $#local4
              i32.store offset=16
              local.get $#local4
              local.get $#local3
              i32.store offset=24
            end
            local.get $#local2
            i32.load offset=20
            local.tee $#local4
            i32.eqz
            br_if $#label2
            local.get $#local3
            local.get $#local4
            i32.store offset=20
            local.get $#local4
            local.get $#local3
            i32.store offset=24
          end
          local.get $#local0
          local.get $#local1
          i32.add
          local.get $#local1
          i32.store
          local.get $#local0
          local.get $#local1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get $#local0
          i32.const 0
          i32.load offset=1055908
          i32.ne
          br_if $#label1
          i32.const 0
          local.get $#local1
          i32.store offset=1055896
          return
        end
        block $#label1
          local.get $#local1
          i32.const 255
          i32.gt_u
          br_if $#label1
          local.get $#local1
          i32.const -8
          i32.and
          i32.const 1055928
          i32.add
          local.set $#local3
          block $#label2
            block $#label3
              i32.const 0
              i32.load offset=1055888
              local.tee $#local4
              i32.const 1
              local.get $#local1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee $#local1
              i32.and
              br_if $#label3
              i32.const 0
              local.get $#local4
              local.get $#local1
              i32.or
              i32.store offset=1055888
              local.get $#local3
              local.set $#local1
              br $#label2
            end
            local.get $#local3
            i32.load offset=8
            local.set $#local1
          end
          local.get $#local1
          local.get $#local0
          i32.store offset=12
          local.get $#local3
          local.get $#local0
          i32.store offset=8
          local.get $#local0
          local.get $#local3
          i32.store offset=12
          local.get $#local0
          local.get $#local1
          i32.store offset=8
          return
        end
        i32.const 31
        local.set $#local3
        block $#label1
          local.get $#local1
          i32.const 16777215
          i32.gt_u
          br_if $#label1
          local.get $#local1
          i32.const 38
          local.get $#local1
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee $#local3
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get $#local3
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set $#local3
        end
        local.get $#local0
        local.get $#local3
        i32.store offset=28
        local.get $#local0
        i64.const 0
        i64.store offset=16 align=4
        local.get $#local3
        i32.const 2
        i32.shl
        i32.const 1056192
        i32.add
        local.set $#local4
        block $#label1
          i32.const 0
          i32.load offset=1055892
          local.tee $#local5
          i32.const 1
          local.get $#local3
          i32.shl
          local.tee $#local2
          i32.and
          br_if $#label1
          local.get $#local4
          local.get $#local0
          i32.store
          i32.const 0
          local.get $#local5
          local.get $#local2
          i32.or
          i32.store offset=1055892
          local.get $#local0
          local.get $#local4
          i32.store offset=24
          local.get $#local0
          local.get $#local0
          i32.store offset=8
          local.get $#local0
          local.get $#local0
          i32.store offset=12
          return
        end
        local.get $#local1
        i32.const 0
        i32.const 25
        local.get $#local3
        i32.const 1
        i32.shr_u
        i32.sub
        local.get $#local3
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set $#local3
        local.get $#local4
        i32.load
        local.set $#local5
        block $#label1
          loop $#label2
            local.get $#local5
            local.tee $#local4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get $#local1
            i32.eq
            br_if $#label1
            local.get $#local3
            i32.const 29
            i32.shr_u
            local.set $#local5
            local.get $#local3
            i32.const 1
            i32.shl
            local.set $#local3
            local.get $#local4
            local.get $#local5
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee $#local2
            i32.load
            local.tee $#local5
            br_if $#label2
          end
          local.get $#local2
          local.get $#local0
          i32.store
          local.get $#local0
          local.get $#local4
          i32.store offset=24
          local.get $#local0
          local.get $#local0
          i32.store offset=12
          local.get $#local0
          local.get $#local0
          i32.store offset=8
          return
        end
        local.get $#local4
        i32.load offset=8
        local.tee $#local1
        local.get $#local0
        i32.store offset=12
        local.get $#local4
        local.get $#local0
        i32.store offset=8
        local.get $#local0
        i32.const 0
        i32.store offset=24
        local.get $#local0
        local.get $#local4
        i32.store offset=12
        local.get $#local0
        local.get $#local1
        i32.store offset=8
      end
    )
    (func $posix_memalign (;127;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32)
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            i32.const 16
            i32.ne
            br_if $#label2
            local.get $#local2
            call $dlmalloc
            local.set $#local1
            br $#label1
          end
          i32.const 28
          local.set $#local3
          local.get $#local1
          i32.const 4
          i32.lt_u
          br_if $#label0
          local.get $#local1
          i32.const 3
          i32.and
          br_if $#label0
          local.get $#local1
          i32.const 2
          i32.shr_u
          local.tee $#local4
          local.get $#local4
          i32.const -1
          i32.add
          i32.and
          br_if $#label0
          block $#label2
            i32.const -64
            local.get $#local1
            i32.sub
            local.get $#local2
            i32.ge_u
            br_if $#label2
            i32.const 48
            return
          end
          local.get $#local1
          i32.const 16
          local.get $#local1
          i32.const 16
          i32.gt_u
          select
          local.get $#local2
          call $internal_memalign
          local.set $#local1
        end
        block $#label1
          local.get $#local1
          br_if $#label1
          i32.const 48
          return
        end
        local.get $#local0
        local.get $#local1
        i32.store
        i32.const 0
        local.set $#local3
      end
      local.get $#local3
    )
    (func $internal_memalign (;128;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32)
      block $#label0
        block $#label1
          local.get $#local0
          i32.const 16
          local.get $#local0
          i32.const 16
          i32.gt_u
          select
          local.tee $#local2
          local.get $#local2
          i32.const -1
          i32.add
          i32.and
          br_if $#label1
          local.get $#local2
          local.set $#local0
          br $#label0
        end
        i32.const 32
        local.set $#local3
        loop $#label1
          local.get $#local3
          local.tee $#local0
          i32.const 1
          i32.shl
          local.set $#local3
          local.get $#local0
          local.get $#local2
          i32.lt_u
          br_if $#label1
        end
      end
      block $#label0
        i32.const -64
        local.get $#local0
        i32.sub
        local.get $#local1
        i32.gt_u
        br_if $#label0
        i32.const 0
        i32.const 48
        i32.store offset=1056384
        i32.const 0
        return
      end
      block $#label0
        local.get $#local0
        i32.const 16
        local.get $#local1
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.get $#local1
        i32.const 11
        i32.lt_u
        select
        local.tee $#local1
        i32.add
        i32.const 12
        i32.add
        call $dlmalloc
        local.tee $#local3
        br_if $#label0
        i32.const 0
        return
      end
      local.get $#local3
      i32.const -8
      i32.add
      local.set $#local2
      block $#label0
        block $#label1
          local.get $#local0
          i32.const -1
          i32.add
          local.get $#local3
          i32.and
          br_if $#label1
          local.get $#local2
          local.set $#local0
          br $#label0
        end
        local.get $#local3
        i32.const -4
        i32.add
        local.tee $#local4
        i32.load
        local.tee $#local5
        i32.const -8
        i32.and
        local.get $#local3
        local.get $#local0
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get $#local0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee $#local3
        i32.const 0
        local.get $#local0
        local.get $#local3
        local.get $#local2
        i32.sub
        i32.const 15
        i32.gt_u
        select
        i32.add
        local.tee $#local0
        local.get $#local2
        i32.sub
        local.tee $#local3
        i32.sub
        local.set $#local6
        block $#label1
          local.get $#local5
          i32.const 3
          i32.and
          br_if $#label1
          local.get $#local0
          local.get $#local6
          i32.store offset=4
          local.get $#local0
          local.get $#local2
          i32.load
          local.get $#local3
          i32.add
          i32.store
          br $#label0
        end
        local.get $#local0
        local.get $#local6
        local.get $#local0
        i32.load offset=4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get $#local0
        local.get $#local6
        i32.add
        local.tee $#local6
        local.get $#local6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $#local4
        local.get $#local3
        local.get $#local4
        i32.load
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get $#local2
        local.get $#local3
        i32.add
        local.tee $#local6
        local.get $#local6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $#local2
        local.get $#local3
        call $dispose_chunk
      end
      block $#label0
        local.get $#local0
        i32.load offset=4
        local.tee $#local3
        i32.const 3
        i32.and
        i32.eqz
        br_if $#label0
        local.get $#local3
        i32.const -8
        i32.and
        local.tee $#local2
        local.get $#local1
        i32.const 16
        i32.add
        i32.le_u
        br_if $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local3
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get $#local0
        local.get $#local1
        i32.add
        local.tee $#local3
        local.get $#local2
        local.get $#local1
        i32.sub
        local.tee $#local1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get $#local0
        local.get $#local2
        i32.add
        local.tee $#local2
        local.get $#local2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get $#local3
        local.get $#local1
        call $dispose_chunk
      end
      local.get $#local0
      i32.const 8
      i32.add
    )
    (func $_Exit (;129;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $__wasi_proc_exit
      unreachable
    )
    (func $__wasilibc_ensure_environ (;130;) (type $#type0)
      block $#label0
        i32.const 0
        i32.load offset=1055524
        i32.const -1
        i32.ne
        br_if $#label0
        call $__wasilibc_initialize_environ
      end
    )
    (func $__wasilibc_initialize_environ (;131;) (type $#type0)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local0
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local0
          i32.const 12
          i32.add
          local.get $#local0
          i32.const 8
          i32.add
          call $__wasi_environ_sizes_get
          br_if $#label1
          block $#label2
            local.get $#local0
            i32.load offset=12
            local.tee $#local1
            br_if $#label2
            i32.const 1056388
            local.set $#local1
            br $#label0
          end
          block $#label2
            block $#label3
              local.get $#local1
              i32.const 1
              i32.add
              local.tee $#local1
              i32.eqz
              br_if $#label3
              local.get $#local0
              i32.load offset=8
              call $malloc
              local.tee $#local2
              i32.eqz
              br_if $#label3
              local.get $#local1
              i32.const 4
              call $calloc
              local.tee $#local1
              br_if $#label2
              local.get $#local2
              call $free
            end
            i32.const 70
            call $_Exit
            unreachable
          end
          local.get $#local1
          local.get $#local2
          call $__wasi_environ_get
          i32.eqz
          br_if $#label0
          local.get $#local2
          call $free
          local.get $#local1
          call $free
        end
        i32.const 71
        call $_Exit
        unreachable
      end
      i32.const 0
      local.get $#local1
      i32.store offset=1055524
      local.get $#local0
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $__wasi_environ_get (;132;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      call $__imported_wasi_snapshot_preview1_environ_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_environ_sizes_get (;133;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      call $__imported_wasi_snapshot_preview1_environ_sizes_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_proc_exit (;134;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $__imported_wasi_snapshot_preview1_proc_exit
      unreachable
    )
    (func $abort (;135;) (type $#type0)
      unreachable
    )
    (func $getcwd (;136;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      i32.const 0
      i32.load offset=1055528
      local.set $#local2
      block $#label0
        block $#label1
          local.get $#local0
          br_if $#label1
          local.get $#local2
          call $strdup
          local.tee $#local0
          br_if $#label0
          i32.const 0
          i32.const 48
          i32.store offset=1056384
          i32.const 0
          return
        end
        block $#label1
          local.get $#local2
          call $strlen
          i32.const 1
          i32.add
          local.get $#local1
          i32.le_u
          br_if $#label1
          i32.const 0
          i32.const 68
          i32.store offset=1056384
          i32.const 0
          return
        end
        local.get $#local0
        local.get $#local2
        call $strcpy
        local.set $#local0
      end
      local.get $#local0
    )
    (func $sbrk (;137;) (type $#type3) (param $#local0 i32) (result i32)
      block $#label0
        local.get $#local0
        br_if $#label0
        memory.size
        i32.const 16
        i32.shl
        return
      end
      block $#label0
        local.get $#local0
        i32.const 65535
        i32.and
        br_if $#label0
        local.get $#local0
        i32.const -1
        i32.le_s
        br_if $#label0
        block $#label1
          local.get $#local0
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee $#local0
          i32.const -1
          i32.ne
          br_if $#label1
          i32.const 0
          i32.const 48
          i32.store offset=1056384
          i32.const -1
          return
        end
        local.get $#local0
        i32.const 16
        i32.shl
        return
      end
      call $abort
      unreachable
    )
    (func $getenv (;138;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      call $__wasilibc_ensure_environ
      block $#label0
        local.get $#local0
        i32.const 61
        call $__strchrnul
        local.tee $#local1
        local.get $#local0
        i32.ne
        br_if $#label0
        i32.const 0
        return
      end
      i32.const 0
      local.set $#local2
      block $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local0
        i32.sub
        local.tee $#local3
        i32.add
        i32.load8_u
        br_if $#label0
        i32.const 0
        i32.load offset=1055524
        local.tee $#local4
        i32.eqz
        br_if $#label0
        local.get $#local4
        i32.load
        local.tee $#local1
        i32.eqz
        br_if $#label0
        local.get $#local4
        i32.const 4
        i32.add
        local.set $#local4
        block $#label1
          loop $#label2
            block $#label3
              local.get $#local0
              local.get $#local1
              local.get $#local3
              call $strncmp
              br_if $#label3
              local.get $#local1
              local.get $#local3
              i32.add
              local.tee $#local1
              i32.load8_u
              i32.const 61
              i32.eq
              br_if $#label1
            end
            local.get $#local4
            i32.load
            local.set $#local1
            local.get $#local4
            i32.const 4
            i32.add
            local.set $#local4
            local.get $#local1
            br_if $#label2
            br $#label0
          end
        end
        local.get $#local1
        i32.const 1
        i32.add
        local.set $#local2
      end
      local.get $#local2
    )
    (func $dummy (;139;) (type $#type0))
    (func $__wasm_call_dtors (;140;) (type $#type0)
      call $dummy
      call $__stdio_exit
    )
    (func $"#func141 dummy" (@name "dummy") (;141;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
    )
    (func $__lctrans (;142;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      call $"#func141 dummy"
    )
    (func $__ofl_lock (;143;) (type $#type10) (result i32)
      i32.const 1056420
    )
    (func $__stdio_exit (;144;) (type $#type0)
      (local $#local0 i32) (local $#local1 i32) (local $#local2 i32)
      block $#label0
        call $__ofl_lock
        i32.load
        local.tee $#local0
        i32.eqz
        br_if $#label0
        loop $#label1
          block $#label2
            local.get $#local0
            i32.load offset=20
            local.get $#local0
            i32.load offset=24
            i32.eq
            br_if $#label2
            local.get $#local0
            i32.const 0
            i32.const 0
            local.get $#local0
            i32.load offset=32
            call_indirect (type $#type5)
            drop
          end
          block $#label2
            local.get $#local0
            i32.load offset=4
            local.tee $#local1
            local.get $#local0
            i32.load offset=8
            local.tee $#local2
            i32.eq
            br_if $#label2
            local.get $#local0
            local.get $#local1
            local.get $#local2
            i32.sub
            i64.extend_i32_s
            i32.const 1
            local.get $#local0
            i32.load offset=36
            call_indirect (type $#type8)
            drop
          end
          local.get $#local0
          i32.load offset=52
          local.tee $#local0
          br_if $#label1
        end
      end
      block $#label0
        i32.const 0
        i32.load offset=1056424
        local.tee $#local0
        i32.eqz
        br_if $#label0
        block $#label1
          local.get $#local0
          i32.load offset=20
          local.get $#local0
          i32.load offset=24
          i32.eq
          br_if $#label1
          local.get $#local0
          i32.const 0
          i32.const 0
          local.get $#local0
          i32.load offset=32
          call_indirect (type $#type5)
          drop
        end
        local.get $#local0
        i32.load offset=4
        local.tee $#local1
        local.get $#local0
        i32.load offset=8
        local.tee $#local2
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get $#local0
        i32.load offset=36
        call_indirect (type $#type8)
        drop
      end
      block $#label0
        i32.const 0
        i32.load offset=1056424
        local.tee $#local0
        i32.eqz
        br_if $#label0
        block $#label1
          local.get $#local0
          i32.load offset=20
          local.get $#local0
          i32.load offset=24
          i32.eq
          br_if $#label1
          local.get $#local0
          i32.const 0
          i32.const 0
          local.get $#local0
          i32.load offset=32
          call_indirect (type $#type5)
          drop
        end
        local.get $#local0
        i32.load offset=4
        local.tee $#local1
        local.get $#local0
        i32.load offset=8
        local.tee $#local2
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get $#local0
        i32.load offset=36
        call_indirect (type $#type8)
        drop
      end
      block $#label0
        i32.const 0
        i32.load offset=1056424
        local.tee $#local0
        i32.eqz
        br_if $#label0
        block $#label1
          local.get $#local0
          i32.load offset=20
          local.get $#local0
          i32.load offset=24
          i32.eq
          br_if $#label1
          local.get $#local0
          i32.const 0
          i32.const 0
          local.get $#local0
          i32.load offset=32
          call_indirect (type $#type5)
          drop
        end
        local.get $#local0
        i32.load offset=4
        local.tee $#local1
        local.get $#local0
        i32.load offset=8
        local.tee $#local2
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local2
        i32.sub
        i64.extend_i32_s
        i32.const 1
        local.get $#local0
        i32.load offset=36
        call_indirect (type $#type8)
        drop
      end
    )
    (func $memcpy (;145;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32)
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.const 32
            i32.gt_u
            br_if $#label2
            local.get $#local1
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label1
            local.get $#local2
            i32.eqz
            br_if $#label1
            local.get $#local0
            local.get $#local1
            i32.load8_u
            i32.store8
            local.get $#local2
            i32.const -1
            i32.add
            local.set $#local3
            local.get $#local0
            i32.const 1
            i32.add
            local.set $#local4
            local.get $#local1
            i32.const 1
            i32.add
            local.tee $#local5
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label0
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            local.get $#local1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get $#local2
            i32.const -2
            i32.add
            local.set $#local3
            local.get $#local0
            i32.const 2
            i32.add
            local.set $#local4
            local.get $#local1
            i32.const 2
            i32.add
            local.tee $#local5
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label0
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            local.get $#local1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get $#local2
            i32.const -3
            i32.add
            local.set $#local3
            local.get $#local0
            i32.const 3
            i32.add
            local.set $#local4
            local.get $#local1
            i32.const 3
            i32.add
            local.tee $#local5
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label0
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            local.get $#local1
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get $#local2
            i32.const -4
            i32.add
            local.set $#local3
            local.get $#local0
            i32.const 4
            i32.add
            local.set $#local4
            local.get $#local1
            i32.const 4
            i32.add
            local.set $#local5
            br $#label0
          end
          local.get $#local0
          local.get $#local1
          local.get $#local2
          memory.copy
          local.get $#local0
          return
        end
        local.get $#local2
        local.set $#local3
        local.get $#local0
        local.set $#local4
        local.get $#local1
        local.set $#local5
      end
      block $#label0
        block $#label1
          local.get $#local4
          i32.const 3
          i32.and
          local.tee $#local2
          br_if $#label1
          block $#label2
            block $#label3
              local.get $#local3
              i32.const 16
              i32.ge_u
              br_if $#label3
              local.get $#local3
              local.set $#local2
              br $#label2
            end
            block $#label3
              local.get $#local3
              i32.const -16
              i32.add
              local.tee $#local2
              i32.const 16
              i32.and
              br_if $#label3
              local.get $#local4
              local.get $#local5
              i64.load align=4
              i64.store align=4
              local.get $#local4
              local.get $#local5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get $#local4
              i32.const 16
              i32.add
              local.set $#local4
              local.get $#local5
              i32.const 16
              i32.add
              local.set $#local5
              local.get $#local2
              local.set $#local3
            end
            local.get $#local2
            i32.const 16
            i32.lt_u
            br_if $#label2
            local.get $#local3
            local.set $#local2
            loop $#label3
              local.get $#local4
              local.get $#local5
              i64.load align=4
              i64.store align=4
              local.get $#local4
              local.get $#local5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get $#local4
              local.get $#local5
              i64.load offset=16 align=4
              i64.store offset=16 align=4
              local.get $#local4
              local.get $#local5
              i64.load offset=24 align=4
              i64.store offset=24 align=4
              local.get $#local4
              i32.const 32
              i32.add
              local.set $#local4
              local.get $#local5
              i32.const 32
              i32.add
              local.set $#local5
              local.get $#local2
              i32.const -32
              i32.add
              local.tee $#local2
              i32.const 15
              i32.gt_u
              br_if $#label3
            end
          end
          block $#label2
            local.get $#local2
            i32.const 8
            i32.lt_u
            br_if $#label2
            local.get $#local4
            local.get $#local5
            i64.load align=4
            i64.store align=4
            local.get $#local5
            i32.const 8
            i32.add
            local.set $#local5
            local.get $#local4
            i32.const 8
            i32.add
            local.set $#local4
          end
          block $#label2
            local.get $#local2
            i32.const 4
            i32.and
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local5
            i32.load
            i32.store
            local.get $#local5
            i32.const 4
            i32.add
            local.set $#local5
            local.get $#local4
            i32.const 4
            i32.add
            local.set $#local4
          end
          block $#label2
            local.get $#local2
            i32.const 2
            i32.and
            i32.eqz
            br_if $#label2
            local.get $#local4
            local.get $#local5
            i32.load16_u align=1
            i32.store16 align=1
            local.get $#local4
            i32.const 2
            i32.add
            local.set $#local4
            local.get $#local5
            i32.const 2
            i32.add
            local.set $#local5
          end
          local.get $#local2
          i32.const 1
          i32.and
          i32.eqz
          br_if $#label0
          local.get $#local4
          local.get $#local5
          i32.load8_u
          i32.store8
          local.get $#local0
          return
        end
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local3
                  i32.const 32
                  i32.lt_u
                  br_if $#label5
                  local.get $#local4
                  local.get $#local5
                  i32.load
                  local.tee $#local3
                  i32.store8
                  block $#label6
                    block $#label7
                      local.get $#local2
                      i32.const -1
                      i32.add
                      br_table $#label4 $#label7 $#label6 $#label4
                    end
                    local.get $#local4
                    local.get $#local3
                    i32.const 8
                    i32.shr_u
                    i32.store8 offset=1
                    local.get $#local4
                    local.get $#local5
                    i32.const 6
                    i32.add
                    i64.load align=2
                    i64.store offset=6 align=4
                    local.get $#local4
                    local.get $#local5
                    i32.load offset=4
                    i32.const 16
                    i32.shl
                    local.get $#local3
                    i32.const 16
                    i32.shr_u
                    i32.or
                    i32.store offset=2
                    local.get $#local4
                    i32.const 18
                    i32.add
                    local.set $#local2
                    local.get $#local5
                    i32.const 18
                    i32.add
                    local.set $#local1
                    i32.const 14
                    local.set $#local6
                    local.get $#local5
                    i32.const 14
                    i32.add
                    i32.load align=2
                    local.set $#local5
                    i32.const 14
                    local.set $#local3
                    br $#label3
                  end
                  local.get $#local4
                  local.get $#local5
                  i32.const 5
                  i32.add
                  i64.load align=1
                  i64.store offset=5 align=4
                  local.get $#local4
                  local.get $#local5
                  i32.load offset=4
                  i32.const 24
                  i32.shl
                  local.get $#local3
                  i32.const 8
                  i32.shr_u
                  i32.or
                  i32.store offset=1
                  local.get $#local4
                  i32.const 17
                  i32.add
                  local.set $#local2
                  local.get $#local5
                  i32.const 17
                  i32.add
                  local.set $#local1
                  i32.const 13
                  local.set $#local6
                  local.get $#local5
                  i32.const 13
                  i32.add
                  i32.load align=1
                  local.set $#local5
                  i32.const 15
                  local.set $#local3
                  br $#label3
                end
                block $#label5
                  block $#label6
                    local.get $#local3
                    i32.const 16
                    i32.ge_u
                    br_if $#label6
                    local.get $#local4
                    local.set $#local2
                    local.get $#local5
                    local.set $#local1
                    br $#label5
                  end
                  local.get $#local4
                  local.get $#local5
                  i32.load8_u
                  i32.store8
                  local.get $#local4
                  local.get $#local5
                  i32.load offset=1 align=1
                  i32.store offset=1 align=1
                  local.get $#local4
                  local.get $#local5
                  i64.load offset=5 align=1
                  i64.store offset=5 align=1
                  local.get $#local4
                  local.get $#local5
                  i32.load16_u offset=13 align=1
                  i32.store16 offset=13 align=1
                  local.get $#local4
                  local.get $#local5
                  i32.load8_u offset=15
                  i32.store8 offset=15
                  local.get $#local4
                  i32.const 16
                  i32.add
                  local.set $#local2
                  local.get $#local5
                  i32.const 16
                  i32.add
                  local.set $#local1
                end
                local.get $#local3
                i32.const 8
                i32.and
                br_if $#label2
                br $#label1
              end
              local.get $#local4
              local.get $#local3
              i32.const 16
              i32.shr_u
              i32.store8 offset=2
              local.get $#local4
              local.get $#local3
              i32.const 8
              i32.shr_u
              i32.store8 offset=1
              local.get $#local4
              local.get $#local5
              i32.const 7
              i32.add
              i64.load align=1
              i64.store offset=7 align=4
              local.get $#local4
              local.get $#local5
              i32.load offset=4
              i32.const 8
              i32.shl
              local.get $#local3
              i32.const 24
              i32.shr_u
              i32.or
              i32.store offset=3
              local.get $#local4
              i32.const 19
              i32.add
              local.set $#local2
              local.get $#local5
              i32.const 19
              i32.add
              local.set $#local1
              i32.const 15
              local.set $#local6
              local.get $#local5
              i32.const 15
              i32.add
              i32.load align=1
              local.set $#local5
              i32.const 13
              local.set $#local3
            end
            local.get $#local4
            local.get $#local6
            i32.add
            local.get $#local5
            i32.store
          end
          local.get $#local2
          local.get $#local1
          i64.load align=1
          i64.store align=1
          local.get $#local2
          i32.const 8
          i32.add
          local.set $#local2
          local.get $#local1
          i32.const 8
          i32.add
          local.set $#local1
        end
        block $#label1
          local.get $#local3
          i32.const 4
          i32.and
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local1
          i32.load align=1
          i32.store align=1
          local.get $#local2
          i32.const 4
          i32.add
          local.set $#local2
          local.get $#local1
          i32.const 4
          i32.add
          local.set $#local1
        end
        block $#label1
          local.get $#local3
          i32.const 2
          i32.and
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local1
          i32.load16_u align=1
          i32.store16 align=1
          local.get $#local2
          i32.const 2
          i32.add
          local.set $#local2
          local.get $#local1
          i32.const 2
          i32.add
          local.set $#local1
        end
        local.get $#local3
        i32.const 1
        i32.and
        i32.eqz
        br_if $#label0
        local.get $#local2
        local.get $#local1
        i32.load8_u
        i32.store8
      end
      local.get $#local0
    )
    (func $memset (;146;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i64)
      block $#label0
        local.get $#local2
        i32.const 33
        i32.lt_u
        br_if $#label0
        local.get $#local0
        local.get $#local1
        local.get $#local2
        memory.fill
        local.get $#local0
        return
      end
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        local.get $#local0
        local.get $#local1
        i32.store8
        local.get $#local0
        local.get $#local2
        i32.add
        local.tee $#local3
        i32.const -1
        i32.add
        local.get $#local1
        i32.store8
        local.get $#local2
        i32.const 3
        i32.lt_u
        br_if $#label0
        local.get $#local0
        local.get $#local1
        i32.store8 offset=2
        local.get $#local0
        local.get $#local1
        i32.store8 offset=1
        local.get $#local3
        i32.const -3
        i32.add
        local.get $#local1
        i32.store8
        local.get $#local3
        i32.const -2
        i32.add
        local.get $#local1
        i32.store8
        local.get $#local2
        i32.const 7
        i32.lt_u
        br_if $#label0
        local.get $#local0
        local.get $#local1
        i32.store8 offset=3
        local.get $#local3
        i32.const -4
        i32.add
        local.get $#local1
        i32.store8
        local.get $#local2
        i32.const 9
        i32.lt_u
        br_if $#label0
        local.get $#local0
        i32.const 0
        local.get $#local0
        i32.sub
        i32.const 3
        i32.and
        local.tee $#local4
        i32.add
        local.tee $#local5
        local.get $#local1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.tee $#local3
        i32.store
        local.get $#local5
        local.get $#local2
        local.get $#local4
        i32.sub
        i32.const 60
        i32.and
        local.tee $#local1
        i32.add
        local.tee $#local2
        i32.const -4
        i32.add
        local.get $#local3
        i32.store
        local.get $#local1
        i32.const 9
        i32.lt_u
        br_if $#label0
        local.get $#local5
        local.get $#local3
        i32.store offset=8
        local.get $#local5
        local.get $#local3
        i32.store offset=4
        local.get $#local2
        i32.const -8
        i32.add
        local.get $#local3
        i32.store
        local.get $#local2
        i32.const -12
        i32.add
        local.get $#local3
        i32.store
        local.get $#local1
        i32.const 25
        i32.lt_u
        br_if $#label0
        local.get $#local5
        local.get $#local3
        i32.store offset=24
        local.get $#local5
        local.get $#local3
        i32.store offset=20
        local.get $#local5
        local.get $#local3
        i32.store offset=16
        local.get $#local5
        local.get $#local3
        i32.store offset=12
        local.get $#local2
        i32.const -16
        i32.add
        local.get $#local3
        i32.store
        local.get $#local2
        i32.const -20
        i32.add
        local.get $#local3
        i32.store
        local.get $#local2
        i32.const -24
        i32.add
        local.get $#local3
        i32.store
        local.get $#local2
        i32.const -28
        i32.add
        local.get $#local3
        i32.store
        local.get $#local1
        local.get $#local5
        i32.const 4
        i32.and
        i32.const 24
        i32.or
        local.tee $#local2
        i32.sub
        local.tee $#local1
        i32.const 32
        i32.lt_u
        br_if $#label0
        local.get $#local3
        i64.extend_i32_u
        i64.const 4294967297
        i64.mul
        local.set $#local6
        local.get $#local5
        local.get $#local2
        i32.add
        local.set $#local2
        loop $#label1
          local.get $#local2
          local.get $#local6
          i64.store offset=24
          local.get $#local2
          local.get $#local6
          i64.store offset=16
          local.get $#local2
          local.get $#local6
          i64.store offset=8
          local.get $#local2
          local.get $#local6
          i64.store
          local.get $#local2
          i32.const 32
          i32.add
          local.set $#local2
          local.get $#local1
          i32.const -32
          i32.add
          local.tee $#local1
          i32.const 31
          i32.gt_u
          br_if $#label1
        end
      end
      local.get $#local0
    )
    (func $__strchrnul (;147;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local1
              i32.const 255
              i32.and
              local.tee $#local2
              i32.eqz
              br_if $#label3
              local.get $#local0
              i32.const 3
              i32.and
              i32.eqz
              br_if $#label1
              block $#label4
                local.get $#local0
                i32.load8_u
                local.tee $#local3
                br_if $#label4
                local.get $#local0
                return
              end
              local.get $#local3
              local.get $#local1
              i32.const 255
              i32.and
              i32.ne
              br_if $#label2
              local.get $#local0
              return
            end
            local.get $#local0
            local.get $#local0
            call $strlen
            i32.add
            return
          end
          block $#label2
            local.get $#local0
            i32.const 1
            i32.add
            local.tee $#local3
            i32.const 3
            i32.and
            br_if $#label2
            local.get $#local3
            local.set $#local0
            br $#label1
          end
          local.get $#local3
          i32.load8_u
          local.tee $#local4
          i32.eqz
          br_if $#label0
          local.get $#local4
          local.get $#local1
          i32.const 255
          i32.and
          i32.eq
          br_if $#label0
          block $#label2
            local.get $#local0
            i32.const 2
            i32.add
            local.tee $#local3
            i32.const 3
            i32.and
            br_if $#label2
            local.get $#local3
            local.set $#local0
            br $#label1
          end
          local.get $#local3
          i32.load8_u
          local.tee $#local4
          i32.eqz
          br_if $#label0
          local.get $#local4
          local.get $#local1
          i32.const 255
          i32.and
          i32.eq
          br_if $#label0
          block $#label2
            local.get $#local0
            i32.const 3
            i32.add
            local.tee $#local3
            i32.const 3
            i32.and
            br_if $#label2
            local.get $#local3
            local.set $#local0
            br $#label1
          end
          local.get $#local3
          i32.load8_u
          local.tee $#local4
          i32.eqz
          br_if $#label0
          local.get $#local4
          local.get $#local1
          i32.const 255
          i32.and
          i32.eq
          br_if $#label0
          local.get $#local0
          i32.const 4
          i32.add
          local.set $#local0
        end
        block $#label1
          block $#label2
            i32.const 16843008
            local.get $#local0
            i32.load
            local.tee $#local3
            i32.sub
            local.get $#local3
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if $#label2
            local.get $#local0
            local.set $#local2
            br $#label1
          end
          local.get $#local2
          i32.const 16843009
          i32.mul
          local.set $#local4
          loop $#label2
            block $#label3
              i32.const 16843008
              local.get $#local3
              local.get $#local4
              i32.xor
              local.tee $#local3
              i32.sub
              local.get $#local3
              i32.or
              i32.const -2139062144
              i32.and
              i32.const -2139062144
              i32.eq
              br_if $#label3
              local.get $#local0
              local.set $#local2
              br $#label1
            end
            local.get $#local0
            i32.load offset=4
            local.set $#local3
            local.get $#local0
            i32.const 4
            i32.add
            local.tee $#local2
            local.set $#local0
            local.get $#local3
            i32.const 16843008
            local.get $#local3
            i32.sub
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if $#label2
          end
        end
        local.get $#local2
        i32.const -1
        i32.add
        local.set $#local3
        loop $#label1
          local.get $#local3
          i32.const 1
          i32.add
          local.tee $#local3
          i32.load8_u
          local.tee $#local0
          i32.eqz
          br_if $#label0
          local.get $#local0
          local.get $#local1
          i32.const 255
          i32.and
          i32.ne
          br_if $#label1
        end
      end
      local.get $#local3
    )
    (func $__stpcpy (;148;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            local.get $#local0
            i32.xor
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label2
            local.get $#local1
            i32.load8_u
            local.set $#local2
            br $#label1
          end
          block $#label2
            block $#label3
              local.get $#local1
              i32.const 3
              i32.and
              br_if $#label3
              local.get $#local1
              local.set $#local3
              br $#label2
            end
            local.get $#local0
            local.get $#local1
            i32.load8_u
            local.tee $#local2
            i32.store8
            block $#label3
              local.get $#local2
              br_if $#label3
              local.get $#local0
              return
            end
            local.get $#local0
            i32.const 1
            i32.add
            local.set $#local2
            block $#label3
              local.get $#local1
              i32.const 1
              i32.add
              local.tee $#local3
              i32.const 3
              i32.and
              br_if $#label3
              local.get $#local2
              local.set $#local0
              br $#label2
            end
            local.get $#local2
            local.get $#local3
            i32.load8_u
            local.tee $#local3
            i32.store8
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            i32.const 2
            i32.add
            local.set $#local2
            block $#label3
              local.get $#local1
              i32.const 2
              i32.add
              local.tee $#local3
              i32.const 3
              i32.and
              br_if $#label3
              local.get $#local2
              local.set $#local0
              br $#label2
            end
            local.get $#local2
            local.get $#local3
            i32.load8_u
            local.tee $#local3
            i32.store8
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            i32.const 3
            i32.add
            local.set $#local2
            block $#label3
              local.get $#local1
              i32.const 3
              i32.add
              local.tee $#local3
              i32.const 3
              i32.and
              br_if $#label3
              local.get $#local2
              local.set $#local0
              br $#label2
            end
            local.get $#local2
            local.get $#local3
            i32.load8_u
            local.tee $#local3
            i32.store8
            local.get $#local3
            i32.eqz
            br_if $#label0
            local.get $#local0
            i32.const 4
            i32.add
            local.set $#local0
            local.get $#local1
            i32.const 4
            i32.add
            local.set $#local3
          end
          block $#label2
            i32.const 16843008
            local.get $#local3
            i32.load
            local.tee $#local2
            i32.sub
            local.get $#local2
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if $#label2
            local.get $#local3
            local.set $#local1
            br $#label1
          end
          loop $#label2
            local.get $#local0
            local.get $#local2
            i32.store
            local.get $#local0
            i32.const 4
            i32.add
            local.set $#local0
            local.get $#local3
            i32.load offset=4
            local.set $#local2
            local.get $#local3
            i32.const 4
            i32.add
            local.tee $#local1
            local.set $#local3
            local.get $#local2
            i32.const 16843008
            local.get $#local2
            i32.sub
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if $#label2
          end
        end
        local.get $#local0
        local.get $#local2
        i32.store8
        block $#label1
          local.get $#local2
          i32.const 255
          i32.and
          br_if $#label1
          local.get $#local0
          return
        end
        local.get $#local1
        i32.const 1
        i32.add
        local.set $#local3
        local.get $#local0
        local.set $#local2
        loop $#label1
          local.get $#local2
          local.get $#local3
          i32.load8_u
          local.tee $#local0
          i32.store8 offset=1
          local.get $#local3
          i32.const 1
          i32.add
          local.set $#local3
          local.get $#local2
          i32.const 1
          i32.add
          local.set $#local2
          local.get $#local0
          br_if $#label1
        end
      end
      local.get $#local2
    )
    (func $strcpy (;149;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      call $__stpcpy
      drop
      local.get $#local0
    )
    (func $strdup (;150;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32)
      block $#label0
        local.get $#local0
        call $strlen
        i32.const 1
        i32.add
        local.tee $#local1
        call $malloc
        local.tee $#local2
        i32.eqz
        br_if $#label0
        local.get $#local2
        local.get $#local0
        local.get $#local1
        call $memcpy
        drop
      end
      local.get $#local2
    )
    (func $strerror (;151;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32)
      block $#label0
        i32.const 0
        i32.load offset=1056416
        local.tee $#local1
        br_if $#label0
        i32.const 1056392
        local.set $#local1
        i32.const 0
        i32.const 1056392
        i32.store offset=1056416
      end
      i32.const 0
      local.get $#local0
      local.get $#local0
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
      local.get $#local1
      i32.load offset=20
      call $__lctrans
    )
    (func $strerror_r (;152;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32)
      block $#label0
        block $#label1
          local.get $#local0
          call $strerror
          local.tee $#local0
          call $strlen
          local.tee $#local3
          local.get $#local2
          i32.lt_u
          br_if $#label1
          i32.const 68
          local.set $#local3
          local.get $#local2
          i32.eqz
          br_if $#label0
          local.get $#local1
          local.get $#local0
          local.get $#local2
          i32.const -1
          i32.add
          local.tee $#local2
          call $memcpy
          local.get $#local2
          i32.add
          i32.const 0
          i32.store8
          i32.const 68
          return
        end
        local.get $#local1
        local.get $#local0
        local.get $#local3
        i32.const 1
        i32.add
        call $memcpy
        drop
        i32.const 0
        local.set $#local3
      end
      local.get $#local3
    )
    (func $strlen (;153;) (type $#type3) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32) (local $#local3 i32)
      local.get $#local0
      local.set $#local1
      block $#label0
        block $#label1
          local.get $#local0
          i32.const 3
          i32.and
          i32.eqz
          br_if $#label1
          block $#label2
            local.get $#local0
            i32.load8_u
            br_if $#label2
            local.get $#local0
            local.get $#local0
            i32.sub
            return
          end
          local.get $#local0
          i32.const 1
          i32.add
          local.tee $#local1
          i32.const 3
          i32.and
          i32.eqz
          br_if $#label1
          local.get $#local1
          i32.load8_u
          i32.eqz
          br_if $#label0
          local.get $#local0
          i32.const 2
          i32.add
          local.tee $#local1
          i32.const 3
          i32.and
          i32.eqz
          br_if $#label1
          local.get $#local1
          i32.load8_u
          i32.eqz
          br_if $#label0
          local.get $#local0
          i32.const 3
          i32.add
          local.tee $#local1
          i32.const 3
          i32.and
          i32.eqz
          br_if $#label1
          local.get $#local1
          i32.load8_u
          i32.eqz
          br_if $#label0
          local.get $#local0
          i32.const 4
          i32.add
          local.tee $#local1
          i32.const 3
          i32.and
          br_if $#label0
        end
        local.get $#local1
        i32.const -4
        i32.add
        local.set $#local2
        local.get $#local1
        i32.const -5
        i32.add
        local.set $#local1
        loop $#label1
          local.get $#local1
          i32.const 4
          i32.add
          local.set $#local1
          i32.const 16843008
          local.get $#local2
          i32.const 4
          i32.add
          local.tee $#local2
          i32.load
          local.tee $#local3
          i32.sub
          local.get $#local3
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.eq
          br_if $#label1
        end
        loop $#label1
          local.get $#local1
          i32.const 1
          i32.add
          local.set $#local1
          local.get $#local2
          i32.load8_u
          local.set $#local3
          local.get $#local2
          i32.const 1
          i32.add
          local.set $#local2
          local.get $#local3
          br_if $#label1
        end
      end
      local.get $#local1
      local.get $#local0
      i32.sub
    )
    (func $strncmp (;154;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32)
      block $#label0
        local.get $#local2
        br_if $#label0
        i32.const 0
        return
      end
      block $#label0
        block $#label1
          local.get $#local0
          i32.load8_u
          local.tee $#local3
          br_if $#label1
          i32.const 0
          local.set $#local3
          br $#label0
        end
        local.get $#local0
        i32.const 1
        i32.add
        local.set $#local0
        local.get $#local2
        i32.const -1
        i32.add
        local.set $#local2
        block $#label1
          loop $#label2
            local.get $#local3
            i32.const 255
            i32.and
            local.get $#local1
            i32.load8_u
            local.tee $#local4
            i32.ne
            br_if $#label1
            local.get $#local4
            i32.eqz
            br_if $#label1
            local.get $#local2
            i32.const 0
            i32.eq
            br_if $#label1
            local.get $#local2
            i32.const -1
            i32.add
            local.set $#local2
            local.get $#local1
            i32.const 1
            i32.add
            local.set $#local1
            local.get $#local0
            i32.load8_u
            local.set $#local3
            local.get $#local0
            i32.const 1
            i32.add
            local.set $#local0
            local.get $#local3
            br_if $#label2
          end
          i32.const 0
          local.set $#local3
        end
        local.get $#local3
        i32.const 255
        i32.and
        local.set $#local3
      end
      local.get $#local3
      local.get $#local1
      i32.load8_u
      i32.sub
    )
    (func $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE (;155;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local1
      i32.const 0
      i32.store offset=24
      local.get $#local1
      i32.const 1
      i32.store offset=12
      local.get $#local1
      i64.const 4
      i64.store offset=16 align=4
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055532
      i32.add
      i32.store offset=8
      local.get $#local1
      i32.const 8
      i32.add
      local.get $#local0
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h2a56d21a869edb00E (;156;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local3
        i32.const 1
        i32.shl
        local.tee $#local4
        i32.const 8
        local.get $#local4
        i32.const 8
        i32.gt_u
        select
        local.tee $#local4
        i32.const 0
        i32.ge_s
        br_if $#label0
        i32.const 0
        i32.const 0
        local.get $#local1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      i32.const 0
      local.set $#local5
      block $#label0
        local.get $#local3
        i32.eqz
        br_if $#label0
        local.get $#local2
        local.get $#local3
        i32.store offset=28
        local.get $#local2
        local.get $#local0
        i32.load offset=4
        i32.store offset=20
        i32.const 1
        local.set $#local5
      end
      local.get $#local2
      local.get $#local5
      i32.store offset=24
      local.get $#local2
      i32.const 8
      i32.add
      i32.const 1
      local.get $#local4
      local.get $#local2
      i32.const 20
      i32.add
      call $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE
      block $#label0
        local.get $#local2
        i32.load offset=8
        i32.const 1
        i32.ne
        br_if $#label0
        local.get $#local2
        i32.load offset=12
        local.get $#local2
        i32.load offset=16
        local.get $#local1
        call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
        unreachable
      end
      local.get $#local2
      i32.load offset=12
      local.set $#local3
      local.get $#local0
      local.get $#local4
      i32.store
      local.get $#local0
      local.get $#local3
      i32.store offset=4
      local.get $#local2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE (;157;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      block $#label0
        local.get $#local0
        i32.eqz
        br_if $#label0
        local.get $#local0
        local.get $#local1
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get $#local2
      call $_ZN5alloc7raw_vec17capacity_overflow17h6d4c27211da198bdE
      unreachable
    )
    (func $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE (;158;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      block $#label0
        local.get $#local2
        i32.const 0
        i32.lt_s
        br_if $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local3
              i32.load offset=4
              i32.eqz
              br_if $#label3
              block $#label4
                local.get $#local3
                i32.load offset=8
                local.tee $#local4
                br_if $#label4
                block $#label5
                  local.get $#local2
                  br_if $#label5
                  local.get $#local1
                  local.set $#local3
                  br $#label1
                end
                global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
                i32.load8_u
                drop
                br $#label2
              end
              local.get $#local3
              i32.load
              local.get $#local4
              local.get $#local1
              local.get $#local2
              call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
              local.set $#local3
              br $#label1
            end
            block $#label3
              local.get $#local2
              br_if $#label3
              local.get $#local1
              local.set $#local3
              br $#label1
            end
            global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
            i32.load8_u
            drop
          end
          local.get $#local2
          local.get $#local1
          call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
          local.set $#local3
        end
        block $#label1
          local.get $#local3
          br_if $#label1
          local.get $#local0
          local.get $#local2
          i32.store offset=8
          local.get $#local0
          local.get $#local1
          i32.store offset=4
          local.get $#local0
          i32.const 1
          i32.store
          return
        end
        local.get $#local0
        local.get $#local2
        i32.store offset=8
        local.get $#local0
        local.get $#local3
        i32.store offset=4
        local.get $#local0
        i32.const 0
        i32.store
        return
      end
      local.get $#local0
      i32.const 0
      i32.store offset=4
      local.get $#local0
      i32.const 1
      i32.store
    )
    (func $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E (;159;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      local.get $#local1
      local.get $#local0
      call $_RNvCscSpY9Juk0HT_7___rustc26___rust_alloc_error_handler
      unreachable
    )
    (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h477892c24ea3724cE (;160;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.const -1
            i32.eq
            br_if $#label2
            i32.const 0
            local.set $#local4
            block $#label3
              local.get $#local2
              i32.const 1
              i32.add
              local.tee $#local5
              i32.const 0
              i32.lt_s
              br_if $#label3
              global.get $GOT.data.internal.__rust_no_alloc_shim_is_unstable
              i32.load8_u
              drop
              i32.const 1
              local.set $#local4
              local.get $#local5
              i32.const 1
              call $_RNvCscSpY9Juk0HT_7___rustc12___rust_alloc
              local.tee $#local6
              i32.eqz
              br_if $#label3
              block $#label4
                local.get $#local2
                i32.eqz
                br_if $#label4
                local.get $#local6
                local.get $#local1
                local.get $#local2
                memory.copy
              end
              block $#label4
                block $#label5
                  local.get $#local2
                  i32.const 7
                  i32.gt_u
                  br_if $#label5
                  local.get $#local2
                  i32.eqz
                  br_if $#label1
                  block $#label6
                    local.get $#local1
                    i32.load8_u
                    br_if $#label6
                    i32.const 0
                    local.set $#local4
                    br $#label4
                  end
                  i32.const 1
                  local.set $#local4
                  local.get $#local2
                  i32.const 1
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=1
                  i32.eqz
                  br_if $#label4
                  i32.const 2
                  local.set $#local4
                  local.get $#local2
                  i32.const 2
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=2
                  i32.eqz
                  br_if $#label4
                  i32.const 3
                  local.set $#local4
                  local.get $#local2
                  i32.const 3
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=3
                  i32.eqz
                  br_if $#label4
                  i32.const 4
                  local.set $#local4
                  local.get $#local2
                  i32.const 4
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=4
                  i32.eqz
                  br_if $#label4
                  i32.const 5
                  local.set $#local4
                  local.get $#local2
                  i32.const 5
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=5
                  i32.eqz
                  br_if $#label4
                  i32.const 6
                  local.set $#local4
                  local.get $#local2
                  i32.const 6
                  i32.eq
                  br_if $#label1
                  local.get $#local1
                  i32.load8_u offset=6
                  i32.eqz
                  br_if $#label4
                  br $#label1
                end
                local.get $#local3
                i32.const 8
                i32.add
                i32.const 0
                local.get $#local1
                local.get $#local2
                call $_ZN4core5slice6memchr14memchr_aligned17hf4db372f52bc45e4E
                local.get $#local3
                i32.load offset=8
                i32.const 1
                i32.and
                i32.eqz
                br_if $#label1
                local.get $#local3
                i32.load offset=12
                local.set $#local4
              end
              local.get $#local0
              local.get $#local4
              i32.store offset=12
              local.get $#local0
              local.get $#local2
              i32.store offset=8
              local.get $#local0
              local.get $#local6
              i32.store offset=4
              local.get $#local0
              local.get $#local5
              i32.store
              br $#label0
            end
            local.get $#local4
            local.get $#local5
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
        local.get $#local3
        local.get $#local2
        i32.store offset=28
        local.get $#local3
        local.get $#local6
        i32.store offset=24
        local.get $#local3
        local.get $#local5
        i32.store offset=20
        local.get $#local3
        local.get $#local3
        i32.const 20
        i32.add
        call $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE
        local.get $#local0
        local.get $#local3
        i64.load
        i64.store offset=4 align=4
        local.get $#local0
        i32.const -2147483648
        i32.store
      end
      local.get $#local3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h0db0245c18a3e2fbE (;161;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        local.get $#local1
        i32.load
        local.tee $#local3
        local.get $#local1
        i32.load offset=8
        local.tee $#local4
        i32.ne
        br_if $#label0
        i32.const 0
        local.set $#local5
        block $#label1
          block $#label2
            block $#label3
              local.get $#local4
              i32.const 1
              i32.add
              local.tee $#local3
              i32.const 0
              i32.ge_s
              br_if $#label3
              br $#label2
            end
            i32.const 0
            local.set $#local5
            block $#label3
              local.get $#local4
              i32.eqz
              br_if $#label3
              local.get $#local2
              local.get $#local4
              i32.store offset=28
              local.get $#local2
              local.get $#local1
              i32.load offset=4
              i32.store offset=20
              i32.const 1
              local.set $#local5
            end
            local.get $#local2
            local.get $#local5
            i32.store offset=24
            local.get $#local2
            i32.const 8
            i32.add
            i32.const 1
            local.get $#local3
            local.get $#local2
            i32.const 20
            i32.add
            call $_ZN5alloc7raw_vec11finish_grow17hcc8a680405f1fc3eE
            local.get $#local2
            i32.load offset=8
            i32.const 1
            i32.ne
            br_if $#label1
            local.get $#local2
            i32.load offset=16
            local.set $#local1
            local.get $#local2
            i32.load offset=12
            local.set $#local5
          end
          local.get $#local5
          local.get $#local1
          global.get $GOT.data.internal.__memory_base
          i32.const 1055572
          i32.add
          call $_ZN5alloc7raw_vec12handle_error17h92ac2e09c8a3cb2fE
          unreachable
        end
        local.get $#local2
        i32.load offset=12
        local.set $#local5
        local.get $#local1
        local.get $#local3
        i32.store
        local.get $#local1
        local.get $#local5
        i32.store offset=4
      end
      local.get $#local1
      local.get $#local4
      i32.const 1
      i32.add
      local.tee $#local5
      i32.store offset=8
      local.get $#local1
      i32.load offset=4
      local.tee $#local1
      local.get $#local4
      i32.add
      i32.const 0
      i32.store8
      block $#label0
        block $#label1
          local.get $#local3
          local.get $#local5
          i32.gt_u
          br_if $#label1
          local.get $#local1
          local.set $#local4
          br $#label0
        end
        block $#label1
          local.get $#local5
          br_if $#label1
          i32.const 1
          local.set $#local4
          local.get $#local1
          local.get $#local3
          i32.const 1
          call $_RNvCscSpY9Juk0HT_7___rustc14___rust_dealloc
          br $#label0
        end
        local.get $#local1
        local.get $#local3
        i32.const 1
        local.get $#local5
        call $_RNvCscSpY9Juk0HT_7___rustc14___rust_realloc
        local.tee $#local4
        br_if $#label0
        i32.const 1
        local.get $#local5
        call $_ZN5alloc5alloc18handle_alloc_error17h90a0b861d5c3cd31E
        unreachable
      end
      local.get $#local0
      local.get $#local5
      i32.store offset=4
      local.get $#local0
      local.get $#local4
      i32.store
      local.get $#local2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E (;162;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      call $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE (;163;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      call $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE
      unreachable
    )
    (func $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E (;164;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      block $#label0
        block $#label1
          local.get $#local0
          i32.load offset=8
          local.tee $#local3
          i32.const 402653184
          i32.and
          i32.eqz
          br_if $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local3
                i32.const 268435456
                i32.and
                br_if $#label4
                local.get $#local2
                i32.const 16
                i32.lt_u
                br_if $#label3
                local.get $#local1
                local.get $#local2
                call $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E
                local.set $#local4
                br $#label2
              end
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local0
                    i32.load16_u offset=14
                    local.tee $#local5
                    br_if $#label6
                    i32.const 0
                    local.set $#local2
                    br $#label5
                  end
                  local.get $#local1
                  local.get $#local2
                  i32.add
                  local.set $#local6
                  i32.const 0
                  local.set $#local2
                  local.get $#local5
                  local.set $#local7
                  local.get $#local1
                  local.set $#local8
                  loop $#label6
                    local.get $#local8
                    local.tee $#local4
                    local.get $#local6
                    i32.eq
                    br_if $#label4
                    block $#label7
                      block $#label8
                        local.get $#local4
                        i32.load8_s
                        local.tee $#local8
                        i32.const -1
                        i32.le_s
                        br_if $#label8
                        local.get $#local4
                        i32.const 1
                        i32.add
                        local.set $#local8
                        br $#label7
                      end
                      block $#label8
                        local.get $#local8
                        i32.const -32
                        i32.ge_u
                        br_if $#label8
                        local.get $#local4
                        i32.const 2
                        i32.add
                        local.set $#local8
                        br $#label7
                      end
                      block $#label8
                        local.get $#local8
                        i32.const -16
                        i32.ge_u
                        br_if $#label8
                        local.get $#local4
                        i32.const 3
                        i32.add
                        local.set $#local8
                        br $#label7
                      end
                      local.get $#local4
                      i32.const 4
                      i32.add
                      local.set $#local8
                    end
                    local.get $#local8
                    local.get $#local4
                    i32.sub
                    local.get $#local2
                    i32.add
                    local.set $#local2
                    local.get $#local7
                    i32.const -1
                    i32.add
                    local.tee $#local7
                    br_if $#label6
                  end
                end
                i32.const 0
                local.set $#local7
              end
              local.get $#local5
              local.get $#local7
              i32.sub
              local.set $#local4
              br $#label2
            end
            block $#label3
              local.get $#local2
              br_if $#label3
              i32.const 0
              local.set $#local2
              i32.const 0
              local.set $#local4
              br $#label2
            end
            local.get $#local2
            i32.const 3
            i32.and
            local.set $#local6
            block $#label3
              block $#label4
                local.get $#local2
                i32.const 4
                i32.ge_u
                br_if $#label4
                i32.const 0
                local.set $#local4
                i32.const 0
                local.set $#local7
                br $#label3
              end
              local.get $#local2
              i32.const 12
              i32.and
              local.set $#local5
              i32.const 0
              local.set $#local4
              i32.const 0
              local.set $#local7
              loop $#label4
                local.get $#local4
                local.get $#local1
                local.get $#local7
                i32.add
                local.tee $#local8
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local8
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local8
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local8
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set $#local4
                local.get $#local5
                local.get $#local7
                i32.const 4
                i32.add
                local.tee $#local7
                i32.ne
                br_if $#label4
              end
            end
            local.get $#local6
            i32.eqz
            br_if $#label2
            local.get $#local1
            local.get $#local7
            i32.add
            local.set $#local8
            loop $#label3
              local.get $#local4
              local.get $#local8
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set $#local4
              local.get $#local8
              i32.const 1
              i32.add
              local.set $#local8
              local.get $#local6
              i32.const -1
              i32.add
              local.tee $#local6
              br_if $#label3
            end
          end
          local.get $#local4
          local.get $#local0
          i32.load16_u offset=12
          local.tee $#local8
          i32.ge_u
          br_if $#label1
          local.get $#local8
          local.get $#local4
          i32.sub
          local.set $#local9
          i32.const 0
          local.set $#local4
          i32.const 0
          local.set $#local5
          block $#label2
            block $#label3
              block $#label4
                local.get $#local3
                i32.const 29
                i32.shr_u
                i32.const 3
                i32.and
                br_table $#label2 $#label4 $#label3 $#label2 $#label2
              end
              local.get $#local9
              local.set $#local5
              br $#label2
            end
            local.get $#local9
            i32.const 65534
            i32.and
            i32.const 1
            i32.shr_u
            local.set $#local5
          end
          local.get $#local3
          i32.const 2097151
          i32.and
          local.set $#local6
          local.get $#local0
          i32.load offset=4
          local.set $#local7
          local.get $#local0
          i32.load
          local.set $#local0
          block $#label2
            loop $#label3
              local.get $#local4
              i32.const 65535
              i32.and
              local.get $#local5
              i32.const 65535
              i32.and
              i32.ge_u
              br_if $#label2
              i32.const 1
              local.set $#local8
              local.get $#local4
              i32.const 1
              i32.add
              local.set $#local4
              local.get $#local0
              local.get $#local6
              local.get $#local7
              i32.load offset=16
              call_indirect (type $#type2)
              br_if $#label0
              br $#label3
            end
          end
          i32.const 1
          local.set $#local8
          local.get $#local0
          local.get $#local1
          local.get $#local2
          local.get $#local7
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
          i32.const 0
          local.set $#local4
          local.get $#local9
          local.get $#local5
          i32.sub
          i32.const 65535
          i32.and
          local.set $#local2
          loop $#label2
            local.get $#local4
            i32.const 65535
            i32.and
            local.tee $#local5
            local.get $#local2
            i32.lt_u
            local.set $#local8
            local.get $#local5
            local.get $#local2
            i32.ge_u
            br_if $#label0
            local.get $#local4
            i32.const 1
            i32.add
            local.set $#local4
            local.get $#local0
            local.get $#local6
            local.get $#local7
            i32.load offset=16
            call_indirect (type $#type2)
            br_if $#label0
            br $#label2
          end
        end
        local.get $#local0
        i32.load
        local.get $#local1
        local.get $#local2
        local.get $#local0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type $#type5)
        local.set $#local8
      end
      local.get $#local8
    )
    (func $_ZN4core9panicking5panic17hd836709591dfc35fE (;165;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      i32.const 0
      i32.store offset=16
      local.get $#local3
      i32.const 1
      i32.store offset=4
      local.get $#local3
      i64.const 4
      i64.store offset=8 align=4
      local.get $#local3
      local.get $#local1
      i32.store offset=28
      local.get $#local3
      local.get $#local0
      i32.store offset=24
      local.get $#local3
      local.get $#local3
      i32.const 24
      i32.add
      i32.store
      local.get $#local3
      local.get $#local2
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE (;166;) (type $#type6) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i32.const 1
      i32.store16 offset=12
      local.get $#local2
      local.get $#local1
      i32.store offset=8
      local.get $#local2
      local.get $#local0
      i32.store offset=4
      local.get $#local2
      i32.const 4
      i32.add
      call $_RNvCscSpY9Juk0HT_7___rustc17rust_begin_unwind
      unreachable
    )
    (func $_ZN4core3fmt5write17h19dbf2ffaf30f068E (;167;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      i32.store offset=4
      local.get $#local3
      local.get $#local0
      i32.store
      local.get $#local3
      i64.const 3758096416
      i64.store offset=8 align=4
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local2
                i32.load offset=16
                local.tee $#local4
                i32.eqz
                br_if $#label4
                local.get $#local2
                i32.load offset=20
                local.tee $#local1
                br_if $#label3
                br $#label2
              end
              local.get $#local2
              i32.load offset=12
              local.tee $#local0
              i32.eqz
              br_if $#label2
              local.get $#local2
              i32.load offset=8
              local.tee $#local1
              local.get $#local0
              i32.const 3
              i32.shl
              i32.add
              local.set $#local5
              local.get $#local0
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set $#local6
              local.get $#local2
              i32.load
              local.set $#local0
              loop $#label4
                block $#label5
                  local.get $#local0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee $#local7
                  i32.eqz
                  br_if $#label5
                  local.get $#local3
                  i32.load
                  local.get $#local0
                  i32.load
                  local.get $#local7
                  local.get $#local3
                  i32.load offset=4
                  i32.load offset=12
                  call_indirect (type $#type5)
                  i32.eqz
                  br_if $#label5
                  i32.const 1
                  local.set $#local1
                  br $#label0
                end
                block $#label5
                  local.get $#local1
                  i32.load
                  local.get $#local3
                  local.get $#local1
                  i32.const 4
                  i32.add
                  i32.load
                  call_indirect (type $#type2)
                  i32.eqz
                  br_if $#label5
                  i32.const 1
                  local.set $#local1
                  br $#label0
                end
                local.get $#local0
                i32.const 8
                i32.add
                local.set $#local0
                local.get $#local1
                i32.const 8
                i32.add
                local.tee $#local1
                local.get $#local5
                i32.eq
                br_if $#label1
                br $#label4
              end
            end
            local.get $#local1
            i32.const 24
            i32.mul
            local.set $#local8
            local.get $#local1
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set $#local6
            local.get $#local2
            i32.load offset=8
            local.set $#local9
            local.get $#local2
            i32.load
            local.set $#local0
            i32.const 0
            local.set $#local7
            loop $#label3
              block $#label4
                local.get $#local0
                i32.const 4
                i32.add
                i32.load
                local.tee $#local1
                i32.eqz
                br_if $#label4
                local.get $#local3
                i32.load
                local.get $#local0
                i32.load
                local.get $#local1
                local.get $#local3
                i32.load offset=4
                i32.load offset=12
                call_indirect (type $#type5)
                i32.eqz
                br_if $#label4
                i32.const 1
                local.set $#local1
                br $#label0
              end
              i32.const 0
              local.set $#local5
              i32.const 0
              local.set $#local10
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local4
                    local.get $#local7
                    i32.add
                    local.tee $#local1
                    i32.const 8
                    i32.add
                    i32.load16_u
                    br_table $#label6 $#label5 $#label4 $#label6
                  end
                  local.get $#local1
                  i32.const 10
                  i32.add
                  i32.load16_u
                  local.set $#local10
                  br $#label4
                end
                local.get $#local9
                local.get $#local1
                i32.const 12
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                i32.load16_u offset=4
                local.set $#local10
              end
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local1
                    i32.load16_u
                    br_table $#label6 $#label5 $#label4 $#label6
                  end
                  local.get $#local1
                  i32.const 2
                  i32.add
                  i32.load16_u
                  local.set $#local5
                  br $#label4
                end
                local.get $#local9
                local.get $#local1
                i32.const 4
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                i32.load16_u offset=4
                local.set $#local5
              end
              local.get $#local3
              local.get $#local5
              i32.store16 offset=14
              local.get $#local3
              local.get $#local10
              i32.store16 offset=12
              local.get $#local3
              local.get $#local1
              i32.const 20
              i32.add
              i32.load
              i32.store offset=8
              block $#label4
                local.get $#local9
                local.get $#local1
                i32.const 16
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                local.tee $#local1
                i32.load
                local.get $#local3
                local.get $#local1
                i32.const 4
                i32.add
                i32.load
                call_indirect (type $#type2)
                i32.eqz
                br_if $#label4
                i32.const 1
                local.set $#local1
                br $#label0
              end
              local.get $#local0
              i32.const 8
              i32.add
              local.set $#local0
              local.get $#local8
              local.get $#local7
              i32.const 24
              i32.add
              local.tee $#local7
              i32.eq
              br_if $#label1
              br $#label3
            end
          end
          i32.const 0
          local.set $#local6
        end
        block $#label1
          local.get $#local6
          local.get $#local2
          i32.load offset=4
          i32.ge_u
          br_if $#label1
          local.get $#local3
          i32.load
          local.get $#local2
          i32.load
          local.get $#local6
          i32.const 3
          i32.shl
          i32.add
          local.tee $#local1
          i32.load
          local.get $#local1
          i32.load offset=4
          local.get $#local3
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          i32.eqz
          br_if $#label1
          i32.const 1
          local.set $#local1
          br $#label0
        end
        i32.const 0
        local.set $#local1
      end
      local.get $#local3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE (;168;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      i32.const 10
      local.set $#local4
      local.get $#local0
      local.set $#local5
      block $#label0
        local.get $#local0
        i32.const 1000
        i32.lt_u
        br_if $#label0
        i32.const 10
        local.set $#local4
        local.get $#local0
        local.set $#local6
        loop $#label1
          local.get $#local3
          i32.const 6
          i32.add
          local.get $#local4
          i32.add
          local.tee $#local7
          i32.const -4
          i32.add
          global.get $GOT.data.internal.__memory_base
          i32.const 1053532
          i32.add
          local.tee $#local8
          local.get $#local6
          local.get $#local6
          i32.const 10000
          i32.div_u
          local.tee $#local5
          i32.const 10000
          i32.mul
          i32.sub
          local.tee $#local9
          i32.const 65535
          i32.and
          i32.const 100
          i32.div_u
          local.tee $#local10
          i32.const 1
          i32.shl
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          local.get $#local7
          i32.const -2
          i32.add
          local.get $#local8
          local.get $#local9
          local.get $#local10
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
          local.get $#local4
          i32.const -4
          i32.add
          local.set $#local4
          local.get $#local6
          i32.const 9999999
          i32.gt_u
          local.set $#local7
          local.get $#local5
          local.set $#local6
          local.get $#local7
          br_if $#label1
        end
      end
      block $#label0
        block $#label1
          local.get $#local5
          i32.const 9
          i32.gt_u
          br_if $#label1
          local.get $#local5
          local.set $#local6
          br $#label0
        end
        local.get $#local3
        i32.const 6
        i32.add
        local.get $#local4
        i32.const -2
        i32.add
        local.tee $#local4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get $#local5
        local.get $#local5
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee $#local6
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
      block $#label0
        block $#label1
          local.get $#local0
          i32.eqz
          br_if $#label1
          local.get $#local6
          i32.eqz
          br_if $#label0
        end
        local.get $#local3
        i32.const 6
        i32.add
        local.get $#local4
        i32.const -1
        i32.add
        local.tee $#local4
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get $#local6
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
      local.get $#local2
      local.get $#local1
      i32.const 1
      i32.const 0
      local.get $#local3
      i32.const 6
      i32.add
      local.get $#local4
      i32.add
      i32.const 10
      local.get $#local4
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local6
      local.get $#local3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local6
    )
    (func $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE (;169;) (type $#type13) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (param $#local5 i32) (result i32)
      (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32) (local $#local12 i32) (local $#local13 i32) (local $#local14 i64)
      block $#label0
        block $#label1
          local.get $#local1
          br_if $#label1
          local.get $#local5
          i32.const 1
          i32.add
          local.set $#local6
          local.get $#local0
          i32.load offset=8
          local.set $#local7
          i32.const 45
          local.set $#local8
          br $#label0
        end
        i32.const 43
        i32.const 1114112
        local.get $#local0
        i32.load offset=8
        local.tee $#local7
        i32.const 2097152
        i32.and
        local.tee $#local1
        select
        local.set $#local8
        local.get $#local1
        i32.const 21
        i32.shr_u
        local.get $#local5
        i32.add
        local.set $#local6
      end
      block $#label0
        block $#label1
          local.get $#local7
          i32.const 8388608
          i32.and
          br_if $#label1
          i32.const 0
          local.set $#local2
          br $#label0
        end
        block $#label1
          local.get $#local3
          i32.const 16
          i32.lt_u
          br_if $#label1
          local.get $#local2
          local.get $#local3
          call $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E
          local.get $#local6
          i32.add
          local.set $#local6
          br $#label0
        end
        block $#label1
          local.get $#local3
          br_if $#label1
          i32.const 0
          local.get $#local6
          i32.add
          local.set $#local6
          br $#label0
        end
        local.get $#local3
        i32.const 3
        i32.and
        local.set $#local9
        block $#label1
          block $#label2
            local.get $#local3
            i32.const 4
            i32.ge_u
            br_if $#label2
            i32.const 0
            local.set $#local1
            i32.const 0
            local.set $#local10
            br $#label1
          end
          local.get $#local3
          i32.const 12
          i32.and
          local.set $#local11
          i32.const 0
          local.set $#local1
          i32.const 0
          local.set $#local10
          loop $#label2
            local.get $#local1
            local.get $#local2
            local.get $#local10
            i32.add
            local.tee $#local12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local12
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local12
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local12
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set $#local1
            local.get $#local11
            local.get $#local10
            i32.const 4
            i32.add
            local.tee $#local10
            i32.ne
            br_if $#label2
          end
        end
        block $#label1
          local.get $#local9
          i32.eqz
          br_if $#label1
          local.get $#local2
          local.get $#local10
          i32.add
          local.set $#local12
          loop $#label2
            local.get $#local1
            local.get $#local12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set $#local1
            local.get $#local12
            i32.const 1
            i32.add
            local.set $#local12
            local.get $#local9
            i32.const -1
            i32.add
            local.tee $#local9
            br_if $#label2
          end
        end
        local.get $#local1
        local.get $#local6
        i32.add
        local.set $#local6
      end
      block $#label0
        block $#label1
          local.get $#local6
          local.get $#local0
          i32.load16_u offset=12
          local.tee $#local11
          i32.ge_u
          br_if $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local7
                i32.const 16777216
                i32.and
                br_if $#label4
                local.get $#local11
                local.get $#local6
                i32.sub
                local.set $#local13
                i32.const 0
                local.set $#local1
                i32.const 0
                local.set $#local11
                block $#label5
                  block $#label6
                    block $#label7
                      local.get $#local7
                      i32.const 29
                      i32.shr_u
                      i32.const 3
                      i32.and
                      br_table $#label5 $#label7 $#label6 $#label7 $#label5
                    end
                    local.get $#local13
                    local.set $#local11
                    br $#label5
                  end
                  local.get $#local13
                  i32.const 65534
                  i32.and
                  i32.const 1
                  i32.shr_u
                  local.set $#local11
                end
                local.get $#local7
                i32.const 2097151
                i32.and
                local.set $#local6
                local.get $#local0
                i32.load offset=4
                local.set $#local9
                local.get $#local0
                i32.load
                local.set $#local10
                loop $#label5
                  local.get $#local1
                  i32.const 65535
                  i32.and
                  local.get $#local11
                  i32.const 65535
                  i32.and
                  i32.ge_u
                  br_if $#label3
                  i32.const 1
                  local.set $#local12
                  local.get $#local1
                  i32.const 1
                  i32.add
                  local.set $#local1
                  local.get $#local10
                  local.get $#local6
                  local.get $#local9
                  i32.load offset=16
                  call_indirect (type $#type2)
                  i32.eqz
                  br_if $#label5
                  br $#label0
                end
              end
              local.get $#local0
              local.get $#local0
              i64.load offset=8 align=4
              local.tee $#local14
              i32.wrap_i64
              i32.const -1612709888
              i32.and
              i32.const 536870960
              i32.or
              i32.store offset=8
              i32.const 1
              local.set $#local12
              local.get $#local0
              i32.load
              local.tee $#local10
              local.get $#local0
              i32.load offset=4
              local.tee $#local9
              local.get $#local8
              local.get $#local2
              local.get $#local3
              call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
              br_if $#label0
              i32.const 0
              local.set $#local1
              local.get $#local11
              local.get $#local6
              i32.sub
              i32.const 65535
              i32.and
              local.set $#local2
              loop $#label4
                local.get $#local1
                i32.const 65535
                i32.and
                local.get $#local2
                i32.ge_u
                br_if $#label2
                i32.const 1
                local.set $#local12
                local.get $#local1
                i32.const 1
                i32.add
                local.set $#local1
                local.get $#local10
                i32.const 48
                local.get $#local9
                i32.load offset=16
                call_indirect (type $#type2)
                i32.eqz
                br_if $#label4
                br $#label0
              end
            end
            i32.const 1
            local.set $#local12
            local.get $#local10
            local.get $#local9
            local.get $#local8
            local.get $#local2
            local.get $#local3
            call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
            br_if $#label0
            local.get $#local10
            local.get $#local4
            local.get $#local5
            local.get $#local9
            i32.load offset=12
            call_indirect (type $#type5)
            br_if $#label0
            i32.const 0
            local.set $#local1
            local.get $#local13
            local.get $#local11
            i32.sub
            i32.const 65535
            i32.and
            local.set $#local0
            loop $#label3
              local.get $#local1
              i32.const 65535
              i32.and
              local.tee $#local2
              local.get $#local0
              i32.lt_u
              local.set $#local12
              local.get $#local2
              local.get $#local0
              i32.ge_u
              br_if $#label0
              local.get $#local1
              i32.const 1
              i32.add
              local.set $#local1
              local.get $#local10
              local.get $#local6
              local.get $#local9
              i32.load offset=16
              call_indirect (type $#type2)
              i32.eqz
              br_if $#label3
              br $#label0
            end
          end
          i32.const 1
          local.set $#local12
          local.get $#local10
          local.get $#local4
          local.get $#local5
          local.get $#local9
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
          local.get $#local0
          local.get $#local14
          i64.store offset=8 align=4
          i32.const 0
          return
        end
        i32.const 1
        local.set $#local12
        local.get $#local0
        i32.load
        local.tee $#local1
        local.get $#local0
        i32.load offset=4
        local.tee $#local10
        local.get $#local8
        local.get $#local2
        local.get $#local3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E
        br_if $#label0
        local.get $#local1
        local.get $#local4
        local.get $#local5
        local.get $#local10
        i32.load offset=12
        call_indirect (type $#type5)
        local.set $#local12
      end
      local.get $#local12
    )
    (func $_ZN4core3str5count14do_count_chars17hf2c60e49a7acad70E (;170;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      block $#label0
        block $#label1
          local.get $#local1
          local.get $#local0
          i32.const 3
          i32.add
          i32.const -4
          i32.and
          local.tee $#local2
          local.get $#local0
          i32.sub
          local.tee $#local3
          i32.lt_u
          br_if $#label1
          local.get $#local1
          local.get $#local3
          i32.sub
          local.tee $#local4
          i32.const 4
          i32.lt_u
          br_if $#label1
          local.get $#local4
          i32.const 3
          i32.and
          local.set $#local5
          i32.const 0
          local.set $#local6
          i32.const 0
          local.set $#local1
          block $#label2
            local.get $#local2
            local.get $#local0
            i32.eq
            local.tee $#local7
            br_if $#label2
            i32.const 0
            local.set $#local1
            block $#label3
              block $#label4
                local.get $#local0
                local.get $#local2
                i32.sub
                local.tee $#local8
                i32.const -4
                i32.le_u
                br_if $#label4
                i32.const 0
                local.set $#local9
                br $#label3
              end
              i32.const 0
              local.set $#local9
              loop $#label4
                local.get $#local1
                local.get $#local0
                local.get $#local9
                i32.add
                local.tee $#local2
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local2
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local2
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get $#local2
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set $#local1
                local.get $#local9
                i32.const 4
                i32.add
                local.tee $#local9
                br_if $#label4
              end
            end
            local.get $#local7
            br_if $#label2
            local.get $#local0
            local.get $#local9
            i32.add
            local.set $#local2
            loop $#label3
              local.get $#local1
              local.get $#local2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set $#local1
              local.get $#local2
              i32.const 1
              i32.add
              local.set $#local2
              local.get $#local8
              i32.const 1
              i32.add
              local.tee $#local8
              br_if $#label3
            end
          end
          local.get $#local0
          local.get $#local3
          i32.add
          local.set $#local0
          block $#label2
            local.get $#local5
            i32.eqz
            br_if $#label2
            local.get $#local0
            local.get $#local4
            i32.const -4
            i32.and
            i32.add
            local.tee $#local2
            i32.load8_s
            i32.const -65
            i32.gt_s
            local.set $#local6
            local.get $#local5
            i32.const 1
            i32.eq
            br_if $#label2
            local.get $#local6
            local.get $#local2
            i32.load8_s offset=1
            i32.const -65
            i32.gt_s
            i32.add
            local.set $#local6
            local.get $#local5
            i32.const 2
            i32.eq
            br_if $#label2
            local.get $#local6
            local.get $#local2
            i32.load8_s offset=2
            i32.const -65
            i32.gt_s
            i32.add
            local.set $#local6
          end
          local.get $#local4
          i32.const 2
          i32.shr_u
          local.set $#local8
          local.get $#local6
          local.get $#local1
          i32.add
          local.set $#local3
          loop $#label2
            local.get $#local0
            local.set $#local4
            local.get $#local8
            i32.eqz
            br_if $#label0
            local.get $#local8
            i32.const 192
            local.get $#local8
            i32.const 192
            i32.lt_u
            select
            local.tee $#local6
            i32.const 3
            i32.and
            local.set $#local7
            local.get $#local6
            i32.const 2
            i32.shl
            local.set $#local5
            i32.const 0
            local.set $#local2
            block $#label3
              local.get $#local8
              i32.const 4
              i32.lt_u
              br_if $#label3
              local.get $#local4
              local.get $#local5
              i32.const 1008
              i32.and
              i32.add
              local.set $#local9
              i32.const 0
              local.set $#local2
              local.get $#local4
              local.set $#local1
              loop $#label4
                local.get $#local1
                i32.const 12
                i32.add
                i32.load
                local.tee $#local0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get $#local0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get $#local1
                i32.const 8
                i32.add
                i32.load
                local.tee $#local0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get $#local0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get $#local1
                i32.const 4
                i32.add
                i32.load
                local.tee $#local0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get $#local0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get $#local1
                i32.load
                local.tee $#local0
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get $#local0
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get $#local2
                i32.add
                i32.add
                i32.add
                i32.add
                local.set $#local2
                local.get $#local1
                i32.const 16
                i32.add
                local.tee $#local1
                local.get $#local9
                i32.ne
                br_if $#label4
              end
            end
            local.get $#local8
            local.get $#local6
            i32.sub
            local.set $#local8
            local.get $#local4
            local.get $#local5
            i32.add
            local.set $#local0
            local.get $#local2
            i32.const 8
            i32.shr_u
            i32.const 16711935
            i32.and
            local.get $#local2
            i32.const 16711935
            i32.and
            i32.add
            i32.const 65537
            i32.mul
            i32.const 16
            i32.shr_u
            local.get $#local3
            i32.add
            local.set $#local3
            local.get $#local7
            i32.eqz
            br_if $#label2
          end
          local.get $#local4
          local.get $#local6
          i32.const 252
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.tee $#local2
          i32.load
          local.tee $#local1
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get $#local1
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.set $#local1
          block $#label2
            local.get $#local7
            i32.const 1
            i32.eq
            br_if $#label2
            local.get $#local2
            i32.load offset=4
            local.tee $#local0
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get $#local0
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get $#local1
            i32.add
            local.set $#local1
            local.get $#local7
            i32.const 2
            i32.eq
            br_if $#label2
            local.get $#local2
            i32.load offset=8
            local.tee $#local2
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get $#local2
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get $#local1
            i32.add
            local.set $#local1
          end
          local.get $#local1
          i32.const 8
          i32.shr_u
          i32.const 459007
          i32.and
          local.get $#local1
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get $#local3
          i32.add
          return
        end
        block $#label1
          local.get $#local1
          br_if $#label1
          i32.const 0
          return
        end
        local.get $#local1
        i32.const 3
        i32.and
        local.set $#local9
        block $#label1
          block $#label2
            local.get $#local1
            i32.const 4
            i32.ge_u
            br_if $#label2
            i32.const 0
            local.set $#local3
            i32.const 0
            local.set $#local2
            br $#label1
          end
          local.get $#local1
          i32.const -4
          i32.and
          local.set $#local8
          i32.const 0
          local.set $#local3
          i32.const 0
          local.set $#local2
          loop $#label2
            local.get $#local3
            local.get $#local0
            local.get $#local2
            i32.add
            local.tee $#local1
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local1
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local1
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get $#local1
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set $#local3
            local.get $#local8
            local.get $#local2
            i32.const 4
            i32.add
            local.tee $#local2
            i32.ne
            br_if $#label2
          end
        end
        local.get $#local9
        i32.eqz
        br_if $#label0
        local.get $#local0
        local.get $#local2
        i32.add
        local.set $#local1
        loop $#label1
          local.get $#local3
          local.get $#local1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set $#local3
          local.get $#local1
          i32.const 1
          i32.add
          local.set $#local1
          local.get $#local9
          i32.const -1
          i32.add
          local.tee $#local9
          br_if $#label1
        end
      end
      local.get $#local3
    )
    (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E (;171;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      global.get $GOT.data.internal.__memory_base
      local.set $#local2
      local.get $#local1
      i32.load
      local.get $#local2
      i32.const 1053348
      i32.add
      i32.const 14
      local.get $#local1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type $#type5)
    )
    (func $_ZN4core4cell22panic_already_borrowed17h8a14b8454fa658ceE (;172;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local1
      i32.const 1
      i32.store offset=12
      local.get $#local1
      i64.const 1
      i64.store offset=20 align=4
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1055588
      i32.add
      i32.store offset=8
      local.get $#local1
      global.get $GOT.func.internal._ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcedb5f6ac7c56fb9E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local1
      i32.const 47
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local1
      local.get $#local1
      i32.const 32
      i32.add
      i32.store offset=16
      local.get $#local1
      i32.const 8
      i32.add
      local.get $#local0
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h04ddcd8be7687b8aE (;173;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32)
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                local.get $#local2
                i32.const 7
                i32.gt_u
                br_if $#label4
                local.get $#local2
                i32.eqz
                br_if $#label1
                local.get $#local1
                i32.load8_u
                br_if $#label3
                i32.const 0
                local.set $#local3
                br $#label0
              end
              block $#label4
                block $#label5
                  local.get $#local1
                  i32.const 3
                  i32.add
                  i32.const -4
                  i32.and
                  local.get $#local1
                  i32.sub
                  local.tee $#local4
                  i32.eqz
                  br_if $#label5
                  i32.const 0
                  local.set $#local3
                  loop $#label6
                    local.get $#local1
                    local.get $#local3
                    i32.add
                    i32.load8_u
                    i32.eqz
                    br_if $#label0
                    local.get $#local4
                    local.get $#local3
                    i32.const 1
                    i32.add
                    local.tee $#local3
                    i32.ne
                    br_if $#label6
                  end
                  local.get $#local4
                  local.get $#local2
                  i32.const -8
                  i32.add
                  local.tee $#local5
                  i32.le_u
                  br_if $#label4
                  br $#label2
                end
                local.get $#local2
                i32.const -8
                i32.add
                local.set $#local5
              end
              loop $#label4
                i32.const 16843008
                local.get $#local1
                local.get $#local4
                i32.add
                local.tee $#local3
                i32.load
                local.tee $#local6
                i32.sub
                local.get $#local6
                i32.or
                i32.const 16843008
                local.get $#local3
                i32.const 4
                i32.add
                i32.load
                local.tee $#local3
                i32.sub
                local.get $#local3
                i32.or
                i32.and
                i32.const -2139062144
                i32.and
                i32.const -2139062144
                i32.ne
                br_if $#label2
                local.get $#local4
                i32.const 8
                i32.add
                local.tee $#local4
                local.get $#local5
                i32.le_u
                br_if $#label4
                br $#label2
              end
            end
            i32.const 1
            local.set $#local3
            local.get $#local2
            i32.const 1
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=1
            i32.eqz
            br_if $#label0
            i32.const 2
            local.set $#local3
            local.get $#local2
            i32.const 2
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=2
            i32.eqz
            br_if $#label0
            i32.const 3
            local.set $#local3
            local.get $#local2
            i32.const 3
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=3
            i32.eqz
            br_if $#label0
            i32.const 4
            local.set $#local3
            local.get $#local2
            i32.const 4
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=4
            i32.eqz
            br_if $#label0
            i32.const 5
            local.set $#local3
            local.get $#local2
            i32.const 5
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=5
            i32.eqz
            br_if $#label0
            i32.const 6
            local.set $#local3
            local.get $#local2
            i32.const 6
            i32.eq
            br_if $#label1
            local.get $#local1
            i32.load8_u offset=6
            br_if $#label1
            br $#label0
          end
          local.get $#local2
          local.get $#local4
          i32.eq
          br_if $#label1
          loop $#label2
            block $#label3
              local.get $#local1
              local.get $#local4
              i32.add
              i32.load8_u
              br_if $#label3
              local.get $#local4
              local.set $#local3
              br $#label0
            end
            local.get $#local2
            local.get $#local4
            i32.const 1
            i32.add
            local.tee $#local4
            i32.ne
            br_if $#label2
          end
        end
        local.get $#local0
        i32.const 1
        i32.store offset=4
        local.get $#local0
        i32.const 1
        i32.store
        return
      end
      block $#label0
        local.get $#local3
        i32.const 1
        i32.add
        local.get $#local2
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local3
        i32.store offset=8
        local.get $#local0
        i32.const 0
        i32.store offset=4
        local.get $#local0
        i32.const 1
        i32.store
        return
      end
      local.get $#local0
      local.get $#local2
      i32.store offset=8
      local.get $#local0
      local.get $#local1
      i32.store offset=4
      local.get $#local0
      i32.const 0
      i32.store
    )
    (func $_ZN4core3str8converts9from_utf817hc11b0c33b11310b8E (;174;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i64) (local $#local9 i64) (local $#local10 i32)
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        i32.const 0
        local.get $#local2
        i32.const -7
        i32.add
        local.tee $#local3
        local.get $#local3
        local.get $#local2
        i32.gt_u
        select
        local.set $#local4
        local.get $#local1
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.get $#local1
        i32.sub
        local.set $#local5
        i32.const 0
        local.set $#local3
        loop $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  local.get $#local1
                  local.get $#local3
                  i32.add
                  i32.load8_u
                  local.tee $#local6
                  i32.extend8_s
                  local.tee $#local7
                  i32.const 0
                  i32.lt_s
                  br_if $#label5
                  local.get $#local5
                  local.get $#local3
                  i32.sub
                  i32.const 3
                  i32.and
                  br_if $#label4
                  local.get $#local3
                  local.get $#local4
                  i32.ge_u
                  br_if $#label3
                  loop $#label6
                    local.get $#local1
                    local.get $#local3
                    i32.add
                    local.tee $#local6
                    i32.const 4
                    i32.add
                    i32.load
                    local.get $#local6
                    i32.load
                    i32.or
                    i32.const -2139062144
                    i32.and
                    br_if $#label3
                    local.get $#local3
                    i32.const 8
                    i32.add
                    local.tee $#local3
                    local.get $#local4
                    i32.lt_u
                    br_if $#label6
                    br $#label3
                  end
                end
                i64.const 1099511627776
                local.set $#local8
                i64.const 4294967296
                local.set $#local9
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          block $#label10
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    block $#label15
                                      block $#label16
                                        global.get $GOT.data.internal.__memory_base
                                        i32.const 1053773
                                        i32.add
                                        local.get $#local6
                                        i32.add
                                        i32.load8_u
                                        i32.const -2
                                        i32.add
                                        br_table $#label16 $#label15 $#label14 $#label6
                                      end
                                      local.get $#local3
                                      i32.const 1
                                      i32.add
                                      local.tee $#local6
                                      local.get $#local2
                                      i32.lt_u
                                      br_if $#label13
                                      i64.const 0
                                      local.set $#local8
                                      i64.const 0
                                      local.set $#local9
                                      br $#label6
                                    end
                                    i64.const 0
                                    local.set $#local8
                                    local.get $#local3
                                    i32.const 1
                                    i32.add
                                    local.tee $#local10
                                    local.get $#local2
                                    i32.lt_u
                                    br_if $#label12
                                    i64.const 0
                                    local.set $#local9
                                    br $#label6
                                  end
                                  i64.const 0
                                  local.set $#local8
                                  local.get $#local3
                                  i32.const 1
                                  i32.add
                                  local.tee $#local10
                                  local.get $#local2
                                  i32.lt_u
                                  br_if $#label11
                                  i64.const 0
                                  local.set $#local9
                                  br $#label6
                                end
                                i64.const 1099511627776
                                local.set $#local8
                                i64.const 4294967296
                                local.set $#local9
                                local.get $#local1
                                local.get $#local6
                                i32.add
                                i32.load8_s
                                i32.const -65
                                i32.gt_s
                                br_if $#label6
                                br $#label5
                              end
                              local.get $#local1
                              local.get $#local10
                              i32.add
                              i32.load8_s
                              local.set $#local10
                              block $#label12
                                block $#label13
                                  block $#label14
                                    local.get $#local6
                                    i32.const -224
                                    i32.add
                                    br_table $#label14 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label12 $#label13 $#label12
                                  end
                                  local.get $#local10
                                  i32.const -32
                                  i32.and
                                  i32.const -96
                                  i32.eq
                                  br_if $#label9
                                  br $#label10
                                end
                                local.get $#local10
                                i32.const -97
                                i32.gt_s
                                br_if $#label10
                                br $#label9
                              end
                              block $#label12
                                local.get $#local7
                                i32.const 31
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 12
                                i32.lt_u
                                br_if $#label12
                                local.get $#local7
                                i32.const -2
                                i32.and
                                i32.const -18
                                i32.ne
                                br_if $#label10
                                local.get $#local10
                                i32.const -64
                                i32.lt_s
                                br_if $#label9
                                br $#label10
                              end
                              local.get $#local10
                              i32.const -64
                              i32.lt_s
                              br_if $#label9
                              br $#label10
                            end
                            local.get $#local1
                            local.get $#local10
                            i32.add
                            i32.load8_s
                            local.set $#local10
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    local.get $#local6
                                    i32.const -240
                                    i32.add
                                    br_table $#label13 $#label14 $#label14 $#label14 $#label12 $#label14
                                  end
                                  local.get $#local7
                                  i32.const 15
                                  i32.add
                                  i32.const 255
                                  i32.and
                                  i32.const 2
                                  i32.gt_u
                                  br_if $#label10
                                  local.get $#local10
                                  i32.const -64
                                  i32.ge_s
                                  br_if $#label10
                                  br $#label11
                                end
                                local.get $#local10
                                i32.const 112
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 48
                                i32.ge_u
                                br_if $#label10
                                br $#label11
                              end
                              local.get $#local10
                              i32.const -113
                              i32.gt_s
                              br_if $#label10
                            end
                            block $#label11
                              local.get $#local3
                              i32.const 2
                              i32.add
                              local.tee $#local6
                              local.get $#local2
                              i32.lt_u
                              br_if $#label11
                              i64.const 0
                              local.set $#local9
                              br $#label6
                            end
                            local.get $#local1
                            local.get $#local6
                            i32.add
                            i32.load8_s
                            i32.const -65
                            i32.gt_s
                            br_if $#label8
                            i64.const 0
                            local.set $#local9
                            local.get $#local3
                            i32.const 3
                            i32.add
                            local.tee $#local6
                            local.get $#local2
                            i32.ge_u
                            br_if $#label6
                            local.get $#local1
                            local.get $#local6
                            i32.add
                            i32.load8_s
                            i32.const -64
                            i32.lt_s
                            br_if $#label5
                            i64.const 3298534883328
                            local.set $#local8
                            br $#label7
                          end
                          i64.const 1099511627776
                          local.set $#local8
                          br $#label7
                        end
                        i64.const 0
                        local.set $#local9
                        local.get $#local3
                        i32.const 2
                        i32.add
                        local.tee $#local6
                        local.get $#local2
                        i32.ge_u
                        br_if $#label6
                        local.get $#local1
                        local.get $#local6
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.le_s
                        br_if $#label5
                      end
                      i64.const 2199023255552
                      local.set $#local8
                    end
                    i64.const 4294967296
                    local.set $#local9
                  end
                  local.get $#local0
                  local.get $#local8
                  local.get $#local3
                  i64.extend_i32_u
                  i64.or
                  local.get $#local9
                  i64.or
                  i64.store offset=4 align=4
                  local.get $#local0
                  i32.const 1
                  i32.store
                  return
                end
                local.get $#local6
                i32.const 1
                i32.add
                local.set $#local3
                br $#label2
              end
              local.get $#local3
              i32.const 1
              i32.add
              local.set $#local3
              br $#label2
            end
            local.get $#local3
            local.get $#local2
            i32.ge_u
            br_if $#label2
            loop $#label3
              local.get $#local1
              local.get $#local3
              i32.add
              i32.load8_s
              i32.const 0
              i32.lt_s
              br_if $#label2
              local.get $#local2
              local.get $#local3
              i32.const 1
              i32.add
              local.tee $#local3
              i32.ne
              br_if $#label3
              br $#label0
            end
          end
          local.get $#local3
          local.get $#local2
          i32.lt_u
          br_if $#label1
        end
      end
      local.get $#local0
      local.get $#local2
      i32.store offset=8
      local.get $#local0
      local.get $#local1
      i32.store offset=4
      local.get $#local0
      i32.const 0
      i32.store
    )
    (func $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE (;175;) (type $#type12) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (result i32)
      (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      i32.const 1
      local.set $#local6
      block $#label0
        local.get $#local0
        i32.load8_u offset=4
        br_if $#label0
        local.get $#local0
        i32.load8_u offset=5
        local.set $#local7
        block $#label1
          local.get $#local0
          i32.load
          local.tee $#local8
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if $#label1
          i32.const 1
          local.set $#local6
          global.get $GOT.data.internal.__memory_base
          local.set $#local9
          local.get $#local8
          i32.load
          local.get $#local9
          i32.const 1053516
          i32.add
          local.get $#local9
          i32.const 1053513
          i32.add
          local.get $#local7
          i32.const 1
          i32.and
          local.tee $#local7
          select
          i32.const 2
          i32.const 3
          local.get $#local7
          select
          local.get $#local8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
          local.get $#local8
          i32.load
          local.get $#local1
          local.get $#local2
          local.get $#local8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
          global.get $GOT.data.internal.__memory_base
          local.set $#local2
          local.get $#local8
          i32.load
          local.get $#local2
          i32.const 1053507
          i32.add
          i32.const 2
          local.get $#local8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
          local.get $#local3
          local.get $#local8
          local.get $#local4
          i32.load offset=12
          call_indirect (type $#type2)
          local.set $#local6
          br $#label0
        end
        i32.const 1
        local.set $#local6
        block $#label1
          local.get $#local7
          i32.const 1
          i32.and
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          local.set $#local7
          local.get $#local8
          i32.load
          local.get $#local7
          i32.const 1053518
          i32.add
          i32.const 3
          local.get $#local8
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
        end
        i32.const 1
        local.set $#local6
        local.get $#local5
        i32.const 1
        i32.store8 offset=15
        local.get $#local5
        global.get $GOT.data.internal.__memory_base
        i32.const 1055668
        i32.add
        i32.store offset=20
        local.get $#local5
        local.get $#local8
        i64.load align=4
        i64.store align=4
        local.get $#local5
        local.get $#local8
        i64.load offset=8 align=4
        i64.store offset=24 align=4
        local.get $#local5
        local.get $#local5
        i32.const 15
        i32.add
        i32.store offset=8
        local.get $#local5
        local.get $#local5
        i32.store offset=16
        local.get $#local5
        local.get $#local1
        local.get $#local2
        call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E
        br_if $#label0
        local.get $#local5
        global.get $GOT.data.internal.__memory_base
        i32.const 1053507
        i32.add
        i32.const 2
        call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E
        br_if $#label0
        local.get $#local3
        local.get $#local5
        i32.const 16
        i32.add
        local.get $#local4
        i32.load offset=12
        call_indirect (type $#type2)
        br_if $#label0
        global.get $GOT.data.internal.__memory_base
        local.set $#local6
        local.get $#local5
        i32.load offset=16
        local.get $#local6
        i32.const 1053521
        i32.add
        i32.const 2
        local.get $#local5
        i32.load offset=20
        i32.load offset=12
        call_indirect (type $#type5)
        local.set $#local6
      end
      local.get $#local0
      i32.const 1
      i32.store8 offset=5
      local.get $#local0
      local.get $#local6
      i32.store8 offset=4
      local.get $#local5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf62913827f669f99E (;176;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      i32.const 3
      local.set $#local3
      local.get $#local0
      i32.load8_u
      local.tee $#local0
      local.set $#local4
      block $#label0
        local.get $#local0
        i32.const 10
        i32.lt_u
        br_if $#label0
        i32.const 1
        local.set $#local3
        local.get $#local2
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get $#local0
        local.get $#local0
        i32.const 100
        i32.div_u
        local.tee $#local4
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
      block $#label0
        block $#label1
          local.get $#local0
          i32.eqz
          br_if $#label1
          local.get $#local4
          i32.eqz
          br_if $#label0
        end
        local.get $#local2
        i32.const 13
        i32.add
        local.get $#local3
        i32.const -1
        i32.add
        local.tee $#local3
        i32.add
        global.get $GOT.data.internal.__memory_base
        i32.const 1053532
        i32.add
        local.get $#local4
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
      local.get $#local1
      i32.const 1
      i32.const 1
      i32.const 0
      local.get $#local2
      i32.const 13
      i32.add
      local.get $#local3
      i32.add
      i32.const 3
      local.get $#local3
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local3
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local3
    )
    (func $_ZN4core6result13unwrap_failed17h401d8e4c8733d97eE (;177;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32)
      (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      local.get $#local5
      local.get $#local1
      i32.store offset=12
      local.get $#local5
      local.get $#local0
      i32.store offset=8
      local.get $#local5
      local.get $#local3
      i32.store offset=20
      local.get $#local5
      local.get $#local2
      i32.store offset=16
      local.get $#local5
      i32.const 2
      i32.store offset=28
      local.get $#local5
      global.get $GOT.data.internal.__memory_base
      i32.const 1055652
      i32.add
      i32.store offset=24
      local.get $#local5
      i64.const 2
      i64.store offset=36 align=4
      local.get $#local5
      global.get $GOT.data.internal.__table_base
      local.tee $#local1
      i32.const 63
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local5
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get $#local5
      local.get $#local1
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local5
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=48
      local.get $#local5
      local.get $#local5
      i32.const 48
      i32.add
      i32.store offset=32
      local.get $#local5
      i32.const 24
      i32.add
      local.get $#local4
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E (;178;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load
      i32.const 1
      local.get $#local1
      call $_ZN4core3fmt3num3imp21_$LT$impl$u20$u32$GT$4_fmt17h64f38de2da5605acE
    )
    (func $_ZN4core6option13unwrap_failed17h6f83cdd4267ddc7bE (;179;) (type $#type1) (param $#local0 i32)
      global.get $GOT.data.internal.__memory_base
      i32.const 1053380
      i32.add
      i32.const 43
      local.get $#local0
      call $_ZN4core9panicking5panic17hd836709591dfc35fE
      unreachable
    )
    (func $_ZN4core6option13expect_failed17he15179d1cacc214eE (;180;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      i32.store offset=12
      local.get $#local3
      local.get $#local0
      i32.store offset=8
      local.get $#local3
      i32.const 1
      i32.store offset=20
      local.get $#local3
      global.get $GOT.data.internal.__memory_base
      i32.const 1053340
      i32.add
      i32.store offset=16
      local.get $#local3
      i64.const 1
      i64.store offset=28 align=4
      local.get $#local3
      global.get $GOT.data.internal.__table_base
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local3
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get $#local3
      local.get $#local3
      i32.const 40
      i32.add
      i32.store offset=24
      local.get $#local3
      i32.const 16
      i32.add
      local.get $#local2
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17ha55d7598a4677f63E (;181;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local1
      local.get $#local0
      i32.load
      local.get $#local0
      i32.load offset=4
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h8ab4d1fff913a5afE (;182;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local0
      i32.load
      local.set $#local0
      i32.const 0
      local.set $#local3
      loop $#label0
        local.get $#local2
        local.get $#local3
        i32.add
        i32.const 127
        i32.add
        local.get $#local0
        i32.const 15
        i32.and
        local.tee $#local4
        i32.const 48
        i32.or
        local.get $#local4
        i32.const 87
        i32.add
        local.get $#local4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get $#local3
        i32.const -1
        i32.add
        local.set $#local3
        local.get $#local0
        i32.const 15
        i32.gt_u
        local.set $#local4
        local.get $#local0
        i32.const 4
        i32.shr_u
        local.set $#local0
        local.get $#local4
        br_if $#label0
      end
      local.get $#local1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get $#local2
      local.get $#local3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get $#local3
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local0
      local.get $#local2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN4core9panicking19assert_failed_inner17he88eb99d70dbb346E (;183;) (type $#type14) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (param $#local5 i32) (param $#local6 i32)
      (local $#local7 i32) (local $#local8 i64)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee $#local7
      global.set $__stack_pointer
      local.get $#local7
      local.get $#local2
      i32.store offset=12
      local.get $#local7
      local.get $#local1
      i32.store offset=8
      local.get $#local7
      local.get $#local4
      i32.store offset=20
      local.get $#local7
      local.get $#local3
      i32.store offset=16
      local.get $#local7
      global.get $GOT.data.internal.__memory_base
      local.tee $#local2
      i32.const 1054100
      i32.add
      local.get $#local0
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      local.tee $#local1
      i32.add
      i32.load
      i32.store offset=28
      local.get $#local7
      local.get $#local2
      i32.const 1055756
      i32.add
      local.get $#local1
      i32.add
      i32.load
      i32.store offset=24
      block $#label0
        local.get $#local5
        i32.load
        i32.eqz
        br_if $#label0
        local.get $#local7
        i32.const 32
        i32.add
        i32.const 16
        i32.add
        local.get $#local5
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get $#local7
        i32.const 32
        i32.add
        i32.const 8
        i32.add
        local.get $#local5
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get $#local7
        local.get $#local5
        i64.load align=4
        i64.store offset=32
        local.get $#local7
        i32.const 4
        i32.store offset=92
        local.get $#local7
        global.get $GOT.data.internal.__memory_base
        i32.const 1055620
        i32.add
        i32.store offset=88
        local.get $#local7
        i64.const 4
        i64.store offset=100 align=4
        local.get $#local7
        global.get $GOT.data.internal.__table_base
        local.tee $#local5
        i32.const 63
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.tee $#local8
        local.get $#local7
        i32.const 16
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=80
        local.get $#local7
        local.get $#local8
        local.get $#local7
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=72
        local.get $#local7
        global.get $GOT.func.internal._ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get $#local7
        i32.const 32
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=64
        local.get $#local7
        local.get $#local5
        i32.const 64
        i32.add
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get $#local7
        i32.const 24
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=56
        local.get $#local7
        local.get $#local7
        i32.const 56
        i32.add
        i32.store offset=96
        local.get $#local7
        i32.const 88
        i32.add
        local.get $#local6
        call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
        unreachable
      end
      local.get $#local7
      i32.const 3
      i32.store offset=92
      local.get $#local7
      i64.const 3
      i64.store offset=100 align=4
      local.get $#local7
      global.get $GOT.data.internal.__memory_base
      i32.const 1055596
      i32.add
      i32.store offset=88
      local.get $#local7
      global.get $GOT.data.internal.__table_base
      local.tee $#local5
      i32.const 63
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local8
      local.get $#local7
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=72
      local.get $#local7
      local.get $#local8
      local.get $#local7
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=64
      local.get $#local7
      local.get $#local5
      i32.const 64
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get $#local7
      i32.const 24
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get $#local7
      local.get $#local7
      i32.const 56
      i32.add
      i32.store offset=96
      local.get $#local7
      i32.const 88
      i32.add
      local.get $#local6
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hedb77f9fb2237382E (;184;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      i32.load
      local.get $#local1
      local.get $#local0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type $#type2)
    )
    (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf5391246709cbe9dE (;185;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local1
      i32.load
      local.get $#local1
      i32.load offset=4
      local.get $#local0
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hce8fa69199286883E (;186;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32) (local $#local11 i32) (local $#local12 i32) (local $#local13 i32) (local $#local14 i32)
      local.get $#local1
      i32.const -1
      i32.add
      local.set $#local3
      local.get $#local0
      i32.load offset=4
      local.set $#local4
      local.get $#local0
      i32.load
      local.set $#local5
      local.get $#local0
      i32.load offset=8
      local.set $#local6
      i32.const 0
      local.set $#local7
      i32.const 0
      local.set $#local8
      i32.const 0
      local.set $#local9
      i32.const 0
      local.set $#local10
      block $#label0
        loop $#label1
          local.get $#local10
          i32.const 1
          i32.and
          br_if $#label0
          block $#label2
            block $#label3
              local.get $#local2
              local.get $#local9
              i32.lt_u
              br_if $#label3
              loop $#label4
                local.get $#local1
                local.get $#local9
                i32.add
                local.set $#local10
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        local.get $#local2
                        local.get $#local9
                        i32.sub
                        local.tee $#local11
                        i32.const 7
                        i32.gt_u
                        br_if $#label8
                        local.get $#local2
                        local.get $#local9
                        i32.ne
                        br_if $#label7
                        local.get $#local2
                        local.set $#local9
                        br $#label3
                      end
                      block $#label8
                        block $#label9
                          local.get $#local10
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee $#local12
                          local.get $#local10
                          i32.sub
                          local.tee $#local13
                          i32.eqz
                          br_if $#label9
                          i32.const 0
                          local.set $#local0
                          loop $#label10
                            local.get $#local10
                            local.get $#local0
                            i32.add
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if $#label5
                            local.get $#local13
                            local.get $#local0
                            i32.const 1
                            i32.add
                            local.tee $#local0
                            i32.ne
                            br_if $#label10
                          end
                          local.get $#local13
                          local.get $#local11
                          i32.const -8
                          i32.add
                          local.tee $#local14
                          i32.le_u
                          br_if $#label8
                          br $#label6
                        end
                        local.get $#local11
                        i32.const -8
                        i32.add
                        local.set $#local14
                      end
                      loop $#label8
                        i32.const 16843008
                        local.get $#local12
                        i32.load
                        local.tee $#local0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get $#local0
                        i32.or
                        i32.const 16843008
                        local.get $#local12
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee $#local0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get $#local0
                        i32.or
                        i32.and
                        i32.const -2139062144
                        i32.and
                        i32.const -2139062144
                        i32.ne
                        br_if $#label6
                        local.get $#local12
                        i32.const 8
                        i32.add
                        local.set $#local12
                        local.get $#local13
                        i32.const 8
                        i32.add
                        local.tee $#local13
                        local.get $#local14
                        i32.le_u
                        br_if $#label8
                        br $#label6
                      end
                    end
                    i32.const 0
                    local.set $#local0
                    loop $#label7
                      local.get $#local10
                      local.get $#local0
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.eq
                      br_if $#label5
                      local.get $#local11
                      local.get $#local0
                      i32.const 1
                      i32.add
                      local.tee $#local0
                      i32.ne
                      br_if $#label7
                    end
                    local.get $#local2
                    local.set $#local9
                    br $#label3
                  end
                  block $#label6
                    local.get $#local11
                    local.get $#local13
                    i32.ne
                    br_if $#label6
                    local.get $#local2
                    local.set $#local9
                    br $#label3
                  end
                  loop $#label6
                    block $#label7
                      local.get $#local10
                      local.get $#local13
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if $#label7
                      local.get $#local13
                      local.set $#local0
                      br $#label5
                    end
                    local.get $#local11
                    local.get $#local13
                    i32.const 1
                    i32.add
                    local.tee $#local13
                    i32.ne
                    br_if $#label6
                  end
                  local.get $#local2
                  local.set $#local9
                  br $#label3
                end
                local.get $#local0
                local.get $#local9
                i32.add
                local.tee $#local13
                i32.const 1
                i32.add
                local.set $#local9
                block $#label5
                  local.get $#local13
                  local.get $#local2
                  i32.ge_u
                  br_if $#label5
                  local.get $#local10
                  local.get $#local0
                  i32.add
                  i32.load8_u
                  i32.const 10
                  i32.ne
                  br_if $#label5
                  i32.const 0
                  local.set $#local10
                  local.get $#local9
                  local.set $#local12
                  local.get $#local9
                  local.set $#local0
                  br $#label2
                end
                local.get $#local9
                local.get $#local2
                i32.le_u
                br_if $#label4
              end
            end
            local.get $#local2
            local.get $#local8
            i32.eq
            br_if $#label0
            i32.const 1
            local.set $#local10
            local.get $#local8
            local.set $#local12
            local.get $#local2
            local.set $#local0
          end
          block $#label2
            block $#label3
              local.get $#local6
              i32.load8_u
              i32.eqz
              br_if $#label3
              local.get $#local5
              global.get $GOT.data.internal.__memory_base
              i32.const 1053509
              i32.add
              i32.const 4
              local.get $#local4
              i32.load offset=12
              call_indirect (type $#type5)
              br_if $#label2
            end
            local.get $#local0
            local.get $#local8
            i32.sub
            local.set $#local11
            i32.const 0
            local.set $#local13
            block $#label3
              local.get $#local0
              local.get $#local8
              i32.eq
              br_if $#label3
              local.get $#local3
              local.get $#local0
              i32.add
              i32.load8_u
              i32.const 10
              i32.eq
              local.set $#local13
            end
            local.get $#local1
            local.get $#local8
            i32.add
            local.set $#local0
            local.get $#local6
            local.get $#local13
            i32.store8
            local.get $#local12
            local.set $#local8
            local.get $#local5
            local.get $#local0
            local.get $#local11
            local.get $#local4
            i32.load offset=12
            call_indirect (type $#type5)
            i32.eqz
            br_if $#label1
          end
        end
        i32.const 1
        local.set $#local7
      end
      local.get $#local7
    )
    (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hb4551229d8c1f98fE (;187;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32)
      local.get $#local0
      i32.load offset=4
      local.set $#local2
      local.get $#local0
      i32.load
      local.set $#local3
      block $#label0
        local.get $#local0
        i32.load offset=8
        local.tee $#local0
        i32.load8_u
        i32.eqz
        br_if $#label0
        local.get $#local3
        global.get $GOT.data.internal.__memory_base
        i32.const 1053509
        i32.add
        i32.const 4
        local.get $#local2
        i32.load offset=12
        call_indirect (type $#type5)
        i32.eqz
        br_if $#label0
        i32.const 1
        return
      end
      local.get $#local0
      local.get $#local1
      i32.const 10
      i32.eq
      i32.store8
      local.get $#local3
      local.get $#local1
      local.get $#local2
      i32.load offset=16
      call_indirect (type $#type2)
    )
    (func $_ZN4core3fmt5Write9write_fmt17h9f87bee6dff8a623E (;188;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      global.get $GOT.data.internal.__memory_base
      i32.const 1055668
      i32.add
      local.get $#local1
      call $_ZN4core3fmt5write17h19dbf2ffaf30f068E
    )
    (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h1d2cb4387ef2f514E (;189;) (type $#type12) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (result i32)
      block $#label0
        local.get $#local2
        i32.const 1114112
        i32.eq
        br_if $#label0
        local.get $#local0
        local.get $#local2
        local.get $#local1
        i32.load offset=16
        call_indirect (type $#type2)
        i32.eqz
        br_if $#label0
        i32.const 1
        return
      end
      block $#label0
        local.get $#local3
        br_if $#label0
        i32.const 0
        return
      end
      local.get $#local0
      local.get $#local3
      local.get $#local4
      local.get $#local1
      i32.load offset=12
      call_indirect (type $#type5)
    )
    (func $_ZN4core3fmt9Formatter9write_str17h5e72d1add4744b56E (;190;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      local.get $#local0
      i32.load
      local.get $#local1
      local.get $#local2
      local.get $#local0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type $#type5)
    )
    (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h7bd8c0320fca2ee5E (;191;) (type $#type15) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (param $#local5 i32) (param $#local6 i32) (param $#local7 i32) (param $#local8 i32) (param $#local9 i32) (param $#local10 i32) (result i32)
      (local $#local11 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local11
      global.set $__stack_pointer
      local.get $#local0
      i32.load
      local.get $#local1
      local.get $#local2
      local.get $#local0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type $#type5)
      local.set $#local2
      local.get $#local11
      i32.const 0
      i32.store8 offset=13
      local.get $#local11
      local.get $#local2
      i32.store8 offset=12
      local.get $#local11
      local.get $#local0
      i32.store offset=8
      local.get $#local11
      i32.const 8
      i32.add
      local.get $#local3
      local.get $#local4
      local.get $#local5
      local.get $#local6
      call $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE
      local.get $#local7
      local.get $#local8
      local.get $#local9
      local.get $#local10
      call $_ZN4core3fmt8builders11DebugStruct5field17h5c9bcca6b82ebfdbE
      local.set $#local10
      local.get $#local11
      i32.load8_u offset=13
      local.tee $#local2
      local.get $#local11
      i32.load8_u offset=12
      local.tee $#local1
      i32.or
      local.set $#local0
      block $#label0
        local.get $#local2
        i32.const 1
        i32.ne
        br_if $#label0
        local.get $#local1
        i32.const 1
        i32.and
        br_if $#label0
        block $#label1
          local.get $#local10
          i32.load
          local.tee $#local0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          local.set $#local2
          local.get $#local0
          i32.load
          local.get $#local2
          i32.const 1053524
          i32.add
          i32.const 2
          local.get $#local0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          local.set $#local0
          br $#label0
        end
        global.get $GOT.data.internal.__memory_base
        local.set $#local2
        local.get $#local0
        i32.load
        local.get $#local2
        i32.const 1053523
        i32.add
        i32.const 1
        local.get $#local0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type $#type5)
        local.set $#local0
      end
      local.get $#local11
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local0
      i32.const 1
      i32.and
    )
    (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hb8379d1150d161c1E (;192;) (type $#type12) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32) (result i32)
      (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      i32.const 1
      local.set $#local6
      block $#label0
        local.get $#local0
        i32.load
        local.tee $#local7
        local.get $#local1
        local.get $#local2
        local.get $#local0
        i32.load offset=4
        local.tee $#local8
        i32.load offset=12
        local.tee $#local9
        call_indirect (type $#type5)
        br_if $#label0
        block $#label1
          block $#label2
            local.get $#local0
            i32.load8_u offset=10
            i32.const 128
            i32.and
            br_if $#label2
            i32.const 1
            local.set $#local6
            local.get $#local7
            global.get $GOT.data.internal.__memory_base
            i32.const 1053526
            i32.add
            i32.const 1
            local.get $#local9
            call_indirect (type $#type5)
            br_if $#label0
            local.get $#local3
            local.get $#local0
            local.get $#local4
            i32.load offset=12
            call_indirect (type $#type2)
            i32.eqz
            br_if $#label1
            br $#label0
          end
          local.get $#local7
          global.get $GOT.data.internal.__memory_base
          i32.const 1053527
          i32.add
          i32.const 2
          local.get $#local9
          call_indirect (type $#type5)
          br_if $#label0
          i32.const 1
          local.set $#local6
          local.get $#local5
          i32.const 1
          i32.store8 offset=15
          local.get $#local5
          local.get $#local8
          i32.store offset=4
          local.get $#local5
          local.get $#local7
          i32.store
          local.get $#local5
          global.get $GOT.data.internal.__memory_base
          i32.const 1055668
          i32.add
          i32.store offset=20
          local.get $#local5
          local.get $#local0
          i64.load offset=8 align=4
          i64.store offset=24 align=4
          local.get $#local5
          local.get $#local5
          i32.const 15
          i32.add
          i32.store offset=8
          local.get $#local5
          local.get $#local5
          i32.store offset=16
          local.get $#local3
          local.get $#local5
          i32.const 16
          i32.add
          local.get $#local4
          i32.load offset=12
          call_indirect (type $#type2)
          br_if $#label0
          global.get $GOT.data.internal.__memory_base
          local.set $#local1
          local.get $#local5
          i32.load offset=16
          local.get $#local1
          i32.const 1053521
          i32.add
          i32.const 2
          local.get $#local5
          i32.load offset=20
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
        end
        block $#label1
          local.get $#local2
          br_if $#label1
          local.get $#local0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if $#label1
          global.get $GOT.data.internal.__memory_base
          local.set $#local2
          i32.const 1
          local.set $#local6
          local.get $#local0
          i32.load
          local.get $#local2
          i32.const 1053529
          i32.add
          i32.const 1
          local.get $#local0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type $#type5)
          br_if $#label0
        end
        global.get $GOT.data.internal.__memory_base
        local.set $#local6
        local.get $#local0
        i32.load
        local.get $#local6
        i32.const 1053337
        i32.add
        i32.const 1
        local.get $#local0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type $#type5)
        local.set $#local6
      end
      local.get $#local5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get $#local6
    )
    (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3c419b7f4946f096E (;193;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      block $#label0
        local.get $#local0
        i32.load8_u
        br_if $#label0
        local.get $#local1
        global.get $GOT.data.internal.__memory_base
        i32.const 1053732
        i32.add
        i32.const 5
        call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
        return
      end
      local.get $#local1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053737
      i32.add
      i32.const 4
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hf4ce0dea7e551c6eE (;194;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      local.get $#local2
      local.get $#local0
      local.get $#local1
      call $_ZN4core3fmt9Formatter3pad17hd183b2eea654b198E
    )
    (func $_ZN4core5slice6memchr14memchr_aligned17hf4db372f52bc45e4E (;195;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32)
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local2
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.tee $#local4
              local.get $#local2
              i32.eq
              br_if $#label3
              local.get $#local3
              local.get $#local4
              local.get $#local2
              i32.sub
              local.tee $#local4
              local.get $#local3
              local.get $#local4
              i32.lt_u
              select
              local.tee $#local4
              i32.eqz
              br_if $#label3
              i32.const 0
              local.set $#local5
              local.get $#local1
              i32.const 255
              i32.and
              local.set $#local6
              i32.const 1
              local.set $#local7
              loop $#label4
                local.get $#local2
                local.get $#local5
                i32.add
                i32.load8_u
                local.get $#local6
                i32.eq
                br_if $#label0
                local.get $#local4
                local.get $#local5
                i32.const 1
                i32.add
                local.tee $#local5
                i32.ne
                br_if $#label4
              end
              local.get $#local4
              local.get $#local3
              i32.const -8
              i32.add
              local.tee $#local8
              i32.gt_u
              br_if $#label1
              br $#label2
            end
            local.get $#local3
            i32.const -8
            i32.add
            local.set $#local8
            i32.const 0
            local.set $#local4
          end
          local.get $#local1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set $#local5
          loop $#label2
            i32.const 16843008
            local.get $#local2
            local.get $#local4
            i32.add
            local.tee $#local6
            i32.load
            local.get $#local5
            i32.xor
            local.tee $#local7
            i32.sub
            local.get $#local7
            i32.or
            i32.const 16843008
            local.get $#local6
            i32.const 4
            i32.add
            i32.load
            local.get $#local5
            i32.xor
            local.tee $#local6
            i32.sub
            local.get $#local6
            i32.or
            i32.and
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.ne
            br_if $#label1
            local.get $#local4
            i32.const 8
            i32.add
            local.tee $#local4
            local.get $#local8
            i32.le_u
            br_if $#label2
          end
        end
        block $#label1
          local.get $#local3
          local.get $#local4
          i32.eq
          br_if $#label1
          local.get $#local1
          i32.const 255
          i32.and
          local.set $#local5
          i32.const 1
          local.set $#local7
          loop $#label2
            block $#label3
              local.get $#local2
              local.get $#local4
              i32.add
              i32.load8_u
              local.get $#local5
              i32.ne
              br_if $#label3
              local.get $#local4
              local.set $#local5
              br $#label0
            end
            local.get $#local3
            local.get $#local4
            i32.const 1
            i32.add
            local.tee $#local4
            i32.ne
            br_if $#label2
          end
        end
        i32.const 0
        local.set $#local7
      end
      local.get $#local0
      local.get $#local5
      i32.store offset=4
      local.get $#local0
      local.get $#local7
      i32.store
    )
    (func $_ZN4core5slice6memchr7memrchr17h708a5008ebee5929E (;196;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32)
      local.get $#local3
      i32.const 0
      local.get $#local3
      local.get $#local2
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get $#local2
      i32.sub
      local.tee $#local4
      i32.sub
      i32.const 7
      i32.and
      local.get $#local3
      local.get $#local4
      i32.lt_u
      select
      local.tee $#local5
      i32.sub
      local.set $#local6
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              local.get $#local3
              local.get $#local5
              i32.lt_u
              br_if $#label3
              block $#label4
                local.get $#local5
                i32.eqz
                br_if $#label4
                block $#label5
                  block $#label6
                    local.get $#local2
                    local.get $#local3
                    i32.add
                    local.tee $#local7
                    i32.const -1
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -1
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local2
                  local.get $#local6
                  i32.add
                  local.tee $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -2
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -2
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -3
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -3
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -4
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -4
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -5
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -5
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -6
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -6
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  block $#label6
                    local.get $#local7
                    i32.const -7
                    i32.add
                    local.tee $#local8
                    i32.load8_u
                    local.get $#local1
                    i32.const 255
                    i32.and
                    i32.ne
                    br_if $#label6
                    local.get $#local5
                    i32.const -7
                    i32.add
                    local.set $#local5
                    br $#label5
                  end
                  local.get $#local9
                  local.get $#local8
                  i32.eq
                  br_if $#label4
                  local.get $#local5
                  i32.const -8
                  i32.or
                  local.set $#local5
                end
                local.get $#local5
                local.get $#local6
                i32.add
                local.set $#local5
                br $#label1
              end
              local.get $#local4
              local.get $#local3
              local.get $#local3
              local.get $#local4
              i32.gt_u
              select
              local.set $#local9
              local.get $#local1
              i32.const 255
              i32.and
              i32.const 16843009
              i32.mul
              local.set $#local4
              block $#label4
                loop $#label5
                  local.get $#local6
                  local.tee $#local5
                  local.get $#local9
                  i32.le_u
                  br_if $#label4
                  local.get $#local5
                  i32.const -8
                  i32.add
                  local.set $#local6
                  i32.const 16843008
                  local.get $#local2
                  local.get $#local5
                  i32.add
                  local.tee $#local8
                  i32.const -8
                  i32.add
                  i32.load
                  local.get $#local4
                  i32.xor
                  local.tee $#local7
                  i32.sub
                  local.get $#local7
                  i32.or
                  i32.const 16843008
                  local.get $#local8
                  i32.const -4
                  i32.add
                  i32.load
                  local.get $#local4
                  i32.xor
                  local.tee $#local8
                  i32.sub
                  local.get $#local8
                  i32.or
                  i32.and
                  i32.const -2139062144
                  i32.and
                  i32.const -2139062144
                  i32.eq
                  br_if $#label5
                end
              end
              local.get $#local5
              local.get $#local3
              i32.gt_u
              br_if $#label2
              local.get $#local2
              i32.const -1
              i32.add
              local.set $#local4
              local.get $#local1
              i32.const 255
              i32.and
              local.set $#local8
              loop $#label4
                block $#label5
                  local.get $#local5
                  br_if $#label5
                  i32.const 0
                  local.set $#local6
                  br $#label0
                end
                local.get $#local4
                local.get $#local5
                i32.add
                local.set $#local6
                local.get $#local5
                i32.const -1
                i32.add
                local.set $#local5
                local.get $#local6
                i32.load8_u
                local.get $#local8
                i32.eq
                br_if $#label1
                br $#label4
              end
            end
            local.get $#local6
            local.get $#local3
            global.get $GOT.data.internal.__memory_base
            i32.const 1055692
            i32.add
            call $_ZN4core5slice5index26slice_start_index_len_fail17h89becdcd89148af6E
            unreachable
          end
          local.get $#local5
          local.get $#local3
          global.get $GOT.data.internal.__memory_base
          i32.const 1055708
          i32.add
          call $_ZN4core5slice5index24slice_end_index_len_fail17h83f086342480bebfE
          unreachable
        end
        i32.const 1
        local.set $#local6
      end
      local.get $#local0
      local.get $#local5
      i32.store offset=4
      local.get $#local0
      local.get $#local6
      i32.store
    )
    (func $_ZN4core5slice5index26slice_start_index_len_fail8do_panic7runtime17h0aceb3cf85b1787dE (;197;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      i32.store offset=4
      local.get $#local3
      local.get $#local0
      i32.store
      local.get $#local3
      i32.const 2
      i32.store offset=12
      local.get $#local3
      global.get $GOT.data.internal.__memory_base
      i32.const 1055724
      i32.add
      i32.store offset=8
      local.get $#local3
      i64.const 2
      i64.store offset=20 align=4
      local.get $#local3
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local4
      local.get $#local3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get $#local3
      local.get $#local4
      local.get $#local3
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local3
      local.get $#local3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get $#local3
      i32.const 8
      i32.add
      local.get $#local2
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core5slice5index24slice_end_index_len_fail8do_panic7runtime17he60b5f89c94a6c7fE (;198;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local3
      local.get $#local1
      i32.store offset=4
      local.get $#local3
      local.get $#local0
      i32.store
      local.get $#local3
      i32.const 2
      i32.store offset=12
      local.get $#local3
      global.get $GOT.data.internal.__memory_base
      i32.const 1055740
      i32.add
      i32.store offset=8
      local.get $#local3
      i64.const 2
      i64.store offset=20 align=4
      local.get $#local3
      global.get $GOT.func.internal._ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h475c24b8c53548f4E
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee $#local4
      local.get $#local3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get $#local3
      local.get $#local4
      local.get $#local3
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get $#local3
      local.get $#local3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get $#local3
      i32.const 8
      i32.add
      local.get $#local2
      call $_ZN4core9panicking9panic_fmt17h23cad18ed7b0e32cE
      unreachable
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h11f1bd59b705e9b5E (;199;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local0
      i32.load8_u
      local.set $#local3
      i32.const 0
      local.set $#local0
      loop $#label0
        local.get $#local2
        local.get $#local0
        i32.add
        i32.const 127
        i32.add
        local.get $#local3
        i32.const 15
        i32.and
        local.tee $#local4
        i32.const 48
        i32.or
        local.get $#local4
        i32.const 55
        i32.add
        local.get $#local4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get $#local0
        i32.const -1
        i32.add
        local.set $#local0
        local.get $#local3
        i32.const 255
        i32.and
        local.tee $#local4
        i32.const 4
        i32.shr_u
        local.set $#local3
        local.get $#local4
        i32.const 15
        i32.gt_u
        br_if $#label0
      end
      local.get $#local1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get $#local2
      local.get $#local0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get $#local0
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local0
      local.get $#local2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h56866b3e0f6851b5E (;200;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local0
      i32.load8_u
      local.set $#local3
      i32.const 0
      local.set $#local0
      loop $#label0
        local.get $#local2
        local.get $#local0
        i32.add
        i32.const 127
        i32.add
        local.get $#local3
        i32.const 15
        i32.and
        local.tee $#local4
        i32.const 48
        i32.or
        local.get $#local4
        i32.const 87
        i32.add
        local.get $#local4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get $#local0
        i32.const -1
        i32.add
        local.set $#local0
        local.get $#local3
        i32.const 255
        i32.and
        local.tee $#local4
        i32.const 4
        i32.shr_u
        local.set $#local3
        local.get $#local4
        i32.const 15
        i32.gt_u
        br_if $#label0
      end
      local.get $#local1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get $#local2
      local.get $#local0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get $#local0
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local0
      local.get $#local2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17he735d85c11c77d9dE (;201;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local0
      i32.load
      local.set $#local0
      i32.const 0
      local.set $#local3
      loop $#label0
        local.get $#local2
        local.get $#local3
        i32.add
        i32.const 127
        i32.add
        local.get $#local0
        i32.const 15
        i32.and
        local.tee $#local4
        i32.const 48
        i32.or
        local.get $#local4
        i32.const 55
        i32.add
        local.get $#local4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get $#local3
        i32.const -1
        i32.add
        local.set $#local3
        local.get $#local0
        i32.const 15
        i32.gt_u
        local.set $#local4
        local.get $#local0
        i32.const 4
        i32.shr_u
        local.set $#local0
        local.get $#local4
        br_if $#label0
      end
      local.get $#local1
      i32.const 1
      global.get $GOT.data.internal.__memory_base
      i32.const 1053530
      i32.add
      i32.const 2
      local.get $#local2
      local.get $#local3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get $#local3
      i32.sub
      call $_ZN4core3fmt9Formatter12pad_integral17hc5ba7287b1ef1d8dE
      local.set $#local0
      local.get $#local2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he9acf4c76e4d361eE (;202;) (type $#type2) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32)
      local.get $#local0
      i32.load
      local.tee $#local0
      local.get $#local0
      i32.const 31
      i32.shr_s
      local.tee $#local2
      i32.xor
      local.get $#local2
      i32.sub
      local.get $#local0
      i32.const -1
      i32.xor
      i32.const 31
      i32.shr_u
      local.get $#local1
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
  (core module $#module1 (;1;)
    (type $#type0 (;0;) (func))
    (type $#type1 (;1;) (func (param i32)))
    (type $#type2 (;2;) (func (param i32 i32)))
    (type $#type3 (;3;) (func (param i32 i64 i32)))
    (type $#type4 (;4;) (func (param i32 i32 i32 i32)))
    (type $#type5 (;5;) (func (param i32 i32 i32 i32 i32)))
    (type $#type6 (;6;) (func (param i32) (result i32)))
    (type $#type7 (;7;) (func (param i32 i32 i32)))
    (type $#type8 (;8;) (func (param i32 i32 i32 i32) (result i32)))
    (type $#type9 (;9;) (func (result i32)))
    (type $#type10 (;10;) (func (param i32 i32 i32) (result i32)))
    (type $#type11 (;11;) (func (param i32 i32) (result i32)))
    (type $#type12 (;12;) (func))
    (import "env" "memory" (memory $#memory0 (;0;) 0))
    (import "__main_module__" "_start" (func $_ZN22wasi_snapshot_preview13run6_start17h6125b5756c6a2506E (;0;) (type $#type0)))
    (import "wasi:cli/environment@0.2.3" "get-environment" (func $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E (;1;) (type $#type1)))
    (import "wasi:filesystem/types@0.2.3" "[resource-drop]descriptor" (func $_ZN141_$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..Descriptor$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h34b1c3918a0eebe8E (;2;) (type $#type1)))
    (import "wasi:io/streams@0.2.3" "[resource-drop]output-stream" (func $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E (;3;) (type $#type1)))
    (import "wasi:filesystem/types@0.2.3" "filesystem-error-code" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types21filesystem_error_code10wit_import17h2b5d8e99a65d0583E (;4;) (type $#type2)))
    (import "wasi:io/error@0.2.3" "[resource-drop]error" (func $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E (;5;) (type $#type1)))
    (import "wasi:io/streams@0.2.3" "[resource-drop]input-stream" (func $_ZN136_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..InputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h5547e0ecd980db5bE (;6;) (type $#type1)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.check-write" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream11check_write10wit_import17hdb0f94e542b67356E (;7;) (type $#type2)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.write" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream5write10wit_import17h1ec4b5645a67d72bE (;8;) (type $#type4)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.blocking-flush" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream14blocking_flush10wit_import17h7bbaef6ef9f5f783E (;9;) (type $#type2)))
    (import "__main_module__" "cabi_realloc" (func $_ZN22wasi_snapshot_preview15State3new12cabi_realloc17h88297338052a23f0E (;10;) (type $#type8)))
    (import "wasi:filesystem/preopens@0.2.2" "get-directories" (func $_ZN22wasi_snapshot_preview111descriptors31wasi_filesystem_get_directories17h9e1fe8bbb8a7f366E (;11;) (type $#type1)))
    (import "wasi:cli/stderr@0.2.3" "get-stderr" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E (;12;) (type $#type9)))
    (import "wasi:io/streams@0.2.3" "[method]output-stream.blocking-write-and-flush" (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush10wit_import17hd0e6bb9bc2c2ef7aE (;13;) (type $#type4)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.write-via-stream" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor16write_via_stream10wit_import17h863d15eb6a8c37dcE (;14;) (type $#type3)))
    (import "wasi:cli/stdin@0.2.3" "get-stdin" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli5stdin9get_stdin10wit_import17he5bdc61203ca72fdE (;15;) (type $#type9)))
    (import "wasi:cli/stdout@0.2.3" "get-stdout" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stdout10get_stdout10wit_import17hfdc059c6457e3d59E (;16;) (type $#type9)))
    (import "wasi:cli/exit@0.2.3" "exit" (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit10wit_import17hefacdcba211d0d02E (;17;) (type $#type1)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.append-via-stream" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream10wit_import17h2638911ab3ccf3ddE (;18;) (type $#type2)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.get-type" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type10wit_import17h708de2cfd599ed76E (;19;) (type $#type2)))
    (import "wasi:filesystem/types@0.2.3" "[method]descriptor.stat" (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat10wit_import17hd8a47ee046c71cb9E (;20;) (type $#type2)))
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $internal_state_ptr (;1;) (mut i32) i32.const 0)
    (global $allocation_state (;2;) (mut i32) i32.const 0)
    (export "wasi:cli/run@0.2.3#run" (func $wasi:cli/run@0.2.3#run))
    (export "fd_write" (func $fd_write))
    (export "environ_get" (func $environ_get))
    (export "environ_sizes_get" (func $environ_sizes_get))
    (export "cabi_import_realloc" (func $cabi_import_realloc))
    (export "proc_exit" (func $proc_exit))
    (func $wasi:cli/run@0.2.3#run (;21;) (type $#type9) (result i32)
      call $allocate_stack
      call $_ZN22wasi_snapshot_preview13run6_start17h6125b5756c6a2506E
      i32.const 0
    )
    (func $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE (;22;) (type $#type9) (result i32)
      (local $#local0 i32)
      block $#label0
        call $get_state_ptr
        local.tee $#local0
        br_if $#label0
        call $_ZN22wasi_snapshot_preview15State3new17h27fd4b5328bc4eeeE
        local.tee $#local0
        call $set_state_ptr
      end
      local.get $#local0
    )
    (func $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE (;23;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local1
      i32.const 32
      i32.store8 offset=47
      local.get $#local1
      i64.const 7308895158390646132
      i64.store offset=39 align=1
      local.get $#local1
      i64.const 8097863973307965728
      i64.store offset=31 align=1
      local.get $#local1
      i64.const 7234307576302018670
      i64.store offset=23 align=1
      local.get $#local1
      i64.const 8028075845441778529
      i64.store offset=15 align=1
      local.get $#local1
      i32.const 15
      i32.add
      i32.const 33
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3217h80b476442303f1eeE
      unreachable
    )
    (func $cabi_import_realloc (;24;) (type $#type8) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i64)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                          local.tee $#local5
                          i32.load
                          i32.const 560490357
                          i32.ne
                          br_if $#label9
                          local.get $#local5
                          i32.load offset=65532
                          i32.const 560490357
                          i32.ne
                          br_if $#label8
                          local.get $#local5
                          i64.load offset=4 align=4
                          local.set $#local6
                          local.get $#local5
                          i32.const 4
                          i32.store offset=4
                          local.get $#local4
                          i32.const 16
                          i32.add
                          local.get $#local5
                          i32.const 20
                          i32.add
                          i32.load
                          i32.store
                          local.get $#local4
                          i32.const 8
                          i32.add
                          local.get $#local5
                          i32.const 12
                          i32.add
                          i64.load align=4
                          i64.store
                          local.get $#local4
                          local.get $#local6
                          i64.store
                          local.get $#local0
                          i32.eqz
                          br_if $#label7
                          local.get $#local1
                          local.get $#local3
                          i32.le_u
                          br_if $#label6
                          local.get $#local2
                          i32.const 1
                          i32.eq
                          br_if $#label0
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
                    local.get $#local4
                    i32.load
                    br_table $#label1 $#label3 $#label4 $#label5 $#label2 $#label1
                  end
                  i32.const 376
                  call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
                  unreachable
                end
                local.get $#local4
                i32.const 12
                i32.add
                local.set $#local0
                block $#label5
                  local.get $#local2
                  i32.const 1
                  i32.eq
                  br_if $#label5
                  local.get $#local0
                  local.get $#local2
                  local.get $#local3
                  call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                  local.set $#local0
                  br $#label0
                end
                local.get $#local4
                local.get $#local4
                i32.load offset=4
                local.tee $#local2
                i32.const 1
                i32.add
                i32.store offset=4
                block $#label5
                  local.get $#local2
                  local.get $#local4
                  i32.load offset=8
                  i32.eq
                  br_if $#label5
                  local.get $#local4
                  local.get $#local4
                  i64.load offset=12 align=4
                  i64.store offset=24 align=4
                  local.get $#local4
                  i32.const 24
                  i32.add
                  i32.const 1
                  local.get $#local3
                  call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                  local.set $#local0
                  br $#label0
                end
                local.get $#local0
                i32.const 1
                local.get $#local3
                call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                local.set $#local0
                br $#label0
              end
              block $#label4
                local.get $#local2
                i32.const 1
                i32.eq
                br_if $#label4
                local.get $#local4
                i32.const 12
                i32.add
                local.get $#local2
                local.get $#local3
                call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
                local.set $#local0
                br $#label0
              end
              local.get $#local4
              i32.const 4
              i32.or
              i32.const 1
              local.get $#local3
              i32.const 1
              i32.add
              call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
              local.set $#local0
              br $#label0
            end
            block $#label3
              local.get $#local2
              i32.const 1
              i32.eq
              br_if $#label3
              local.get $#local4
              i32.const 8
              i32.add
              local.get $#local2
              local.get $#local3
              call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
              local.set $#local0
              br $#label0
            end
            local.get $#local4
            local.get $#local4
            i32.load offset=4
            local.get $#local3
            i32.add
            i32.store offset=4
            local.get $#local4
            local.get $#local4
            i64.load offset=8
            i64.store offset=24 align=4
            local.get $#local4
            i32.const 24
            i32.add
            i32.const 1
            local.get $#local3
            call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
            local.set $#local0
            br $#label0
          end
          i32.const 418
          call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
          local.get $#local4
          i32.const 8250
          i32.store16 offset=24 align=1
          local.get $#local4
          i32.const 24
          i32.add
          i32.const 2
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get $#local4
          i64.const 748834980320733542
          i64.store offset=40 align=1
          local.get $#local4
          i64.const 7957688057596965985
          i64.store offset=32 align=1
          local.get $#local4
          i64.const 7165064744911531886
          i64.store offset=24 align=1
          local.get $#local4
          i32.const 24
          i32.add
          i32.const 24
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get $#local4
          i32.const 10
          i32.store8 offset=24
          local.get $#local4
          i32.const 24
          i32.add
          i32.const 1
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          unreachable
        end
        local.get $#local4
        i32.const 4
        i32.or
        local.get $#local2
        local.get $#local3
        call $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E
        local.set $#local0
        local.get $#local4
        i32.const 4
        i32.store
      end
      local.get $#local5
      i32.const 4
      i32.add
      local.tee $#local5
      local.get $#local4
      i64.load
      i64.store align=4
      local.get $#local5
      i32.const 16
      i32.add
      local.get $#local4
      i32.const 16
      i32.add
      i32.load
      i32.store
      local.get $#local5
      i32.const 8
      i32.add
      local.get $#local4
      i32.const 8
      i32.add
      i64.load
      i64.store align=4
      local.get $#local4
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get $#local0
    )
    (func $_ZN22wasi_snapshot_preview19BumpAlloc5alloc17h5b31295cc825cc65E (;25;) (type $#type10) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            i32.popcnt
            i32.const 1
            i32.ne
            br_if $#label2
            local.get $#local0
            i32.load offset=4
            local.tee $#local4
            local.get $#local1
            local.get $#local0
            i32.load
            local.tee $#local5
            i32.add
            i32.const -1
            i32.add
            i32.const 0
            local.get $#local1
            i32.sub
            i32.and
            local.get $#local5
            i32.sub
            local.tee $#local1
            i32.lt_u
            br_if $#label1
            local.get $#local4
            local.get $#local1
            i32.sub
            local.tee $#local4
            local.get $#local2
            i32.ge_u
            br_if $#label0
            i32.const 438
            call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
            local.get $#local3
            i32.const 8250
            i32.store16 offset=3 align=1
            local.get $#local3
            i32.const 3
            i32.add
            i32.const 2
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            local.get $#local3
            i32.const 10
            i32.store8 offset=31
            local.get $#local3
            i32.const 1701278305
            i32.store offset=27 align=1
            local.get $#local3
            i64.const 7791349879831294825
            i64.store offset=19 align=1
            local.get $#local3
            i64.const 2334406575183130223
            i64.store offset=11 align=1
            local.get $#local3
            i64.const 7598805550979902561
            i64.store offset=3 align=1
            local.get $#local3
            i32.const 3
            i32.add
            i32.const 29
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            local.get $#local3
            i32.const 10
            i32.store8 offset=3
            local.get $#local3
            i32.const 3
            i32.add
            i32.const 1
            call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
            unreachable
          end
          i32.const 448
          call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
          local.get $#local3
          i32.const 8250
          i32.store16 offset=3 align=1
          local.get $#local3
          i32.const 3
          i32.add
          i32.const 2
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get $#local3
          i32.const 2676
          i32.store16 offset=19 align=1
          local.get $#local3
          i64.const 7954884637768641633
          i64.store offset=11 align=1
          local.get $#local3
          i64.const 2334106421097295465
          i64.store offset=3 align=1
          local.get $#local3
          i32.const 3
          i32.add
          i32.const 18
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          local.get $#local3
          i32.const 10
          i32.store8 offset=3
          local.get $#local3
          i32.const 3
          i32.add
          i32.const 1
          call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
          unreachable
        end
        i32.const 452
        call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
        local.get $#local3
        i32.const 8250
        i32.store16 offset=3 align=1
        local.get $#local3
        i32.const 3
        i32.add
        i32.const 2
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get $#local3
        i32.const 10
        i32.store8 offset=21
        local.get $#local3
        i32.const 25972
        i32.store16 offset=19 align=1
        local.get $#local3
        i64.const 7017575155838820463
        i64.store offset=11 align=1
        local.get $#local3
        i64.const 8367798494427701606
        i64.store offset=3 align=1
        local.get $#local3
        i32.const 3
        i32.add
        i32.const 19
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get $#local3
        i32.const 10
        i32.store8 offset=3
        local.get $#local3
        i32.const 3
        i32.add
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        unreachable
      end
      local.get $#local0
      local.get $#local4
      local.get $#local2
      i32.sub
      i32.store offset=4
      local.get $#local0
      local.get $#local5
      local.get $#local1
      i32.add
      local.tee $#local1
      local.get $#local2
      i32.add
      i32.store
      local.get $#local3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get $#local1
    )
    (func $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE (;26;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local1
      i32.const 32
      i32.store8 offset=47
      local.get $#local1
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get $#local1
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get $#local1
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get $#local1
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get $#local1
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get $#local1
      i32.const 11
      i32.add
      i32.const 37
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
      local.get $#local1
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E (;27;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E
      i32.store offset=12
      local.get $#local2
      i32.const 4
      i32.add
      local.get $#local2
      i32.const 12
      i32.add
      local.get $#local0
      local.get $#local1
      call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E
      block $#label0
        local.get $#local2
        i32.load offset=4
        local.tee $#local1
        i32.const 2
        i32.eq
        br_if $#label0
        local.get $#local1
        br_if $#label0
        local.get $#local2
        i32.load offset=8
        local.tee $#local1
        i32.const -1
        i32.eq
        br_if $#label0
        local.get $#local1
        call $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E
      end
      block $#label0
        local.get $#local2
        i32.load offset=12
        local.tee $#local1
        i32.const -1
        i32.eq
        br_if $#label0
        local.get $#local1
        call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
      end
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE (;28;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local0
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get $#local1
      i32.const 10
      i32.store8 offset=15
      local.get $#local1
      i32.const 15
      i32.add
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $environ_get (;29;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
            local.tee $#local3
            i32.load
            i32.const 560490357
            i32.ne
            br_if $#label2
            local.get $#local3
            i32.load offset=65532
            i32.const 560490357
            i32.ne
            br_if $#label1
            local.get $#local3
            i32.const 59032
            i32.store offset=20
            local.get $#local3
            i32.const -1
            i32.store offset=12
            local.get $#local3
            local.get $#local1
            i32.store offset=8
            local.get $#local3
            local.get $#local3
            i32.const 6192
            i32.add
            i32.store offset=16
            local.get $#local3
            i32.load offset=4
            local.set $#local1
            local.get $#local3
            i32.const 2
            i32.store offset=4
            local.get $#local1
            i32.const 4
            i32.ne
            br_if $#label0
            local.get $#local2
            i64.const 0
            i64.store align=4
            local.get $#local2
            call $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E
            local.get $#local2
            i32.load offset=4
            local.set $#local4
            local.get $#local2
            i32.load
            local.set $#local1
            local.get $#local3
            i32.const 4
            i32.store offset=4
            block $#label3
              local.get $#local4
              i32.eqz
              br_if $#label3
              loop $#label4
                local.get $#local1
                i32.const 12
                i32.add
                i32.load
                local.set $#local3
                local.get $#local1
                i32.const 8
                i32.add
                i32.load
                local.set $#local5
                local.get $#local1
                i32.const 4
                i32.add
                i32.load
                local.set $#local6
                local.get $#local0
                local.get $#local1
                i32.load
                local.tee $#local7
                i32.store
                local.get $#local7
                local.get $#local6
                i32.add
                i32.const 61
                i32.store8
                local.get $#local5
                local.get $#local3
                i32.add
                i32.const 0
                i32.store8
                local.get $#local1
                i32.const 16
                i32.add
                local.set $#local1
                local.get $#local0
                i32.const 4
                i32.add
                local.set $#local0
                local.get $#local4
                i32.const -1
                i32.add
                local.tee $#local4
                br_if $#label4
              end
            end
            local.get $#local2
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
      local.get $#local2
      i32.const 8250
      i32.store16 align=1
      local.get $#local2
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local2
      i32.const 10
      i32.store8 offset=28
      local.get $#local2
      i32.const 1952805664
      i32.store offset=24 align=1
      local.get $#local2
      i64.const 8747223464599642400
      i64.store offset=16 align=1
      local.get $#local2
      i64.const 8245937404367563884
      i64.store offset=8 align=1
      local.get $#local2
      i64.const 6998721855778483561
      i64.store align=1
      local.get $#local2
      i32.const 29
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local2
      i32.const 10
      i32.store8
      local.get $#local2
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $environ_sizes_get (;30;) (type $#type11) (param $#local0 i32) (param $#local1 i32) (result i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    call $get_allocation_state
                    i32.const -2
                    i32.add
                    br_table $#label5 $#label6 $#label5 $#label6
                  end
                  i32.const 0
                  local.set $#local3
                  local.get $#local0
                  i32.const 0
                  i32.store
                  br $#label4
                end
                call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                local.tee $#local3
                i32.load
                i32.const 560490357
                i32.ne
                br_if $#label3
                local.get $#local3
                i32.load offset=65532
                i32.const 560490357
                i32.ne
                br_if $#label2
                local.get $#local3
                i32.const 59032
                i32.store offset=16
                local.get $#local3
                local.get $#local3
                i32.const 6192
                i32.add
                i32.store offset=12
                local.get $#local3
                i32.load offset=4
                local.set $#local4
                local.get $#local3
                i64.const 1
                i64.store offset=4 align=4
                local.get $#local4
                i32.const 4
                i32.ne
                br_if $#label1
                local.get $#local2
                i64.const 0
                i64.store align=4
                local.get $#local2
                call $_ZN22wasi_snapshot_preview124wasi_cli_get_environment17hf457ad3f43839874E
                local.get $#local2
                i32.load offset=4
                local.set $#local4
                local.get $#local3
                i32.load offset=4
                local.set $#local5
                local.get $#local3
                i32.const 4
                i32.store offset=4
                local.get $#local5
                i32.const 1
                i32.ne
                br_if $#label0
                local.get $#local3
                i32.load offset=8
                local.set $#local3
                local.get $#local0
                local.get $#local4
                i32.store
                local.get $#local3
                local.get $#local4
                i32.const 1
                i32.shl
                i32.add
                local.set $#local3
              end
              local.get $#local1
              local.get $#local3
              i32.store
              local.get $#local2
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
        local.get $#local2
        i32.const 8250
        i32.store16 align=1
        local.get $#local2
        i32.const 2
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get $#local2
        i32.const 10
        i32.store8 offset=28
        local.get $#local2
        i32.const 1952805664
        i32.store offset=24 align=1
        local.get $#local2
        i64.const 8747223464599642400
        i64.store offset=16 align=1
        local.get $#local2
        i64.const 8245937404367563884
        i64.store offset=8 align=1
        local.get $#local2
        i64.const 6998721855778483561
        i64.store align=1
        local.get $#local2
        i32.const 29
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        local.get $#local2
        i32.const 10
        i32.store8
        local.get $#local2
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
        unreachable
      end
      i32.const 628
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview15State11descriptors17hfc2c9cda66bef96dE (;31;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 6160
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local1
          i32.load offset=24
          br_if $#label1
          local.get $#local1
          i32.const -1
          i32.store offset=24
          local.get $#local1
          i32.const 32
          i32.add
          local.set $#local3
          block $#label2
            local.get $#local1
            i32.load offset=6180
            i32.const 2
            i32.ne
            br_if $#label2
            local.get $#local2
            local.get $#local1
            call $_ZN22wasi_snapshot_preview111descriptors11Descriptors3new17h23d25887e02c514fE
            local.get $#local3
            local.get $#local2
            i32.const 6160
            call $memcpy
            drop
            local.get $#local1
            i32.load offset=6180
            i32.const 2
            i32.eq
            br_if $#label0
          end
          local.get $#local0
          local.get $#local1
          i32.const 24
          i32.add
          i32.store offset=4
          local.get $#local0
          local.get $#local3
          i32.store
          local.get $#local2
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
    (func $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E (;32;) (type $#type6) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.set $#local1
      i32.const 6
      local.set $#local2
      block $#label0
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        block $#label9
                          block $#label10
                            block $#label11
                              block $#label12
                                block $#label13
                                  block $#label14
                                    block $#label15
                                      block $#label16
                                        block $#label17
                                          block $#label18
                                            block $#label19
                                              block $#label20
                                                block $#label21
                                                  block $#label22
                                                    block $#label23
                                                      block $#label24
                                                        block $#label25
                                                          block $#label26
                                                            block $#label27
                                                              block $#label28
                                                                block $#label29
                                                                  block $#label30
                                                                    block $#label31
                                                                      block $#label32
                                                                        block $#label33
                                                                          block $#label34
                                                                            block $#label35
                                                                              block $#label36
                                                                                local.get $#local0
                                                                                i32.const 255
                                                                                i32.and
                                                                                br_table $#label36 $#label0 $#label35 $#label34 $#label33 $#label32 $#label31 $#label30 $#label29 $#label28 $#label27 $#label26 $#label25 $#label24 $#label23 $#label22 $#label21 $#label20 $#label19 $#label18 $#label17 $#label16 $#label15 $#label14 $#label13 $#label12 $#label11 $#label10 $#label9 $#label8 $#label7 $#label6 $#label5 $#label4 $#label3 $#label2 $#label1 $#label36
                                                                              end
                                                                              local.get $#local1
                                                                              i32.const 2
                                                                              i32.store16 offset=14
                                                                              local.get $#local1
                                                                              i32.const 14
                                                                              i32.add
                                                                              local.set $#local0
                                                                              local.get $#local1
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
        local.set $#local2
      end
      local.get $#local2
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type17h38d62f575f54468dE (;33;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local1
      i32.load
      local.get $#local2
      i32.const 14
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type10wit_import17h708de2cfd599ed76E
      i32.const 0
      local.set $#local3
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.load8_u offset=14
            br_table $#label1 $#label2 $#label0
          end
          i32.const 1
          local.set $#local3
        end
        local.get $#local2
        i32.load8_u offset=15
        local.set $#local1
      end
      local.get $#local0
      local.get $#local1
      i32.store8 offset=1
      local.get $#local0
      local.get $#local3
      i32.const 1
      i32.and
      i32.store8
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat17hbce2ab26bb1887cbE (;34;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local1
      i32.load
      local.get $#local2
      i32.const 8
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat10wit_import17hd8a47ee046c71cb9E
      local.get $#local2
      i32.load8_u offset=16
      local.set $#local1
      block $#label0
        block $#label1
          local.get $#local2
          i32.load8_u offset=8
          br_if $#label1
          local.get $#local0
          local.get $#local2
          i32.load offset=104
          i32.store offset=88
          local.get $#local0
          local.get $#local2
          i64.load offset=96
          i64.store offset=80
          local.get $#local0
          local.get $#local2
          i32.load offset=80
          i32.store offset=64
          local.get $#local0
          local.get $#local2
          i64.load offset=72
          i64.store offset=56
          local.get $#local0
          local.get $#local2
          i32.load offset=56
          i32.store offset=40
          local.get $#local0
          local.get $#local2
          i64.load offset=48
          i64.store offset=32
          local.get $#local0
          local.get $#local2
          i64.load offset=32
          i64.store offset=16
          local.get $#local0
          local.get $#local2
          i64.load offset=24
          i64.store offset=8
          local.get $#local0
          local.get $#local1
          i32.store8
          local.get $#local0
          local.get $#local2
          i32.load8_u offset=88
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=72
          local.get $#local0
          local.get $#local2
          i32.load8_u offset=64
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=48
          local.get $#local0
          local.get $#local2
          i32.load8_u offset=40
          i32.const 0
          i32.ne
          i64.extend_i32_u
          i64.store offset=24
          br $#label0
        end
        local.get $#local0
        i64.const 2
        i64.store offset=72
        local.get $#local0
        local.get $#local1
        i32.store8
      end
      local.get $#local2
      i32.const 112
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview15State17with_import_alloc17h8e53b3b47ee99160E (;35;) (type $#type7) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32)
      (local $#local3 i32) (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee $#local3
      global.set $__stack_pointer
      local.get $#local1
      i32.load offset=4
      local.set $#local4
      local.get $#local1
      local.get $#local2
      i64.load align=4
      i64.store offset=4 align=4
      local.get $#local1
      i32.const 12
      i32.add
      local.get $#local2
      i32.const 8
      i32.add
      i64.load align=4
      i64.store align=4
      local.get $#local1
      i32.const 20
      i32.add
      local.get $#local2
      i32.const 16
      i32.add
      i32.load
      i32.store
      block $#label0
        local.get $#local4
        i32.const 4
        i32.ne
        br_if $#label0
        local.get $#local3
        i64.const 0
        i64.store align=4
        local.get $#local3
        call $_ZN22wasi_snapshot_preview111descriptors31wasi_filesystem_get_directories17h9e1fe8bbb8a7f366E
        local.get $#local0
        local.get $#local3
        i64.load align=4
        i64.store align=4
        local.get $#local0
        i32.const 24
        i32.add
        local.get $#local1
        i32.const 4
        i32.add
        local.tee $#local1
        i32.const 16
        i32.add
        i32.load
        i32.store
        local.get $#local0
        i32.const 16
        i32.add
        local.get $#local1
        i32.const 8
        i32.add
        i64.load align=4
        i64.store align=4
        local.get $#local0
        local.get $#local1
        i64.load align=4
        i64.store offset=8 align=4
        local.get $#local1
        i32.const 4
        i32.store
        local.get $#local3
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 2884
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get $#local3
      i32.const 8250
      i32.store16 align=1
      local.get $#local3
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local3
      i32.const 10
      i32.store8 offset=28
      local.get $#local3
      i32.const 1952805664
      i32.store offset=24 align=1
      local.get $#local3
      i64.const 8747223464599642400
      i64.store offset=16 align=1
      local.get $#local3
      i64.const 8245937404367563884
      i64.store offset=8 align=1
      local.get $#local3
      i64.const 6998721855778483561
      i64.store align=1
      local.get $#local3
      i32.const 29
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local3
      i32.const 10
      i32.store8
      local.get $#local3
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream17h9367b14712760239E (;36;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local2
      i64.const 0
      i64.store offset=8
      local.get $#local1
      i32.load
      local.get $#local2
      i32.const 8
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream10wit_import17h2638911ab3ccf3ddE
      block $#label0
        block $#label1
          local.get $#local2
          i32.load8_u offset=8
          local.tee $#local1
          br_if $#label1
          local.get $#local0
          local.get $#local2
          i32.load offset=12
          i32.store offset=4
          br $#label0
        end
        local.get $#local0
        local.get $#local2
        i32.load8_u offset=12
        i32.store8 offset=1
      end
      local.get $#local0
      local.get $#local1
      i32.store8
      local.get $#local2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E (;37;) (type $#type4) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      (local $#local4 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      local.get $#local1
      i32.load
      local.get $#local2
      local.get $#local3
      local.get $#local4
      i32.const 4
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush10wit_import17hd0e6bb9bc2c2ef7aE
      block $#label0
        block $#label1
          local.get $#local4
          i32.load8_u offset=4
          br_if $#label1
          local.get $#local0
          i32.const 2
          i32.store
          br $#label0
        end
        local.get $#local0
        i64.const 1
        local.get $#local4
        i64.load32_u offset=12
        i64.const 32
        i64.shl
        local.get $#local4
        i32.load8_u offset=8
        select
        i64.store align=4
      end
      local.get $#local4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E (;38;) (type $#type6) (param $#local0 i32) (result i32)
      (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local0
      local.get $#local1
      i32.const 14
      i32.add
      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types21filesystem_error_code10wit_import17h2b5d8e99a65d0583E
      block $#label0
        block $#label1
          local.get $#local1
          i32.load8_u offset=14
          br_if $#label1
          i32.const 29
          local.set $#local2
          br $#label0
        end
        local.get $#local1
        i32.load8_u offset=15
        call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
        local.set $#local2
      end
      block $#label0
        local.get $#local0
        i32.const -1
        i32.eq
        br_if $#label0
        local.get $#local0
        call $_ZN128_$LT$wasi_snapshot_preview1..bindings..wasi..io..error..Error$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17hd7edbeeb13c77ae3E
      end
      local.get $#local1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get $#local2
    )
    (func $_ZN4core3ptr68drop_in_place$LT$wasi_snapshot_preview1..descriptors..Descriptor$GT$17h4b6e45103ecd4053E (;39;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      block $#label0
        local.get $#local0
        i32.load
        i32.const 1
        i32.ne
        br_if $#label0
        block $#label1
          local.get $#local0
          i32.load offset=8
          i32.eqz
          br_if $#label1
          local.get $#local0
          i32.load offset=12
          local.tee $#local1
          i32.const -1
          i32.eq
          br_if $#label1
          local.get $#local1
          call $_ZN136_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..InputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h5547e0ecd980db5bE
        end
        block $#label1
          local.get $#local0
          i32.load offset=16
          i32.eqz
          br_if $#label1
          local.get $#local0
          i32.load offset=20
          local.tee $#local1
          i32.const -1
          i32.eq
          br_if $#label1
          local.get $#local1
          call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
        end
        local.get $#local0
        i32.load8_u offset=41
        i32.const 2
        i32.eq
        br_if $#label0
        local.get $#local0
        i32.load offset=24
        local.tee $#local0
        i32.const -1
        i32.eq
        br_if $#label0
        local.get $#local0
        call $_ZN141_$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..Descriptor$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h34b1c3918a0eebe8E
      end
    )
    (func $fd_write (;40;) (type $#type8) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee $#local4
      global.set $__stack_pointer
      block $#label0
        block $#label1
          block $#label2
            call $get_allocation_state
            i32.const -2
            i32.add
            br_table $#label1 $#label2 $#label1 $#label2
          end
          local.get $#local3
          i32.const 0
          i32.store
          i32.const 29
          local.set $#local1
          br $#label0
        end
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      block $#label8
                        local.get $#local2
                        i32.const 2
                        i32.lt_u
                        br_if $#label8
                        local.get $#local1
                        local.get $#local2
                        i32.const 3
                        i32.shl
                        i32.add
                        i32.const -8
                        i32.add
                        local.set $#local5
                        loop $#label9
                          local.get $#local1
                          i32.load offset=4
                          local.tee $#local6
                          br_if $#label6
                          local.get $#local1
                          i32.const 8
                          i32.add
                          local.set $#local1
                          local.get $#local2
                          i32.const -1
                          i32.add
                          local.tee $#local2
                          i32.const 1
                          i32.gt_u
                          br_if $#label9
                        end
                        local.get $#local5
                        local.set $#local1
                        br $#label7
                      end
                      local.get $#local2
                      i32.eqz
                      br_if $#label5
                    end
                    local.get $#local1
                    i32.load offset=4
                    local.set $#local6
                  end
                  local.get $#local1
                  i32.load
                  local.set $#local7
                  call $_ZN22wasi_snapshot_preview15State3ptr17h839dd1b6941dd4ebE
                  local.tee $#local1
                  i32.load
                  i32.const 560490357
                  i32.ne
                  br_if $#label4
                  local.get $#local1
                  i32.load offset=65532
                  i32.const 560490357
                  i32.ne
                  br_if $#label3
                  local.get $#local4
                  i32.const 8
                  i32.add
                  local.get $#local1
                  call $_ZN22wasi_snapshot_preview15State11descriptors17hfc2c9cda66bef96dE
                  i32.const 8
                  local.set $#local1
                  local.get $#local4
                  i32.load offset=12
                  local.set $#local2
                  local.get $#local4
                  i32.load offset=8
                  local.tee $#local5
                  i32.load16_u offset=6144
                  local.get $#local0
                  i32.le_u
                  br_if $#label1
                  local.get $#local5
                  local.get $#local0
                  i32.const 48
                  i32.mul
                  i32.add
                  local.tee $#local0
                  i32.load
                  i32.const 1
                  i32.ne
                  br_if $#label1
                  local.get $#local4
                  i32.const 16
                  i32.add
                  local.get $#local0
                  i32.const 8
                  i32.add
                  call $_ZN22wasi_snapshot_preview111descriptors7Streams16get_write_stream17h51434d5cfb257d25E
                  block $#label6
                    local.get $#local4
                    i32.load16_u offset=16
                    br_if $#label6
                    local.get $#local4
                    i32.load offset=20
                    local.set $#local1
                    block $#label7
                      local.get $#local0
                      i32.load8_u offset=41
                      local.tee $#local5
                      i32.const 2
                      i32.eq
                      br_if $#label7
                      local.get $#local4
                      i32.const 16
                      i32.add
                      local.get $#local5
                      i32.const 1
                      i32.and
                      local.get $#local1
                      local.get $#local7
                      local.get $#local6
                      call $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E
                      local.get $#local4
                      i32.load16_u offset=16
                      br_if $#label6
                      br $#label2
                    end
                    local.get $#local4
                    i32.const 16
                    i32.add
                    i32.const 1
                    local.get $#local1
                    local.get $#local7
                    local.get $#local6
                    call $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E
                    local.get $#local4
                    i32.load16_u offset=16
                    i32.eqz
                    br_if $#label2
                  end
                  local.get $#local4
                  i32.load16_u offset=18
                  local.set $#local1
                  br $#label1
                end
                i32.const 0
                local.set $#local1
                local.get $#local3
                i32.const 0
                i32.store
                br $#label0
              end
              i32.const 2745
              call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
              unreachable
            end
            i32.const 2746
            call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
            unreachable
          end
          local.get $#local4
          i32.load offset=20
          local.set $#local1
          block $#label2
            local.get $#local0
            i32.load8_u offset=41
            i32.const 2
            i32.eq
            br_if $#label2
            block $#label3
              local.get $#local0
              i32.load8_u offset=40
              br_if $#label3
              local.get $#local0
              local.get $#local0
              i64.load offset=32
              local.get $#local1
              i64.extend_i32_u
              i64.add
              i64.store offset=32
              br $#label2
            end
            local.get $#local4
            i32.const 16
            i32.add
            local.get $#local0
            i32.const 24
            i32.add
            call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor4stat17hbce2ab26bb1887cbE
            block $#label3
              local.get $#local4
              i64.load offset=88
              i64.const 2
              i64.eq
              br_if $#label3
              local.get $#local0
              local.get $#local4
              i64.load offset=32
              i64.store offset=32
              br $#label2
            end
            local.get $#local4
            i32.load8_u offset=16
            call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
            local.set $#local1
            br $#label1
          end
          local.get $#local3
          local.get $#local1
          i32.store
          i32.const 0
          local.set $#local1
        end
        local.get $#local2
        local.get $#local2
        i32.load
        i32.const 1
        i32.add
        i32.store
      end
      local.get $#local4
      i32.const 112
      i32.add
      global.set $__stack_pointer
      local.get $#local1
      i32.const 65535
      i32.and
    )
    (func $_ZN22wasi_snapshot_preview111descriptors7Streams16get_write_stream17h51434d5cfb257d25E (;41;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i64) (local $#local5 i32) (local $#local6 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      local.get $#local1
      i32.const 12
      i32.add
      local.set $#local3
      block $#label0
        block $#label1
          block $#label2
            local.get $#local1
            i32.load offset=8
            br_if $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    local.get $#local1
                    i32.load8_u offset=33
                    i32.const 2
                    i32.eq
                    br_if $#label6
                    block $#label7
                      local.get $#local1
                      i32.load8_u offset=20
                      i32.const 3
                      i32.ne
                      br_if $#label7
                      local.get $#local0
                      i32.const 8
                      i32.store16 offset=2
                      br $#label4
                    end
                    block $#label7
                      local.get $#local1
                      i32.load8_u offset=32
                      br_if $#label7
                      local.get $#local1
                      i64.load offset=24
                      local.set $#local4
                      local.get $#local2
                      i64.const 0
                      i64.store offset=8
                      local.get $#local1
                      i32.load offset=16
                      local.get $#local4
                      local.get $#local2
                      i32.const 8
                      i32.add
                      call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor16write_via_stream10wit_import17h863d15eb6a8c37dcE
                      local.get $#local2
                      i32.load8_u offset=8
                      br_if $#label5
                      br $#label3
                    end
                    local.get $#local2
                    i32.const 8
                    i32.add
                    local.get $#local1
                    i32.const 16
                    i32.add
                    call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor17append_via_stream17h9367b14712760239E
                    local.get $#local2
                    i32.load8_u offset=8
                    i32.eqz
                    br_if $#label3
                    local.get $#local0
                    local.get $#local2
                    i32.load8_u offset=9
                    call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
                    i32.store16 offset=2
                    br $#label4
                  end
                  local.get $#local0
                  i32.const 8
                  i32.store16 offset=2
                  br $#label4
                end
                local.get $#local0
                local.get $#local2
                i32.load8_u offset=12
                call $_ZN22wasi_snapshot_preview1152_$LT$impl$u20$core..convert..From$LT$wasi_snapshot_preview1..bindings..wasi..filesystem..types..ErrorCode$GT$$u20$for$u20$wasi..lib_generated..Errno$GT$4from17h5049efef10c5cff2E
                i32.store16 offset=2
              end
              i32.const 1
              local.set $#local1
              br $#label1
            end
            local.get $#local2
            i32.load offset=12
            local.set $#local5
            local.get $#local1
            i32.load offset=8
            local.tee $#local6
            br_if $#label0
            block $#label3
              local.get $#local6
              i32.eqz
              br_if $#label3
              local.get $#local3
              i32.load
              local.tee $#local6
              i32.const -1
              i32.eq
              br_if $#label3
              local.get $#local6
              call $_ZN137_$LT$wasi_snapshot_preview1..bindings..wasi..io..streams..OutputStream$u20$as$u20$wasi_snapshot_preview1..bindings.._rt..WasmResource$GT$4drop4drop17h75318ef9c980de26E
            end
            local.get $#local1
            local.get $#local5
            i32.store offset=12
            local.get $#local1
            i32.const 1
            i32.store offset=8
          end
          local.get $#local0
          local.get $#local3
          i32.store offset=4
          i32.const 0
          local.set $#local1
        end
        local.get $#local0
        local.get $#local1
        i32.store16
        local.get $#local2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 159
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview112BlockingMode5write17hed96c7c791c7f7a5E (;42;) (type $#type5) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (param $#local4 i32)
      (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local5
      global.set $__stack_pointer
      block $#label0
        block $#label1
          local.get $#local1
          i32.eqz
          br_if $#label1
          local.get $#local4
          local.set $#local1
          block $#label2
            block $#label3
              block $#label4
                loop $#label5
                  local.get $#local5
                  local.get $#local2
                  local.get $#local3
                  local.get $#local1
                  i32.const 4096
                  local.get $#local1
                  i32.const 4096
                  i32.lt_u
                  select
                  local.tee $#local6
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream24blocking_write_and_flush17hc7527ea8a37227f8E
                  block $#label6
                    local.get $#local5
                    i32.load
                    local.tee $#local7
                    i32.const 2
                    i32.eq
                    br_if $#label6
                    local.get $#local7
                    br_table $#label4 $#label3 $#label4
                  end
                  local.get $#local3
                  local.get $#local6
                  i32.add
                  local.set $#local3
                  local.get $#local1
                  local.get $#local6
                  i32.sub
                  local.tee $#local1
                  br_if $#label5
                end
                local.get $#local0
                i32.const 0
                i32.store16
                local.get $#local0
                local.get $#local4
                i32.store offset=4
                br $#label0
              end
              local.get $#local5
              i32.load offset=4
              call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
              local.set $#local1
              br $#label2
            end
            i32.const 29
            local.set $#local1
          end
          local.get $#local0
          i32.const 1
          i32.store16
          local.get $#local0
          local.get $#local1
          i32.store16 offset=2
          br $#label0
        end
        local.get $#local2
        i32.load
        local.get $#local5
        call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream11check_write10wit_import17hdb0f94e542b67356E
        block $#label1
          block $#label2
            block $#label3
              block $#label4
                block $#label5
                  block $#label6
                    block $#label7
                      local.get $#local5
                      i32.load8_u
                      br_if $#label7
                      local.get $#local5
                      i32.load offset=8
                      local.set $#local1
                      br $#label6
                    end
                    i32.const 0
                    local.set $#local1
                    i64.const 1
                    local.get $#local5
                    i64.load32_u offset=12
                    i64.const 32
                    i64.shl
                    local.get $#local5
                    i32.load8_u offset=8
                    select
                    local.tee $#local8
                    i32.wrap_i64
                    i32.const 1
                    i32.ne
                    br_if $#label5
                  end
                  local.get $#local2
                  i32.load
                  local.get $#local3
                  local.get $#local4
                  local.get $#local1
                  local.get $#local4
                  local.get $#local1
                  i32.lt_u
                  select
                  local.tee $#local1
                  local.get $#local5
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream5write10wit_import17h1ec4b5645a67d72bE
                  local.get $#local5
                  i32.load8_u
                  br_if $#label3
                  local.get $#local2
                  i32.load
                  local.get $#local5
                  call $_ZN22wasi_snapshot_preview18bindings4wasi2io7streams12OutputStream14blocking_flush10wit_import17h7bbaef6ef9f5f783E
                  local.get $#local5
                  i32.load8_u
                  br_if $#label4
                  local.get $#local0
                  i32.const 0
                  i32.store16
                  local.get $#local0
                  local.get $#local1
                  i32.store offset=4
                  br $#label0
                end
                local.get $#local8
                i64.const 32
                i64.shr_u
                i32.wrap_i64
                call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
                local.set $#local1
                local.get $#local0
                i32.const 1
                i32.store16
                local.get $#local0
                local.get $#local1
                i32.store16 offset=2
                br $#label0
              end
              i64.const 1
              local.get $#local5
              i64.load32_u offset=8
              i64.const 32
              i64.shl
              local.get $#local5
              i32.load8_u offset=4
              select
              local.tee $#local8
              i64.const 1
              i64.and
              i64.eqz
              br_if $#label2
              i32.const 0
              local.set $#local1
              local.get $#local0
              i32.const 0
              i32.store offset=4
              br $#label1
            end
            block $#label3
              block $#label4
                i64.const 1
                local.get $#local5
                i64.load32_u offset=8
                i64.const 32
                i64.shl
                local.get $#local5
                i32.load8_u offset=4
                select
                local.tee $#local8
                i64.const 1
                i64.and
                i64.eqz
                br_if $#label4
                i32.const 0
                local.set $#local1
                local.get $#local0
                i32.const 0
                i32.store offset=4
                br $#label3
              end
              local.get $#local0
              local.get $#local8
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
              i32.store16 offset=2
              i32.const 1
              local.set $#local1
            end
            local.get $#local0
            local.get $#local1
            i32.store16
            br $#label0
          end
          local.get $#local0
          local.get $#local8
          i64.const 32
          i64.shr_u
          i32.wrap_i64
          call $_ZN22wasi_snapshot_preview121stream_error_to_errno17heaf124eb01ef5781E
          i32.store16 offset=2
          i32.const 1
          local.set $#local1
        end
        local.get $#local0
        local.get $#local1
        i32.store16
      end
      local.get $#local5
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $proc_exit (;43;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      local.get $#local0
      i32.const 0
      i32.ne
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit17h506fb275a8b6a599E
      i32.const 2280
      call $_ZN22wasi_snapshot_preview16macros18eprint_unreachable17h00c8203e9b7ff55aE
      local.get $#local1
      i32.const 8250
      i32.store16 offset=10 align=1
      local.get $#local1
      i32.const 10
      i32.add
      i32.const 2
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local1
      i32.const 2593
      i32.store16 offset=46 align=1
      local.get $#local1
      i32.const 1953069157
      i32.store offset=42 align=1
      local.get $#local1
      i64.const 2338537461596644384
      i64.store offset=34 align=1
      local.get $#local1
      i64.const 7957695015159098981
      i64.store offset=26 align=1
      local.get $#local1
      i64.const 7882825952909664372
      i64.store offset=18 align=1
      local.get $#local1
      i64.const 7599935561254793064
      i64.store offset=10 align=1
      local.get $#local1
      i32.const 10
      i32.add
      i32.const 38
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      local.get $#local1
      i32.const 10
      i32.store8 offset=10
      local.get $#local1
      i32.const 10
      i32.add
      i32.const 1
      call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit17h506fb275a8b6a599E (;44;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli4exit4exit10wit_import17hefacdcba211d0d02E
    )
    (func $_ZN22wasi_snapshot_preview15State3new17h27fd4b5328bc4eeeE (;45;) (type $#type9) (result i32)
      (local $#local0 i32)
      block $#label0
        call $get_allocation_state
        i32.const 2
        i32.ne
        br_if $#label0
        i32.const 3
        call $set_allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $_ZN22wasi_snapshot_preview15State3new12cabi_realloc17h88297338052a23f0E
        local.set $#local0
        i32.const 4
        call $set_allocation_state
        local.get $#local0
        i32.const 2
        i32.store offset=6180
        local.get $#local0
        i32.const 0
        i32.store offset=24
        local.get $#local0
        i64.const 17740359541
        i64.store
        local.get $#local0
        i32.const 65480
        i32.add
        i32.const 0
        i32.const 37
        call $memset
        drop
        local.get $#local0
        i32.const 560490357
        i32.store offset=65532
        local.get $#local0
        i32.const 11822
        i32.store16 offset=65528
        local.get $#local0
        i32.const 0
        i32.store offset=65520
        local.get $#local0
        return
      end
      i32.const 2777
      call $_ZN22wasi_snapshot_preview16macros11assert_fail17h7acabc36093cc74aE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview111descriptors11Descriptors3new17h23d25887e02c514fE (;46;) (type $#type2) (param $#local0 i32) (param $#local1 i32)
      (local $#local2 i32) (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32)
      global.get $__stack_pointer
      i32.const 6256
      i32.sub
      local.tee $#local2
      global.set $__stack_pointer
      i32.const 0
      local.set $#local3
      local.get $#local2
      i32.const 0
      i32.store offset=6156
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli5stdin9get_stdin10wit_import17he5bdc61203ca72fdE
      local.set $#local4
      local.get $#local2
      i32.const 2
      i32.store8 offset=49
      local.get $#local2
      i32.const 0
      i32.store8 offset=32
      local.get $#local2
      i64.const 0
      i64.store offset=24
      local.get $#local2
      i32.const 1
      i32.store offset=8
      local.get $#local2
      local.get $#local4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=16
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stdout10get_stdout10wit_import17hfdc059c6457e3d59E
      local.set $#local4
      local.get $#local2
      i32.const 2
      i32.store8 offset=97
      local.get $#local2
      i32.const 1
      i32.store8 offset=80
      local.get $#local2
      i64.const 0
      i64.store offset=64
      local.get $#local2
      i32.const 1
      i32.store offset=56
      local.get $#local2
      local.get $#local4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=72
      call $_ZN22wasi_snapshot_preview18bindings4wasi3cli6stderr10get_stderr10wit_import17hb69a737fc73ae844E
      local.set $#local4
      local.get $#local2
      i32.const 3
      i32.store16 offset=6152
      local.get $#local2
      i32.const 2
      i32.store8 offset=145
      local.get $#local2
      i32.const 2
      i32.store8 offset=128
      local.get $#local2
      i64.const 0
      i64.store offset=112
      local.get $#local2
      i32.const 1
      i32.store offset=104
      local.get $#local2
      local.get $#local4
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 1
      i64.or
      i64.store offset=120
      local.get $#local2
      i32.const 59032
      i32.store offset=6184
      local.get $#local2
      local.get $#local1
      i32.const 6192
      i32.add
      i32.store offset=6180
      local.get $#local2
      i64.const 1
      i64.store offset=6172 align=4
      local.get $#local2
      i32.const 6208
      i32.add
      local.get $#local1
      local.get $#local2
      i32.const 6172
      i32.add
      call $_ZN22wasi_snapshot_preview15State17with_import_alloc17h8e53b3b47ee99160E
      block $#label0
        block $#label1
          block $#label2
            local.get $#local2
            i32.load offset=6212
            local.tee $#local5
            i32.eqz
            br_if $#label2
            local.get $#local2
            i32.load offset=6208
            local.set $#local1
            local.get $#local2
            i32.const 152
            i32.add
            local.set $#local6
            local.get $#local2
            i32.const 6208
            i32.add
            i32.const 8
            i32.add
            local.set $#local4
            loop $#label3
              local.get $#local2
              i32.const 6192
              i32.add
              i32.const 8
              i32.add
              local.get $#local1
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get $#local2
              local.get $#local1
              i64.load align=4
              i64.store offset=6192
              local.get $#local2
              local.get $#local2
              i32.const 6192
              i32.add
              call $_ZN22wasi_snapshot_preview18bindings4wasi10filesystem5types10Descriptor8get_type17h38d62f575f54468dE
              local.get $#local2
              i32.load8_u
              br_if $#label1
              local.get $#local2
              i32.load8_u offset=1
              local.set $#local7
              local.get $#local4
              i64.const 0
              i64.store
              local.get $#local4
              i32.const 8
              i32.add
              i64.const 0
              i64.store
              local.get $#local2
              local.get $#local2
              i32.load offset=6200
              i32.store offset=6252
              local.get $#local2
              i32.const 256
              i32.store16 offset=6248
              local.get $#local2
              i64.const 0
              i64.store offset=6240
              local.get $#local2
              local.get $#local2
              i32.load offset=6192
              i32.store offset=6232
              local.get $#local2
              i32.const 1
              i32.store offset=6208
              local.get $#local2
              local.get $#local7
              i32.store8 offset=6236
              local.get $#local3
              i32.const 125
              i32.eq
              br_if $#label0
              local.get $#local6
              local.get $#local2
              i32.const 6208
              i32.add
              i32.const 48
              call $memcpy
              local.set $#local6
              local.get $#local2
              local.get $#local3
              i32.const 4
              i32.add
              i32.store16 offset=6152
              local.get $#local1
              i32.const 12
              i32.add
              local.set $#local1
              local.get $#local6
              i32.const 48
              i32.add
              local.set $#local6
              local.get $#local3
              i32.const 1
              i32.add
              local.tee $#local7
              local.set $#local3
              local.get $#local5
              local.get $#local7
              i32.ne
              br_if $#label3
            end
          end
          local.get $#local0
          local.get $#local2
          i32.const 8
          i32.add
          i32.const 6160
          call $memcpy
          drop
          local.get $#local2
          i32.const 6256
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 159
        call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
        unreachable
      end
      local.get $#local2
      i32.const 6208
      i32.add
      call $_ZN4core3ptr68drop_in_place$LT$wasi_snapshot_preview1..descriptors..Descriptor$GT$17h4b6e45103ecd4053E
      i32.const 159
      call $_ZN22wasi_snapshot_preview16macros11unreachable17hb774b17288a5f2eaE
      unreachable
    )
    (func $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE (;47;) (type $#type1) (param $#local0 i32)
      (local $#local1 i32) (local $#local2 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee $#local1
      global.set $__stack_pointer
      block $#label0
        local.get $#local0
        i32.eqz
        br_if $#label0
        local.get $#local0
        i32.const 10
        i32.div_u
        local.tee $#local2
        call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
        local.get $#local1
        local.get $#local2
        i32.const 246
        i32.mul
        local.get $#local0
        i32.add
        i32.const 48
        i32.or
        i32.store8 offset=15
        local.get $#local1
        i32.const 15
        i32.add
        i32.const 1
        call $_ZN22wasi_snapshot_preview16macros5print17h869d9c6901eb5991E
      end
      local.get $#local1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $_ZN22wasi_snapshot_preview16macros10eprint_u3217h80b476442303f1eeE (;48;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      call $_ZN22wasi_snapshot_preview16macros10eprint_u3215eprint_u32_impl17hfdccb9b65729bfeaE
    )
    (func $get_state_ptr (;49;) (type $#type9) (result i32)
      global.get $internal_state_ptr
    )
    (func $set_state_ptr (;50;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      global.set $internal_state_ptr
    )
    (func $get_allocation_state (;51;) (type $#type9) (result i32)
      global.get $allocation_state
    )
    (func $set_allocation_state (;52;) (type $#type1) (param $#local0 i32)
      local.get $#local0
      global.set $allocation_state
    )
    (func $memset (;53;) (type $#type10) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32)
      block $#label0
        block $#label1
          local.get $#local2
          i32.const 16
          i32.ge_u
          br_if $#label1
          local.get $#local0
          local.set $#local3
          br $#label0
        end
        local.get $#local0
        i32.const 0
        local.get $#local0
        i32.sub
        i32.const 3
        i32.and
        local.tee $#local4
        i32.add
        local.set $#local5
        block $#label1
          local.get $#local4
          i32.eqz
          br_if $#label1
          local.get $#local0
          local.set $#local3
          loop $#label2
            local.get $#local3
            local.get $#local1
            i32.store8
            local.get $#local3
            i32.const 1
            i32.add
            local.tee $#local3
            local.get $#local5
            i32.lt_u
            br_if $#label2
          end
        end
        local.get $#local5
        local.get $#local2
        local.get $#local4
        i32.sub
        local.tee $#local4
        i32.const -4
        i32.and
        local.tee $#local2
        i32.add
        local.set $#local3
        block $#label1
          local.get $#local2
          i32.const 1
          i32.lt_s
          br_if $#label1
          local.get $#local1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set $#local2
          loop $#label2
            local.get $#local5
            local.get $#local2
            i32.store
            local.get $#local5
            i32.const 4
            i32.add
            local.tee $#local5
            local.get $#local3
            i32.lt_u
            br_if $#label2
          end
        end
        local.get $#local4
        i32.const 3
        i32.and
        local.set $#local2
      end
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        local.get $#local3
        local.get $#local2
        i32.add
        local.set $#local5
        loop $#label1
          local.get $#local3
          local.get $#local1
          i32.store8
          local.get $#local3
          i32.const 1
          i32.add
          local.tee $#local3
          local.get $#local5
          i32.lt_u
          br_if $#label1
        end
      end
      local.get $#local0
    )
    (func $memcpy (;54;) (type $#type10) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (result i32)
      (local $#local3 i32) (local $#local4 i32) (local $#local5 i32) (local $#local6 i32) (local $#local7 i32) (local $#local8 i32) (local $#local9 i32) (local $#local10 i32)
      block $#label0
        block $#label1
          local.get $#local2
          i32.const 16
          i32.ge_u
          br_if $#label1
          local.get $#local0
          local.set $#local3
          br $#label0
        end
        local.get $#local0
        i32.const 0
        local.get $#local0
        i32.sub
        i32.const 3
        i32.and
        local.tee $#local4
        i32.add
        local.set $#local5
        block $#label1
          local.get $#local4
          i32.eqz
          br_if $#label1
          local.get $#local0
          local.set $#local3
          local.get $#local1
          local.set $#local6
          loop $#label2
            local.get $#local3
            local.get $#local6
            i32.load8_u
            i32.store8
            local.get $#local6
            i32.const 1
            i32.add
            local.set $#local6
            local.get $#local3
            i32.const 1
            i32.add
            local.tee $#local3
            local.get $#local5
            i32.lt_u
            br_if $#label2
          end
        end
        local.get $#local5
        local.get $#local2
        local.get $#local4
        i32.sub
        local.tee $#local7
        i32.const -4
        i32.and
        local.tee $#local8
        i32.add
        local.set $#local3
        block $#label1
          block $#label2
            local.get $#local1
            local.get $#local4
            i32.add
            local.tee $#local9
            i32.const 3
            i32.and
            i32.eqz
            br_if $#label2
            local.get $#local8
            i32.const 1
            i32.lt_s
            br_if $#label1
            local.get $#local9
            i32.const 3
            i32.shl
            local.tee $#local6
            i32.const 24
            i32.and
            local.set $#local2
            local.get $#local9
            i32.const -4
            i32.and
            local.tee $#local10
            i32.const 4
            i32.add
            local.set $#local1
            i32.const 0
            local.get $#local6
            i32.sub
            i32.const 24
            i32.and
            local.set $#local4
            local.get $#local10
            i32.load
            local.set $#local6
            loop $#label3
              local.get $#local5
              local.get $#local6
              local.get $#local2
              i32.shr_u
              local.get $#local1
              i32.load
              local.tee $#local6
              local.get $#local4
              i32.shl
              i32.or
              i32.store
              local.get $#local1
              i32.const 4
              i32.add
              local.set $#local1
              local.get $#local5
              i32.const 4
              i32.add
              local.tee $#local5
              local.get $#local3
              i32.lt_u
              br_if $#label3
              br $#label1
            end
          end
          local.get $#local8
          i32.const 1
          i32.lt_s
          br_if $#label1
          local.get $#local9
          local.set $#local1
          loop $#label2
            local.get $#local5
            local.get $#local1
            i32.load
            i32.store
            local.get $#local1
            i32.const 4
            i32.add
            local.set $#local1
            local.get $#local5
            i32.const 4
            i32.add
            local.tee $#local5
            local.get $#local3
            i32.lt_u
            br_if $#label2
          end
        end
        local.get $#local7
        i32.const 3
        i32.and
        local.set $#local2
        local.get $#local9
        local.get $#local8
        i32.add
        local.set $#local1
      end
      block $#label0
        local.get $#local2
        i32.eqz
        br_if $#label0
        local.get $#local3
        local.get $#local2
        i32.add
        local.set $#local5
        loop $#label1
          local.get $#local3
          local.get $#local1
          i32.load8_u
          i32.store8
          local.get $#local1
          i32.const 1
          i32.add
          local.set $#local1
          local.get $#local3
          i32.const 1
          i32.add
          local.tee $#local3
          local.get $#local5
          i32.lt_u
          br_if $#label1
        end
      end
      local.get $#local0
    )
    (func $allocate_stack (;55;) (type $#type12)
      global.get $allocation_state
      i32.const 0
      i32.eq
      if $#label0
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
  (core module $#module2 (;2;)
    (type $#type0 (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type $#type1 (;1;) (func (param i32 i32) (result i32)))
    (type $#type2 (;2;) (func (param i32)))
    (type $#type3 (;3;) (func (param i32)))
    (type $#type4 (;4;) (func (param i32 i32)))
    (type $#type5 (;5;) (func (param i32 i64 i32)))
    (type $#type6 (;6;) (func (param i32 i32 i32 i32)))
    (table $#table0 (;0;) 15 15 funcref)
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
    (export "$imports" (table $#table0))
    (func $adapt-wasi_snapshot_preview1-fd_write (;0;) (type $#type0) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32) (result i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      local.get $#local3
      i32.const 0
      call_indirect (type $#type0)
    )
    (func $adapt-wasi_snapshot_preview1-environ_get (;1;) (type $#type1) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      i32.const 1
      call_indirect (type $#type1)
    )
    (func $adapt-wasi_snapshot_preview1-environ_sizes_get (;2;) (type $#type1) (param $#local0 i32) (param $#local1 i32) (result i32)
      local.get $#local0
      local.get $#local1
      i32.const 2
      call_indirect (type $#type1)
    )
    (func $adapt-wasi_snapshot_preview1-proc_exit (;3;) (type $#type2) (param $#local0 i32)
      local.get $#local0
      i32.const 3
      call_indirect (type $#type2)
    )
    (func $indirect-wasi:cli/environment@0.2.3-get-environment (;4;) (type $#type3) (param $#local0 i32)
      local.get $#local0
      i32.const 4
      call_indirect (type $#type3)
    )
    (func $indirect-wasi:filesystem/types@0.2.3-filesystem-error-code (;5;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 5
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.write-via-stream" (;6;) (type $#type5) (param $#local0 i32) (param $#local1 i64) (param $#local2 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      i32.const 6
      call_indirect (type $#type5)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.append-via-stream" (;7;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 7
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.get-type" (;8;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 8
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:filesystem/types@0.2.3-[method]descriptor.stat" (;9;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 9
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.check-write" (;10;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 10
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.write" (;11;) (type $#type6) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      local.get $#local3
      i32.const 11
      call_indirect (type $#type6)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-flush" (;12;) (type $#type4) (param $#local0 i32) (param $#local1 i32)
      local.get $#local0
      local.get $#local1
      i32.const 12
      call_indirect (type $#type4)
    )
    (func $"indirect-wasi:io/streams@0.2.3-[method]output-stream.blocking-write-and-flush" (;13;) (type $#type6) (param $#local0 i32) (param $#local1 i32) (param $#local2 i32) (param $#local3 i32)
      local.get $#local0
      local.get $#local1
      local.get $#local2
      local.get $#local3
      i32.const 13
      call_indirect (type $#type6)
    )
    (func $indirect-wasi:filesystem/preopens@0.2.2-get-directories (;14;) (type $#type3) (param $#local0 i32)
      local.get $#local0
      i32.const 14
      call_indirect (type $#type3)
    )
    (@producers
      (processed-by "wit-component" "0.223.0")
    )
  )
  (core module $#module3 (;3;)
    (type $#type0 (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type $#type1 (;1;) (func (param i32 i32) (result i32)))
    (type $#type2 (;2;) (func (param i32)))
    (type $#type3 (;3;) (func (param i32)))
    (type $#type4 (;4;) (func (param i32 i32)))
    (type $#type5 (;5;) (func (param i32 i64 i32)))
    (type $#type6 (;6;) (func (param i32 i32 i32 i32)))
    (import "" "0" (func $#func0 (;0;) (type $#type0)))
    (import "" "1" (func $#func1 (;1;) (type $#type1)))
    (import "" "2" (func $#func2 (;2;) (type $#type1)))
    (import "" "3" (func $#func3 (;3;) (type $#type2)))
    (import "" "4" (func $#func4 (;4;) (type $#type3)))
    (import "" "5" (func $#func5 (;5;) (type $#type4)))
    (import "" "6" (func $#func6 (;6;) (type $#type5)))
    (import "" "7" (func $#func7 (;7;) (type $#type4)))
    (import "" "8" (func $#func8 (;8;) (type $#type4)))
    (import "" "9" (func $#func9 (;9;) (type $#type4)))
    (import "" "10" (func $#func10 (;10;) (type $#type4)))
    (import "" "11" (func $#func11 (;11;) (type $#type6)))
    (import "" "12" (func $#func12 (;12;) (type $#type4)))
    (import "" "13" (func $#func13 (;13;) (type $#type6)))
    (import "" "14" (func $#func14 (;14;) (type $#type3)))
    (import "" "$imports" (table $#table0 (;0;) 15 15 funcref))
    (elem $#elem0 (;0;) (i32.const 0) func $#func0 $#func1 $#func2 $#func3 $#func4 $#func5 $#func6 $#func7 $#func8 $#func9 $#func10 $#func11 $#func12 $#func13 $#func14)
    (@producers
      (processed-by "wit-component" "0.223.0")
    )
  )
  (core instance $#instance0 (;0;) (instantiate $#module2))
  (alias core export $#instance0 "0" (core func $#func0 (;0;)))
  (alias core export $#instance0 "1" (core func $#func1 (;1;)))
  (alias core export $#instance0 "2" (core func $#func2 (;2;)))
  (alias core export $#instance0 "3" (core func $#func3 (;3;)))
  (core instance $#instance1 (;1;)
    (export "fd_write" (func $#func0))
    (export "environ_get" (func $#func1))
    (export "environ_sizes_get" (func $#func2))
    (export "proc_exit" (func $#func3))
  )
  (core instance $#instance2 (;2;) (instantiate $#module0           ;; NOTE: instantiates my main
      (with "wasi_snapshot_preview1" (instance $#instance1))
    )
  )
  (alias core export $#instance2 "memory" (core memory $#memory0 (;0;)))
  (core instance $#instance3 (;3;)
    (export "memory" (memory $#memory0))
  )
  (alias core export $#instance2 "_start" (core func $#func4 (;4;)))
  (alias core export $#instance2 "cabi_realloc" (core func $#func5 (;5;)))
  (core instance $#instance4 (;4;)
    (export "_start" (func $#func4))
    (export "cabi_realloc" (func $#func5))
  )
  (alias core export $#instance0 "4" (core func $#func6 (;6;)))
  (core instance $#instance5 (;5;)
    (export "get-environment" (func $#func6))
  )
  (alias export $#instance8 "descriptor" (type $#type18 (;18;)))
  (core func $#func7 (;7;) (canon resource.drop $#type18))
  (alias core export $#instance0 "5" (core func $#func8 (;8;)))
  (alias core export $#instance0 "6" (core func $#func9 (;9;)))
  (alias core export $#instance0 "7" (core func $#func10 (;10;)))
  (alias core export $#instance0 "8" (core func $#func11 (;11;)))
  (alias core export $#instance0 "9" (core func $#func12 (;12;)))
  (core instance $#instance6 (;6;)
    (export "[resource-drop]descriptor" (func $#func7))
    (export "filesystem-error-code" (func $#func8))
    (export "[method]descriptor.write-via-stream" (func $#func9))
    (export "[method]descriptor.append-via-stream" (func $#func10))
    (export "[method]descriptor.get-type" (func $#func11))
    (export "[method]descriptor.stat" (func $#func12))
  )
  (alias export $#instance3 "output-stream" (type $#type19 (;19;)))
  (core func $#func13 (;13;) (canon resource.drop $#type19))
  (alias export $#instance3 "input-stream" (type $#type20 (;20;)))
  (core func $#func14 (;14;) (canon resource.drop $#type20))
  (alias core export $#instance0 "10" (core func $#func15 (;15;)))
  (alias core export $#instance0 "11" (core func $#func16 (;16;)))
  (alias core export $#instance0 "12" (core func $#func17 (;17;)))
  (alias core export $#instance0 "13" (core func $#func18 (;18;)))
  (core instance $#instance7 (;7;)
    (export "[resource-drop]output-stream" (func $#func13))
    (export "[resource-drop]input-stream" (func $#func14))
    (export "[method]output-stream.check-write" (func $#func15))
    (export "[method]output-stream.write" (func $#func16))
    (export "[method]output-stream.blocking-flush" (func $#func17))
    (export "[method]output-stream.blocking-write-and-flush" (func $#func18))
  )
  (alias export $#instance2 "error" (type $#type21 (;21;)))
  (core func $#func19 (;19;) (canon resource.drop $#type21))
  (core instance $#instance8 (;8;)
    (export "[resource-drop]error" (func $#func19))
  )
  (alias core export $#instance0 "14" (core func $#func20 (;20;)))
  (core instance $#instance9 (;9;)
    (export "get-directories" (func $#func20))
  )
  (alias export $#instance6 "get-stderr" (func $#func0 (;0;)))
  (core func $#func21 (;21;) (canon lower (func $#func0)))
  (core instance $#instance10 (;10;)
    (export "get-stderr" (func $#func21))
  )
  (alias export $#instance4 "get-stdin" (func $#func1 (;1;)))
  (core func $#func22 (;22;) (canon lower (func $#func1)))
  (core instance $#instance11 (;11;)
    (export "get-stdin" (func $#func22))
  )
  (alias export $#instance5 "get-stdout" (func $#func2 (;2;)))
  (core func $#func23 (;23;) (canon lower (func $#func2)))
  (core instance $#instance12 (;12;)
    (export "get-stdout" (func $#func23))
  )
  (alias export $#instance1 "exit" (func $#func3 (;3;)))
  (core func $#func24 (;24;) (canon lower (func $#func3)))
  (core instance $#instance13 (;13;)
    (export "exit" (func $#func24))
  )
  (core instance $#instance14 (;14;) (instantiate $#module1
      (with "env" (instance $#instance3))
      (with "__main_module__" (instance $#instance4))
      (with "wasi:cli/environment@0.2.3" (instance $#instance5))
      (with "wasi:filesystem/types@0.2.3" (instance $#instance6))
      (with "wasi:io/streams@0.2.3" (instance $#instance7))
      (with "wasi:io/error@0.2.3" (instance $#instance8))
      (with "wasi:filesystem/preopens@0.2.2" (instance $#instance9))
      (with "wasi:cli/stderr@0.2.3" (instance $#instance10))
      (with "wasi:cli/stdin@0.2.3" (instance $#instance11))
      (with "wasi:cli/stdout@0.2.3" (instance $#instance12))
      (with "wasi:cli/exit@0.2.3" (instance $#instance13))
    )
  )
  (alias core export $#instance0 "$imports" (core table $#table0 (;0;)))
  (alias core export $#instance14 "fd_write" (core func $#func25 (;25;)))
  (alias core export $#instance14 "environ_get" (core func $#func26 (;26;)))
  (alias core export $#instance14 "environ_sizes_get" (core func $#func27 (;27;)))
  (alias core export $#instance14 "proc_exit" (core func $#func28 (;28;)))
  (alias export $#instance0 "get-environment" (func $#func4 (;4;)))
  (alias core export $#instance14 "cabi_import_realloc" (core func $#func29 (;29;)))
  (core func $#func30 (;30;) (canon lower (func $#func4) (memory $#memory0) (realloc $#func29) string-encoding=utf8))
  (alias export $#instance8 "filesystem-error-code" (func $#func5 (;5;)))
  (core func $#func31 (;31;) (canon lower (func $#func5) (memory $#memory0)))
  (alias export $#instance8 "[method]descriptor.write-via-stream" (func $#func6 (;6;)))
  (core func $#func32 (;32;) (canon lower (func $#func6) (memory $#memory0)))
  (alias export $#instance8 "[method]descriptor.append-via-stream" (func $#func7 (;7;)))
  (core func $#func33 (;33;) (canon lower (func $#func7) (memory $#memory0)))
  (alias export $#instance8 "[method]descriptor.get-type" (func $#func8 (;8;)))
  (core func $#func34 (;34;) (canon lower (func $#func8) (memory $#memory0)))
  (alias export $#instance8 "[method]descriptor.stat" (func $#func9 (;9;)))
  (core func $#func35 (;35;) (canon lower (func $#func9) (memory $#memory0)))
  (alias export $#instance3 "[method]output-stream.check-write" (func $#func10 (;10;)))
  (core func $#func36 (;36;) (canon lower (func $#func10) (memory $#memory0)))
  (alias export $#instance3 "[method]output-stream.write" (func $#func11 (;11;)))
  (core func $#func37 (;37;) (canon lower (func $#func11) (memory $#memory0)))
  (alias export $#instance3 "[method]output-stream.blocking-flush" (func $#func12 (;12;)))
  (core func $#func38 (;38;) (canon lower (func $#func12) (memory $#memory0)))
  (alias export $#instance3 "[method]output-stream.blocking-write-and-flush" (func $#func13 (;13;)))
  (core func $#func39 (;39;) (canon lower (func $#func13) (memory $#memory0)))
  (alias export $#instance9 "get-directories" (func $#func14 (;14;)))
  (core func $#func40 (;40;) (canon lower (func $#func14) (memory $#memory0) (realloc $#func29) string-encoding=utf8))
  (core instance $#instance15 (;15;)
    (export "$imports" (table $#table0))
    (export "0" (func $#func25))
    (export "1" (func $#func26))
    (export "2" (func $#func27))
    (export "3" (func $#func28))
    (export "4" (func $#func30))
    (export "5" (func $#func31))
    (export "6" (func $#func32))
    (export "7" (func $#func33))
    (export "8" (func $#func34))
    (export "9" (func $#func35))
    (export "10" (func $#func36))
    (export "11" (func $#func37))
    (export "12" (func $#func38))
    (export "13" (func $#func39))
    (export "14" (func $#func40))
  )
  (core instance $#instance16 (;16;) (instantiate $#module3
      (with "" (instance $#instance15))
    )
  )
  (type $#type22 (;22;) (result))
  (type $#type23 (;23;) (func (result $#type22)))
  (alias core export $#instance14 "wasi:cli/run@0.2.3#run" (core func $#func41 (;41;)))
  (func $#func15 (;15;) (type $#type23) (canon lift (core func $#func41)))
  (component $#component0 (;0;)
    (type $#type0 (;0;) (result))
    (type $#type1 (;1;) (func (result $#type0)))
    (import "import-func-run" (func $#func0 (;0;) (type $#type1)))
    (type $#type2 (;2;) (result))
    (type $#type3 (;3;) (func (result $#type2)))
    (export $#func1 (;1;) "run" (func $#func0) (func (type $#type3)))
  )
  (instance $#instance10 (;10;) (instantiate $#component0
      (with "import-func-run" (func $#func15))
    )
  )
  (export $#instance11 (;11;) "wasi:cli/run@0.2.3" (instance $#instance10))
  (@producers
    (processed-by "wit-component" "0.223.0")
  )
)
