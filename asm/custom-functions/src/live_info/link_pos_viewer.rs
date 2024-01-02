use crate::system::text_print::write_to_screen;
use crate::LINK_PTR;

pub struct LinkPosViwer {}
impl LinkPosViwer {
    pub fn display() {
        if unsafe { !LINK_PTR.is_null() } {
            let (x, y, z) = unsafe { ((*LINK_PTR).pos_x, (*LINK_PTR).pos_y, (*LINK_PTR).pos_z) };
            write_to_screen(format_args!("pos:\n{x:.3}\n{y:.3}\n{z:.3}"), 10, 20);
        }
    }
}
