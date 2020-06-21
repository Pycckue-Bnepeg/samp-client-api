use winapi::shared::windef::HWND;

pub mod camera;
pub mod d3d9;
pub mod d9_proxy;
pub mod device_proxy;
pub mod display;
pub mod entity;
pub mod game;
pub mod matrix;
pub mod menu_manager;
pub mod object;
pub mod physical;
pub mod rw;
pub mod sprite;
pub mod world;

const GTA_HWND_PTR: *const HWND = 0xC97C1C as *const HWND;

pub fn hwnd() -> HWND {
    unsafe { GTA_HWND_PTR.read() }
}
