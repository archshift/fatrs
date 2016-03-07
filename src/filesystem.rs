use file::File;
use foreign;

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
				None => return Err(foreign::FRESULT::FR_INVALID_NAME)
			};
			foreign::f_mount(&mut fatfs, string, opt)
		};
		match res {
			foreign::FRESULT::FR_OK => {
				Ok(Filesystem {
					fatfs: fatfs,
					path: path,
				})
			},
			_ => Err(res)
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
