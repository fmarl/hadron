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

pub mod control;
pub mod rflags;

use core::arch::asm;

/// A model specific register.
pub struct Msr(u32);

impl Msr {
    #[inline]
    pub const fn new(reg: u32) -> Self {
        Self(reg)
    }

    /// # Safety
    /// The MSR must exist on this CPU, and reading it must not have side effects that
    /// break memory safety.
    #[inline]
    pub unsafe fn read(&self) -> u64 {
        let high: u32;
        let low: u32;

        // SAFETY: Guaranteed by the caller.
        unsafe {
            asm!(
                "rdmsr",
                in("ecx") self.0,
                out("eax") low,
                out("edx") high,
                options(nomem, nostack, preserves_flags),
            );
        }

        (u64::from(high) << 32) | u64::from(low)
    }

    /// # Safety
    /// The MSR must exist on this CPU, and writing `value` must not break memory safety.
    #[inline]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "splits the value into EDX:EAX"
    )]
    pub unsafe fn write(&self, value: u64) {
        let high = (value >> 32) as u32;
        let low = value as u32;

        // SAFETY: Guaranteed by the caller.
        unsafe {
            asm!(
                "wrmsr",
                in("ecx") self.0,
                in("eax") low,
                in("edx") high,
                options(nostack, preserves_flags),
            );
        }
    }
}
