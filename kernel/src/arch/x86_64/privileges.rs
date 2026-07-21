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

/// Represents a protection ring level.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
#[repr(u8)]
pub enum PLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl PLevel {
    /// Creates a privilege level from the lowest two bits of `value`.
    #[inline]
    pub const fn from_bits_truncate(value: u16) -> Self {
        match value & 0b11 {
            0 => Self::Ring0,
            1 => Self::Ring1,
            2 => Self::Ring2,
            _ => Self::Ring3,
        }
    }
}

/// Error for a value that is not a privilege level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidPLevel(pub u16);

impl TryFrom<u16> for PLevel {
    type Error = InvalidPLevel;

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= 3 {
            Ok(Self::from_bits_truncate(value))
        } else {
            Err(InvalidPLevel(value))
        }
    }
}
