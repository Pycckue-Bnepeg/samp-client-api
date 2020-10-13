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
