#![no_std]

extern crate rcstring;

pub(crate) mod foreign {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    pub type voidp_mut = *mut ::ctypes::c_void;
    pub type voidp = *const ::ctypes::c_void;


    #[cfg(not(feature = "unicode"))]
    pub(crate) unsafe fn make_tchar_string(string: &::rcstring::CString) -> Option<*const TCHAR> {
        let bytes = ::core::slice::from_raw_parts(string.into_raw() as *const u8, string.len());
        if bytes.iter().find(|&&x| x > 127).is_some() {
            // Non-ascii: found a byte over 127
            return None;
        }

        Some(string.into_raw() as *const TCHAR)
    }

    include!(concat!(env!("OUT_DIR"), "/fatfs_bindings.rs"));
}

pub(crate) mod ctypes;

mod file;
mod filesystem;

pub use file::*;
pub use filesystem::*;

