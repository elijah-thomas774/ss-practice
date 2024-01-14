mod flag_info;
mod input_viewer;
mod link_pos_viewer;

#[link_section = "data"]
pub static mut INPUT_VIEWER: bool = false;
#[link_section = "data"]
pub static mut LINK_POS_VIEWER: bool = false;
#[link_section = "data"]
pub static mut SCENE_FLAG_VIEWER: bool = false;

pub fn display() {
    if unsafe { INPUT_VIEWER } {
        input_viewer::display();
    }
    if unsafe { LINK_POS_VIEWER } {
        link_pos_viewer::display_pos();
    }
    if unsafe { SCENE_FLAG_VIEWER } {
        flag_info::disp_scene_flags();
    }
}
