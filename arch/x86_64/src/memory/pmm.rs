/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
 * Copyright (c) 2023-2025 Florian Marrero Liestmann.
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

//! Physical Memory Manager (PMM)
//!
//! Manages allocation and deallocation of physical memory pages.
//! Uses a bitmap allocator for tracking free/used pages.

use x86_64_hal::structures::memory::PhysicalAddress;

/// Size of a physical page (4 KiB)
pub const PAGE_SIZE: usize = 4096;

/// Physical Memory Manager
pub struct PhysicalMemoryManager {
    // TODO: Implement bitmap allocator
    _placeholder: u8,
}

impl PhysicalMemoryManager {
    /// Create a new PMM (placeholder)
    pub const fn new() -> Self {
        Self { _placeholder: 0 }
    }

    /// Allocate a physical page
    /// Returns None if no memory available
    pub fn allocate_page(&mut self) -> Option<PhysicalAddress> {
        // TODO: Implement allocation
        None
    }

    /// Free a physical page
    pub fn free_page(&mut self, _addr: PhysicalAddress) {
        // TODO: Implement deallocation
    }
}
