use super::simple_menu::SimpleMenu;
use crate::menus::main_menu::MainMenu;
use crate::system::button::*;
use crate::system::heap::*;
use crate::system::text_print::write_to_screen;

use cstr::cstr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum HeapMenuState {
    Off,
    Main,
    Sub,
}

pub struct HeapMenu {
    state:  HeapMenuState,
    cursor: u32,
}

impl HeapMenu {
    fn _input(&mut self) {
        let mut next_state = self.state;

        match self.state {
            HeapMenuState::Off => {},
            HeapMenuState::Main => {
                if is_pressed(B) {
                    next_state = HeapMenuState::Off;
                } else if is_pressed(A) {
                    match self.cursor {
                        0 | 1 => {
                            next_state = HeapMenuState::Sub;
                        },
                        _ => {},
                    }
                }
            },
            HeapMenuState::Sub => {
                if is_pressed(B) {
                    next_state = HeapMenuState::Main;
                }
            },
        }
        self.state = next_state;
    }

    fn _display(&mut self) {
        match self.state {
            HeapMenuState::Off => {},
            HeapMenuState::Main => {
                let mut menu = SimpleMenu::<3, 20>::new(10, 10, 10, "Heap Menu");
                menu.add_entry("Root Heap MEM1");
                menu.add_entry("Root Heap MEM2");
                self.cursor = menu.move_cursor(self.cursor);
                menu.draw();
            },
            HeapMenuState::Sub => {
                let heap_name = unsafe {
                    match self.cursor {
                        0 => (*get_root_heap_mem1()).get_name(),
                        1 => (*get_root_heap_mem2()).get_name(),
                        _ => cstr!(""),
                    }
                };
                write_to_screen(
                    format_args!("Heap Name: {:<20}", heap_name.to_str().unwrap()),
                    0,
                    0,
                );
            },
        }
    }
}

#[link_section = "data"]
#[no_mangle]
pub static mut HEAP_MENU: HeapMenu = HeapMenu {
    state:  HeapMenuState::Off,
    cursor: 0,
};

impl HeapMenu {
    pub fn enable() {
        unsafe { HEAP_MENU.state = HeapMenuState::Main };
    }
    // returns true if in off state
    pub fn input() -> bool {
        unsafe {
            HEAP_MENU._input();
            return HEAP_MENU.state == HeapMenuState::Off;
        }
    }

    pub fn display() {
        unsafe { HEAP_MENU._display() };
    }
}
