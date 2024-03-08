use core::ffi::c_char;

pub mod button;
pub mod gx;
pub mod heap;
pub mod math;
pub mod ppc;

extern "C" {
    static mut GAME_FRAME: u32;
    pub fn printf(str: *const c_char, ...);
}
