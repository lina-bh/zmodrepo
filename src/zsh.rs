#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn featuresarray(m: *mut module, f: *mut features) -> *mut *mut c_char;
    pub fn handlefeatures(m: *mut module, f: *mut features, enables: *mut *mut c_int) -> c_int;
    pub fn setfeatureenables(m: *mut module, f: *mut features, e: *mut c_int) -> c_int;
}
