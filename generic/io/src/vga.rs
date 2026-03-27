/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
 * Copyright (c) 2025 Florian Marrero Liestmann.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use core::fmt;
use core::ptr::{read_volatile, write_volatile};
use lazy_static::lazy_static;
use spin::Mutex;

const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: *mut Buffer,
}

unsafe impl Send for Writer {}

impl Writer {
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::LightGreen, Color::Black),
            buffer: VGA_BUFFER as *mut Buffer,
        }
    }

    fn write_char_at(&mut self, row: usize, col: usize, screen_char: ScreenChar) {
        unsafe {
            let ptr = &mut (*self.buffer).chars[row][col] as *mut ScreenChar;
            write_volatile(ptr, screen_char);
        }
    }

    fn read_char_at(&self, row: usize, col: usize) -> ScreenChar {
        unsafe {
            let ptr = &(*self.buffer).chars[row][col] as *const ScreenChar;
            read_volatile(ptr)
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }

                let row = VGA_HEIGHT - 1;
                let col = self.column_position;

                self.write_char_at(row, col, ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let character = self.read_char_at(row, col);
                self.write_char_at(row - 1, col, character);
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..VGA_WIDTH {
            self.write_char_at(row, col, blank);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub fn _kprint(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
