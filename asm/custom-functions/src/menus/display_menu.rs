use super::simple_menu::SimpleMenu;
use crate::live_info::main::LiveInfo;
use crate::menus::main_menu::MainMenu;
use crate::system::button::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DisplayMenuState {
    Off,
    Main,
}

pub struct DisplayMenu {
    state:           DisplayMenuState,
    cursor:          u32,
    input_viewer:    bool,
    link_pos_viewer: bool,
}

impl DisplayMenu {
    fn _input(&mut self) {
        let mut next_state = self.state;

        match self.state {
            DisplayMenuState::Off => {},
            DisplayMenuState::Main => {
                if is_pressed(B) {
                    next_state = DisplayMenuState::Off;
                } else if is_pressed(A) {
                    match self.cursor {
                        0 => {
                            self.input_viewer = !self.input_viewer;
                            LiveInfo::set_input_viewer(self.input_viewer);
                        },
                        1 => {
                            self.link_pos_viewer = !self.link_pos_viewer;
                            LiveInfo::set_link_pos_viewer(self.link_pos_viewer);
                        },
                        _ => {},
                    }
                }
            },
        }
        self.state = next_state;
    }

    fn _display(&mut self) {
        let mut menu = SimpleMenu::<3, 20>::new(10, 10, 10, "Display Menu");
        menu.add_entry_args(format_args!(
            "Input Viewer [{}]",
            if self.input_viewer { "x" } else { " " }
        ));
        menu.add_entry_args(format_args!(
            "Link Pos Viewer [{}]",
            if self.link_pos_viewer { "x" } else { " " }
        ));
        self.cursor = menu.move_cursor(self.cursor);
        menu.draw();
    }
}

#[link_section = "data"]
#[no_mangle]
pub static mut DISPLAY_MENU: DisplayMenu = DisplayMenu {
    state:           DisplayMenuState::Off,
    cursor:          0,
    input_viewer:    false,
    link_pos_viewer: false,
};

impl DisplayMenu {
    pub fn enable() {
        unsafe { DISPLAY_MENU.state = DisplayMenuState::Main };
    }
    // returns true if in off state
    pub fn input() -> bool {
        unsafe {
            DISPLAY_MENU._input();
            return DISPLAY_MENU.state == DisplayMenuState::Off;
        }
    }

    pub fn display() {
        unsafe { DISPLAY_MENU._display() };
    }
}
