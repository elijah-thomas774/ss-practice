use crate::system::button::*;
use crate::system::heap::*;
use crate::utils::char_writer::write_to_screen;
use crate::utils::menu::SimpleMenu;

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

#[no_mangle]
#[link_section = "data"]
static mut HEAP_MENU: HeapMenu = HeapMenu {
    state:  HeapMenuState::Off,
    cursor: 0,
};

impl super::Menu for HeapMenu {
    fn enable() {
        let heap_menu = unsafe { &mut HEAP_MENU };
        heap_menu.state = HeapMenuState::Main;
    }

    fn disable() {
        let heap_menu = unsafe { &mut HEAP_MENU };
        heap_menu.state = HeapMenuState::Off;
    }
    fn input() {
        let heap_menu = unsafe { &mut HEAP_MENU };

        match heap_menu.state {
            HeapMenuState::Off => {},
            HeapMenuState::Main => {
                if is_pressed(B) {
                    heap_menu.state = HeapMenuState::Off;
                } else if is_pressed(A) {
                    match heap_menu.cursor {
                        0 | 1 => {
                            heap_menu.state = HeapMenuState::Sub;
                        },
                        _ => {},
                    }
                }
            },
            HeapMenuState::Sub => {
                if is_pressed(B) {
                    heap_menu.state = HeapMenuState::Main;
                }
            },
        }
    }
    fn display() {
        let heap_menu = unsafe { &mut HEAP_MENU };

        match heap_menu.state {
            HeapMenuState::Off => {},
            HeapMenuState::Main => {
                let mut menu: SimpleMenu<3> = SimpleMenu::new();
                menu.set_heading("Heap Menu");
                menu.set_cursor(heap_menu.cursor);
                menu.add_entry("Root Heap MEM1");
                menu.add_entry("Root Heap MEM2");
                menu.draw();
                heap_menu.cursor = menu.move_cursor();
            },
            HeapMenuState::Sub => {
                let heap_name = unsafe {
                    match heap_menu.cursor {
                        0 => (*get_root_heap_mem1()).get_name(),
                        1 => (*get_root_heap_mem2()).get_name(),
                        _ => cstr!(""),
                    }
                };
                write_to_screen(
                    format_args!("Heap Name: {:<20}", heap_name.to_str().unwrap()),
                    0f32,
                    0f32,
                );
            },
        }
    }

    fn is_active() -> bool {
        let heap_menu = unsafe { &mut HEAP_MENU };
        heap_menu.state != HeapMenuState::Off
    }
}
