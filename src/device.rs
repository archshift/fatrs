use core::mem;
use core::slice;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

const NUM_VOLUMES: usize = ::foreign::_VOLUMES as usize;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
lazy_static! {
    static ref DEVICE_MAP: Mutex<Option<DeviceMap>> = Mutex::new(None);
}

#[derive(Debug)]
pub enum IoctlCmd {
    GetSectorCount,
    GetBlockSize,
    CtrlSync,
}

pub enum IoctlRes {
    SectorCount(u32),
    BlockSize(u32),
    None
}


pub struct GenericDevice {
    user_data: [u8; 512],
    init_fn: fn(*mut u8) -> ::foreign::DSTATUS,
    read_fn: fn(*mut u8, usize, usize, &mut [u8]) -> ::foreign::DRESULT,
    write_fn: fn(*mut u8, usize, usize, &[u8]) -> ::foreign::DRESULT,
    ioctl_fn: fn(*mut u8, IoctlCmd) -> Result<IoctlRes, ::foreign::DRESULT>,
    drop_fn: fn(*mut u8)
}

impl Drop for GenericDevice {
    fn drop(&mut self) {
        (self.drop_fn)(self.user_data.as_mut_ptr())
    }
}

pub struct Device<State> {
    pub state: State,
    pub init_fn: fn(&mut State) -> ::foreign::DSTATUS,
    pub read_fn: fn(&mut State, usize, usize, &mut [u8]) -> ::foreign::DRESULT,
    pub write_fn: fn(&mut State, usize, usize, &[u8]) -> ::foreign::DRESULT,
    pub ioctl_fn: fn(&mut State, IoctlCmd) -> Result<IoctlRes, ::foreign::DRESULT>,
}

impl<State> Device<State> {
    pub fn generify(&self) -> GenericDevice {
        unsafe {
            let mut user_data = [0; 512];
            let generic_state_ptr = &self.state as *const State as *const u8;
            let generic_state_bytes = slice::from_raw_parts(generic_state_ptr, mem::size_of::<State>());
            user_data[..mem::size_of::<State>()].copy_from_slice(generic_state_bytes);
            let ret = GenericDevice {
                user_data: user_data,
                init_fn: mem::transmute(self.init_fn),
                read_fn: mem::transmute(self.read_fn),
                write_fn: mem::transmute(self.write_fn),
                ioctl_fn: mem::transmute(self.ioctl_fn),
                drop_fn: mem::transmute(mem::drop::<State> as usize)
            };
            mem::forget(self);
            ret
        }
    }
}

pub struct FsCtx {
	pub(crate) fatfs: ::foreign::FATFS,
}

pub struct DeviceMap {
    devices: [GenericDevice; NUM_VOLUMES]
}

impl DeviceMap {
    pub fn init_ctx<'a>(map: [GenericDevice; NUM_VOLUMES]) -> Result<FsCtx, &'static str> {
        if INITIALIZED.swap(true, Ordering::Acquire) == true {
            return Err("Already initialized!")
        }
        let mut global_map = DEVICE_MAP.lock();
        *global_map = Some(DeviceMap { devices: map });
        Ok(FsCtx {
			fatfs: unsafe { mem::zeroed() }
        })
    }
}

#[no_mangle]
pub extern fn disk_initialize(vol: ::foreign::BYTE) -> ::foreign::DSTATUS {
    if let Some(ref mut map) = *DEVICE_MAP.lock() {
        let dev = &mut map.devices[vol as usize];
        (dev.init_fn)(dev.user_data.as_mut_ptr())
    } else {
        ::foreign::STA_NODISK as ::foreign::DSTATUS
    }
}

#[no_mangle]
pub extern fn disk_status(vol: ::foreign::BYTE) -> ::foreign::DSTATUS {
    0
}

#[no_mangle]
pub extern fn disk_read(vol: ::foreign::BYTE, buf: *mut ::foreign::BYTE,
    sector: ::foreign::DWORD, count: ::foreign::UINT) -> ::foreign::DRESULT {

    if let Some(ref mut map) = *DEVICE_MAP.lock() {
        let dev = &mut map.devices[vol as usize];
        (dev.read_fn)(dev.user_data.as_mut_ptr(), sector as usize, count as usize,
            unsafe { slice::from_raw_parts_mut(buf, ::SECTOR_SIZE * count as usize) })
    } else {
        ::foreign::DRESULT_RES_NOTRDY as ::foreign::DRESULT
    }
}

#[no_mangle]
pub extern fn disk_write(vol: ::foreign::BYTE, buf: *const ::foreign::BYTE,
    sector: ::foreign::DWORD, count: ::foreign::UINT) -> ::foreign::DRESULT {
    
    if let Some(ref mut map) = *DEVICE_MAP.lock() {
        let dev = &mut map.devices[vol as usize];
        (dev.write_fn)(dev.user_data.as_mut_ptr(), sector as usize, count as usize,
            unsafe { slice::from_raw_parts(buf, ::SECTOR_SIZE * count as usize) })
    } else {
        ::foreign::DRESULT_RES_NOTRDY as ::foreign::DRESULT
    }
}

#[no_mangle]
pub extern fn disk_ioctl(vol: ::foreign::BYTE, cmd: ::foreign::BYTE,
    buf: ::foreign::voidp_mut) -> ::foreign::DRESULT {
    
    if let Some(ref mut map) = *DEVICE_MAP.lock() {
        let dev = &mut map.devices[vol as usize];
        let cmdenum = match cmd as u32 {
            ::foreign::CTRL_SYNC => IoctlCmd::CtrlSync,
            ::foreign::GET_SECTOR_COUNT => IoctlCmd::GetSectorCount,
            ::foreign::GET_BLOCK_SIZE => IoctlCmd::GetBlockSize,
            ::foreign::GET_SECTOR_SIZE => {
                unsafe { *(buf as *mut u16) = ::SECTOR_SIZE as u16 };
                return 0
            },
            _ => unimplemented!()
        };
        let res = (dev.ioctl_fn)(dev.user_data.as_mut_ptr(), cmdenum);
        unsafe {
            match res {
                Ok(IoctlRes::SectorCount(c)) => { *(buf as *mut u32) = c; 0 }
                Ok(IoctlRes::BlockSize(s)) => { *(buf as *mut u32) = s; 0 }
                Ok(IoctlRes::None) => 0,
                Err(val) => val
            }
        }
    } else {
        ::foreign::DRESULT_RES_NOTRDY as ::foreign::DRESULT
    }
}

#[no_mangle]
pub extern fn get_fattime() -> ::foreign::DWORD {
    0
}