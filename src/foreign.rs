use core::slice;

use rcstring::CString;

pub type voidp = *const u8;
pub type voidp_mut = *mut u8;

#[cfg(feature = "int16")]
pub type INT = i16;
#[cfg(feature = "int16")]
pub type UINT = u16;

#[cfg(not(feature = "int16"))]
pub type INT = i32;
#[cfg(not(feature = "int16"))]
pub type UINT = u32;

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;

pub type SHORT = i16;
pub type LONG = i32;

#[cfg(feature = "unicode")]
pub type TCHAR = WORD;
#[cfg(not(feature = "unicode"))]
pub type TCHAR = i8;

pub enum FILINFO {}
pub enum DIR {}

#[repr(C)]
#[derive(Debug)]
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
    pub fn f_read(fp: *mut FIL, buff: voidp_mut, btr: UINT, br: *mut UINT) -> FRESULT;
    pub fn f_write(fp: *mut FIL, buff: voidp, btw: UINT, bw: *mut UINT) -> FRESULT;
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

#[cfg(not(feature = "unicode"))]
pub unsafe fn make_tchar_string(string: &CString) -> Option<*const TCHAR> {
    let bytes = slice::from_raw_parts(string.into_raw() as *const u8, string.len());
    if bytes.iter().find(|&&x| x > 127).is_some() {
        // Non-ascii: found a byte over 127
        return None;
    }

    Some(string.into_raw() as *const TCHAR)
}


#[cfg(test)]
mod test {
    use core::mem::size_of;

    #[test]
    fn test_type_sizes() {
        use super::{ BYTE, SHORT, WORD, TCHAR, INT, UINT, LONG, DWORD };

        extern {
            static fatfs_test_sizeof_BYTE: usize;
            static fatfs_test_sizeof_SHORT: usize;
            static fatfs_test_sizeof_WORD: usize;
            static fatfs_test_sizeof_TCHAR: usize;
            static fatfs_test_sizeof_INT: usize;
            static fatfs_test_sizeof_UINT: usize;
            static fatfs_test_sizeof_LONG: usize;
            static fatfs_test_sizeof_DWORD: usize;
        }

        assert_eq!(size_of::<BYTE>(), fatfs_test_sizeof_BYTE);
        assert_eq!(size_of::<SHORT>(), fatfs_test_sizeof_SHORT);
        assert_eq!(size_of::<WORD>(), fatfs_test_sizeof_WORD);
        assert_eq!(size_of::<TCHAR>(), fatfs_test_sizeof_TCHAR);
        assert_eq!(size_of::<INT>(), fatfs_test_sizeof_INT);
        assert_eq!(size_of::<UINT>(), fatfs_test_sizeof_UINT);
        assert_eq!(size_of::<LONG>(), fatfs_test_sizeof_LONG);
        assert_eq!(size_of::<DWORD>(), fatfs_test_sizeof_DWORD);
    }

    #[test]
    fn test_struct_sizes() {
        use core::mem;
        use super::{ FATFS, FIL };

        extern {
            static fatfs_test_sizeof_FATFS: usize;
            static fatfs_test_sizeof_FIL: usize;
        }

        assert_eq!(size_of::<FATFS>(), fatfs_test_sizeof_FATFS);
        assert_eq!(size_of::<FIL>(), fatfs_test_sizeof_FIL);
    }
}