# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := hadron

# Convenience macro to reliably declare user overridable variables.
define DEFAULT_VAR =
    ifeq ($(origin $1),default)
        override $(1) := $(2)
    endif
    ifeq ($(origin $1),undefined)
        override $(1) := $(2)
    endif
endef

# Compiler for building the 'limine' executable for the host.
override DEFAULT_HOST_CC := cc
$(eval $(call DEFAULT_VAR,HOST_CC,$(DEFAULT_HOST_CC)))

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: all-syndicate
all-syndicate: $(IMAGE_NAME)-syndicate.img

.PHONY: run-syndicate
run-syndicate: $(IMAGE_NAME)-syndicate.img
	qemu-system-x86_64 -m 2G -drive file=$(IMAGE_NAME)-syndicate.img,format=raw -serial file:serial.log

.PHONY: run-syndicate-debug
run-syndicate-debug: $(IMAGE_NAME)-syndicate.img
	qemu-system-x86_64 -m 2G -drive file=$(IMAGE_NAME)-syndicate.img,format=raw -serial file:serial.log -no-reboot -no-shutdown -s -S

.PHONY: run
run: $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: run-debug
run-debug: $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(IMAGE_NAME).iso -boot d -no-reboot -no-shutdown -s -S

.PHONY: gdb
gdb: kernel
	gdb "kernel/hadron.elf" -ex "target remote :1234"

.PHONY: run-uefi
run-uefi: ovmf $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf/OVMF.fd -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: run-uefi-debug
run-uefi-debug: ovmf $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf/OVMF.fd -cdrom $(IMAGE_NAME).iso -boot d -no-reboot -no-shutdown -s -S


.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd
	qemu-system-x86_64 -M q35 -m 2G -hda $(IMAGE_NAME).hdd

.PHONY: run-hdd-uefi
run-hdd-uefi: ovmf $(IMAGE_NAME).hdd
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf/OVMF.fd -hda $(IMAGE_NAME).hdd

ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

limine:
	git clone https://github.com/limine-bootloader/limine.git --branch=v5.x-branch-binary --depth=1
	$(MAKE) -C limine CC="$(HOST_CC)"

# Clone and build Syndicate bootloader
syndicate:
	@if [ ! -d "../syndicate-bootloader" ]; then \
		echo "Cloning Syndicate bootloader..."; \
		cd .. && git clone https://github.com/fmarl/syndicate syndicate-bootloader; \
	fi
	@echo "Building Syndicate bootloader..."
	$(MAKE) -C ../syndicate-bootloader

.PHONY: kernel
kernel:
	$(MAKE) -C kernel

.PHONY: stage2
stage2:
	$(MAKE) -C boot

$(IMAGE_NAME).iso: limine kernel
	rm -rf iso_root
	mkdir -p iso_root
	cp -v kernel/hadron.elf \
		limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso
	rm -rf iso_root

$(IMAGE_NAME).hdd: limine kernel
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	parted -s $(IMAGE_NAME).hdd mklabel gpt
	parted -s $(IMAGE_NAME).hdd mkpart ESP fat32 2048s 100%
	parted -s $(IMAGE_NAME).hdd set 1 esp on
	./limine/limine bios-install $(IMAGE_NAME).hdd
	sudo losetup -Pf --show $(IMAGE_NAME).hdd >loopback_dev
	sudo mkfs.fat -F 32 `cat loopback_dev`p1
	mkdir -p img_mount
	sudo mount `cat loopback_dev`p1 img_mount
	sudo mkdir -p img_mount/EFI/BOOT
	sudo cp -v kernel/hadron.elf limine.cfg limine/limine-bios.sys img_mount/
	sudo cp -v limine/BOOTX64.EFI img_mount/EFI/BOOT/
	sudo cp -v limine/BOOTIA32.EFI img_mount/EFI/BOOT/
	sync
	sudo umount img_mount
	sudo losetup -d `cat loopback_dev`
	rm -rf loopback_dev img_mount

.PHONY: clean
clean:
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd $(IMAGE_NAME)-syndicate.img serial.log
	$(MAKE) -C kernel clean
	$(MAKE) -C boot clean

# Build bootable image with Syndicate bootloader
$(IMAGE_NAME)-syndicate.img: syndicate kernel stage2
	@echo "Creating Syndicate bootable image..."
	rm -f $(IMAGE_NAME)-syndicate.img
	# Create 64MB disk image
	dd if=/dev/zero of=$(IMAGE_NAME)-syndicate.img bs=1M count=64
	# Write Syndicate MBR bootloader
	dd if=../syndicate-bootloader/loader.bin of=$(IMAGE_NAME)-syndicate.img conv=notrunc bs=512 count=1
	# Create FAT32 partition (skip first 1MB for alignment)
	parted -s $(IMAGE_NAME)-syndicate.img mklabel msdos
	parted -s $(IMAGE_NAME)-syndicate.img mkpart primary fat32 1MiB 100%
	parted -s $(IMAGE_NAME)-syndicate.img set 1 boot on
	# Format partition as FAT32
	@echo "Formatting FAT32 partition..."
	mformat -i $(IMAGE_NAME)-syndicate.img@@1M -F -v "HADRON" ::
	# Copy Stage 2 bootloader (KERNEL.BIN)
	mcopy -i $(IMAGE_NAME)-syndicate.img@@1M boot/KERNEL.BIN ::KERNEL.BIN
	# Copy Hadron kernel
	mcopy -i $(IMAGE_NAME)-syndicate.img@@1M kernel/hadron.elf ::HADRON.ELF
	@echo "Bootable image created: $(IMAGE_NAME)-syndicate.img"

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	$(MAKE) -C kernel distclean