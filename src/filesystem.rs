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
			foreign::f_mount(&mut fatfs, foreign::make_tchar_string(&path).unwrap(), opt)
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
			foreign::f_mount(ptr::null_mut(), foreign::make_tchar_string(&self.path).unwrap(), 0)
		};
	}
}
