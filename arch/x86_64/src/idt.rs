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

use core::arch::asm;
use core::marker::PhantomData;
use lazy_static::lazy_static;
use x86_64::structures::{memory::VirtualAddress, table::DescriptorTablePointer};
use x86_64::types::paging::PageFaultErrorCode;

extern "x86-interrupt" fn default_handler(stack_frame: InterruptStackFrame) {
    kprintln!(
        ":: KERNEL PANIC ::\nCalled default_handler. Stacktrace:\n{:#?}",
        stack_frame
    );
}

pub type InterruptHandlerFunction = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type InterruptHandlerFunctionWithErrorCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultInterruptHandlerFunction =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt
    };
}

/// Table to handle interrupts and exceptions. Affects x86 and x86_64.
/// See: https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html
/// Intel® 64 and IA-32 Architectures Software Developer’s Manual Combined Volumes: 1, 2A, 2B, 2C, 2D, 3A, 3B, 3C, 3D, and 4
/// Page 3220 - 3254
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    /// Interrupt 0x0: Fault
    /// Indicates the divisor operand for a DIV or IDIV instruction is 0 or that the result cannot be represented in the
    /// number of bits specified for the destination operand.
    ///
    /// No error code is pushed on the stack.
    pub divide_error: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x1: Trap or Fault, dependens on the contents of DR6 and other debug registers. (See page 3221)
    /// Indicates that one or more of several debug-exception conditions has been detected. Whether the exception is a
    /// fault or a trap depends on the condition (see Table 6-3). See Chapter 18, “Debug, Branch Profile, TSC, and Intel®
    /// Resource Director Technology (Intel® RDT) Features,” for detailed information about the debug exceptions.
    ///
    /// No error code is pushed on the stack. Please see the debug registers.
    pub debug: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x2: Not applicable
    /// The nonmaskable interrupt (NMI) is generated externally by asserting the processor’s NMI pin or through an NMI
    /// request set by the I/O APIC to the local APIC. This interrupt causes the NMI interrupt handler to be called.
    pub non_maskable_interrupt: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x3: Trap
    /// A breakpoint (INT 3) can be used by the operating system or a debugging system to arrest normal execution.
    /// For more information consult page 3224 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual. (Combined volumes)
    ///
    /// No error code is pushed on the stack.
    pub breakpoint: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x4: Trap
    /// Indicates that an overflow trap occurred when an INTO instruction was executed. The INTO instruction checks the
    /// state of the OF flag in the EFLAGS register. If the OF flag is set, an overflow trap is generated.
    /// For more information consult page 3225 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual. (Combined volumes)
    ///
    /// No error code is pushed on the stack.
    pub overflow: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x5: Fault
    /// Indicates that a BOUND-range-exceeded fault occurred when a BOUND instruction was executed. The BOUND
    /// instruction checks that a signed array index is within the upper and lower bounds of an array located in memory. If
    /// the array index is not within the bounds of the array, a BOUND-range-exceeded fault is generated.
    ///
    /// No error code is pushed on the stack.
    pub bound_range_exceeded: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x6: Fault
    /// This fault occurs when an invalid opcode is detected by the execution unit.
    /// As there are many circumstances that can cause this fault, please consult page 3227 of
    /// the Intel® 64 and IA-32 Architectures Software Developer’s Manual Combined Volumes: 1, 2A, 2B, 2C, 2D, 3A, 3B, 3C, 3D, and 4
    ///
    /// No error code is pushed on the stack.
    pub invalid_opcode: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x7: Fault
    /// This exceptions happends when:
    /// - The processor executed an x87 FPU floating-point instruction while the EM flag in control register CR0 was set.
    /// - The processor executed a WAIT/FWAIT instruction while the MP and TS flags of register CR0 were set,
    ///   regardless of the setting of the EM flag.
    /// - The processor executed an x87 FPU, MMX, or SSE/SSE2/SSE3 instruction (with the exception of MOVNTI,
    ///   PAUSE, PREFETCHh, SFENCE, LFENCE, MFENCE, and CLFLUSH) while the TS flag in control register CR0 was set
    ///   and the EM flag is clear
    /// If it is unclear what all this means, have a look at page 3228.
    ///
    /// No error code is pushed on the stack.
    pub device_not_available: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x8: Abort
    /// Indicates that the processor detected a second exception while calling an exception handler for a prior exception.
    /// Normally, when the processor detects another exception while trying to call an exception handler, the two exceptions
    /// can be handled serially. If, however, the processor cannot handle them serially, it signals the double-fault
    /// exception.
    ///
    /// The error code 0x0 is pushed on the stack of the double-fault handler.
    pub double_fault: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    // Interrupt 0x9: Reserved
    //
    /// Interrupt 0xA: Fault
    /// Indicates that there was an error related to a TSS. Such an error might be detected during a task switch or during
    /// the execution of instructions that use information from a TSS. Table 6-6 shows the conditions that cause an invalid
    /// TSS exception to be generated.
    /// See page 3232, Table 6-6 for invalid TSS conditions.
    ///
    /// An error code containing the segment selector index for the segment descriptor that caused the violation is pushed
    /// onto the stack of the exception handler.
    pub invalid_tss: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xB: Fault
    /// Indicates that the present flag of a segment or gate descriptor is clear.
    /// This can happen when:
    /// - attempting to load CS, DS, ES, FS, or GS registers, for example while performing a task switch.
    /// - attempting to load the LDTR using an LLDT instruction.
    /// - executing the LTR instruction and the TSS is marked not present.
    /// - attempting to use a gate descriptor or TSS that is marked segment-not-present, but is otherwise valid.
    ///
    /// An error code containing the segment selector index for the segment descriptor that caused the violation is pushed
    /// onto the stack of the exception handler.
    pub segment_not_present: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xC: Fault
    /// This can happen when:
    /// - A limit violation is detected during an operation that refers to the SS register. Operations that can cause a limit
    ///   violation include stack-oriented instructions such as POP, PUSH, CALL, RET, IRET, ENTER, and LEAVE, as well as
    ///   other memory references which implicitly or explicitly use the SS register (for example, MOV AX, [BP+6] or
    ///   MOV AX, SS:[EAX+6]). The ENTER instruction generates this exception when there is not enough stack space
    ///   for allocating local variables
    /// - A not-present stack segment is detected when attempting to load the SS register. This violation can occur
    ///   during the execution of a task switch, a CALL instruction to a different privilege level, a return to a different
    ///   privilege level, an LSS instruction, or a MOV or POP instruction to the SS register.
    /// - A canonical violation is detected in 64-bit mode during an operation that reference memory using the stack
    ///   pointer register containing a non-canonical memory address.
    /// You should probably extending the limit of the stack segment or loading the missing stack segmet into memory.
    ///
    /// If the exception is caused by a not-present stack segment or by overflow of the new stack during an inter-privilege-
    /// level call, the error code contains a segment selector for the segment that caused the exception. Here, the exception
    /// handler can test the present flag in the segment descriptor pointed to by the segment selector to determine
    /// the cause of the exception. For a normal limit violation (on a stack segment already in use) the error code is set to 0x0.
    pub stack_fault: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xD: Fault
    /// Indicates that the processor detected one of a class of protection violations called general-protection violations.
    /// See page 3237 - 3239 of the
    /// Intel® 64 and IA-32 Architectures Software Developer’s Manual Combined Volumes: 1, 2A, 2B, 2C, 2D, 3A, 3B, 3C, 3D, and 4
    /// for more information. (Look also for General Protection Exception in 64-bit Mode)
    ///
    /// The processor pushes an error code onto the exception handler's stack. If the fault condition was detected while
    /// loading a segment descriptor, the error code contains a segment selector to or IDT vector number for the
    /// descriptor; otherwise, the error code is 0.
    pub general_protection: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0xE: Fault
    /// Indicates that, with paging enabled (the PG flag in the CR0 register is set), the processor detected one of many
    /// conditions while using the page-translation mechanism to translate a linear address to a physical address.
    /// For all conditions, look at page 3240.
    ///
    /// The processor provides the page-fault handler with two items of information to aid in diagosing the exception and recovering from it.
    pub page_fault: InterruptDescriptorTableEntry<PageFaultInterruptHandlerFunction>,

    // Interrupt 0xF: Reserved
    //
    /// Interrupt 0x10: Fault
    /// Indicates that the x87 FPU has detected a floating-point error. The NE flag in the register CR0 must be set for an
    /// interrupt 16 (floating-point error exception) to be generated. (See Section 2.5, “Control Registers,” for a detailed
    /// description of the NE flag.)
    ///
    /// No error code is pushed on the stack.
    pub x87_fpu_floating_point: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x11: Fault
    /// Indicates that the processor detected an unaligned memory operand when alignment checking was enabled.
    /// Alignment checks are only carried out in data (or stack) accesses (not in code fetches or system segment accesses).
    /// An example of an alignment-check violation is a word stored at an odd byte address, or a doubleword stored at an
    /// address that is not an integer multiple of 4. Table 6-7 lists the alignment requirements various data types recog-
    /// nized by the processor.
    ///
    /// The error code is null; all bits are clear except possibly bit 0 — EXT; see Section 6.13. EXT is set if the #AC is
    /// recognized during delivery of an event other than a software interrupt (see “INT n/INTO/INT3/INT1—Call to Inter-
    /// rupt Procedure” in Chapter 3 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2A).
    pub alignment_check: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,

    /// Interrupt 0x12: Abort
    /// Indicates that the processor detected an internal machine error or a bus error, or that an external agent detected
    /// a bus error. See page 3248 for more information.
    ///
    /// No error code is pushed on the stack.
    pub machine_check: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x13: Fault
    /// Indicates the processor has detected an SSE/SSE2/SSE3 SIMD floating-point exception. The appropriate status
    /// flag in the MXCSR register must be set and the particular exception unmasked for this interrupt to be generated.
    /// See page 3249 for more information.
    ///
    /// No error code is pushed on the stack.
    pub simd_floating_point: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x14: Fault
    /// Indicates that the processor detected an EPT violation in VMX non-root operation. Not all EPT violations cause
    /// virtualization exceptions. See Section 26.5.7.2 for details.
    /// The exception handler can recover from EPT violations and restart the program or task without any loss of program
    /// continuity. In some cases, however, the problem that caused the EPT violation may be uncorrectable.
    ///
    /// No error code is pushed on the stack.
    pub virtualization: InterruptDescriptorTableEntry<InterruptHandlerFunction>,

    /// Interrupt 0x15: Fault
    /// Indicates a control flow transfer attempt violated the control flow enforcement technology constraints.
    ///
    /// The processor provides the control protection exception handler with following information
    /// through the error code on the stack.
    pub control_protection: InterruptDescriptorTableEntry<InterruptHandlerFunctionWithErrorCode>,
}

