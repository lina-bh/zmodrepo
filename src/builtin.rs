use super::zsh;
use failure::Error;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::{mem, ptr};

pub type BuiltinCb =
    extern "C" fn(*const c_char, *const *const c_char, *mut zsh::options, c_int) -> c_int;

#[derive(PartialEq, Debug, Clone)]
pub struct Builtin {
    nam: CString,
    flags: c_int,
    handlerfunc: BuiltinCb,
    minargs: c_int,
    maxargs: c_int,
    funcid: c_int,
    optstr: CString,
    defopts: Option<CString>,
}

impl Builtin {
    pub fn new(
        nam: &str,
        flags: c_int,
        handlerfunc: BuiltinCb,
        minargs: c_int,
        maxargs: c_int,
        funcid: Option<c_int>,
        optstr: &str,
        defopts: Option<&str>,
    ) -> Result<Builtin, Error> {
        let nam = CString::new(nam)?;
        let optstr = CString::new(optstr)?;
        let defopts = match defopts {
            Some(s) => Some(CString::new(s)?),
            None => None,
        };
        let funcid = funcid.unwrap_or(0);
        Ok(Builtin {
            nam,
            flags,
            handlerfunc,
            minargs,
            maxargs,
            funcid,
            optstr,
            defopts,
        })
    }

    pub fn into_zsh_builtin(self) -> zsh::builtin {
        let handlerfunc = Some(unsafe { mem::transmute(self.handlerfunc) });
        zsh::builtin {
            node: zsh::hashnode {
                next: ptr::null_mut(),
                nam: self.nam.into_raw(),
                flags: self.flags,
            },
            handlerfunc,
            minargs: self.minargs,
            maxargs: self.maxargs,
            funcid: self.funcid,
            optstr: self.optstr.into_raw(),
            defopts: self.defopts.map_or(ptr::null_mut(), CString::into_raw),
        }
    }

    pub unsafe fn from_zsh_builtin(builtin: zsh::builtin) -> Self {
        let handlerfunc = mem::transmute(builtin.handlerfunc.unwrap());
        Self {
            nam: CString::from_raw(builtin.node.nam),
            flags: builtin.node.flags,
            handlerfunc,
            minargs: builtin.minargs,
            maxargs: builtin.maxargs,
            funcid: builtin.funcid,
            optstr: CString::from_raw(builtin.optstr),
            defopts: if builtin.defopts.is_null() {
                None
            } else {
                Some(CString::from_raw(builtin.defopts))
            },
        }
    }
}
