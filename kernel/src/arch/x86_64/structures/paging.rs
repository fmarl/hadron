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

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PageFaultErrorCode: u64 {
        const CAUSED_BY_PROTECTION_VIOLATION = 1;
        const CAUSED_BY_WRITE = 1 << 1;
        const CAUSED_BY_USER_MODE = 1 << 2;
        const CAUSED_BY_MALFORMED_TABLE = 1 << 3;
        const CAUSED_BY_INSTRUCTION_FETCH = 1 << 4;
        const CAUSED_BY_PROTECTION_KEY = 1 << 5;
        const CAUSED_BY_SHADOW_STACK = 1 << 6;
        const CAUSED_BY_HLAT_PAGING = 1 << 7;
        const CAUSED_BY_SGX = 1 << 15;
    }
}
