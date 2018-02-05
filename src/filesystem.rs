use foreign;
use file;

use core::mem;
use core::ptr;

use rcstring::CString;

pub struct Filesystem<'a> {
	fatfs: foreign::FATFS,
	path: CString<'a>,
}

impl<'a> Filesystem<'a> {
	pub fn mount(path: CString<'a>, opt: u8) -> Result<Filesystem, foreign::FRESULT> {
		let mut fatfs: foreign::FATFS = unsafe { mem::zeroed() };
		let res = unsafe {
			let string = match foreign::make_tchar_string(&path) {
				Some(s) => s,
				None => return Err(foreign::FRESULT_FR_INVALID_NAME)
			};
			foreign::f_mount(&mut fatfs, string, opt)
		};
		match res {
			foreign::FRESULT_FR_OK => {
				Ok(Filesystem {
					fatfs: fatfs,
					path: path,
				})
			},
			_ => Err(res)
		}
	}

	pub fn open<'b>(&'b mut self, path: &CString, mode: u8)
        -> Result<file::File<'b, 'a>, foreign::FRESULT>
        where 'a: 'b {

        let mut fil: foreign::FIL = unsafe { mem::zeroed() };
		let res = unsafe {
			let string = match foreign::make_tchar_string(path) {
				Some(s) => s,
				None => return Err(foreign::FRESULT_FR_INVALID_NAME)
			};
			foreign::f_open(&mut fil, string, mode)
		};
		match res {
			foreign::FRESULT_FR_OK => {
				Ok(file::File {
                    fs: self,
					fil: fil
				})
			},
			e @ _ => Err(e),
		}
	}
}

impl<'a> Drop for Filesystem<'a> {
	fn drop(&mut self) {
		unsafe {
			let string = match foreign::make_tchar_string(&self.path) {
				Some(s) => s,
				None => return
			};
			foreign::f_mount(ptr::null_mut(), string, 0)
		};
	}
}
