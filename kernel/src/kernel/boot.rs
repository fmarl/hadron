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

pub const BOOT_INFO_MAGIC: u64 = 0x4841_4452_4F4E_0001; // "HADRON" + version

/// Handed over by Stage 2, see `boot_info` in boot/stage2.asm.
#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub memory_map_ptr: u64,
    pub memory_map_count: u64,
    pub reserved: [u64; 5],
}

impl BootInfo {
    pub const fn is_valid(&self) -> bool {
        self.magic == BOOT_INFO_MAGIC
    }
}

/// E820 memory types
#[expect(dead_code, reason = "the memory map is not parsed yet")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    BootloaderReclaimable = 6,
    KernelAndModules = 7,
}

#[expect(dead_code, reason = "the memory map is not parsed yet")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMapEntry {
    pub base: u64,
    pub length: u64,
    pub mem_type: MemoryType,
    pub reserved: u32,
}

/// Bring up the kernel. Called once from `_start` with the boot info from Stage 2.
pub fn init_kernel(boot_info: &BootInfo) -> ! {
    crate::plat::init_console();

    kprintln!("Copyright (C) 2025 Florian Marrero Liestmann\n");
    kprintln!("Booting hadron...");

    if boot_info.is_valid() {
        kprintln!("Boot info valid");
    } else {
        kprintln!("WARNING: Invalid boot info");
    }

    crate::arch::init_cpu();

    #[cfg(feature = "selftest")]
    {
        kprintln!("\nTesting exception handling...");
        kprintln!("Triggering breakpoint exception (INT 3)...");
        crate::arch::instructions::int3();
        kprintln!("Breakpoint exception handled successfully!\n");
    }

    kprintln!("Kernel initialization complete.");
    kprintln!("System ready.\n");

    #[cfg(debug_assertions)]
    kprint!("Reached halt()");

    crate::arch::halt()
}
