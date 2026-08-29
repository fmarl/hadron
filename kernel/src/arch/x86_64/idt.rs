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

use super::registers::control::Cr2;
use super::structures::paging::PageFaultErrorCode;
use super::structures::{memory::VirtualAddress, table::DescriptorTablePointer};
use core::arch::asm;
use core::fmt;
use core::marker::PhantomData;
use spin::Once;

fn report(name: &str, stack_frame: &InterruptStackFrame, details: Option<fmt::Arguments>) {
    ekprintln!("\n:: EXCEPTION: {name} ::");
    if let Some(details) = details {
        ekprintln!("{details}");
    }
    ekprintln!("{stack_frame:#?}");
}

fn fatal(name: &str, stack_frame: &InterruptStackFrame, details: Option<fmt::Arguments>) -> ! {
    report(name, stack_frame, details);
    super::halt()
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    fatal("DIVIDE BY ZERO", &stack_frame, None);
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    report("DEBUG", &stack_frame, None);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    report("BREAKPOINT", &stack_frame, None);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    fatal("INVALID OPCODE", &stack_frame, None);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    fatal(
        "DOUBLE FAULT",
        &stack_frame,
        Some(format_args!("Error Code: {error_code:#x}")),
    );
}

extern "x86-interrupt" fn general_protection_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    let table = if error_code & 0x2 != 0 {
        "IDT"
    } else if error_code & 0x4 != 0 {
        "LDT"
    } else {
        "GDT"
    };
    fatal(
        "GENERAL PROTECTION FAULT",
        &stack_frame,
        Some(format_args!(
            "Error Code: {error_code:#x} ({table} index {:#x}, external: {})",
            error_code >> 3,
            error_code & 0x1 != 0
        )),
    );
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    fatal(
        "PAGE FAULT",
        &stack_frame,
        Some(format_args!(
            "Accessed Address: {:?}\n{error_code:?}",
            Cr2::read()
        )),
    );
}

extern "x86-interrupt" fn stack_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    fatal(
        "STACK FAULT",
        &stack_frame,
        Some(format_args!("Error Code: {error_code:#x}")),
    );
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    fatal(
        "SEGMENT NOT PRESENT",
        &stack_frame,
        Some(format_args!("Error Code: {error_code:#x}")),
    );
}

pub type InterruptHandlerFunction = extern "x86-interrupt" fn(InterruptStackFrame);
pub type InterruptHandlerFunctionWithErrorCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type DivergingInterruptHandlerFunctionWithErrorCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;
pub type PageFaultInterruptHandlerFunction =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

static IDT: Once<InterruptDescriptorTable> = Once::new();

fn build_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();

    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
    idt.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    idt.general_protection
        .set_handler_fn(general_protection_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.stack_fault.set_handler_fn(stack_fault_handler);
    idt.segment_not_present
        .set_handler_fn(segment_not_present_handler);

    idt
}

/// The interrupt descriptor table for the CPU exceptions.
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    /// Interrupt 0x0: Fault
    pub divide_error: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x1: Trap or Fault
    pub debug: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x2: Not applicable
    pub non_maskable_interrupt: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x3: Trap
    pub breakpoint: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x4: Trap
    pub overflow: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x5: Fault
    pub bound_range_exceeded: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x6: Fault
    pub invalid_opcode: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x7: Fault
    pub device_not_available: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x8: Abort
    pub double_fault: InterruptDescriptorTableEntry<DivergingInterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0x9: Reserved (Coprocessor Segment Overrun on older CPUs)
    reserved_0x09: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0xA: Fault
    pub invalid_tss: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xB: Fault
    pub segment_not_present: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xC: Fault
    pub stack_fault: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xD: Fault
    pub general_protection: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xE: Fault
    pub page_fault: InterruptDescriptorTableEntry<PageFaultInterruptHandlerFunction>,

    /// Interrupt 0xF: Reserved
    reserved_0x0f: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x10: Fault
    pub x87_fpu_floating_point: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x11: Fault
    pub alignment_check: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0x12: Abort
    pub machine_check: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x13: Fault
    pub simd_floating_point: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x14: Fault
    pub virtualization: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x15: Fault
    pub control_protection: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,
}

/// The stack frame the CPU pushes on an interrupt.
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: VirtualAddress,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: VirtualAddress,
    pub stack_segment: u64,
}

impl fmt::Debug for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InterruptStackFrame")
            .field("instruction_pointer", &self.instruction_pointer)
            .field("code_segment", &self.code_segment)
            .field("cpu_flags", &format_args!("{:#x}", self.cpu_flags))
            .field("stack_pointer", &self.stack_pointer)
            .field("stack_segment", &self.stack_segment)
            .finish()
    }
}
#[repr(C)]
pub struct InterruptDescriptorTableEntry<T> {
    low_ptr: u16,
    gdt_selector: u16,
    options: InterruptDescriptorTableEntryOptions,
    middle_ptr: u16,
    high_ptr: u32,
    reserved: u32,
    phantom: PhantomData<T>,
}

