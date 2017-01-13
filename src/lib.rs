extern crate libc;


use std::ffi::CString;
use std::ffi::CStr;


#[derive(Debug)]
pub struct Shadow {
    pub name: String,
    pub password: String,
    pub last_change: i64,
    pub min: i64,
    pub max: i64,
    pub warn: i64,
    pub inactive: i64,
    pub expire: i64,
}

impl Shadow {
    unsafe fn from_ptr(spwd: *const libc::spwd) -> Shadow {
        Shadow {
            name: CStr::from_ptr((*spwd).sp_namp).to_str().unwrap().to_owned(),
            password: CStr::from_ptr((*spwd).sp_pwdp).to_str().unwrap().to_owned(),
            last_change: (*spwd).sp_lstchg,
            min: (*spwd).sp_min,
            max: (*spwd).sp_max,
            warn: (*spwd).sp_warn,
            inactive: (*spwd).sp_inact,
            expire: (*spwd).sp_expire,
        }
    }

    pub fn from_name(user: &str) -> Option<Shadow> {
        let c_user = CString::new(user).unwrap();

        let spwd = unsafe { libc::getspnam(c_user.as_ptr()) };

        if spwd.is_null() {
            None
        } else {
            Some(unsafe { Shadow::from_ptr(spwd) })
        }
    }

    pub fn iter_all() -> ShadowIter {
        ShadowIter::default()
    }
}

#[derive(Default)]
pub struct ShadowIter {
    started: bool,
    done: bool,
}

impl Iterator for ShadowIter {
    type Item = Shadow;

    fn next(&mut self) -> Option<Shadow> {
        self.started = true;
        if !self.done {
            let spwd = unsafe { libc::getspent() };
            if spwd.is_null() {
                unsafe { libc::endspent() };
                self.done = true;
                None
            } else {
                Some(unsafe { Shadow::from_ptr(spwd) })
            }
        } else {
            None
        }
    }
}

impl Drop for ShadowIter {
    fn drop(&mut self) {
        if self.started && !self.done {
            unsafe { libc::endspent() };
        }
    }
}
