use super::rwplcore::*;
use super::rwcore::*;

use std::ffi::c_void;

pub const rpATOMIC: u8 = 1;
pub const rpCLUMP: u8 = 2;

pub type RpAtomicCallBack = Option<
    unsafe extern "C" fn(
        atomic: *mut RpAtomic,
        data: *mut c_void,
    ) -> *mut RpAtomic,
>;

pub type RpMaterialCallBack = Option<
    unsafe extern "C" fn(
        material: *mut RpMaterial,
        data: *mut c_void,
    ) -> *mut RpMaterial,
>;

type RpMaterialCreate = extern "C" fn() -> *mut RpMaterial;
type RpMaterialSetTexture = extern "C" fn(*mut RpMaterial, *mut RwTexture) -> *mut RpMaterial;
type RpMaterialDestroy = extern "C" fn (*mut RpMaterial) -> i32;

#[repr(C)]
pub struct RpMaterial {
    pub texture: *mut RwTexture,
    pub color: RwRGBA,
    pub pipeline: *mut (), // RxPipeline
    pub surface_props: RwSurfaceProperties,
    pub ref_count: i16,
    pad: i16,
}

impl RpMaterial {
    pub fn new() -> *mut RpMaterial {
        let func: RpMaterialCreate = unsafe { std::mem::transmute(0x74D990) };
        func()
    }

    pub fn set_texture(&mut self, texture: *mut RwTexture) -> *mut RpMaterial {
        let func: RpMaterialSetTexture = unsafe { std::mem::transmute(0x74DBC0) };
        func(self, texture)
    }

    pub fn destroy(&mut self) {
        let func: RpMaterialDestroy = unsafe { std::mem::transmute(0x74DA20) };
        func(self);
    }
}

#[repr(C)]
pub struct RpMaterialList {
    pub materials: *mut *mut RpMaterial,
    pub numMaterials: i32,
    pub space: i32,
}

impl RpMaterialList {
    pub fn as_mut_slice(&mut self) -> &mut [*mut RpMaterial] {
        unsafe {
            std::slice::from_raw_parts_mut(self.materials, self.numMaterials as usize)
        }
    }
}

#[repr(C)]
pub struct RpGeometry {
    pub object: RwObject,
    pub flags: u32,
    pub lockedSinceLastInst: u16,
    pub refCount: i16,
    pub numTriangles: i32,
    pub numVertices: i32,
    pub numMorphTargets: i32,
    pub numTexCoordSets: i32,
    pub matList: RpMaterialList,
    pub triangles: *mut (), // RpTriangle
    pub preLitLum: *mut RwRGBA,
    pub texCoords: [*mut (); 8usize], // RwTexCoords
    pub mesh: *mut (), // RpMeshHeader
    pub repEntry: *mut RwResEntry,
    pub morphTarget: *mut (), // RpMorphTarget
}

#[repr(C)]
pub struct RpInterpolator {
    pub flags: i32,
    pub startMorphTarget: i16,
    pub endMorphTarget: i16,
    pub time: f32,
    pub recipTime: f32,
    pub position: f32,
}

pub type RpClumpCallBack = Option<
    unsafe extern "C" fn(
        clump: *mut RpClump,
        data: *mut c_void,
    ) -> *mut RpClump,
>;

#[repr(C)]
pub struct RpClump {
    pub object: RwObject,
    pub atomicList: RwLinkList,
    pub lightList: RwLinkList,
    pub cameraList: RwLinkList,
    pub inWorldLink: RwLLLink,
    pub callback: RpClumpCallBack,
}

pub type RpAtomicCallBackRender = Option<
    unsafe extern "C" fn(atomic: *mut RpAtomic) -> *mut RpAtomic,
>;

#[repr(C)]
pub struct RpAtomic {
    pub object: RwObjectHasFrame,
    pub repEntry: *mut RwResEntry,
    pub geometry: *mut RpGeometry,
    pub boundingSphere: RwSphere,
    pub worldBoundingSphere: RwSphere,
    pub clump: *mut RpClump,
    pub inClumpLink: RwLLLink,
    pub renderCallBack: RpAtomicCallBackRender,
    pub interpolator: RpInterpolator,
    pub renderFrame: u16,
    pub pad: u16,
    pub llWorldSectorsInAtomic: RwLinkList,
    pub pipeline: *mut (), // RxPipeline
}
