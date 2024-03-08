use core::fmt::Write;

use crate::system::button::*;
use crate::system::heap::*;
use crate::utils::char_writer::write_to_screen;
use crate::utils::console::Console;
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
                let heap = unsafe {
                    match heap_menu.cursor {
                        0 => get_root_heap_mem1().as_ref(),
                        1 => get_root_heap_mem2().as_ref(),
                        _ => get_root_heap_mem1().as_ref(),
                    }
                    .unwrap()
                };
                let heap_name = heap.get_name();
                let (size, free) = (heap.get_total_size(), heap.get_free_size());
                let List::<Heap> { count, .. } = heap.children;

                let mut console = Console::with_pos_and_size(0f32, 0f32, 120f32, 85f32);
                console.set_bg_color(0x0000007F);
                console.set_font_color(0xFFFFFFFF);
                console.set_font_size(0.25f32);
                console.set_dynamic_size(true);
                let _ = console.write_fmt(format_args!(
                    "Heap Name: {:<20}\n Size: {size}\n Free: {free}\nNum Children: {count}\n",
                    heap_name
                ));

                for i in 0..count {
                    let child = heap.children.get_idx(i);
                    if let Some(child) = child {
                        let _ = console.write_fmt(format_args!(
                            "{i}: {:6.2}% of ({:>8}) {:<20}\n",
                            (child.get_free_size() as f32) * 100.0f32
                                / (child.get_total_size() as f32),
                            child.get_total_size(),
                            child.get_name(),
                        ));
                    }
                }
                console.draw();
            },
        }
    }

    fn is_active() -> bool {
        let heap_menu = unsafe { &mut HEAP_MENU };
        heap_menu.state != HeapMenuState::Off
    }
}
