use libc;

use core::slice;

use rcstring::CString;

pub type BYTE = libc::c_uchar;
pub type UINT = libc::c_uint;
pub type WORD = libc::c_ushort;
pub type DWORD = libc::c_ulong;

#[cfg(feature = "unicode")]
pub type TCHAR = libc::c_ushort;

#[cfg(not(feature = "unicode"))]
pub type TCHAR = libc::c_char;

pub enum FILINFO {}
pub enum DIR {}

#[repr(C)]
pub enum FRESULT {
    FR_OK = 0,
    FR_DISK_ERR,
    FR_INT_ERR,
    FR_NOT_READY,
    FR_NO_FILE,
    FR_NO_PATH,
    FR_INVALID_NAME,
    FR_DENIED,
    FR_EXIST,
    FR_INVALID_OBJECT,
    FR_WRITE_PROTECTED,
    FR_INVALID_DRIVE,
    FR_NOT_ENABLED,
    FR_NO_FILESYSTEM,
    FR_MKFS_ABORTED,
    FR_TIMEOUT,
    FR_LOCKED,
    FR_NOT_ENOUGH_CORE,
    FR_TOO_MANY_OPEN_FILES,
    FR_INVALID_PARAMETER
}

#[repr(C)]
pub struct FATFS {
    pub fs_type: BYTE,
    pub drv: BYTE,
    pub csize: BYTE,
    pub n_fats: BYTE,
    pub wflag: BYTE,
    pub fsi_flag: BYTE,
    pub id: WORD,
    pub n_rootdir: WORD,
// #if _MAX_SS != _MIN_SS
//     pub ssize: WORD,
// #endif
// #if _FS_REENTRANT
//     _SYNC_t sobj;
// #endif
// #if !_FS_READONLY
    pub last_clust: DWORD,
    pub free_clust: DWORD,
// #endif
// #if _FS_RPATH
//     pub cdir: DWORD,
// #endif
    pub n_fatent: DWORD,
    pub fsize: DWORD,
    pub volbase: DWORD,
    pub fatbase: DWORD,
    pub dirbase: DWORD,
    pub database: DWORD,
    pub winsect: DWORD,
    pub win: [BYTE; 512], // _MAX_SS],
}

#[repr(C)]
pub struct FIL {
    pub fs: *mut FATFS,
    pub id: WORD,
    pub flag: BYTE,
    pub err: BYTE,
    pub fptr: DWORD,
    pub fsize: DWORD,
    pub sclust: DWORD,
    pub clust: DWORD,
    pub dsect: DWORD,
// #if !_FS_READONLY
    pub dir_sect: DWORD,
    pub dir_ptr: *mut BYTE,
// #endif
// #if _USE_FASTSEEK
    // cltbl: *mut DWORD,
// #endif
// #if _FS_LOCK
    // lockid: UINT,
// #endif
// #if !_FS_TINY
    pub buf: [BYTE; 512], // _MAX_SS],
// #endif
}

extern {
    pub fn f_open(fp: *mut FIL, path: *const TCHAR, mode: BYTE) -> FRESULT;
    pub fn f_close(fp: *mut FIL) -> FRESULT;
    pub fn f_read(fp: *mut FIL, buff: *mut libc::c_void, btr: UINT, br: *mut UINT) -> FRESULT;
    pub fn f_write(fp: *mut FIL, buff: *const libc::c_void, btw: UINT, bw: *mut UINT) -> FRESULT;
    pub fn f_forward(fp: *mut FIL, func: extern fn(*const BYTE, UINT) -> UINT, btf: UINT, bf: UINT) -> FRESULT;
    pub fn f_lseek(fp: *mut FIL, ofs: DWORD) -> FRESULT;
    pub fn f_truncate(fp: *mut FIL) -> FRESULT;
    pub fn f_sync(fp: *mut FIL) -> FRESULT;
    pub fn f_opendir(dp: *mut DIR, path: *const TCHAR) -> FRESULT;
    pub fn f_closedir(dp: *mut DIR) -> FRESULT;
    pub fn f_readdir(dp: *mut DIR, fno: *mut FILINFO) -> FRESULT;
    // pub fn f_findfirst(dp: *mut DIR, fno: *mut FILINFO, path: *const TCHAR, pattern: *const TCHAR) -> FRESULT;
    // pub fn f_findnext(dp: *mut DIR, fno: *mut FILINFO) -> FRESULT;
    pub fn f_mkdir(path: *const TCHAR) -> FRESULT;
    pub fn f_unlink(path: *const TCHAR) -> FRESULT;
    pub fn f_rename(path_old: *const TCHAR, path_new: *const TCHAR) -> FRESULT;
    pub fn f_stat(path: *const TCHAR, fno: *mut FILINFO) -> FRESULT;
    pub fn f_chmod(path: *const TCHAR, attr: BYTE, mask: BYTE) -> FRESULT;
    pub fn f_utime(path: *const TCHAR, fno: *const FILINFO) -> FRESULT;
    pub fn f_chdir(path: *const TCHAR) -> FRESULT;
    pub fn f_chdrive(path: *const TCHAR) -> FRESULT;
    pub fn f_getcwd(buff: *mut TCHAR, len: UINT) -> FRESULT;
    pub fn f_getfree(path: *const TCHAR, nclst: *mut DWORD, fatfs: *mut *mut FATFS) -> FRESULT;
    pub fn f_getlabel(path: *const TCHAR, label: *mut TCHAR, vsn: *mut DWORD) -> FRESULT;
    pub fn f_setlabel(label: *const TCHAR) -> FRESULT;
    pub fn f_mount(fs: *mut FATFS, path: *const TCHAR, opt: BYTE) -> FRESULT;
    pub fn f_mkfs(path: *const TCHAR, sfd: BYTE, au: UINT) -> FRESULT;
    // pub fn f_fdisk(pdrv: BYTE, szt: *const DWORD, work: *mut libc::c_void) -> FRESULT;
}

