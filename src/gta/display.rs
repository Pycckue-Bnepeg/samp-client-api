use super::matrix::CVector;

const DONT_DRAW_RADAR: *mut bool = 0xBAA3FB as *mut bool;

#[repr(C)]
#[derive(Debug)]
pub struct CSprite2d {
    pub m_pTexture: *mut (), // RwTexture
}

type CalcScreenCoords =
    extern "C" fn(*const CVector, *mut CVector, *mut f32, *mut f32, bool, bool) -> bool;

pub fn calc_screen_coords(input: &CVector) -> Option<(f32, f32)> {
    const FUNC: usize = 0x70CE30;

    let mut out = CVector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut x = 0.0;
    let mut y = 0.0;

    let convert: CalcScreenCoords = unsafe { std::mem::transmute(FUNC) };

    if convert(input, &mut out, &mut x, &mut y, true, true) {
        Some((out.x, out.y))
    } else {
        None
    }
}

pub fn is_radar_enabled() -> bool {
    unsafe { !(*DONT_DRAW_RADAR) }
}
