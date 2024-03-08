use crate::system::heap::*;
use core::fmt::Write;

use crate::utils::console::Console;

pub fn disp_heaps() {
    let heap = unsafe { get_root_heap_mem1().as_ref().unwrap() };
    let heap_name = heap.get_name();
    let (size, free) = (heap.get_total_size(), heap.get_free_size());
    let List::<Heap> { count, .. } = heap.children;

    let mut console = Console::with_pos_and_size(0f32, 0f32, 120f32, 85f32);
    console.set_bg_color(0x000000CF);
    console.set_font_color(0xFFFFFFFF);
    console.set_font_size(0.2f32);
    console.set_dynamic_size(true);

    fn print_heap_info(console: &mut Console, child: &Heap, i: u16) {
        let total_size = child.get_total_size();
        let free_size = child.get_free_size();
        let used_size = total_size - free_size;
        let _ = console.write_fmt(format_args!(
            " {i}: {:6.2}% ({used_size:>8} /{total_size:>8}) {:<20}\n",
            (used_size as f32) * 100.0f32 / (total_size as f32),
            child.get_name(),
        ));
    }

    let _ = console.write_fmt(format_args!(
        "Heap Name: {:<20}\n Size: {size}\n Free: {free}\nChildren:\n",
        heap_name
    ));
    for i in 0..count {
        let child = heap.children.get_idx(i);
        if let Some(child) = child {
            print_heap_info(&mut console, child, i);
        }
    }

    let heap = unsafe { get_root_heap_mem2().as_ref().unwrap() };
    let heap_name = heap.get_name();
    let (size, free) = (heap.get_total_size(), heap.get_free_size());
    let List::<Heap> { count, .. } = heap.children;
    let _ = console.write_fmt(format_args!(
        "\nHeap Name: {:<20}\n Size: {size}\n Free: {free}\nChildren:\n",
        heap_name
    ));

    for i in 0..count {
        let child = heap.children.get_idx(i);
        if let Some(child) = child {
            print_heap_info(&mut console, child, i);
        }
    }
    console.draw();
}
