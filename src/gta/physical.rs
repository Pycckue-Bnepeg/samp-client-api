use super::entity::CEntity;
use super::matrix::*;

use std::os::raw::*;

#[repr(C)]
pub struct CPhysical {
    pub _base: CEntity,
    pub field_38: c_int,
    pub m_nLastCollisionTime: c_uint,
    pub m_nPhysicalFlags: u32, // flags
    pub m_vecMoveSpeed: CVector,
    pub m_vecTurnSpeed: CVector,
    pub m_vecFrictionMoveSpeed: CVector,
    pub m_vecFrictionTurnSpeed: CVector,
    pub m_vecForce: CVector,
    pub m_vecTorque: CVector,
    pub m_fMass: f32,
    pub m_fTurnMass: f32,
    pub m_fVelocityFrequency: f32,
    pub m_fAirResistance: f32,
    pub m_fElasticity: f32,
    pub m_fBuoyancyConstant: f32,
    pub m_vecCentreOfMass: CVector,
    pub m_pCollisionList: *mut c_void,
    pub m_pMovingList: *mut c_void,
    pub field_B8: c_char,
    pub m_nNumEntitiesCollided: c_uchar,
    pub m_nContactSurface: c_uchar,
    pub field_BB: c_char,
    pub m_apCollidedEntities: [*mut CEntity; 6usize],
    pub m_fMovingSpeed: f32,
    pub m_fDamageIntensity: f32,
    pub m_pDamageEntity: *mut CEntity,
    pub m_vecLastCollisionImpactVelocity: CVector,
    pub m_vecLastCollisionPosn: CVector,
    pub m_nPieceType: c_ushort,
    pub field_FA: c_short,
    pub m_pAttachedTo: *mut CPhysical,
    pub m_vecAttachOffset: CVector,
    pub m_vecAttachedEntityPosn: CVector,
    pub m_qAttachedEntityRotation: CQuaternion,
    pub m_pEntityIgnoredCollision: *mut CEntity,
    pub m_fContactSurfaceBrightness: f32,
    pub m_fDynamicLighting: f32,
    pub m_pShadowData: *mut (), // CRealTimeShadow
}

impl CPhysical {
    pub fn entity(&self) -> &CEntity {
        &self._base
    }
}