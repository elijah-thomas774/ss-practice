use super::action_menu::ActionMenu;
use super::display_menu::DisplayMenu;
use super::heap_menu::HeapMenu;
use super::simple_menu::SimpleMenu;
use super::warp_menu::WarpMenu;
use crate::system::button::*;
use crate::system::text_print::{GXLoadPosMtxImm, GXSetCurrentMtx, Matrix, TextWriterBase};

use wchar::wchz;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MenuState {
    Off,
    MenuSelect,
    DisplayMenu,
    WarpMenu,
    HeapMenu,
    ActionMenu,
}

pub struct MainMenu {
    state:       MenuState,
    main_cursor: u32,
    force_close: bool,
}

#[link_section = "data"]
#[no_mangle]
pub static mut MAIN_MENU: MainMenu = MainMenu {
    state:       MenuState::Off,
    main_cursor: 0,
    force_close: false,
};

#[repr(C)]
struct Screen {
    stuff: u32, // Theres a lot more
}

extern "C" {
    static SCREEN: *mut Screen;
    fn FillBufferGX__Screen(scrn: *mut Screen, _: u32, clr: *const u32);
}

impl MainMenu {
    // returns treu if menu is active
    pub fn disable() {
        unsafe { MAIN_MENU.force_close = true };
        set_buttons_not_pressed(B);
    }

    pub fn display() -> bool {
        if unsafe { MAIN_MENU.state != MenuState::Off } {
            let mut writer = TextWriterBase::new();
            writer.m_char_writer.m_scale = [0.5f32, 0.5f32];
            writer.set_position(10, 420);
            writer.print_symbol(wchz!(u16, "\x20"));
            writer.print(wchz!(u16, "Select\t"));
            writer.print_symbol(wchz!(u16, "\x21"));
            writer.print(wchz!(u16, "Back\t"));
            writer.print_symbol(wchz!(u16, "\x2F\x30"));
            writer.print(wchz!(u16, "Up/Down\t"));
            writer.print_symbol(wchz!(u16, "\x31\x32"));
            writer.print(wchz!(u16, "Change Value"));
        }
        let mut next_menu = unsafe { MAIN_MENU.state };
        match unsafe { MAIN_MENU.state } {
            MenuState::Off => {
                if is_down(DPAD_RIGHT) && is_down(TWO) {
                    next_menu = MenuState::MenuSelect;
                }
            },
            MenuState::MenuSelect => {
                let mut menu = SimpleMenu::<5, 20>::new(10, 10, 10, "Main Menu Select");
                menu.add_entry("Display Menu");
                menu.add_entry("Warp Menu");
                menu.add_entry("Heap Menu");
                menu.add_entry("Action Menu");
                unsafe {
                    MAIN_MENU.main_cursor = menu.move_cursor(MAIN_MENU.main_cursor);
                }
                menu.draw();
                if is_pressed(B) {
                    next_menu = MenuState::Off;
                    set_buttons_not_pressed(B);
                } else if is_pressed(A) {
                    next_menu = match menu.current_line {
                        0 => {
                            DisplayMenu::enable();
                            MenuState::DisplayMenu
                        },
                        1 => {
                            WarpMenu::enable();
                            MenuState::WarpMenu
                        },
                        2 => {
                            HeapMenu::enable();
                            MenuState::HeapMenu
                        },
                        3 => {
                            ActionMenu::enable();
                            MenuState::ActionMenu
                        },
                        _ => next_menu,
                    };
                }
            },
            MenuState::DisplayMenu => {
                DisplayMenu::display();
                if DisplayMenu::input() {
                    next_menu = MenuState::MenuSelect;
                }
            },
            MenuState::WarpMenu => {
                WarpMenu::display();
                if WarpMenu::input() {
                    next_menu = MenuState::MenuSelect;
                }
            },
            MenuState::HeapMenu => {
                HeapMenu::display();
                if HeapMenu::input() {
                    next_menu = MenuState::MenuSelect;
                }
            },
            MenuState::ActionMenu => {
                ActionMenu::display();
                if ActionMenu::input() {
                    next_menu = MenuState::MenuSelect;
                }
            },
        }
        unsafe {
            if MAIN_MENU.force_close {
                MAIN_MENU.force_close = false;
                MAIN_MENU.state = MenuState::Off;
            } else {
                MAIN_MENU.state = next_menu;
            }
        }
        return next_menu != MenuState::Off;
    }
}