/// Loads the kernel IDT. Requires the GDT, as the gates use its code segment.
pub fn init() {
    IDT.call_once(build_idt).load();

    #[cfg(debug_assertions)]
    kprintln!("IDT initialized");
}

#[repr(transparent)]
pub struct InterruptDescriptorTableEntryOptions(u16);

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            divide_error: InterruptDescriptorTableEntry::null(),
            debug: InterruptDescriptorTableEntry::null(),
            non_maskable_interrupt: InterruptDescriptorTableEntry::null(),
            breakpoint: InterruptDescriptorTableEntry::null(),
            overflow: InterruptDescriptorTableEntry::null(),
            bound_range_exceeded: InterruptDescriptorTableEntry::null(),
            invalid_opcode: InterruptDescriptorTableEntry::null(),
            device_not_available: InterruptDescriptorTableEntry::null(),
            double_fault: InterruptDescriptorTableEntry::null(),
            reserved_0x09: InterruptDescriptorTableEntry::null(),
            invalid_tss: InterruptDescriptorTableEntry::null(),
            segment_not_present: InterruptDescriptorTableEntry::null(),
            stack_fault: InterruptDescriptorTableEntry::null(),
            general_protection: InterruptDescriptorTableEntry::null(),
            page_fault: InterruptDescriptorTableEntry::null(),
            reserved_0x0f: InterruptDescriptorTableEntry::null(),
            x87_fpu_floating_point: InterruptDescriptorTableEntry::null(),
            alignment_check: InterruptDescriptorTableEntry::null(),
            machine_check: InterruptDescriptorTableEntry::null(),
            simd_floating_point: InterruptDescriptorTableEntry::null(),
            virtualization: InterruptDescriptorTableEntry::null(),
            control_protection: InterruptDescriptorTableEntry::null(),
        }
    }

    #[inline]
    pub const fn reset(&mut self) {
        *self = Self::new();
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "the IDT has 22 entries of 16 bytes"
    )]
    fn as_descriptor_table_pointer(&self) -> DescriptorTablePointer {
        use core::mem::size_of;

        DescriptorTablePointer {
            base: VirtualAddress::from_ptr(self),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }

    /// Loads this IDT.
    #[inline]
    pub fn load(&'static self) {
        // SAFETY: the table is 'static, and set_handler_fn only installs matching handlers.
        unsafe {
            asm!(
                "lidt [{}]",
                in(reg) &self.as_descriptor_table_pointer(),
                options(readonly, nostack, preserves_flags)
            );
        }
    }
}

/// A handler function that fits an IDT entry of type `Self`.
pub trait HandlerFn {
    fn addr(self) -> u64;
}

macro_rules! impl_handler_fn {
    ($($ty:ty),*) => {
        $(
            impl HandlerFn for $ty {
                #[inline]
                fn addr(self) -> u64 {
                    self as usize as u64
                }
            }
        )*
    };
}

impl_handler_fn!(
    InterruptHandlerFunction,
    InterruptHandlerFunctionWithErrorCode,
    DivergingInterruptHandlerFunctionWithErrorCode,
    PageFaultInterruptHandlerFunction
);

impl<T> InterruptDescriptorTableEntry<T> {
    #[inline]
    pub const fn null() -> Self {
        Self {
            low_ptr: 0,
            gdt_selector: 0,
            options: InterruptDescriptorTableEntryOptions::new(),
            middle_ptr: 0,
            high_ptr: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

impl<T: HandlerFn> InterruptDescriptorTableEntry<T> {
    /// Points this entry at `handler` and marks it present.
    ///
    /// # Panics
    /// If the GDT is not initialized yet.
    #[inline]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "splits the handler address into the gate fields"
    )]
    pub fn set_handler_fn(&mut self, handler: T) -> &mut InterruptDescriptorTableEntryOptions {
        let addr = handler.addr();

        self.low_ptr = addr as u16;
        self.middle_ptr = (addr >> 16) as u16;
        self.high_ptr = (addr >> 32) as u32;

        self.gdt_selector = super::gdt::selectors().kernel_code.bits();
        self.options.set_present(true);

        &mut self.options
    }
}

impl Default for InterruptDescriptorTableEntryOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl InterruptDescriptorTableEntryOptions {
    /// A not present 64-bit interrupt gate.
    #[inline]
    pub const fn new() -> Self {
        Self(0b1110_0000_0000)
    }

    #[inline]
    pub const fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            self.0 |= 1 << 15;
        } else {
            self.0 &= !(1 << 15);
        }
        self
    }

    #[inline]
    pub const fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        if disable {
            self.0 &= !(1 << 8); // Clear interrupt gate bit
        } else {
            self.0 |= 1 << 8;
        }
        self
    }

    /// `index` is the index into the TSS interrupt stack table. The hardware IST field
    /// is 1-based (0 means "no stack switch"), so `index + 1` is stored.
    ///
    /// # Panics
    /// If `index` is 7 or larger.
    #[inline]
    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        assert!(index < 7, "The interrupt stack table has only 7 entries.");
        self.0 = (self.0 & 0xFFF8) | (index + 1);
        self
    }
}
