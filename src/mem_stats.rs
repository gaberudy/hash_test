extern crate libc;

use std::ffi::CStr;

use std::ptr;

extern "C" {
    fn je_stats_print(
        write_cb: extern "C" fn(*const libc::c_void, *const libc::c_char),
        cbopaque: *const libc::c_void,
        opts: *const libc::c_char,
    );
}
extern "C" fn write_cb(_: *const libc::c_void, message: *const libc::c_char) {
    print!(
        "{}",
        String::from_utf8_lossy(unsafe { CStr::from_ptr(message as *const i8).to_bytes() })
    );
}

pub fn stats_print() {
    unsafe { je_stats_print(write_cb, ptr::null(), ptr::null()) };
}

extern "C" {
    fn mem_start();
    fn mem_stop();
}

pub fn native_mem_stats_start() {
    unsafe {
        mem_start();
    }
}

pub fn native_mem_stats_stop() {
    unsafe {
        mem_stop();
    }
}
