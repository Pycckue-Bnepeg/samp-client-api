pub mod rpworld;
pub mod rwcore;
pub mod rwplcore;

// RpClump* RpClumpForAllAtomics(RpClump* clump, RpAtomicCallBack callback, void* pData); // 0x749B70
// RpGeometry* RpGeometryForAllMaterials(RpGeometry* geometry, RpMaterialCallBack fpCallBack, void* pData); // 0x74C790
// RwGlobals *&RwEngineInstance = *(RwGlobals **)0xC97B24;
static mut RW_ENGINE_INSTANCE: *mut *mut rwplcore::RwGlobals = 0xC97B24 as *mut *mut _;

type RpClumpForAllAtomics = extern "C" fn(
    *mut rpworld::RpClump,
    rpworld::RpAtomicCallBack,
    *mut std::os::raw::c_void,
) -> *mut rpworld::RpClump;
type RpGeometryForAllMaterials = extern "C" fn(
    *mut rpworld::RpGeometry,
    rpworld::RpMaterialCallBack,
    *mut std::os::raw::c_void,
) -> *mut rpworld::RpGeometry;

pub fn rpgeometry_for_all_materials(
    geometry: *mut rpworld::RpGeometry,
    fpCallBack: rpworld::RpMaterialCallBack,
    pData: *mut std::os::raw::c_void,
) -> *mut rpworld::RpGeometry {
    let func: RpGeometryForAllMaterials = unsafe { std::mem::transmute(0x74C790) };
    func(geometry, fpCallBack, pData)
}

pub fn rpclump_for_all_atomics(
    clump: *mut rpworld::RpClump,
    callback: rpworld::RpAtomicCallBack,
    pData: *mut std::os::raw::c_void,
) -> *mut rpworld::RpClump {
    let func: RpClumpForAllAtomics = unsafe { std::mem::transmute(0x749B70) };
    func(clump, callback, pData)
}

// TODO: generic
pub fn set_render_state(state: u32, value: u32) {
    let rw = unsafe { &mut (**RW_ENGINE_INSTANCE) };
    let value = unsafe { std::mem::transmute(value) };
    (rw.open_device.render_state_set)(state, value);
}

pub fn render_state(state: u32) -> u32 {
    let mut value = 0;

    let rw = unsafe { &mut (**RW_ENGINE_INSTANCE) };
    let value_ref = unsafe { std::mem::transmute(&mut value) };
    (rw.open_device.render_state_get)(state, value_ref);

    value
}
