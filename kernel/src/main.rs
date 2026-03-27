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

mod boot;
mod cores;

use generic_exception::hcf;
use generic_io::{kprint, kprintln};

#[unsafe(no_mangle)]
unsafe extern "C" fn _start(boot_info_ptr: *const boot::BootInfo) -> ! {
    #[cfg(target_arch = "x86_64")]
    _start_x86_64(boot_info_ptr)
}

fn _start_x86_64(boot_info_ptr: *const boot::BootInfo) -> ! {
    // Clear screen first
    generic_io::WRITER.lock().clear_screen();

    kprintln!("Copyright (C) 2025 Florian Marrero Liestmann\n");
    kprintln!("Booting hadron...");

    // Validate boot info
    let boot_info = unsafe { &*boot_info_ptr };
    if boot_info.is_valid() {
        kprintln!("Boot info valid");
    } else {
        kprintln!("WARNING: Invalid boot info");
    }

    kprintln!("Setting up GDT: ");
    arch_x86_64::gdt::init();

    kprintln!("Setting up IDT: ");
    arch_x86_64::idt::init();

    // Test exception handling (only in debug builds)
    #[cfg(debug_assertions)]
    {
        kprintln!("\nTesting exception handling...");
        kprintln!("Triggering breakpoint exception (INT 3)...");
        arch_x86_64::int3();
        kprintln!("Breakpoint exception handled successfully!\n");
    }

    kprintln!("Kernel initialization complete.");
    kprintln!("System ready.\n");

    #[cfg(debug_assertions)]
    kprint!("Reached hcf()");

    hcf()
}
