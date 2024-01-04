use super::simple_menu::SimpleMenu;
use crate::live_info::main::get_instance;
use crate::menus::main_menu::MainMenu;
use crate::system::button::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DisplayMenuState {
    Off,
    Main,
}

pub struct DisplayMenu {
    state:  DisplayMenuState,
    cursor: u32,
}

impl DisplayMenu {
    fn _input(&mut self) {
        let mut next_state = self.state;
        let instance = get_instance();
        match self.state {
            DisplayMenuState::Off => {},
            DisplayMenuState::Main => {
                if is_pressed(B) {
                    next_state = DisplayMenuState::Off;
                } else if is_pressed(A) {
                    match self.cursor {
                        0 => {
                            instance.input_viewer = !instance.input_viewer;
                        },
                        1 => {
                            instance.link_pos_viewer = !instance.link_pos_viewer;
                        },
                        2 => {
                            instance.scene_flag_viewer = !instance.scene_flag_viewer;
                        },
                        _ => {},
                    }
                }
            },
        }
        self.state = next_state;
    }

    fn _display(&mut self) {
        let mut menu = SimpleMenu::<6, 20>::new(10f32, 10f32, 10, "Display Menu");
        let instance = get_instance();
        menu.add_entry_args(format_args!(
            "Input Viewer [{}]",
            if instance.input_viewer { "x" } else { " " }
        ));
        menu.add_entry_args(format_args!(
            "Link Pos Viewer [{}]",
            if instance.link_pos_viewer { "x" } else { " " }
        ));
        menu.add_entry_args(format_args!(
            "Scene Flag Viewer [{}]",
            if instance.scene_flag_viewer { "x" } else { " " }
        ));
        self.cursor = menu.move_cursor(self.cursor);
        menu.draw();
    }
}

#[link_section = "data"]
#[no_mangle]
pub static mut DISPLAY_MENU: DisplayMenu = DisplayMenu {
    state:  DisplayMenuState::Off,
    cursor: 0,
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
