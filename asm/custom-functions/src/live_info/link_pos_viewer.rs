use crate::system::text_print::write_to_screen;
use crate::LINK_PTR;
use core::fmt::Write;

use super::console::Console;

pub struct LinkPosViwer {}
impl LinkPosViwer {
    pub fn display() {
        if unsafe { !LINK_PTR.is_null() } {
            let (x, y, z) = unsafe { ((*LINK_PTR).pos_x, (*LINK_PTR).pos_y, (*LINK_PTR).pos_z) };
            let mut console = Console::with_pos_and_size(0f32, 120f32, 120f32, 85f32);
            console.set_bg_color(0x0000007F);
            console.set_font_color(0xFFFFFFFF);
            console.set_font_size(0.25f32);
            console.set_dynamic_size(true);
            console.write_fmt(format_args!("pos:\nx:{x:>9.2}\ny:{y:>9.2}\nz:{z:>9.2}"));
            console.draw();
        }
    }
}
