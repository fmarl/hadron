# Hadron

Hadron is a minimal microkernel with L4 architecture.

This is still at the beginning of the development.
I am using this project mainly to learn more about os development and L4 microkernels.

## Features
At this moment the kernel doesn't have any features. I'm still in the phase of implementing the minimum features for a working x86_64 kernel.

## Roadmap

### Basic
- [x] GDT
- [ ] IDT
- [ ] SIMD
- [ ] PMM
- [ ] VMM
- [ ] Paging
- [ ] ACPI
- [ ] APIC
- [ ] IOAPIC
- [ ] LAPIC
- [ ] HPET
- [ ] APIC-TIMER
- [ ] SMP
- [ ] Multitasking

### Microkernel
- [ ] Inter-Task-IPC
- [ ] Intra-Task-IPC
- [ ] Servers

### Userspace
- [ ] ZFS Server
- [ ] Syscall compliance with FreeBSD

I'll try to reuse the FreeBSD Userland.

Later I plan to port the kernel to RISC-V.

## Dependencies
If you use Guix, `guix shell -m manifest.scm` provides everything, e.g. `guix shell -m manifest.scm -- make run`.
Else you need to have the following packages installed:
```
rustc (nightly, with rust-src)
nasm
mtools
sfdisk
qemu
git
```
and a C-compiler of your choice.

## Build

Hadron boots via the [Syndicate](https://github.com/fmarl/syndicate) BIOS bootloader,
which is cloned next to this repository. The bootable disk image is built via the included Makefile:
```
make
```
and you can run it in QEMU:
```
make run
```
