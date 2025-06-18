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

pub mod rflags;

use core::arch::asm;

pub struct Msr(u32);

impl Msr {
    #[inline]
    pub const fn new(reg: u32) -> Msr {
        Msr(reg)
    }

    #[inline]
    pub unsafe fn read(&self) -> u64 {
        let high: u32;
        let low: u32;

        asm!(
            "rdmsr",
            in("ecx") self.0,
            out("eax") low, 
            out("edx") high,
            options(nomem, nostack, preserves_flags),
        );

        ((high as u64) << 32) | (low as u64)
    }

    #[inline]
    pub unsafe fn write(&self, value: u64) {
        let high = (value >> 32) as u32;
        let low = value as u32;

        asm!(
            "wrmsr",
            in("ecx") self.0,
            in("eax") low, 
            in("edx") high,
            options(nostack, preserves_flags),
        );
    }
}
