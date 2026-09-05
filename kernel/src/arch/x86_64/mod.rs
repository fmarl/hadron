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

//! `x86_64` architecture support.

// The hardware definitions are kept complete, even where the kernel does not use them yet
#![allow(dead_code)]
// Talking to the CPU needs unsafe; the generic kernel code must not use it
#![allow(unsafe_code)]

pub mod gdt;
pub mod idt;
pub mod instructions;
pub mod registers;
pub mod structures;

mod privileges;
mod segmentation;

/// Set up the CPU: segmentation, TSS and the exception vectors.
pub fn init_cpu() {
    gdt::init();
    idt::init();
}

pub fn halt() -> ! {
    instructions::disable();
    loop {
        instructions::hlt();
    }
}
