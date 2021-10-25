#[repr(C)]
pub struct RwRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct RwSurfaceProperties {
    pub ambient: f32,  // ambient reflection coefficient
    pub specular: f32, // specular reflection coefficient
    pub diffuse: f32,  // reflection coefficient
}

#[repr(C)]
pub struct RwLLLink {
    next: *mut RwLLLink,
    prev: *mut RwLLLink,
}

#[repr(C)]
pub struct RwLinkList {
    pub link: RwLLLink,
}

#[repr(C)]
pub struct RwObject {
    pub obj_type: u8,
    sub_type: u8,
    flags: u8,
    private_flags: u8,
    parent: *mut (),
}

#[repr(C)]
pub struct RwResEntry {
    link: RwLLLink,
    size: i32,
    owner: *mut (),
    owner_ref: *mut *mut RwResEntry,
    destroy_notify: *mut (), // func RwResEntryDestroyNotify
}

#[repr(C)]
pub struct RwSphere {
    pub center: RwV3d,
    pub radius: f32,
}

#[repr(C)]
pub struct RwV3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct RwGlobals {
    pub cur_camera: *mut (),
    pub cur_world: *mut (),
    pub render_frame: u16,
    pub light_frame: u16,
    pad: [u16; 2],
    pub open_device: RwDevice,
    // TODO: ...
}

pub type RwRenderStateSetFunction = extern "C" fn(state: u32, param: *mut ()) -> bool;
pub type RwRenderStateGetFunction = extern "C" fn(state: u32, param: *mut ()) -> bool;

pub const RENDERSTATETEXTUREFILTER: u32 = 9;
pub const FILTERNEAREST: u32 = 1;

#[repr(C)]
pub struct RwDevice {
    pub gamma_correction: f32,
    pub system: *mut (), // func ptr
    pub z_buffer_near: f32,
    pub z_buffer_far: f32,
    pub render_state_set: RwRenderStateSetFunction,
    pub render_state_get: RwRenderStateGetFunction,
    // TODO: ...
}
