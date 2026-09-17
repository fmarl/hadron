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

/// Enables interrupts.
#[inline]
pub fn enable() {
    // SAFETY: interrupts only enter handlers from the loaded IDT.
    // No nomem, so this also works as a compiler barrier.
    unsafe {
        asm!("sti", options(nostack));
    }
}

/// Disables interrupts.
#[inline]
pub fn disable() {
    // SAFETY: masking interrupts is always fine.
    // No nomem, so this also works as a compiler barrier.
    unsafe {
        asm!("cli", options(nostack));
    }
}

#[inline]
pub fn int3() {
    // SAFETY: enters the breakpoint handler of the loaded IDT.
    // No nomem, the handler touches memory.
    unsafe {
        asm!("int3", options(nostack));
    }
}

#[inline]
pub fn hlt() {
    // SAFETY: only waits for the next interrupt.
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}
