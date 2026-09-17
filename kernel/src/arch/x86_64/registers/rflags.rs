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
use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[repr(transparent)]
    pub struct RFlags: u64 {
        const ID = 1 << 21;
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        const VIRTUAL_INTERRUPT = 1 << 19;
        const ALIGNMENT_CHECK = 1 << 18;
        const VIRTUAL_8086_MODE = 1 << 17;
        const RESUME_FLAG = 1 << 16;
        const NESTED_TASK = 1 << 14;
        const IOPL_HIGH = 1 << 13;
        const IOPL_LOW = 1 << 12;
        const OVERFLOW_FLAG = 1 << 11;
        const DIRECTION_FLAG = 1 << 10;
        const INTERRUPT_FLAG = 1 << 9;
        const TRAP_FLAG = 1 << 8;
        const SIGN_FLAG = 1 << 7;
        const ZERO_FLAG = 1 << 6;
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        const PARITY_FLAG = 1 << 2;
        const CARRY_FLAG = 1;
    }
}

#[inline]
pub fn read() -> RFlags {
    RFlags::from_bits_retain(read_raw())
}

/// Writes RFLAGS, keeping the reserved bits.
///
/// # Safety
/// Flags like IF, TF or IOPL change how the CPU behaves. That must not break memory safety.
#[inline]
pub unsafe fn write(flags: RFlags) {
    let old_flags = read_raw();
    let reserved = old_flags & !(RFlags::all().bits());
    let new_flags = reserved | flags.bits();

    // SAFETY: upheld by the caller.
    unsafe {
        asm!(
            "push {}; popfq",
            in(reg) new_flags,
            // No nomem/preserves_flags: popfq changes RFLAGS (incl. IF), and memory
            // accesses must not be moved across a change of the interrupt flag.
        );
    }
}

#[inline]
fn read_raw() -> u64 {
    let result: u64;

    // SAFETY: only reads RFLAGS.
    unsafe {
        asm!("pushfq; pop {}", out(reg) result, options(preserves_flags));
    }

    result
}
