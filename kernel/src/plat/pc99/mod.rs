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

//! PC99 platform: the standard PC hardware around an `x86_64` CPU.

// Talking to the hardware needs unsafe; the generic kernel code must not use it
#![allow(unsafe_code)]

pub mod serial;
pub mod vga;

/// Prepare the kernel consoles.
pub fn init_console() {
    serial::init();
    vga::WRITER.lock().clear_screen();
}

/// Write to all kernel consoles: the screen and COM1.
pub fn console_print(args: core::fmt::Arguments, force_unlock: bool) {
    vga::print(args, force_unlock);
    serial::print(args, force_unlock);
}
