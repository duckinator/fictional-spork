use agb::display;
use agb::display::bitmap3::Bitmap3;

use super::font::{CHARS, CHAR_IDX_OFFSET};

const ROWS: i32 = (display::HEIGHT / 8) as i32;
const COLS: i32 = (display::WIDTH / 8) as i32;

pub fn putchar_at(bitmap: &mut Bitmap3, row: i32, col: i32, letter: char) {
    let chr = CHARS[letter as usize - CHAR_IDX_OFFSET];
    let x = col * 8;
    let y = row * 8;
    for xoff in 0..8 {
        for yoff in 0..8 {
            if chr[((yoff * 8) + xoff) as usize] != 0 {
                bitmap.draw_point(x + xoff, y + yoff, 0x001F);
            }
            //let pixel = if chr[((yoff * 8) + xoff) as usize] == 0 { 0x0000 } else { 0x001F };
            //bitmap.draw_point(x as i32 + xoff, y as i32 + yoff, pixel);
        }
    }
}

fn scroll_text(_bitmap: &mut Bitmap3) {
    // TODO: Implement me.
}

pub fn print_at(bitmap: &mut Bitmap3, row: i32, col: i32, text: &str) {
    let mut row = row;
    let mut col = col;
    for letter in text.chars() {
        if letter == '\r' {
            col = 0;
            continue;
        }

        if letter == '\n' || col >= COLS {
            row += 1;
            col = 0;

            // This lets us avoid having 22 empty things in font::CHARS.
            if letter == '\n' {
                continue;
            }
        }

        if row >= ROWS {
            scroll_text(bitmap);
        }

        putchar_at(bitmap, row, col, letter);

        col += 1;
    }
}
