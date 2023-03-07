use log::error;

use super::BitmapARGB32;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct Font {
    pub bitmap: BitmapARGB32,
    pub glyph_width: u16,
    pub glyph_height: u16,
    pub map: HashMap<char, u16>,
}

/// Prints text to a bitmap at the given coordinates
pub fn print(x: usize, y: usize, font: &Font, dest: &mut BitmapARGB32,txt: &str) {
    let glyphs_per_row = font.bitmap.width / font.glyph_width as usize;
    let mut print_pos_x = x;
    let mut print_pos_y = y;
    for c in txt.chars() {
        if c == '\n' {
            print_pos_x = x;
            print_pos_y += font.glyph_height as usize;
            continue;
        } else if let Some(glyph_index) = font.map.get(&c) {
            let glyph_x = font.glyph_width * (glyph_index % glyphs_per_row as u16);
            let glyph_y = font.glyph_height * (glyph_index / glyphs_per_row as u16);
            dest.blit_region(
                print_pos_x as u16,
                print_pos_y as u16,
                &font.bitmap,
                glyph_x as usize,
                glyph_y as usize,
                font.glyph_width as usize,
                font.glyph_height as usize,
            );
            print_pos_x += font.glyph_width as usize;
        } else {
            error!("Character {} not found in font", c)
        }
    }
}