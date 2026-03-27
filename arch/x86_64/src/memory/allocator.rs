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

//! Heap Allocator
//!
//! Provides dynamic memory allocation for the kernel.
//! Implements GlobalAlloc trait for use with Rust's alloc crate.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Simple bump allocator (placeholder)
pub struct BumpAllocator {
    _placeholder: u8,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub const fn new() -> Self {
        Self { _placeholder: 0 }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // TODO: Implement allocation
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // TODO: Implement deallocation
    }
}

/// Global allocator instance
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();