#[repr(transparent)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
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

pub fn init() {
    IDT.init()
}

#[repr(transparent)]
struct InterruptDescriptorTableEntryOptions(u16);

impl core::fmt::Debug for InterruptStackFrame {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl core::fmt::Debug for InterruptStackFrameValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Hex(u64);

        impl core::fmt::Debug for Hex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

#[repr(C)]
struct InterruptStackFrameValue {
    instruction_pointer: VirtualAddress,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: VirtualAddress,
    stack_segment: u64,
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
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
            invalid_tss: InterruptDescriptorTableEntry::null(),
            segment_not_present: InterruptDescriptorTableEntry::null(),
            stack_fault: InterruptDescriptorTableEntry::null(),
            general_protection: InterruptDescriptorTableEntry::null(),
            page_fault: InterruptDescriptorTableEntry::null(),
            x87_fpu_floating_point: InterruptDescriptorTableEntry::null(),
            alignment_check: InterruptDescriptorTableEntry::null(),
            machine_check: InterruptDescriptorTableEntry::null(),
            simd_floating_point: InterruptDescriptorTableEntry::null(),
            virtualization: InterruptDescriptorTableEntry::null(),
            control_protection: InterruptDescriptorTableEntry::null(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn as_descriptor_table_pointer(&self) -> DescriptorTablePointer {
        use core::mem::size_of;

        DescriptorTablePointer {
            base: VirtualAddress::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }

    #[inline]
    pub fn init(&'static self) {
        unsafe {
            asm!(
                "lidt [{}]",
                in(reg) &self.as_descriptor_table_pointer(),
                options(readonly, nostack, preserves_flags)
            );
        }
    }
}

impl<T> InterruptDescriptorTableEntry<T> {
    #[inline]
    pub fn null() -> Self {
        Self {
            low_ptr: 0,
            gdt_selector: 0,
            options: InterruptDescriptorTableEntryOptions::default(),
            middle_ptr: 0,
            high_ptr: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

impl Default for InterruptDescriptorTableEntryOptions {
    fn default() -> Self {
        Self(0b1110_0000_0000)
    }
}
