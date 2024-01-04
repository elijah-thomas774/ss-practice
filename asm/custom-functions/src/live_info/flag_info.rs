use core::fmt::Write;

use crate::system::flag_managers::SceneflagManager;

use super::console::Console;

pub fn disp_scene_flags() {
    let flags = unsafe { &*SceneflagManager::get_flags() };
    let mut console = Console::with_pos(0f32, 0f32);
    console.set_bg_color(0x000000CF);
    console.set_font_color(0xFFFFFFFF);
    console.set_font_size(0.3f32);
    console.write_str("Scene Flags:\n");
    for flag_bytes in flags {
        let val_hi = (*flag_bytes >> 8) as u8;
        let val_lo = (*flag_bytes >> 0) as u8;
        console.write_fmt(format_args!(" {val_hi:02X} {val_lo:02X}"));
    }
    console.draw();
}
