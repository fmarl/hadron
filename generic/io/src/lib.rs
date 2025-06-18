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
#![no_std]
#![feature(allow_internal_unstable)]

#[macro_use]
pub mod framebuffer;

#[allow_internal_unstable(print_internals, format_args_nl)]
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::io::framebuffer::_kprint(format_args!($($arg)*)));
}

#[allow_internal_unstable(print_internals, format_args_nl)]
#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => ({
    $crate::io::framebuffer::_kprint(format_args_nl!($($arg)*));
    })
}
