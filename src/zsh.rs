#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::{c_char, c_int};
use std::ptr;

mod c {
    extern "C" {
        pub fn featuresarray(m: *mut module, f: *mut features) -> *mut *mut c_char;
        pub fn handlefeatures(m: *mut module, f: *mut features, enables: *mut *mut c_int) -> c_int;
        pub fn setfeatureenables(m: *mut module, f: *mut features, e: *mut c_int) -> c_int;
    }
}

pub unsafe fn featuresarray(f: *const features) -> *const *const c_char {
    let mut features = *f.clone();
    c::featuresarray(ptr::null_mut(), &features)
}
