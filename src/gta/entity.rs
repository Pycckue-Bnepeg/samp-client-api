use std::os::raw::*;
use super::matrix::CPlaceable;

#[repr(C)]
pub struct CEntity {
    pub _base: CPlaceable,
    pub rw_entity: *mut (), // RwObject, RpClump or RwAtomic
    pub _bitfield_1: u32,
    pub m_nRandomSeed: c_ushort,
    pub m_nModelIndex: c_ushort,
    pub m_pReferences: *mut (), // CReference
    pub m_pStreamingLink: *mut c_void,
    pub m_nScanCode: c_short,
    pub m_nIplIndex: c_char,
    pub m_nAreaCode: c_uchar,
    pub lod: u32, // LodIndex or Lod ptr
    pub m_nNumLodChildren: c_uchar,
    pub m_nNumLodChildrenRendered: c_uchar,
    pub _bitfield_2: u8, // entity type
}

impl CEntity {
    pub fn placeable(&self) -> &CPlaceable {
        &self._base
    }

    pub fn entity_type(&self) -> u8 {
        self._bitfield_2 & 0b111
    }
}
