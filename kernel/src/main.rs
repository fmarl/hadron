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

mod cores;

use generic_exception::hcf;
use generic_io::{kprint, kprintln};

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "x86_64")]
    _start_x86_64()
}

fn _start_x86_64() -> ! {
    kprintln!("Copyright (C) 2025 Florian Marrero Liestmann\n");
    kprintln!("Booting hadron...");

    kprintln!("Setting up GDT: ");
    arch_x86_64::gdt::init();

    kprintln!("Setting up IDT: ");
    arch_x86_64::idt::init();

    #[cfg(debug_assertions)]
    kprint!("Reached hcf()");

    hcf()
}
