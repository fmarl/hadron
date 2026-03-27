; Hadron Stage 2 Bootloader
; Loaded by Syndicate at 0x0100:0x0000 (0x1000)
; Transitions to Long Mode and loads the Hadron kernel

[BITS 16]
[ORG 0x1000]

; Entry point from Syndicate Stage 1
stage2_entry:
    cli

    ; Save drive number if passed
    mov [boot_drive], dl

    ; Print Stage 2 message
    mov si, msg_stage2
    call print_string_rm

    ; Enable A20 line (in case Syndicate didn't)
    call enable_a20

    ; Get memory map before leaving Real Mode
    call get_memory_map

    ; Check for Long Mode support
    call check_long_mode

    ; Set up page tables for Long Mode
    call setup_page_tables

    ; Load GDT
    lgdt [gdt_descriptor]

    ; Enter Protected Mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; Far jump to 32-bit code
    jmp 0x08:protected_mode_entry

;-----------------------------------------------------------
; 16-bit Real Mode Functions
;-----------------------------------------------------------

; Print string in Real Mode
; SI = pointer to null-terminated string
print_string_rm:
    pusha
.loop:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp .loop
.done:
    popa
    ret

; Enable A20 line
enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

; Get memory map using INT 15h E820h
; Memory map stored at 0x8000
get_memory_map:
    pusha
    mov di, 0x8000          ; Destination for memory map
    xor ebx, ebx            ; EBX = 0 to start
    xor bp, bp              ; Entry count
    mov edx, 0x534D4150     ; 'SMAP' signature

.loop:
    mov eax, 0xE820         ; INT 15h E820h function
    mov ecx, 24             ; Request 24 bytes
    int 0x15

    jc .done                ; Carry set = end or error

    cmp eax, 0x534D4150     ; Must return 'SMAP'
    jne .done

    cmp ecx, 0              ; Zero length entry?
    je .skip

    inc bp                  ; Increment entry count
    add di, 24              ; Next entry (24 bytes)

.skip:
    test ebx, ebx           ; EBX = 0 means last entry
    jz .done
    jmp .loop

.done:
    mov [memory_map_entries], bp
    popa
    ret

; Check for Long Mode support
check_long_mode:
    ; Check for CPUID support
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    xor eax, ecx
    jz .no_long_mode

    ; Check for extended CPUID
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    ; Check for Long Mode
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret

.no_long_mode:
    mov si, msg_no_long_mode
    call print_string_rm
    hlt
    jmp $

; Setup identity-mapped page tables
setup_page_tables:
    ; Clear page table area (extended for 8MB)
    mov edi, 0x70000
    mov ecx, 0x9000 / 4  ; Increased from 0x5000
    xor eax, eax
    rep stosd

    ; PML4T[0] -> PDPT
    mov edi, 0x70000
    mov dword [edi], 0x71003  ; Present, RW

    ; PDPT[0] -> PDT
    mov edi, 0x71000
    mov dword [edi], 0x72003

    ; PDT: Map 4 x 2MB = 8MB (using 2MB pages for simplicity)
    mov edi, 0x72000
    mov eax, 0x83  ; Present, RW, Page Size (2MB pages)
    mov ecx, 4     ; 4 entries = 8MB
.map_pdt:
    stosd
    mov dword [edi], 0  ; High 32 bits
    add edi, 4
    add eax, 0x200000   ; Next 2MB
    loop .map_pdt

    ; Also map higher half (for kernel at 0xFFFFFFFF80000000)
    ; PML4T[511] -> PDPT
    mov edi, 0x70000 + 511 * 8
    mov dword [edi], 0x74003

    ; PDPT[510] -> PDT (reuse same PDT for higher half)
    mov edi, 0x74000 + 510 * 8
    mov dword [edi], 0x72003  ; Reuse identity-mapped PDT

    ret

;-----------------------------------------------------------
; 32-bit Protected Mode
;-----------------------------------------------------------
[BITS 32]
protected_mode_entry:
    ; Set up segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x7C00

    ; Print message
    mov esi, msg_protected
    call print_string_pm

    ; Enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Load PML4
    mov eax, 0x70000
    mov cr3, eax

    ; Enable Long Mode
    mov ecx, 0xC0000080  ; EFER MSR
    rdmsr
    or eax, 1 << 8       ; Set LM bit
    wrmsr

    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; Jump to 64-bit code
    jmp 0x18:long_mode_entry

; Print string in Protected Mode
; ESI = pointer to null-terminated string
print_string_pm:
    pusha
    mov edi, [vga_cursor_pm]
    mov ah, 0x0F
.loop:
    lodsb
    test al, al
    jz .done
    cmp al, 0x0A           ; Newline?
    je .newline
    stosw
    jmp .loop
.newline:
    ; Move to next line
    mov eax, edi
    sub eax, 0xB8000
    shr eax, 1             ; Divide by 2 (each char = 2 bytes)
    mov ebx, 80
    xor edx, edx
    div ebx                ; EAX = row, EDX = col
    inc eax                ; Next row
    mov edx, 80
    mul edx                ; EAX = row * 80
    shl eax, 1             ; Multiply by 2
    add eax, 0xB8000
    mov edi, eax
    jmp .loop
.done:
    mov [vga_cursor_pm], edi
    popa
    ret

;-----------------------------------------------------------
; 64-bit Long Mode
;-----------------------------------------------------------
[BITS 64]
DEFAULT REL
long_mode_entry:
    ; Clear segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up stack
    mov rsp, 0x7C00

    ; Print success message
    mov rsi, msg_long_mode
    call print_string_64

    ; Load and parse ELF kernel
    mov rsi, msg_load_kernel
    call print_string_64

    call load_elf_kernel

    ; Jump to kernel
    mov rsi, msg_jump_kernel
    call print_string_64

    ; Update boot info with memory map count
    movzx rax, word [memory_map_entries]
    mov [boot_info + 16], rax  ; memory_map_count offset

    ; Set up boot info pointer in RDI
    mov rdi, boot_info

    ; Jump to kernel entry point
    mov rax, [kernel_entry]
    call rax

    ; If we return, halt
    cli
    hlt
    jmp $

; Load ELF kernel from 0x10000
load_elf_kernel:
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi

    ; ELF is at 0x10000 (loaded by Syndicate)
    mov rbx, 0x10000

    ; Check ELF magic (0x7F 'E' 'L' 'F')
    cmp dword [rbx], 0x464C457F
    jne .invalid_elf

    ; Check ELF class (must be 64-bit)
    cmp byte [rbx + 0x04], 2
    jne .invalid_elf

    ; Check architecture (must be x86-64)
    cmp word [rbx + 0x12], 0x3E
    jne .invalid_elf

    ; Get entry point (offset 0x18 in ELF64 header)
    mov rax, [rbx + 0x18]

    ; Validate entry point (must be in higher half)
    mov rcx, 0xFFFFFFFF80000000
    cmp rax, rcx
    jb .invalid_elf

    mov [kernel_entry], rax

    ; Get program header offset (offset 0x20)
    mov rdi, [rbx + 0x20]
    add rdi, rbx  ; Make absolute

    ; Get program header count (offset 0x38, 16-bit)
    movzx rcx, word [rbx + 0x38]

    ; Get program header size (offset 0x36, 16-bit)
    movzx rdx, word [rbx + 0x36]

.load_segments:
    ; Check if this is PT_LOAD (type = 1)
    cmp dword [rdi], 1
    jne .next_segment

    ; Get segment info
    mov rsi, [rdi + 0x08]  ; p_offset
    add rsi, rbx           ; Source address
    mov r8,  [rdi + 0x10]  ; p_vaddr (destination)
    mov r9,  [rdi + 0x20]  ; p_filesz
    mov r10, [rdi + 0x28]  ; p_memsz

    ; Bounds check: don't load beyond 16MB
    mov rax, r8
    add rax, r10
    cmp rax, 0x1000000     ; 16MB limit
    ja .next_segment       ; Skip if too large

    ; Copy file data (p_filesz bytes)
    push rcx
    push rdi
    mov rdi, r8
    mov rcx, r9
    test rcx, rcx
    jz .zero_bss           ; Skip copy if filesz = 0
    rep movsb

.zero_bss:
    ; Zero BSS (p_memsz - p_filesz bytes)
    mov rcx, r10
    sub rcx, r9            ; memsz - filesz = BSS size
    test rcx, rcx
    jz .done_segment       ; No BSS to zero
    xor al, al
    rep stosb

.done_segment:
    pop rdi
    pop rcx

.next_segment:
    add rdi, rdx  ; Next program header
    dec rcx
    jnz .load_segments

    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    ret

.invalid_elf:
    mov rsi, msg_invalid_elf
    call print_string_64
    cli
    hlt
    jmp $

; Print string in Long Mode
; RSI = pointer to null-terminated string
print_string_64:
    push rax
    push rbx
    push rcx
    push rdx
    push rdi
    mov rdi, [vga_cursor_64]
    mov ah, 0x0F
.loop:
    lodsb
    test al, al
    jz .done
    cmp al, 0x0A           ; Newline?
    je .newline
    stosw
    jmp .loop
.newline:
    ; Move to next line
    mov rax, rdi
    sub rax, 0xB8000
    shr rax, 1             ; Divide by 2
    mov rbx, 80
    xor rdx, rdx
    div rbx                ; RAX = row, RDX = col
    inc rax                ; Next row
    mov rdx, 80
    mul rdx                ; RAX = row * 80
    shl rax, 1
    add rax, 0xB8000
    mov rdi, rax
    jmp .loop
.done:
    mov [vga_cursor_64], rdi
    pop rdi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    ret

;-----------------------------------------------------------
; Data
;-----------------------------------------------------------
boot_drive: db 0

msg_stage2:         db 'Hadron Stage 2 Bootloader', 0xD, 0xA, 0
msg_no_long_mode:   db 'ERROR: Long Mode not supported', 0xD, 0xA, 0
msg_protected:      db 'Protected Mode OK', 0xA, 0
msg_long_mode:      db 'Long Mode OK', 0xA, 0
msg_load_kernel:    db 'Parsing ELF...', 0xA, 0
msg_jump_kernel:    db 'Jumping to kernel...', 0xA, 0
msg_invalid_elf:    db 'ERROR: Invalid ELF kernel', 0xA, 0

kernel_entry:       dq 0
vga_cursor_pm:      dd 0xB8000  ; Protected Mode VGA cursor
vga_cursor_64:      dq 0xB8000  ; Long Mode VGA cursor

; Boot info structure
; Must match BootInfo structure in kernel/src/boot.rs
align 8
boot_info:
    dq 0x484144524F4E0001  ; Magic: "HADRON" + version 1
    dq 0x8000               ; memory_map_ptr (physical address)
    dq 0                    ; memory_map_count (filled by get_memory_map)
    dq 0, 0, 0, 0, 0        ; reserved[5]

memory_map_entries: dw 0

;-----------------------------------------------------------
; GDT (Global Descriptor Table)
;-----------------------------------------------------------
align 8
gdt_start:
    ; Null descriptor
    dq 0

    ; Code segment (32-bit)
    dw 0xFFFF       ; Limit
    dw 0            ; Base (low)
    db 0            ; Base (mid)
    db 10011010b    ; Access: present, ring 0, code, exec/read
    db 11001111b    ; Flags: 4KB granularity, 32-bit
    db 0            ; Base (high)

    ; Data segment (32-bit)
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b    ; Access: present, ring 0, data, read/write
    db 11001111b
    db 0

    ; Code segment (64-bit)
    dw 0xFFFF
    dw 0
    db 0
    db 10011010b
    db 10101111b    ; Flags: 4KB granularity, 64-bit
    db 0

    ; Data segment (64-bit)
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b
    db 10101111b
    db 0

gdt_descriptor:
    dw $ - gdt_start - 1
    dd gdt_start

; Pad to make it a reasonable size for Syndicate to load
times 4096-($-$$) db 0
