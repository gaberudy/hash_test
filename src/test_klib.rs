extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    pub fn test_int(
        N: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn test_str(
        N: ::std::os::raw::c_int,
        data: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn insert_or_remove_int(data: &Vec<u32>, je_stats: bool) {
    unsafe {
        test_int(data.len() as i32, data.as_ptr());
    }
}

pub fn insert_or_remove_str(data: &Vec<CString>, je_stats: bool) {
    let mut v = Vec::<*const c_char>::with_capacity(data.len());
    for d in data {
        v.push(d.as_ptr())
    }
    unsafe {
        test_str(v.len() as i32, v.as_ptr());
    }
}
