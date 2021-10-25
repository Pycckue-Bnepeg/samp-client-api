use std::ptr::NonNull;

use crate::gta::matrix::CRect;
use crate::gta::rw::rwcore::RwTexture;
use crate::gta::rw::rwplcore::RwRGBA;

type SpriteCreate = extern "thiscall" fn(this: *mut SpriteInner);
type SpriteDestroy = extern "thiscall" fn(this: *mut SpriteInner);
type SpriteDraw =
    extern "thiscall" fn(this: *mut SpriteInner, posn: *mut CRect, color: *mut RwRGBA);

type SpriteSetRenderState = extern "thiscall" fn(this: *mut SpriteInner);

pub struct Sprite {
    inner: NonNull<SpriteInner>,
}

#[repr(C)]
struct SpriteInner {
    texture: *mut RwTexture,
}

impl Sprite {
    pub fn new() -> Sprite {
        let inner = Box::new(SpriteInner {
            texture: std::ptr::null_mut(),
        });

        let inner = Box::into_raw(inner);

        let func: SpriteCreate = unsafe { std::mem::transmute(0x727230) };
        func(inner);

        Sprite {
            inner: unsafe { NonNull::new_unchecked(inner) },
        }
    }

    pub fn set_texture(&mut self, texture: *mut RwTexture) {
        unsafe {
            self.inner.as_mut().texture = texture;
        }
    }

    pub fn set_render_state(&mut self) {
        let func: SpriteSetRenderState = unsafe { std::mem::transmute(0x727B30) };
        func(self.inner.as_ptr());
    }

    pub fn draw(&mut self, mut pos: CRect, mut color: RwRGBA) {
        let func: SpriteDraw = unsafe { std::mem::transmute(0x728350) };
        func(self.inner.as_ptr(), &mut pos, &mut color);
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        let func: SpriteDestroy = unsafe { std::mem::transmute(0x7281E0) };
        func(self.inner.as_ptr());

        let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
    }
}
