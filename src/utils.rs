use std::ffi::CString;
use winapi::shared::minwindef::{DWORD, FARPROC, HMODULE};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
use winapi::um::winuser::*;

use crate::gta::matrix::CVector;

pub fn error_message_box<T: AsRef<str>, M: AsRef<str>>(title: T, message: M) {
    let title = CString::new(title.as_ref()).unwrap();
    let message = CString::new(message.as_ref()).unwrap();
    let flags = MB_OK | MB_ICONERROR;

    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            message.as_ptr() as *const _,
            title.as_ptr() as *const _,
            flags,
        );
    }
}

pub fn is_key_pressed(key: i32) -> bool {
    let key_state = unsafe { GetKeyState(key) as u16 };
    key_state >> 15 == 1
}

pub fn module_handle(name: &str) -> HMODULE {
    let c_name = CString::new(name).unwrap();

    unsafe { GetModuleHandleA(c_name.as_ptr() as *const _) }
}

// TODO: log in a file
pub fn handle_result<T, E: std::fmt::Debug>(result: Result<T, E>) -> Option<T> {
    if let Err(err) = result.as_ref() {
        println!("{:?}", err);
    }

    result.ok()
}

pub fn distance(v1: &CVector, v2: &CVector) -> f32 {
    let sum = (v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.z - v1.z).powi(2);
    sum.sqrt()
}

pub fn key_name(key: u32) -> String {
    let mut scan_code = unsafe { MapVirtualKeyA(key, MAPVK_VK_TO_VSC) };

    match key as i32 {
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_RCONTROL | VK_RMENU | VK_LWIN | VK_RWIN
        | VK_APPS | VK_PRIOR | VK_NEXT | VK_END | VK_HOME | VK_INSERT | VK_DELETE | VK_DIVIDE
        | VK_NUMLOCK => scan_code |= KF_EXTENDED as u32,

        VK_XBUTTON1 => return "Mouse 4".to_string(),
        VK_XBUTTON2 => return "Mouse 5".to_string(),

        _ => (),
    }

    let mut buffer = vec![0u8; 128];

    let result =
        unsafe { GetKeyNameTextA((scan_code << 16) as _, buffer.as_mut_ptr() as *mut _, 128) };

    if result == 0 {
        String::from("Unknown key")
    } else {
        let idx = buffer.iter().position(|a| *a == 0).unwrap_or(buffer.len());
        String::from_utf8_lossy(&buffer[0..idx])
            .to_owned()
            .to_string()
    }
}

pub fn find_function<T>(module: &str, func: &str) -> Option<T> {
    let module = CString::new(module).unwrap();
    let func = CString::new(func).unwrap();

    let module_ptr = unsafe { LoadLibraryA(module.as_ptr() as *const _) };

    if module_ptr.is_null() {
        println!("D3D9 (utils): Module {:?} isn't found", module);
        return None;
    }

    let func_ptr = unsafe { GetProcAddress(module_ptr, func.as_ptr() as *const _) };

    if func_ptr.is_null() {
        println!(
            "D3D9 (utils): Function {:?} in {:?} module isn't found",
            func, module
        );
        return None;
    }

    Some(unsafe { std::mem::transmute_copy(&func_ptr) })
}

#[repr(C)]
pub struct FixedFileInfo {
    signature: DWORD,
    struc_version: DWORD,
    pub file_version_ms: DWORD,
    pub file_version_ls: DWORD,
    product_version_ms: DWORD,
    product_version_ls: DWORD,
    file_flags_mask: DWORD,
    file_flags: DWORD,
    file_os: DWORD,
    file_type: DWORD,
    file_subtype: DWORD,
    file_date_ms: DWORD,
    file_date_ls: DWORD,
}
