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
use spin::Mutex;

const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Light green on black
const COLOR: u16 = 0x0A;

/// Text mode writer. Fills the screen top to bottom and scrolls once it is full.
pub struct Writer {
    row: usize,
    column: usize,
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    pub const fn new() -> Self {
        Self { row: 0, column: 0 }
    }

    /// A cell holds the character in the low and the color in the high byte. It is
    /// accessed as one `u16`, so the hardware always sees both together.
    fn cell(row: usize, column: usize) -> *mut u16 {
        assert!(row < VGA_HEIGHT && column < VGA_WIDTH);
        (VGA_BUFFER as *mut u16).wrapping_add(row * VGA_WIDTH + column)
    }

    fn write_cell(row: usize, column: usize, value: u16) {
        // SAFETY: cell() keeps us inside the identity mapped text buffer.
        unsafe { write_volatile(Self::cell(row, column), value) };
    }

    fn read_cell(row: usize, column: usize) -> u16 {
        // SAFETY: see write_cell.
        unsafe { read_volatile(Self::cell(row, column)) }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
            return;
        }

        if self.column >= VGA_WIDTH {
            self.new_line();
        }

        Self::write_cell(self.row, self.column, (COLOR << 8) | u16::from(byte));
        self.column += 1;
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
        if self.row + 1 < VGA_HEIGHT {
            self.row += 1;
        } else {
            Self::scroll();
        }
        self.column = 0;
    }

    fn scroll() {
        for row in 1..VGA_HEIGHT {
            for column in 0..VGA_WIDTH {
                Self::write_cell(row - 1, column, Self::read_cell(row, column));
            }
        }
        Self::clear_row(VGA_HEIGHT - 1);
    }

    fn clear_row(row: usize) {
        for column in 0..VGA_WIDTH {
            Self::write_cell(row, column, (COLOR << 8) | u16::from(b' '));
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            Self::clear_row(row);
        }
        self.row = 0;
        self.column = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

/// Print to the screen. With `force_unlock`, a held lock is broken instead of waited for
/// (see `machine::io::emergency_print`).
pub fn print(args: fmt::Arguments, force_unlock: bool) {
    use core::fmt::Write;
    if force_unlock && WRITER.is_locked() {
        // SAFETY: only on fatal paths. The lock holder either never runs again or can
        // live with a garbled line.
        unsafe { WRITER.force_unlock() };
    }
    WRITER.lock().write_fmt(args).unwrap();
}
