# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

# Default target.
.PHONY: all
all:
	cd .. && cargo build --profile $(RUST_PROFILE) -Zjson-target-spec --package hadron
	cp ../target/x86_64-unknown-hadron/$(RUST_PROFILE_SUBDIR)/hadron hadron.elf

# Remove object files and the final executable.
.PHONY: clean
clean:
	cd .. && cargo clean
	rm -rf hadron.elf

.PHONY: distclean
distclean: clean