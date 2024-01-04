use super::flag_info;
use super::input_viewer::InputViewer;
use super::link_pos_viewer::LinkPosViwer;

pub struct LiveInfo {
    pub input_viewer:      bool,
    pub link_pos_viewer:   bool,
    pub scene_flag_viewer: bool,
}

#[no_mangle]
#[link_section = "data"]
pub static mut LIVE_INFO: LiveInfo = LiveInfo {
    input_viewer:      false,
    link_pos_viewer:   false,
    scene_flag_viewer: false,
};

impl LiveInfo {
    fn _display(&self) {
        if self.input_viewer {
            InputViewer::display();
        }
        if self.link_pos_viewer {
            LinkPosViwer::display();
        }
        if self.scene_flag_viewer {
            flag_info::disp_scene_flags();
        }
    }

    pub fn display() {
        unsafe { LIVE_INFO._display() };
    }
}
pub fn get_instance() -> &'static mut LiveInfo {
    return unsafe { &mut LIVE_INFO };
}
