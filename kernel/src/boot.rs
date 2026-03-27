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

/// Magic number to identify valid boot info
pub const BOOT_INFO_MAGIC: u64 = 0x4841_4452_4F4E_0001; // "HADRON" + version

/// Boot information passed from Stage 2 bootloader to kernel
#[repr(C)]
pub struct BootInfo {
    /// Magic number for validation
    pub magic: u64,

    /// Pointer to memory map (to be implemented)
    pub memory_map_ptr: u64,

    /// Number of memory map entries
    pub memory_map_count: u64,

    /// Reserved for future use
    pub reserved: [u64; 5],
}

impl BootInfo {
    /// Validate boot info magic number
    pub fn is_valid(&self) -> bool {
        self.magic == BOOT_INFO_MAGIC
    }

    /// Create empty boot info (for bootloader)
    #[allow(dead_code)]
    pub const fn new() -> Self {
        BootInfo {
            magic: BOOT_INFO_MAGIC,
            memory_map_ptr: 0,
            memory_map_count: 0,
            reserved: [0; 5],
        }
    }
}

/// Memory map entry types
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryType {
    /// Usable RAM
    Usable = 1,
    /// Reserved - unusable
    Reserved = 2,
    /// ACPI reclaimable memory
    AcpiReclaimable = 3,
    /// ACPI NVS memory
    AcpiNvs = 4,
    /// Bad memory
    BadMemory = 5,
    /// Bootloader reclaimable
    BootloaderReclaimable = 6,
    /// Kernel and modules
    KernelAndModules = 7,
}

/// Memory map entry
#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMapEntry {
    /// Physical start address
    pub base: u64,
    /// Length in bytes
    pub length: u64,
    /// Memory type
    pub mem_type: MemoryType,
    /// Reserved for alignment
    pub reserved: u32,
}
