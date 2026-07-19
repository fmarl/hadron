# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := hadron

.PHONY: all
all: $(IMAGE_NAME).img

.PHONY: run
run: $(IMAGE_NAME).img
	qemu-system-x86_64 -m 2G -drive file=$(IMAGE_NAME).img,format=raw -serial file:serial.log

.PHONY: run-debug
run-debug: $(IMAGE_NAME).img
	qemu-system-x86_64 -m 2G -drive file=$(IMAGE_NAME).img,format=raw -serial file:serial.log -no-reboot -no-shutdown -s -S

.PHONY: gdb
gdb: kernel
	gdb "kernel/hadron.elf" -ex "target remote :1234"

# Clone and build Syndicate bootloader
syndicate:
	@if [ ! -d "../syndicate-bootloader" ]; then \
		echo "Cloning Syndicate bootloader..."; \
		cd .. && git clone https://github.com/fmarl/syndicate syndicate-bootloader; \
	fi
	@echo "Building Syndicate bootloader..."
	$(MAKE) -C ../syndicate-bootloader STAGE2_FILE=KERNEL.BIN KERNEL_FILE=HADRON.ELF

.PHONY: kernel
kernel:
	$(MAKE) -C kernel

.PHONY: stage2
stage2:
	$(MAKE) -C boot

.PHONY: clean
clean:
	rm -rf $(IMAGE_NAME).img serial.log
	$(MAKE) -C kernel clean
	$(MAKE) -C boot clean

# Build bootable image with Syndicate bootloader
$(IMAGE_NAME).img: syndicate kernel stage2
	@echo "Creating Syndicate bootable image..."
	rm -f $(IMAGE_NAME).img
	# Create 64MB disk image
	dd if=/dev/zero of=$(IMAGE_NAME).img bs=1M count=64
	# Create bootable FAT32 (LBA) partition at sector 2048, as Syndicate expects
	echo 'start=2048, type=c, bootable' | sfdisk -q $(IMAGE_NAME).img
	# Format partition as FAT32
	@echo "Formatting FAT32 partition..."
	mformat -i $(IMAGE_NAME).img@@1M -F -T $$((64 * 2048 - 2048)) -v "HADRON" ::
	# Copy Stage 2 bootloader (KERNEL.BIN)
	mcopy -i $(IMAGE_NAME).img@@1M boot/KERNEL.BIN ::KERNEL.BIN
	# Copy stripped Hadron kernel (Syndicate loads at most 512 KiB)
	strip -o kernel/hadron-stripped.elf kernel/hadron.elf
	@test $$(stat -c %s kernel/hadron-stripped.elf) -le 524288 || \
		{ echo "kernel/hadron-stripped.elf exceeds 512 KiB"; exit 1; }
	mcopy -i $(IMAGE_NAME).img@@1M kernel/hadron-stripped.elf ::HADRON.ELF
	# Write Syndicate stage 1: 446 bytes into the MBR, the rest into the following sectors
	dd if=../syndicate-bootloader/loader.bin of=$(IMAGE_NAME).img bs=446 count=1 conv=notrunc
	dd if=../syndicate-bootloader/loader.bin of=$(IMAGE_NAME).img bs=512 skip=1 seek=1 conv=notrunc
	@echo "Bootable image created: $(IMAGE_NAME).img"

.PHONY: distclean
distclean: clean
	$(MAKE) -C kernel distclean
