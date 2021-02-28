// Copyright (c) 2021 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
#![no_std]
extern crate hanzi_ime;

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

// Shared memory buffers for interprocess communication between
// WebAssembly VM host (javscript) and WASM module (ime_engine)
// These ARE NOT thread safe! Be careful!
pub const BUF_SIZE: usize = 150;
pub static mut IN: [u8; BUF_SIZE] = [0; BUF_SIZE];
pub static mut OUT: [u8; BUF_SIZE] = [0; BUF_SIZE];
pub static mut OUT_POS: usize = 0;

/// Apply IME translations to input buffer and copy result into output buffer
#[no_mangle]
pub extern "C" fn translate_zh_hans() {
    // TODO: copy translated input string into the output buffer
    let _ = hanzi_ime::translate_zh_hans(&"");
}

/// Export buffer size
#[no_mangle]
pub extern "C" fn buf_size() -> i32 {
    BUF_SIZE as i32
}

/// Export pointer to input string buffer shared memory for javascript + wasm32
#[no_mangle]
pub extern "C" fn input_str_buf_ptr() -> *const u8 {
    unsafe { IN.as_ptr() }
}

/// Export pointer to output string buffer
#[no_mangle]
pub extern "C" fn output_str_buf_ptr() -> *const u8 {
    unsafe { OUT.as_ptr() }
}

/// Export output string buffer position
#[no_mangle]
pub extern "C" fn output_str_buf_pos() -> i32 {
    unsafe { OUT_POS as i32 }
}
