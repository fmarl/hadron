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
    // SAFETY: Interrupts only run handlers from the loaded IDT. No nomem: this acts as a
    // compiler barrier for the critical section.
    unsafe {
        asm!("sti", options(nostack));
    }
}

/// Disables interrupts.
#[inline]
pub fn disable() {
    // SAFETY: Masking interrupts cannot break memory safety. No nomem: this acts as a
    // compiler barrier for the critical section.
    unsafe {
        asm!("cli", options(nostack));
    }
}

/// Triggers a breakpoint exception (INT 3).
#[inline]
pub fn int3() {
    // SAFETY: Runs the breakpoint handler of the loaded IDT. No nomem: the handler may
    // access memory.
    unsafe {
        asm!("int3", options(nostack));
    }
}

/// Waits for the next interrupt.
#[inline]
pub fn hlt() {
    // SAFETY: Halting until the next interrupt has no side effects.
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}
