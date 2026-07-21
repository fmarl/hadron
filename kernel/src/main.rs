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
#![no_main]
#![feature(abi_x86_interrupt)]
#![warn(clippy::undocumented_unsafe_blocks)]

// Declared first, so its console macros are visible in all other modules
#[macro_use]
mod machine;

mod arch;
mod kernel;
mod plat;

use kernel::boot::BootInfo;

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start(boot_info: *const BootInfo) -> ! {
    // SAFETY: Stage 2 passes a pointer to its BootInfo, which stays mapped; the magic is
    // validated in init_kernel.
    kernel::boot::init_kernel(unsafe { &*boot_info })
}
