/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
 * Copyright (c) 2023-2025 Florian Marrero Liestmann.
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

//! 16550 UART on COM1, polled.

use core::fmt;
use spin::Mutex;

const COM1_PORT: u16 = 0x3F8;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(SerialPort::new(COM1_PORT));

/// Program the UART. Must run before the first `print`.
pub fn init() {
    SERIAL1.lock().init();
}

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    const fn new(port: u16) -> Self {
        Self { port }
    }

    fn init(&mut self) {
        // SAFETY: only programs the COM1 registers.
        unsafe {
            // Disable interrupts
            outb(self.port + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(self.port + 3, 0x80);
            // Set divisor to 3 (38400 baud)
            outb(self.port, 0x03);
            outb(self.port + 1, 0x00);
            // 8 bits, no parity, one stop bit
            outb(self.port + 3, 0x03);
            // Enable FIFO, clear with 14-byte threshold
            outb(self.port + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(self.port + 4, 0x0B);
        }
    }

    fn send(&mut self, byte: u8) {
        // SAFETY: only touches the COM1 status and data registers.
        unsafe {
            // Wait for transmit buffer to be empty
            while (inb(self.port + 5) & 0x20) == 0 {}
            outb(self.port, byte);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.send(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Writes a byte to an I/O port.
///
/// # Safety
/// Port writes can do anything to the device behind the port.
#[inline]
unsafe fn outb(port: u16, value: u8) {
    // SAFETY: upheld by the caller.
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Reads a byte from an I/O port.
///
/// # Safety
/// Port reads can have side effects on the device behind the port.
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    // SAFETY: upheld by the caller.
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

/// Print to COM1. With `force_unlock`, a held lock is broken instead of waited for
/// (see `machine::io::emergency_print`).
pub fn print(args: fmt::Arguments, force_unlock: bool) {
    use core::fmt::Write;
    if force_unlock && SERIAL1.is_locked() {
        // SAFETY: see vga::print.
        unsafe { SERIAL1.force_unlock() };
    }
    SERIAL1.lock().write_fmt(args).unwrap();
}