pub enum DRESULT {
    RES_OK = 0,
    RES_ERROR,
    RES_WRPRT,
    RES_NOTRDY,
    RES_PARERR,
}

pub type DSTATUS = BYTE;
pub const STA_NOINIT: DSTATUS = 0x01;
pub const STA_NODISK: DSTATUS = 0x02;
pub const STA_PROTECT: DSTATUS = 0x04;

pub static mut disk_initialize_func: Option<fn(BYTE) -> DSTATUS> = None;
pub static mut disk_ioctl_func: Option<fn(BYTE, BYTE, *mut libc::c_void) -> DRESULT> = None;
pub static mut disk_read_func: Option<fn(BYTE, *mut BYTE, DWORD, UINT) -> DRESULT> = None;
pub static mut disk_write_func: Option<fn(BYTE, *const BYTE, DWORD, UINT) -> DRESULT> = None;
pub static mut disk_status_func: Option<fn(BYTE) -> DSTATUS> = None;
pub static mut get_fattime_func: Option<fn() -> DWORD> = None;

#[no_mangle]
pub extern fn disk_initialize(pdrv: BYTE) -> DSTATUS {
    if let Some(f) = unsafe { disk_initialize_func } {
        return f(pdrv);
    }
    STA_NOINIT
}

#[no_mangle]
pub extern fn disk_ioctl(pdrv: BYTE, cmd: BYTE, buff: *mut libc::c_void) -> DRESULT {
    if let Some(f) = unsafe { disk_ioctl_func } {
        return f(pdrv, cmd, buff);
    }
    DRESULT::RES_NOTRDY
}

#[no_mangle]
pub extern fn disk_read(pdrv: BYTE, buff: *mut BYTE, sector: DWORD, count: UINT) -> DRESULT {
    if let Some(f) = unsafe { disk_read_func } {
        return f(pdrv, buff, sector, count);
    }
    DRESULT::RES_NOTRDY
}

#[no_mangle]
pub extern fn disk_write(pdrv: BYTE, buff: *const BYTE, sector: DWORD, count: UINT) -> DRESULT {
    if let Some(f) = unsafe { disk_write_func } {
        return f(pdrv, buff, sector, count);
    }
    DRESULT::RES_NOTRDY
}

#[no_mangle]
pub extern fn disk_status(pdrv: BYTE) -> DSTATUS {
    if let Some(f) = unsafe { disk_status_func } {
        return f(pdrv);
    }
    STA_NOINIT
}

#[no_mangle]
pub extern fn get_fattime() -> DWORD {
    if let Some(f) = unsafe { get_fattime_func } {
        return f();
    }
    0
}

#[cfg(not(feature = "unicode"))]
pub unsafe fn make_tchar_string(string: &CString) -> Option<*const TCHAR> {
    if slice::from_raw_parts(string.into_raw(), string.len()).iter().find(|&&x| x > 127).is_some() {
        // Non-ascii: found a byte over 127
        return None;
    }

    Some(string.into_raw())
}
