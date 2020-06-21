use super::display::CSprite2d;
use super::matrix::CVector2D;

const MENU_MANAGER: usize = 0xBA6748;
const MAX_VOLUME: i8 = 64;

#[repr(C)]
pub struct CMenuManager {
    pub field_0: std::os::raw::c_char,
    pub field_1: [std::os::raw::c_char; 3],
    pub m_fStatsScrollSpeed: f32,
    pub field_8: std::os::raw::c_char,
    pub field_9: [std::os::raw::c_char; 23],
    pub field_20: std::os::raw::c_char,
    pub m_bHudOn: bool,
    pub field_22: [std::os::raw::c_char; 2],
    pub m_nRadarMode: std::os::raw::c_int,
    pub field_28: [std::os::raw::c_char; 4],
    pub m_nTargetBlipIndex: std::os::raw::c_int,
    pub field_30: std::os::raw::c_char,
    pub field_31: std::os::raw::c_char,
    pub m_bDontDrawFrontEnd: bool,
    pub m_bActivateMenuNextFrame: bool,
    pub m_bMenuAccessWidescreen: bool,
    pub field_35: std::os::raw::c_char,
    pub field_36: [std::os::raw::c_char; 2],
    pub field_38: std::os::raw::c_int,
    pub m_nBrightness: std::os::raw::c_int,
    pub m_fDrawDistance: f32,
    pub m_bShowSubtitles: bool,
    pub field_45: [std::os::raw::c_char; 4],
    pub field_49: std::os::raw::c_char,
    pub m_bMapLegend: bool,
    pub m_bWidescreenOn: bool,
    pub m_bFrameLimiterOn: bool,
    pub m_bRadioAutoSelect: bool,
    pub field_4E: std::os::raw::c_char,
    pub m_nSfxVolume: std::os::raw::c_char,
    pub m_nRadioVolume: std::os::raw::c_char,
    pub m_bRadioEq: bool,
    pub m_nRadioStation: std::os::raw::c_char,
    pub field_53: std::os::raw::c_char,
    pub m_nSelectedMenuItem: std::os::raw::c_int,
    pub field_58: std::os::raw::c_char,
    pub drawRadarOrMap: std::os::raw::c_char,
    pub field_5A: std::os::raw::c_char,
    pub field_5B: std::os::raw::c_char,
    pub m_bMenuActive: bool,
    pub doGameReload: std::os::raw::c_char,
    pub field_5E: std::os::raw::c_char,
    pub isSaveDone: std::os::raw::c_char,
    pub m_bLoadingData: bool,
    pub field_61: [std::os::raw::c_char; 3],
    pub m_fMapZoom: f32,
    pub m_fMapBaseX: f32,
    pub m_fMapBaseY: f32,
    pub m_vMousePos: CVector2D,
    pub field_78: std::os::raw::c_char,
    pub field_79: [std::os::raw::c_char; 3],
    pub titleLanguage: std::os::raw::c_int,
    pub textLanguage: std::os::raw::c_int,
    pub m_nLanguage: std::os::raw::c_char,
    pub m_nPreviousLanguage: std::os::raw::c_char,
    pub field_86: [std::os::raw::c_char; 2],
    pub field_88: std::os::raw::c_int,
    pub m_bLanguageChanged: bool,
    pub field_8D: [std::os::raw::c_char; 3],
    pub field_90: std::os::raw::c_int,
    pub field_94: std::os::raw::c_int,
    pub m_pJPegBuffer: *mut std::os::raw::c_char,
    pub field_9C: [std::os::raw::c_char; 16],
    pub field_AC: std::os::raw::c_int,
    pub m_nRadioMode: std::os::raw::c_char,
    pub invertPadX1: std::os::raw::c_char,
    pub invertPadY1: std::os::raw::c_char,
    pub invertPadX2: std::os::raw::c_char,
    pub invertPadY2: std::os::raw::c_char,
    pub swapPadAxis1: std::os::raw::c_char,
    pub swapPadAxis2: std::os::raw::c_char,
    pub field_B7: std::os::raw::c_char,
    pub m_bDrawMouse: bool,
    pub field_B9: [std::os::raw::c_char; 3],
    pub m_nMousePosLeft: std::os::raw::c_int,
    pub m_nMousePosTop: std::os::raw::c_int,
    pub m_bMipMapping: bool,
    pub m_bTracksAutoScan: bool,
    pub field_C6: std::os::raw::c_short,
    pub m_nAppliedAntiAliasingLevel: std::os::raw::c_int,
    pub m_nAntiAliasingLevel: std::os::raw::c_int,
    pub m_nController: std::os::raw::c_char,
    pub field_D1: [std::os::raw::c_char; 3],
    pub m_nAppliedResolution: std::os::raw::c_int,
    pub m_nResolution: std::os::raw::c_int,
    pub field_DC: std::os::raw::c_int,
    pub mousePosLeftA: std::os::raw::c_int,
    pub mousePosTopA: std::os::raw::c_int,
    pub m_bSavePhotos: bool,
    pub m_bMainMenuSwitch: bool,
    pub m_nPlayerNumber: std::os::raw::c_char,
    pub field_EB: std::os::raw::c_char,
    pub field_EC: std::os::raw::c_int,
    pub field_F0: std::os::raw::c_int,
    pub field_F4: std::os::raw::c_char,
    pub field_F5: [std::os::raw::c_char; 3],
    pub m_apTextures: [CSprite2d; 25],
    pub m_bTexturesLoaded: bool,
    pub m_nCurrentMenuPage: std::os::raw::c_uchar,
    pub field_15E: std::os::raw::c_char,
    pub m_bSelectedSaveGame: std::os::raw::c_uchar,
    pub field_160: std::os::raw::c_char,
    pub field_161: std::os::raw::c_char,
    pub m_szMpackName: [std::os::raw::c_char; 8],
    pub field_16A: [std::os::raw::c_char; 6486],
    pub field_1AC0: std::os::raw::c_int,
    pub field_1AC4: std::os::raw::c_int,
    pub field_1AC8: std::os::raw::c_int,
    pub field_1ACC: std::os::raw::c_int,
    pub field_1AD0: std::os::raw::c_int,
    pub field_1AD4: std::os::raw::c_int,
    pub field_1AD8: std::os::raw::c_int,
    pub field_1ADC: std::os::raw::c_short,
    pub m_bChangeVideoMode: bool,
    pub field_1ADF: std::os::raw::c_char,
    pub field_1AE0: std::os::raw::c_int,
    pub field_1AE4: std::os::raw::c_int,
    pub field_1AE8: std::os::raw::c_char,
    pub field_1AE9: std::os::raw::c_char,
    pub field_1AEA: std::os::raw::c_char,
    pub m_bScanningUserTracks: bool,
    pub field_1AEC: std::os::raw::c_int,
    pub field_1AF0: std::os::raw::c_char,
    pub field_1AF1: std::os::raw::c_char,
    pub field_1AF2: std::os::raw::c_char,
    pub field_1AF3: std::os::raw::c_char,
    pub field_1AF4: std::os::raw::c_int,
    pub field_1AF8: std::os::raw::c_int,
    pub field_1AFC: std::os::raw::c_int,
    pub field_1B00: std::os::raw::c_int,
    pub field_1B04: std::os::raw::c_int,
    pub field_1B08: std::os::raw::c_char,
    pub field_1B09: std::os::raw::c_char,
    pub field_1B0A: std::os::raw::c_char,
    pub field_1B0B: std::os::raw::c_char,
    pub field_1B0C: std::os::raw::c_int,
    pub field_1B10: std::os::raw::c_char,
    pub field_1B11: std::os::raw::c_char,
    pub field_1B12: std::os::raw::c_char,
    pub field_1B13: std::os::raw::c_char,
    pub field_1B14: std::os::raw::c_char,
    pub field_1B15: std::os::raw::c_char,
    pub field_1B16: std::os::raw::c_char,
    pub field_1B17: std::os::raw::c_char,
    pub EventToDo: std::os::raw::c_int,
    pub field_1B1C: std::os::raw::c_int,
    pub m_nTexturesRound: std::os::raw::c_uchar,
    pub m_nNumberOfMenuOptions: std::os::raw::c_uchar,
    pub field_1B22: std::os::raw::c_short,
    pub field_1B24: std::os::raw::c_int,
    pub field_1B28: std::os::raw::c_char,
    pub field_1B29: std::os::raw::c_char,
    pub field_1B2A: std::os::raw::c_short,
    pub field_1B2C: std::os::raw::c_int,
    pub field_1B30: std::os::raw::c_int,
    pub field_1B34: std::os::raw::c_short,
    pub field_1B36: std::os::raw::c_short,
    pub field_1B38: std::os::raw::c_int,
    pub field_1B3C: std::os::raw::c_char,
    pub field_1B3D: std::os::raw::c_char,
    pub field_1B3E: std::os::raw::c_char,
    pub field_1B3F: std::os::raw::c_char,
    pub field_1B40: std::os::raw::c_int,
    pub field_1B44: std::os::raw::c_char,
    pub field_1B45: std::os::raw::c_char,
    pub field_1B46: std::os::raw::c_short,
    pub field_1B48: std::os::raw::c_int,
    pub field_1B4C: std::os::raw::c_int,
    pub m_nBackgroundSprite: std::os::raw::c_char,
    pub field_1B51: std::os::raw::c_char,
    pub field_1B52: std::os::raw::c_short,
    pub field_1B54: std::os::raw::c_int,
    pub field_1B58: std::os::raw::c_int,
    pub field_1B5C: std::os::raw::c_char,
    pub field_1B5D: std::os::raw::c_char,
    pub field_1B5E: std::os::raw::c_short,
    pub field_1B60: std::os::raw::c_int,
    pub field_1B64: std::os::raw::c_int,
    pub field_1B68: std::os::raw::c_int,
    pub field_1B6C: std::os::raw::c_int,
    pub field_1B70: std::os::raw::c_int,
    pub field_1B74: std::os::raw::c_int,
}

