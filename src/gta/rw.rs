pub mod rpworld;
pub mod rwcore;
pub mod rwplcore;

// RpClump* RpClumpForAllAtomics(RpClump* clump, RpAtomicCallBack callback, void* pData); // 0x749B70
// RpGeometry* RpGeometryForAllMaterials(RpGeometry* geometry, RpMaterialCallBack fpCallBack, void* pData); // 0x74C790

type RpClumpForAllAtomics = extern "C" fn(*mut rpworld::RpClump, rpworld::RpAtomicCallBack, *mut std::os::raw::c_void) -> *mut rpworld::RpClump;
type RpGeometryForAllMaterials = extern "C" fn(*mut rpworld::RpGeometry, rpworld::RpMaterialCallBack, *mut std::os::raw::c_void) -> *mut rpworld::RpGeometry;

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
