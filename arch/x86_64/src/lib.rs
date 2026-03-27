/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
 * Copyright (c) 2023 Florian Marrero Liestmann.
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
#![feature(abi_x86_interrupt)]

// Core modules
pub mod gdt;
pub mod idt;
mod privileges;
mod segmentation;

// Organized module structure
pub mod interrupts;
pub mod memory;
pub mod io;

// Re-export commonly used functions
pub use x86_64_hal::op::interrupts::int3;