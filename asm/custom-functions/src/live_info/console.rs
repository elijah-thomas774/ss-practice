use core::{fmt::Write, ops::Deref};

use crate::system::text_print::{TextWriterBase, WCharWriter};

pub struct Console {
    pos:          [f32; 2],
    size:         [f32; 2],
    font_size:    f32,
    bg_color:     u32,
    font_color:   u32,
    dynamic_size: bool,
    buffer:       WCharWriter<512>,
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.buffer.write_str(s)
    }
}
impl Console {
    pub fn with_pos_and_size(posx: f32, posy: f32, width: f32, height: f32) -> Self {
        Self {
            pos:          [posx, posy],
            size:         [width, height],
            font_size:    0.5f32,
            bg_color:     0x0000003F,
            font_color:   0x000000FF,
            dynamic_size: false,
            buffer:       WCharWriter::<512>::new(),
        }
    }

    pub fn with_pos(posx: f32, posy: f32) -> Self {
        Self {
            pos:          [posx, posy],
            size:         [0.0f32, 0.0f32],
            font_size:    0.5f32,
            bg_color:     0x0000003F,
            font_color:   0x000000FF,
            dynamic_size: true,
            buffer:       WCharWriter::<512>::new(),
        }
    }

    fn resize_bounds(&mut self, font_width: f32, font_height: f32) {
        // Find num Lines (number of \n + 1)
        let mut num_lines = 1;
        let mut longest_line = 0;
        let mut curr_len = 0;
        for c in self.buffer.buf.iter() {
            if *c == 0x000A {
                num_lines += 1;
                if curr_len > longest_line {
                    longest_line = curr_len
                }
                curr_len = 0;
            } else {
                curr_len += 1;
            }
        }
        // If it doesnt end in new line
        if curr_len > longest_line {
            longest_line = curr_len;
        }

        // Set size
        self.size = [
            4f32 + longest_line as f32 * font_width,
            4f32 + num_lines as f32 * font_height,
        ];
    }

    pub fn set_dynamic_size(&mut self, val: bool) {
        self.dynamic_size = val;
    }

    pub fn set_font_color(&mut self, clr: u32) {
        self.font_color = clr;
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }

    pub fn set_bg_color(&mut self, clr: u32) {
        self.bg_color = clr;
    }

    pub fn draw(&mut self) {
        let mut writer = TextWriterBase::new();
        writer.set_font_color([self.font_color; 2]);
        writer.m_char_writer.m_scale = [self.font_size; 2];
        if self.dynamic_size {
            self.resize_bounds(writer.get_font_width(), writer.get_font_height());
        }
        writer.set_fixed_width();
        crate::menus::main_menu::draw_rect(
            self.pos[0],
            self.pos[1],
            self.size[0],
            self.size[1],
            0.0f32,
            self.bg_color,
        );
        writer.set_position(self.pos[0] + 2f32, self.pos[1] + 2f32);
        self.buffer.draw(&mut writer);
    }
}
