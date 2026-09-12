# Hadron Stage 2 Bootloader Makefile

.PHONY: all clean

all: STAGE2.BIN

STAGE2.BIN: stage2.asm
	nasm -f bin -o STAGE2.BIN stage2.asm

clean:
	rm -f STAGE2.BIN
