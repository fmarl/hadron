# Hadron Stage 2 Bootloader Makefile

.PHONY: all clean

all: KERNEL.BIN

KERNEL.BIN: stage2.asm
	nasm -f bin -o KERNEL.BIN stage2.asm

clean:
	rm -f KERNEL.BIN
