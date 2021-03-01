// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

/// Panic Handler for no_std.
use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::wasm32::unreachable();
    }
}

#[link(wasm_import_module = "js")]
extern "C" {
    pub fn js_log_trace(code: i32);
}

// Export location & size of IPC message buffers in VM shared memory
#[no_mangle]
pub unsafe extern "C" fn wasm_query_buf_ptr() -> *const u8 {
    super::ipc_mem::IN.as_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn wasm_reply_buf_ptr() -> *const u8 {
    super::ipc_mem::OUT.as_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn wasm_buffer_size() -> usize {
    super::ipc_mem::BUF_SIZE
}
