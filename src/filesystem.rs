use foreign;
use file;

use core::mem;
use core::ptr;

use rcstring::CString;

pub struct Filesystem<'a> {
	ctx: &'a mut ::FsCtx,
	path: foreign::TcharContainer,
}

static mut WORK_BUFFER: [u8; 2048] = [0u8; 2048];

impl<'a> Filesystem<'a> {
	pub fn mount(ctx: &'a mut ::FsCtx, path: &str, opt: u8)
		-> Result<Filesystem<'a>, foreign::FRESULT> {

		let path = foreign::to_tchar_string(path);
		let res = unsafe { foreign::f_mount(&mut ctx.fatfs, path.ptr(), opt) };
		match res {
			foreign::FRESULT_FR_OK => {
				Ok(Filesystem {
					ctx: ctx,
					path: path,
				})
			},
			_ => Err(res)
		}
	}

	pub fn mkfs(&self, opt: u8) -> foreign::FRESULT {
        unsafe {
            foreign::f_mkfs(self.path.ptr(), opt, 0)
        }
    }

	pub fn open<'b>(&'b mut self, path: &str, mode: u8)
        -> Result<file::File<'b, 'a>, foreign::FRESULT>
        where 'a: 'b {

        let mut fil: foreign::FIL = unsafe { mem::zeroed() };
		let path = foreign::to_tchar_string(path);
		let res = unsafe { foreign::f_open(&mut fil, path.ptr(), mode) };
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
		unsafe { foreign::f_mount(ptr::null_mut(), self.path.ptr(), 0); }
	}
}
