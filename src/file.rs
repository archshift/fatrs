use foreign;
use filesystem;

use core::mem;

use rcstring::CString;

pub struct File<'a, 'b: 'a> {
    pub(crate) fs: &'a mut filesystem::Filesystem<'b>, 
	pub(crate) fil: foreign::FIL,
}

impl<'a, 'b> File<'a, 'b> {
	fn ffi_fp(&mut self) -> *mut foreign::FIL {
		&mut self.fil
	}

	pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, foreign::FRESULT> {
		let mut bytes_read: foreign::UINT = 0;
		match unsafe { foreign::f_read(self.ffi_fp(), buf.as_mut_ptr() as foreign::voidp_mut, buf.len() as foreign::UINT, &mut bytes_read) } {
			foreign::FRESULT_FR_OK => Ok(bytes_read as usize),
			e @ _ => Err(e)
		}
	}

	pub fn write(&mut self, buf: &[u8]) -> Result<usize, foreign::FRESULT> {
		let mut bytes_written: foreign::UINT = 0;
		match unsafe { foreign::f_write(self.ffi_fp(), buf.as_ptr() as foreign::voidp, buf.len() as foreign::UINT, &mut bytes_written) } {
			foreign::FRESULT_FR_OK => Ok(bytes_written as usize),
			e @ _ => Err(e)
		}
	}

	pub fn lseek(&mut self, offset: u32) -> Result<(), foreign::FRESULT> {
		match unsafe { foreign::f_lseek(self.ffi_fp(), offset as foreign::DWORD) } {
			foreign::FRESULT_FR_OK => Ok(()),
			e @ _ => Err(e)
		}
	}

	pub fn tell(&self) -> Result<u32, foreign::FRESULT> {
		Ok(self.fil.fptr as u32)
	}

	pub fn truncate(&mut self) -> Result<(), foreign::FRESULT> {
		match unsafe { foreign::f_truncate(self.ffi_fp()) } {
			foreign::FRESULT_FR_OK => Ok(()),
			e @ _ => Err(e)
		}
	}

	pub fn sync(&mut self) -> Result<(), foreign::FRESULT> {
		match unsafe { foreign::f_sync(self.ffi_fp()) } {
			foreign::FRESULT_FR_OK => Ok(()),
			e @ _ => Err(e)
		}
	}

	pub fn size(&self) -> Result<u32, foreign::FRESULT> {
		Ok(self.fil.fsize as u32)
	}
}

impl<'a, 'b> Drop for File<'a, 'b>{
	fn drop(&mut self) {
		unsafe { foreign::f_close(self.ffi_fp()); }
	}
}
