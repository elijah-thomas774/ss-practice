use super::simple_menu::SimpleMenu;
use crate::menus::main_menu::MainMenu;
use crate::system::button::*;
use crate::system::file_manager::FileManager;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActionMenuState {
    Off,
    Main,
}

pub struct ActionMenu {
    state:  ActionMenuState,
    cursor: u32,
}

impl ActionMenu {
    fn _input(&mut self) {
        let mut next_state = self.state;

        match self.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                if is_pressed(B) {
                    next_state = ActionMenuState::Off;
                } else if is_pressed(A) {
                    match self.cursor {
                        0 => {
                            next_state = ActionMenuState::Off;
                        },
                        _ => {},
                    }
                }
            },
        }
        self.state = next_state;
    }

    fn _display(&mut self) {
        let mut menu = SimpleMenu::<3, 20>::new(10f32, 10f32, 10, "Action Menu");
        menu.add_entry("Save to file");
        self.cursor = menu.move_cursor(self.cursor);
        menu.draw();
    }
}

#[link_section = "data"]
#[no_mangle]
pub static mut ACTION_MENU: ActionMenu = ActionMenu {
    state:  ActionMenuState::Off,
    cursor: 0,
};

impl ActionMenu {
    pub fn enable() {
        unsafe { ACTION_MENU.state = ActionMenuState::Main };
    }
    // returns true if in off state
    pub fn input() -> bool {
        unsafe {
            ACTION_MENU._input();
            return ACTION_MENU.state == ActionMenuState::Off;
        }
    }

    pub fn display() {
        unsafe { ACTION_MENU._display() };
    }
}
