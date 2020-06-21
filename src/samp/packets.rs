use super::{handle, CStdString, BOOL, D3DCOLOR, GTAREF, ID, NUMBER, TICK};
use crate::gta::matrix::{CVector, RwMatrix};

#[repr(C, packed)]
pub struct TrailerData {
    pub m_nId: ID,
    pub m_position: CVector,
    pub m_fQuaternion: [f32; 4],
    pub m_speed: CVector,
    pub m_turnSpeed: CVector,
}

#[repr(C, packed)]
pub struct PassengerData {
    pub m_nVehicleId: ID,
    pub m_nSeatId: std::os::raw::c_uchar,
    pub m_nCurrentWeapon: std::os::raw::c_uchar,
    pub m_nHealth: std::os::raw::c_uchar,
    pub m_nArmor: std::os::raw::c_uchar,
    pub m_controllerState: ControllerState,
    pub m_position: CVector,
}

#[repr(C, packed)]
pub struct OnfootData {
    pub m_controllerState: ControllerState,
    pub m_position: CVector,
    pub m_fQuaternion: [f32; 4],
    pub m_nHealth: std::os::raw::c_uchar,
    pub m_nArmor: std::os::raw::c_uchar,
    pub m_nCurrentWeapon: std::os::raw::c_uchar,
    pub m_nSpecialAction: std::os::raw::c_uchar,
    pub m_speed: CVector,
    pub m_surfingOffset: CVector,
    pub m_nSurfingVehicleId: ID,
    pub m_animation: Animation,
}

#[repr(C, packed)]
pub struct IncarData {
    pub m_nVehicle: ID,
    pub m_controllerState: ControllerState,
    pub m_fQuaternion: [f32; 4usize],
    pub m_position: CVector,
    pub m_speed: CVector,
    pub m_fHealth: f32,
    pub m_nDriverHealth: std::os::raw::c_uchar,
    pub m_nDriverArmor: std::os::raw::c_uchar,
    pub m_nCurrentWeapon: std::os::raw::c_uchar,
    pub m_bSirenEnabled: bool,
    pub m_bLandingGear: bool,
    pub m_nTrailerId: ID,
    pub __bindgen_anon_1: f32,
}

#[repr(C, packed)]
pub struct AimData {
    pub m_nCameraMode: std::os::raw::c_uchar,
    pub m_aimf1: CVector,
    pub m_aimPos: CVector,
    pub m_fAimZ: f32,
    pub _bitfield_1: u8,
    pub m_nAspectRatio: std::os::raw::c_char,
}

#[repr(C, packed)]
pub struct ControllerState {
    pub m_sLeftStickX: std::os::raw::c_short,
    pub m_sLeftStickY: std::os::raw::c_short,
    pub __bindgen_anon_1: std::os::raw::c_short,
}

#[repr(C, packed)]
pub struct Animation {
    pub shit: u32,
}
