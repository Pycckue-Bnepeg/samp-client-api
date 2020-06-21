use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

use super::rwplcore::*;

pub const rwTEXTUREBASENAMELENGTH: usize = 32;

pub type RwObjectHasFrameSyncFunction = Option<
    unsafe extern "C" fn(object: *mut RwObjectHasFrame) -> *mut RwObjectHasFrame,
>;

#[repr(C)]
pub struct RwObjectHasFrame {
    pub object: RwObject,
    pub lFrame: RwLLLink,
    pub sync: RwObjectHasFrameSyncFunction,
}

type RwRasterCreate = extern "C" fn(i32, i32, i32, i32) -> *mut RwRaster;
type RwRasterLock = extern "C" fn(*mut RwRaster, u8, i32) -> *mut u8;
type RwRasterUnlock = extern "C" fn(*mut RwRaster);
type RwRasterDestroy = extern "C" fn (*mut RwRaster) -> i32;

#[repr(C)]
pub struct RwRaster {
    pub parent: *mut RwRaster,
    pub cpPixels: *mut u8,
    pub palette: *mut u8,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub stride: i32,
    pub nOffsetX: i16,
    pub nOffsetY: i16,
    pub cType: u8,
    pub cFlags: u8,
    pub privateFlags: u8,
    pub cFormat: u8,
    pub originalPixels: *mut u8,
    pub originalWidth: i32,
    pub originalHeight: i32,
    pub originalStride: i32,
}

impl RwRaster {
    pub fn new(width: i32, height: i32) -> *mut RwRaster {
        let func: RwRasterCreate = unsafe { std::mem::transmute(0x7FB230) };

        func(width, height, 1, 0x0500 | 0x04)
    }

    pub fn lock(&mut self, level: u8) -> *mut u8 {
        let func: RwRasterLock = unsafe { std::mem::transmute(0x7FB2D0) };

        func(self, level, 3)
    }

    pub fn unlock(&mut self) {
        let func: RwRasterUnlock = unsafe { std::mem::transmute(0x7FAEC0) };

        func(self)
    }

    pub fn destroy(&mut self) -> i32 {
        let func: RwRasterDestroy = unsafe { std::mem::transmute(0x7FB020) };

        func(self)
    }
}

type RwTextureCreate = extern "C" fn(*mut RwRaster) -> *mut RwTexture;
type RwTextureDestroy = extern "C" fn (*mut RwTexture) -> i32;

#[repr(C)]
pub struct RwTexture {
    pub raster: *mut RwRaster,
    dict: *mut (), // RwTexDictionary
    in_dictionary: RwLLLink,
    name: [u8; rwTEXTUREBASENAMELENGTH],
    mask: [u8; rwTEXTUREBASENAMELENGTH],
    filter_addressing: u32,
    ref_count: i32,
}

impl RwTexture {
    pub fn new(raster: *mut RwRaster) -> *mut RwTexture {
        let func: RwTextureCreate = unsafe { std::mem::transmute(0x7F37C0) };

        func(raster)
    }

    pub fn raster(&mut self) -> &mut RwRaster {
        unsafe { &mut *self.raster }
    }

    pub fn name(&self) -> Cow<'_, str> {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(&self.name[..])
                .to_string_lossy()
        }
    }

    pub fn destroy(&mut self) -> i32 {
        let func: RwTextureDestroy = unsafe { std::mem::transmute(0x7F3820) };

        func(self)
    }
}
