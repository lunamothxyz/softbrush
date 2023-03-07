#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

pub mod font;
pub mod dbgui;

#[derive(Debug, Clone)]
pub struct BitmapARGB32 {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl BitmapARGB32 {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            // set A to 255 and R, G, B to 0
            data: vec![0xFF000000; width * height],
        }
    }

    /// Sets the pixel at the given coordinates to the given color
    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.data[x + y * self.width] = color.to_argb32();
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        Color::from_argb32(self.data[x + y * self.width])
    }

    /// Blit the given buffer onto this buffer at the given coordinates
    pub fn blit(&mut self, x: u16, y: u16, src: &BitmapARGB32) {
        for y2 in 0..src.height {
            for x2 in 0..src.width {
                let color = src.data[x2 + y2 * src.width];
                self.data[(x + x2 as u16) as usize + (y + y2 as u16) as usize * self.width] = color;
            }
        }
    }

    /// Blit a region of the given buffer onto this buffer at the given coordinates
    pub fn blit_region(
        &mut self,
        x: u16,
        y: u16,
        src: &BitmapARGB32,
        src_x: usize,
        src_y: usize,
        src_width: usize,
        src_height: usize,
    ) {
        for y2 in 0..src_height {
            for x2 in 0..src_width {
                let color = src.data[(src_x + x2) + (src_y + y2) * src.width];
                self.data[(x + x2 as u16) as usize + (y + y2 as u16) as usize * self.width] = color;
            }
        }
    }

    /// Returns a buffer scaled by the given factor
    pub fn to_scaled(&mut self, factor: usize) -> BitmapARGB32 {
        let mut new_buf = BitmapARGB32::new(self.width * factor, self.height * factor);

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.data[x + y * self.width];
                for y2 in 0..factor {
                    for x2 in 0..factor {
                        new_buf.data[(x * factor + x2) + (y * factor + y2) * new_buf.width] = color;
                    }
                }
            }
        }

        new_buf
    }
}

pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub fn to_argb32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    pub fn from_argb32(argb32: u32) -> Self {
        Self {
            a: (argb32 >> 24) as u8,
            r: (argb32 >> 16) as u8,
            g: (argb32 >> 8) as u8,
            b: argb32 as u8,
        }
    }
}