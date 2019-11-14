extern crate failure;

mod builtin;
mod zsh;

use builtin::Builtin;
use std::cell::RefCell;
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};

thread_local! {
    static BUILTINS: RefCell<Vec<zsh::builtin>> = RefCell::new(vec![
                Builtin::new("rustexample", 0, rustexample, 0, 0, None, "flags", None)
                    .unwrap()
                    .into_zsh_builtin(),
            ]);
    static MODULE_FEATURES: RefCell<zsh::features> = RefCell::new({
        let mut conddefs = Vec::new();
        let mut mathfuncs = Vec::new();
        let mut paramdefs = Vec::new();
        BUILTINS.with(|builtins| {
            let mut builtins = builtins.borrow_mut();
            zsh::features {
                bn_list: builtins.as_mut_ptr(),
                bn_size: builtins.len() as c_int,
                cd_list: conddefs.as_mut_ptr(),
                cd_size: conddefs.len() as c_int,
                mf_list: mathfuncs.as_mut_ptr(),
                mf_size: mathfuncs.len() as c_int,
                pd_list: paramdefs.as_mut_ptr(),
                pd_size: paramdefs.len() as c_int,
                n_abstract: 0,
            }
        })
    });
}

pub extern "C" fn rustexample(
    nam: *const c_char,
    args: *const *const c_char,
    opts: *mut zsh::options,
    _func: c_int,
) -> c_int {
    println!("hello from rust!");
    0
}

#[no_mangle]
pub extern "C" fn setup_(_m: *mut zsh::module) -> c_int {
    println!("zrepomod loaded");
    0
}

#[no_mangle]
pub extern "C" fn features_(m: *mut zsh::module, features: *mut *mut *mut c_char) -> c_int {
    MODULE_FEATURES.with(|module_features| {
        unsafe { *features = zsh::featuresarray(m, module_features.as_ptr()) };
    });
    // let _ = unsafe { Builtin::from_zsh_builtin(example) };
    0
}

#[no_mangle]
pub extern "C" fn enables_(m: *mut zsh::module, enables: *mut *mut c_int) -> c_int {
    MODULE_FEATURES.with(|module_features| unsafe {
        zsh::handlefeatures(m, module_features.as_ptr(), enables)
    })
}

#[no_mangle]
pub extern "C" fn boot_(m: *mut zsh::module) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_(m: *mut zsh::module) -> c_int {
    MODULE_FEATURES.with(|module_features| unsafe {
        zsh::setfeatureenables(m, module_features.as_ptr(), ptr::null_mut())
    })
}

#[no_mangle]
pub extern "C" fn finish_(_m: *mut zsh::module) -> c_int {
    // MODULE_FEATURES.with(|module_features| mem::drop(module_features.into_inner()));
    BUILTINS.with(|builtins| {
        builtins.borrow_mut().drain(0..).for_each(|builtin| unsafe {
            Builtin::from_zsh_builtin(builtin);
        });
    });
    println!("zrepomod unloaded");
    0
}
