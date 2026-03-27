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

//! Memory management module
//!
//! This module contains GDT, paging, and memory management subsystems.

// Note: gdt module is at super::gdt, access via crate::gdt

// TODO: Add memory management modules
pub mod allocator;
pub mod pmm;
pub mod vmm;
