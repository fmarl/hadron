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

use core::arch::asm;
use core::mem::size_of;

use spin::Once;

use super::segmentation::{
    CodeSegment, Descriptor, Segment32, SegmentSelector, TaskStateSegment, load_tss,
};
use super::structures::{memory::VirtualAddress, table::DescriptorTablePointer};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const STACK_SIZE: usize = 4096 * 5;

#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

/// Only ever used by the CPU, through the TSS.
static mut DOUBLE_FAULT_STACK: Stack = Stack([0; STACK_SIZE]);

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<Gdt> = Once::new();

/// The kernel GDT together with the selectors of its entries.
struct Gdt {
    table: GlobalDescriptorTable,
    selectors: Selectors,
}

#[derive(Debug, Clone, Copy)]
pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub tss: SegmentSelector,
}

fn tss() -> &'static TaskStateSegment {
    TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();

        // The stack grows down, so the IST entry points at its end
        let stack_start = VirtualAddress::from_ptr(&raw const DOUBLE_FAULT_STACK);
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_start + STACK_SIZE;

        tss
    })
}

/// Loads the kernel GDT and reloads CS and the task register.
pub fn init() {
    let gdt = GDT.call_once(|| {
        let mut table = GlobalDescriptorTable::new();
        let kernel_code = table.add(Descriptor::kernel_code_segment());
        let tss = table.add(Descriptor::tss_segment(tss()));

        Gdt {
            table,
            selectors: Selectors { kernel_code, tss },
        }
    });

    gdt.table.load();

    #[cfg(debug_assertions)]
    kprintln!(
        "GDT initialized.\nCS: {}\nTSS: {}",
        gdt.selectors.kernel_code.bits(),
        gdt.selectors.tss.bits()
    );

    // SAFETY: Both selectors were created by adding their descriptors to the GDT that was
    // just loaded, and the TSS is 'static.
    unsafe {
        CodeSegment::set_reg(gdt.selectors.kernel_code);
        load_tss(gdt.selectors.tss);
    }
}

/// The selectors of the kernel GDT.
///
/// # Panics
/// If called before `init`.
pub fn selectors() -> &'static Selectors {
    &GDT.get().expect("GDT is not initialized").selectors
}

pub struct GlobalDescriptorTable {
    table: [u64; 8],
    len: usize,
}

impl Default for GlobalDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self {
            table: [0; 8],
            len: 1, // entry 0 must be NULL
        }
    }

    #[inline]
    pub fn as_raw_slice(&self) -> &[u64] {
        &self.table[..self.len]
    }

    /// Creates a GDT from raw descriptor values.
    ///
    /// # Safety
    /// The values must be valid descriptors, as `load` makes the CPU use them.
    ///
    /// # Panics
    /// If `slice` has more than 8 entries.
    #[inline]
    pub const unsafe fn from_raw_slice(slice: &[u64]) -> Self {
        let len = slice.len();
        let mut table = [0; 8];
        let mut i = 0;

        assert!(len <= 8, "A GDT cannot be longer than 8 elements.");

        while i < len {
            table[i] = slice[i];
            i += 1;
        }

        Self { table, len }
    }

    #[inline]
    const fn push(&mut self, value: u64) -> usize {
        let i = self.len;

        self.table[i] = value;
        self.len += 1;

        i
    }

    /// Adds a descriptor and returns its selector.
    ///
    /// # Panics
    /// If the GDT is full.
    #[inline]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the GDT has at most 8 entries"
    )]
    pub fn add(&mut self, entry: Descriptor) -> SegmentSelector {
        let i = match entry {
            Descriptor::SystemSegment(low, high) => {
                assert!(
                    self.len + 2 <= self.table.len(),
                    "Not enough space in GDT for holding a SystemSegment."
                );

                let i = self.push(low);
                self.push(high);

                i
            }

            Descriptor::UserSegment(value) => {
                assert!(
                    self.len < self.table.len(),
                    "Not enough space in GDT for holding a UserSegment."
                );

                self.push(value)
            }
        };

        SegmentSelector::new(i as u16, entry.dpl())
    }

    /// Loads this GDT. Segment registers keep their old values until they are reloaded.
    #[inline]
    pub fn load(&'static self) {
        // SAFETY: The table lives forever and only holds descriptors created by `add` or
        // promised to be valid by the caller of `from_raw_slice`.
        unsafe {
            asm!(
                "lgdt [{}]",
                in(reg) &self.as_descriptor_table_pointer(),
                options(readonly, nostack, preserves_flags)
            );
        }
    }

    #[inline]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the GDT has at most 8 entries"
    )]
    fn as_descriptor_table_pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: VirtualAddress::from_ptr(self.table.as_ptr()),
            limit: (self.len * size_of::<u64>() - 1) as u16,
        }
    }
}
