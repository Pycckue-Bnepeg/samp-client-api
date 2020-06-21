use super::physical::CPhysical;
use super::rw::rwcore::RwTexture;

use std::os::raw::*;

#[repr(C)]
pub struct CObject {
    pub _base: CPhysical,
    pub m_pControlCodeList: *mut c_void,
    pub m_nObjectType: c_uchar,
    pub m_nBonusValue: c_uchar,
    pub m_wCostValue: c_ushort,
    pub m_nObjectFlags: u32,
    pub m_nColDamageEffect: c_uchar,
    pub m_nStoredColDamageEffect: c_uchar,
    pub field_146: c_char,
    pub m_nGarageDoorGarageIndex: c_char,
    pub m_nLastWeaponDamage: c_uchar,
    pub _bitfield_1: u8,
    pub m_nRefModelIndex: c_short,
    pub m_nCarColor: [c_uchar; 4usize],
    pub m_dwRemovalTime: c_int,
    pub m_fHealth: f32,
    pub m_fDoorStartAngle: f32,
    pub m_fScale: f32,
    pub m_pObjectInfo: *mut (), // CObjectInfo
    pub m_pFire: *mut (), // CFire
    pub m_wScriptTriggerIndex: c_short,
    pub m_wRemapTxd: c_short,
    pub m_pRemapTexture: *mut RwTexture,
    pub m_pDummyObject: *mut (), // CDummyObject
    pub m_dwBurnTime: c_int,
    pub m_fBurnDamage: f32,
}

impl CObject {
    pub fn physical(&self) -> &CPhysical {
        &self._base
    }
}