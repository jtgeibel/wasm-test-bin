#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]
use core::intrinsics;

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

// Entry point for this program.
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_msg: core::fmt::Arguments,
                                _file: &'static str,
                                _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
