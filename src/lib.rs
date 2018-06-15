#![no_std]

#[macro_use] extern crate lazy_static;
extern crate spin;

pub mod foreign {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    pub type voidp_mut = *mut ::ctypes::c_void;
    pub type voidp = *const ::ctypes::c_void;


    #[cfg(not(feature = "unicode"))]
    pub(crate) fn make_tchar_string<'a>(string: &'a str) -> impl Iterator<Item = TCHAR> + 'a {
        string.chars().filter(|c| c.is_ascii()).map(|c| c as u32 as TCHAR)
    }

    pub(crate) struct TcharContainer {
        pub buf: [TCHAR; 4096]
    }

    impl TcharContainer {
        pub(crate) fn ptr(&self) -> *const TCHAR {
            (&self.buf).as_ptr()
        }
    }

    pub(crate) fn to_tchar_string<'a>(string: &str) -> TcharContainer {
        let mut ret = TcharContainer {
            buf: [0; 4096]
        };
        assert!(string.len() < ret.buf.len());
        for (i, c) in make_tchar_string(string).enumerate() {
            ret.buf[i] = c;
        }
        ret
    }

    include!(concat!(env!("OUT_DIR"), "/fatfs_bindings.rs"));
}

pub(crate) mod ctypes;

mod device;
mod file;
mod filesystem;

pub use device::*;
pub use file::*;
pub use filesystem::*;

pub const SECTOR_SIZE: usize = foreign::_MIN_SS as usize;