impl CMenuManager {
    pub fn get<'a>() -> &'a CMenuManager {
        unsafe { &*(MENU_MANAGER as *const _) }
    }

    pub fn is_active(&self) -> bool {
        self.m_bMenuActive
    }

    pub fn current_page(&self) -> eMenuPage::Type {
        self.m_nCurrentMenuPage as _
    }

    pub fn sfx_volume(&self) -> f32 {
        self.m_nSfxVolume as f32 / MAX_VOLUME as f32
    }

    pub fn is_menu_active() -> bool {
        Self::get().is_active()
    }
}

pub mod eMenuPage {
    pub type Type = i32;
    pub const MENUPAGE_STATS: Type = 0;
    pub const MENUPAGE_START_GAME: Type = 1;
    pub const MENUPAGE_BRIEF: Type = 2;
    pub const MENUPAGE_AUDIO_SETTINGS: Type = 3;
    pub const MENUPAGE_DISPLAY_SETTINGS: Type = 4;
    pub const MENUPAGE_MAP: Type = 5;
    pub const MENUPAGE_NEW_GAME_ASK: Type = 6;
    pub const MENUPAGE_SELECT_GAME: Type = 7;
    pub const MENUPAGE_MISSIONPACK_LOADING_ASK: Type = 8;
    pub const MENUPAGE_LOAD_GAME: Type = 9;
    pub const MENUPAGE_DELETE_GAME: Type = 10;
    pub const MENUPAGE_LOAD_GAME_ASK: Type = 11;
    pub const MENUPAGE_DELETE_GAME_ASK: Type = 12;
    pub const MENUPAGE_LOAD_FIRST_SAVE: Type = 13;
    pub const MENUPAGE_DELETE_FINISHED: Type = 14;
    pub const MENUPAGE_DELETE_SUCCESSFUL: Type = 15;
    pub const MENUPAGE_GAME_SAVE: Type = 16;
    pub const MENUPAGE_SAVE_WRITE_ASK: Type = 17;
    pub const MENUPAGE_SAVE_DONE_1: Type = 18;
    pub const MENUPAGE_SAVE_DONE_2: Type = 19;
    pub const MENUPAGE_GAME_SAVED: Type = 20;
    pub const MENUPAGE_GAME_LOADED: Type = 21;
    pub const MENUPAGE_GAME_WARNING_DONT_SAVE: Type = 22;
    pub const MENUPAGE_ASK_DISPLAY_DEFAULT_SETS: Type = 23;
    pub const MENUPAGE_ASK_AUDIO_DEFAULT_SETS: Type = 24;
    pub const MENUPAGE_ASK_CONTROLLER_DEFAULT_SETS: Type = 25;
    pub const MENUPAGE_USER_TRACKS_OPTIONS: Type = 26;
    pub const MENUPAGE_DISPLAY_ADVANCED: Type = 27;
    pub const MENUPAGE_LANGUAGE: Type = 28;
    pub const MENUPAGE_SAVE_GAME_DONE: Type = 29;
    pub const MENUPAGE_SAVE_GAME_FAILED: Type = 30;
    pub const MENUPAGE_SAVE_WRITE_FAILED: Type = 31;
    pub const MENUPAGE_SAVE_FAILED_FILE_ERROR: Type = 32;
    pub const MENUPAGE_OPTIONS: Type = 33;
    pub const MENUPAGE_MAIN_MENU: Type = 34;
    pub const MENUPAGE_QUIT_GAME_ASK: Type = 35;
    pub const MENUPAGE_CONTROLLER_SETUP: Type = 36;
    pub const MENUPAGE_REDEFINE_CONTROLS: Type = 37;
    pub const MENUPAGE_CONTROLS_VEHICLE_ONFOOT: Type = 38;
    pub const MENUPAGE_MOUSE_SETTINGS: Type = 39;
    pub const MENUPAGE_JOYPAD_SETTINGS: Type = 40;
    pub const MENUPAGE_PAUSE_MENU: Type = 41;
    pub const MENUPAGE_QUIT_GAME_2: Type = 42;
    pub const MENUPAGE_EMPTY: Type = 43;
}
