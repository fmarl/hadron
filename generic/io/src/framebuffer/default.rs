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

use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use limine::{Framebuffer, NonNullPtr};
use spin::Mutex;

use crate::framebuffer::font::{FONT, FONT_DIMENSIONS};
use crate::framebuffer::init;

lazy_static! {
    pub static ref WRITER: Mutex<FramebufferWriter> = Mutex::new(FramebufferWriter::new());
}

pub enum Colors {
    Red,
    Green,
    White,
    Black,
}

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub struct FramebufferWriter {
    framebuffer: &'static NonNullPtr<Framebuffer>,
    row: u64,
    col: u64,
    pub fg: Pixel,
    pub bg: Pixel,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn set(&mut self, to: impl Into<Pixel>) {
        let to: Pixel = to.into();
        self.r = to.r;
        self.g = to.g;
        self.b = to.b;
    }

    pub fn reset(&mut self) {
        self.r = 255;
        self.g = 255;
        self.b = 255;
    }

    fn as_bits(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl From<Colors> for Pixel {
    fn from(c: Colors) -> Self {
        match c {
            Colors::Red => Self::new(255, 0, 0),
            Colors::Green => Self::new(0, 255, 0),
            Colors::White => Self::new(255, 255, 255),
            Colors::Black => Self::new(0, 0, 0),
        }
    }
}

impl FramebufferWriter {
    pub fn new() -> Self {
        Self {
            framebuffer: init(),
            row: 0,
            col: 0,
            fg: Pixel::from(Colors::White),
            bg: Pixel::from(Colors::Black),
        }
    }

    fn write(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            '\t' => self.tab(),
            _ => {
                let offset = (char as u8 - 32) as usize * 16;

                for y in 0..16 {
                    for x in 0..8 {
                        let cx = self.col as usize + (8 - x);
                        let cy = self.row as usize + y;

                        let ptr_offset = (cx * (self.framebuffer.bpp / 8) as usize
                            + cy * self.framebuffer.pitch as usize)
                            as usize;

                        if FONT[y + offset as usize] >> x & 1 == 1 {
                            unsafe {
                                *(self
                                    .framebuffer
                                    .address
                                    .as_ptr()
                                    .unwrap()
                                    .offset(ptr_offset as isize)
                                    as *mut u32) = self.fg.as_bits();
                            }
                        } else {
                            unsafe {
                                *(self
                                    .framebuffer
                                    .address
                                    .as_ptr()
                                    .unwrap()
                                    .offset(ptr_offset as isize)
                                    as *mut u32) = self.bg.as_bits();
                            }
                        }
                    }
                }

                self.check_clear_row();
            }
        }
    }

    #[inline]
    fn new_line(&mut self) {
        self.row += FONT_DIMENSIONS.1 as u64;
        self.col = 0;
    }

    #[inline]
    fn tab(&mut self) {
        for _ in 0..12 {
            let _ = self.write_char(' ');
        }
    }

    #[inline]
    fn check_clear_row(&mut self) {
        if self.col == self.framebuffer.pitch {
            let _ = self.write_char('\n');
        } else {
            self.col += FONT_DIMENSIONS.0 as u64;
        }

        if self.row == self.framebuffer.height {
            let top_row_bytes = self.framebuffer.pitch as usize * FONT_DIMENSIONS.1 as usize;

            for offset in 0..top_row_bytes {
                unsafe { *(self.framebuffer.address.as_ptr().unwrap().add(offset) as *mut u32) = 0 }
            }

            self.row -= 1;
        }
    }
}

impl Write for FramebufferWriter {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(for char in s.chars() {
            self.write(char);
        })
    }
}
