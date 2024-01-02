use super::input_viewer::InputViewer;
use super::link_pos_viewer::LinkPosViwer;

pub struct LiveInfo {
    input_viewer:    bool,
    link_pos_viewer: bool,
}

#[no_mangle]
#[link_section = "data"]
static mut LIVE_INFO: LiveInfo = LiveInfo {
    input_viewer:    false,
    link_pos_viewer: false,
};

impl LiveInfo {
    pub fn set_input_viewer(active: bool) {
        unsafe { LIVE_INFO.input_viewer = active };
    }
    pub fn set_link_pos_viewer(active: bool) {
        unsafe { LIVE_INFO.link_pos_viewer = active };
    }

    fn _display(&self) {
        if self.input_viewer {
            InputViewer::display();
        }
        if self.link_pos_viewer {
            LinkPosViwer::display();
        }
    }

    pub fn display() {
        unsafe { LIVE_INFO._display() };
    }
}
