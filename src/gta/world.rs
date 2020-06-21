use super::matrix::CVector;

// CVector const& origin, CVector const& target, bool buildings, bool vehicles, bool peds, bool objects, bool dummies, bool doSeeThroughCheck, bool doCameraIgnoreCheck
type GetIsLineOfSightClear = extern "C" fn(
    origin: *const CVector,
    target: *const CVector,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
) -> bool;

pub fn is_line_of_sight_clear(from: &CVector, to: &CVector) -> bool {
    const FUNC: usize = 0x56A490;

    let line_of_sight_clear: GetIsLineOfSightClear = unsafe { std::mem::transmute(FUNC) };

    // buildings , vehicles , peds , objects , dummies ,
    line_of_sight_clear(from, to, true, false, false, true, true, true, true)
}
