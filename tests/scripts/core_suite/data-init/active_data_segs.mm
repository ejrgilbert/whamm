use whamm_core;

var data_len: u32 = active_data_len(APP_MEMID);
var ptr: i32 = whamm_core.mem_alloc(data_len as i32);
memcpy(APP_MEMID, active_data_start(APP_MEMID), memid(whamm_core), ptr as u32, data_len);
whamm_core.puts(ptr, data_len as i32);

wasm:opcode:memory.init:before {
    whamm_core.puts(ptr, data_len as i32);
}
