use crate::system::button::*;
use crate::system::math::Vec3f;
use crate::utils::menu::SimpleMenu;

use cstr::cstr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActionMenuState {
    Off,
    Main,
}

pub struct ActionMenu {
    state:  ActionMenuState,
    cursor: u32,
}

#[no_mangle]
#[link_section = "data"]
static mut ACTION_MENU: ActionMenu = ActionMenu {
    state:  ActionMenuState::Off,
    cursor: 0,
};

struct SavedInfo {
    pos:   Vec3f,
    angle: [u16; 3],
}

impl super::Menu for ActionMenu {
    fn enable() {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state = ActionMenuState::Main;
    }

    fn disable() {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state = ActionMenuState::Off;
    }
    fn input() {
        let action_menu = unsafe { &mut ACTION_MENU };

        match action_menu.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::Off;
                } else if is_pressed(A) {
                    match action_menu.cursor {
                        0 => unsafe {},
                        1 => unsafe {},
                        _ => {},
                    }
                }
            },
        }
    }
    fn display() {
        let action_menu = unsafe { &mut ACTION_MENU };

        match action_menu.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                let mut menu: SimpleMenu<3> = SimpleMenu::new();
                menu.set_heading("Action Menu");
                menu.set_cursor(action_menu.cursor);
                menu.add_entry("Save File to Temp Slot");
                menu.add_entry("Load File from Temp Slot");
                menu.draw();
                action_menu.cursor = menu.move_cursor();
            },
        }
    }

    fn is_active() -> bool {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state != ActionMenuState::Off
    }
}
