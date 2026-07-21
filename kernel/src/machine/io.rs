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

//! Kernel console output.

use core::fmt;

macro_rules! kprint {
    ($($arg:tt)*) => ($crate::machine::io::print(format_args!($($arg)*)));
}

macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => (kprint!("{}\n", format_args!($($arg)*)));
}

/// Like `kprint!`, but usable from exception handlers and panics (see `emergency_print`).
macro_rules! ekprint {
    ($($arg:tt)*) => ($crate::machine::io::emergency_print(format_args!($($arg)*)));
}

macro_rules! ekprintln {
    () => (ekprint!("\n"));
    ($($arg:tt)*) => (ekprint!("{}\n", format_args!($($arg)*)));
}

pub fn print(args: fmt::Arguments) {
    crate::plat::console_print(args, false);
}

/// Print from exception or panic context. If a console lock is held, the interrupted code
/// was printing; waiting for it would deadlock on a single core, so the lock is broken instead.
pub fn emergency_print(args: fmt::Arguments) {
    crate::plat::console_print(args, true);
}
