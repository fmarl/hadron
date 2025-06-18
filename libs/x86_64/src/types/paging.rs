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
    // See: https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html
    // Intel® 64 and IA-32 Architectures Software Developer’s Manual Combined Volumes: 1, 2A, 2B, 2C, 2D, 3A, 3B, 3C, 3D, and 4
    // Page 3240 - 3241
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        /// P flag (bit 0)
        /// This flag is 0 if there is no translation for the linear address because the P flag was 0 in one of the paging-
        /// structure entries used to translate that address.
        const CAUSED_BY_PROTECTION_VIOLATION = 1;

        /// W/R (bit 1)
        /// If the access causing the page-fault exception was a write, this flag is 1; otherwise, it is 0. This flag
        /// describes the access causing the page-fault exception, not the access rights specified by paging.
        const CAUSED_BY_WRITE = 1 << 1;

        /// U/S (bit 2)
        /// If a user-mode access caused the page-fault exception, this flag is 1; it is 0 if a supervisor-mode access did so.
        /// This flag describes the access causing the page-fault exception, not the access rights specified by paging.
        const CAUSED_BY_USER_MODE = 1 << 2;

        /// RSVD flag (bit 3)
        /// This flag is 1 if there is no translation for the linear address because a reserved bit was set in one of the
        /// paging-structure entries used to translate that address
        const CAUSED_BY_MALFORMED_TABLE = 1 << 3;

        /// I/D flag (bit 4)
        /// This flag is 1 if the access causing the page-fault exception was an instruction fetch. This flag describes the
        /// access causing the page-fault exception, not the access rights specified by paging.
        const CAUSED_BY_INSTRUCTION_FETCH = 1 << 4;

        /// PK flag (bit 5)
        /// This flag is 1 if the access causing the page-fault exception was a data access to a linear address with a
        /// protection key for which the protection-key rights registers disallow access.
        const CAUSED_BY_PROTECTION_KEY = 1 << 5;

        /// SS (bit 6)
        /// If the access causing the page-fault exception was a shadow-stack access (including shadow-stack
        /// accesses in enclave mode), this flag is 1; otherwise, it is 0. This flag describes the access causing the page-
        /// fault exception, not the access rights specified by paging
        const CAUSED_BY_SHADOW_STACK = 1 << 6;

        /// HLAT (bit 7)
        /// This flag is 1 if there is no translation for the linear address using HLAT paging because, in one of the
        /// paging-structure entries used to translate that address, either the P flag was 0 or a reserved bit was set. An
        /// error code will set this flag only if it clears bit 0 or sets bit 3. This flag will not be set by a page fault resulting
        /// from a violation of access rights, nor for one encountered during ordinary paging, including the case in
        /// which there has been a restart of HLAT paging.
        const CAUSED_BY_HLAT_PAGING = 1 << 7;

        /// SGX (bit 15)
        /// This flag is 1 if the exception is unrelated to paging and resulted from violation of SGX-specific access-
        /// control requirements. Because such a violation can occur only if there is no ordinary page fault, this flag is
        /// set only if the P flag (bit 0) is 1 and the RSVD flag (bit 3) and the PK flag (bit 5) are both 0
        const CAUSED_BY_SGX = 1 << 15;
    }
}
