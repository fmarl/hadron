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

/// Represents a protection ring level.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
#[repr(u8)]
pub enum PLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl PLevel {
    #[inline]
    pub const fn from_u16(value: u16) -> PLevel {
        match value {
            0 => PLevel::Ring0,
            1 => PLevel::Ring1,
            2 => PLevel::Ring2,
            3 => PLevel::Ring3,
            _ => core::panic!("invalid PLevel"),
        }
    }
}
