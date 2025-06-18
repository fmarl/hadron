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

use super::privileges::PLevel;
use bit_field::BitField;
use bitflags::bitflags;
use core::{arch::asm, mem::size_of};
use x86_64::{registers::Msr, structures::memory::VirtualAddress};

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

pub struct CodeSegment;

impl Segment32 for CodeSegment {
    fn get_reg() -> SegmentSelector {
        let result: u16;

        unsafe {
            asm!(
                "mov {0:x}, cs",
                out(reg) result,
                options(nomem, nostack, preserves_flags)
            );
        }

        SegmentSelector(result)
    }

    fn set_reg(sel: SegmentSelector) {
        unsafe {
            asm!(
                "push {sel}",
                "lea {tmp}, [1f + rip]",
                "push {tmp}",
                "retfq",
                "2:",
                sel = in(reg) u64::from(sel.0),
                tmp = lateout(reg) _,
                options(preserves_flags),
            );
        }
    }
}

pub struct StackSegment;

pub struct DataSegment;

pub struct FSegment;

pub struct GSegment;

impl SegmentSelector {
    pub const NULL: Self = Self::new(0, PLevel::Ring0);

    #[inline]
    pub const fn new(index: u16, privilege_level: PLevel) -> Self {
        Self(index << 3 | (privilege_level as u16))
    }

    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    // Requested Privilege Level ()
    #[inline]
    pub fn privilege_level(self) -> PLevel {
        PLevel::from_u16(self.0.get_bits(0..2))
    }

    /// Set the privilege level for this Segment selector.
    #[inline]
    pub fn set_privilege_level(&mut self, privilege_level: PLevel) {
        self.0.set_bits(0..2, privilege_level as u16);
    }
}

bitflags! {
    /// Flags for a GDT descriptor. Not all flags are valid for all descriptor types.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct DescriptorFlags: u64 {
        const ACCESSED          = 1 << 40;
        const WRITABLE          = 1 << 41;
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const DPL_RING_3        = 3 << 45;
        const PRESENT           = 1 << 47;
        const AVAILABLE         = 1 << 52;
        const LONG_MODE         = 1 << 53;
        const DEFAULT_SIZE      = 1 << 54;
        const GRANULARITY       = 1 << 55;
        const LIMIT_0_15        = 0xFFFF;
        const LIMIT_16_19       = 0xF << 48;
        const BASE_0_23         = 0xFF_FFFF << 16;
        const BASE_24_31        = 0xFF << 56;
    }
}

impl DescriptorFlags {
    const COMMON: Self = Self::from_bits_truncate(
        Self::USER_SEGMENT.bits()
            | Self::PRESENT.bits()
            | Self::WRITABLE.bits()
            | Self::ACCESSED.bits()
            | Self::LIMIT_0_15.bits()
            | Self::LIMIT_16_19.bits()
            | Self::GRANULARITY.bits(),
    );

    pub const KERNEL_DATA: Self =
        Self::from_bits_truncate(Self::COMMON.bits() | Self::DEFAULT_SIZE.bits());

    pub const KERNEL_CODE32: Self = Self::from_bits_truncate(
        Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::DEFAULT_SIZE.bits(),
    );

    pub const KERNEL_CODE64: Self = Self::from_bits_truncate(
        Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::LONG_MODE.bits(),
    );

    pub const USER_DATA: Self =
        Self::from_bits_truncate(Self::KERNEL_DATA.bits() | Self::DPL_RING_3.bits());

    pub const USER_CODE32: Self =
        Self::from_bits_truncate(Self::KERNEL_CODE32.bits() | Self::DPL_RING_3.bits());

    pub const USER_CODE64: Self =
        Self::from_bits_truncate(Self::KERNEL_CODE64.bits() | Self::DPL_RING_3.bits());
}

impl Descriptor {
    #[inline]
    pub const fn dpl(self) -> PLevel {
        let value_low = match self {
            Descriptor::UserSegment(v) => v,
            Descriptor::SystemSegment(v, _) => v,
        };
        let dpl = (value_low & DescriptorFlags::DPL_RING_3.bits()) >> 45;

        PLevel::from_u16(dpl as u16)
    }

    #[inline]
    pub const fn kernel_code_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::KERNEL_CODE64.bits())
    }

    #[inline]
    pub const fn kernel_data_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::KERNEL_DATA.bits())
    }

    #[inline]
    pub const fn user_data_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::USER_DATA.bits())
    }

    /// Creates a segment descriptor for a 64-bit ring 3 code segment. Suitable
    /// for use with `sysret` or `sysexit`.
    #[inline]
    pub const fn user_code_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::USER_CODE64.bits())
    }

    #[inline]
    pub fn tss_segment(tss: &'static TaskStateSegment) -> Descriptor {
        // SAFETY: The pointer is derived from a &'static reference, which ensures its validity.
        unsafe { Self::tss_segment_unchecked(tss) }
    }

    #[inline]
    pub unsafe fn tss_segment_unchecked(tss: *const TaskStateSegment) -> Descriptor {
        use self::DescriptorFlags as Flags;

        let ptr = tss as u64;

        let mut low = Flags::PRESENT.bits();
        // base
        low.set_bits(16..40, ptr.get_bits(0..24));
        low.set_bits(56..64, ptr.get_bits(24..32));
        // limit (the `-1` in needed since the bound is inclusive)
        low.set_bits(0..16, (size_of::<TaskStateSegment>() - 1) as u64);
        // type (0b1001 = available 64-bit tss)
        low.set_bits(40..44, 0b1001);

        let mut high = 0;
        high.set_bits(0..32, ptr.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }
}

#[repr(C, packed(4))]
pub struct TaskStateSegment {
    r1: u32,
    privilege_stack_table: [VirtualAddress; 3],
    r2: u64,
    pub interrupt_stack_table: [VirtualAddress; 7],
    r3: u64,
    r4: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub fn new() -> Self {
        Self {
            r1: 0,
            privilege_stack_table: [VirtualAddress::default(); 3],
            r2: 0,
            interrupt_stack_table: [VirtualAddress::default(); 7],
            r3: 0,
            r4: 0,
            iomap_base: size_of::<TaskStateSegment>() as u16,
        }
    }
}

pub trait Segment32 {
    fn get_reg() -> SegmentSelector;
    fn set_reg(sel: SegmentSelector);
}

pub trait Segment64: Segment32 {
    const BASE: Msr;
    fn read_base() -> VirtualAddress;
    fn write_base(base: VirtualAddress);
}
