# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

# Allow nightly features on a stable toolchain (e.g. Guix). No-op on nightly.
export RUSTC_BOOTSTRAP ?= 1

# Default target.
.PHONY: all
all:
	cd .. && cargo build --profile $(RUST_PROFILE) --package hadron
	cp ../target/x86_64-unknown-hadron/$(RUST_PROFILE_SUBDIR)/hadron hadron.elf

# Remove object files and the final executable.
.PHONY: clean
clean:
	cd .. && cargo clean
	rm -rf hadron.elf hadron-stripped.elf

.PHONY: distclean
distclean: clean