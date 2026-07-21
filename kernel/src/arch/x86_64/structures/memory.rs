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

use core::fmt;
use core::ops::Add;

/// A canonical 64-bit virtual address: bits 48..64 are copies of bit 47.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    /// Creates a virtual address.
    ///
    /// # Panics
    /// If `addr` is not canonical.
    #[inline]
    pub const fn new(addr: u64) -> Self {
        match Self::try_new(addr) {
            Some(addr) => addr,
            None => panic!("virtual address is not canonical"),
        }
    }

    /// Creates a virtual address, or `None` if `addr` is not canonical.
    #[inline]
    pub const fn try_new(addr: u64) -> Option<Self> {
        let truncated = Self::new_truncate(addr);
        if truncated.0 == addr {
            Some(truncated)
        } else {
            None
        }
    }

    /// Creates a virtual address by sign extending bit 47.
    #[inline]
    pub const fn new_truncate(addr: u64) -> Self {
        Self(((addr << 16).cast_signed() >> 16).cast_unsigned())
    }

    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as u64)
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Add<u64> for VirtualAddress {
    type Output = Self;

    /// # Panics
    /// On overflow or if the result is not canonical.
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.0.checked_add(rhs).expect("virtual address overflow"))
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as u64
    }
}

/// A physical address. `x86_64` supports at most 52 physical address bits.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    const MAX: u64 = (1 << 52) - 1;

    /// Creates a physical address.
    ///
    /// # Panics
    /// If `addr` has bits above bit 51 set.
    #[inline]
    pub const fn new(addr: u64) -> Self {
        assert!(addr <= Self::MAX, "physical address exceeds 52 bits");
        Self(addr)
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Add<u64> for PhysicalAddress {
    type Output = Self;

    /// # Panics
    /// On overflow or if the result exceeds 52 bits.
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.0.checked_add(rhs).expect("physical address overflow"))
    }
}
