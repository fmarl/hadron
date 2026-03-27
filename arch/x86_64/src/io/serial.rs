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

//! Serial Port Driver (COM1)
//!
//! Provides output to serial port for debugging and logging.
//! Useful for QEMU: qemu -serial file:serial.log

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

/// I/O port addresses for COM1
const COM1_PORT: u16 = 0x3F8;

lazy_static! {
    /// Global serial port instance
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = SerialPort::new(COM1_PORT);
        serial.init();
        Mutex::new(serial)
    };
}

/// Serial port driver
pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    /// Create a new serial port instance
    const fn new(port: u16) -> Self {
        Self { port }
    }

    /// Initialize the serial port
    fn init(&mut self) {
        unsafe {
            // Disable interrupts
            outb(self.port + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(self.port + 3, 0x80);
            // Set divisor to 3 (38400 baud)
            outb(self.port + 0, 0x03);
            outb(self.port + 1, 0x00);
            // 8 bits, no parity, one stop bit
            outb(self.port + 3, 0x03);
            // Enable FIFO, clear with 14-byte threshold
            outb(self.port + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(self.port + 4, 0x0B);
        }
    }

    /// Send a byte to the serial port
    fn send(&mut self, byte: u8) {
        unsafe {
            // Wait for transmit buffer to be empty
            while (inb(self.port + 5) & 0x20) == 0 {}
            outb(self.port, byte);
        }
    }

    /// Write a string to the serial port
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.send(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

/// Write to serial port (unsafe I/O port access)
#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

/// Read from serial port (unsafe I/O port access)
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// Print to serial port
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::io::serial::_print(format_args!($($arg)*));
    };
}

/// Print to serial port with newline
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Serial write failed");
}
