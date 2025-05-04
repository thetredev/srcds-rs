SHELL := /bin/bash

HL2SDK_DIR = vendor/hl2sdk/css
HL2SDK_INCLUDE_DIRS = -I$(HL2SDK_DIR)/public -I$(HL2SDK_DIR)/public/tier0 -I$(HL2SDK_DIR)/public/tier1
HL2SDK_LINK_DIRS = -L$(HL2SDK_DIR)/lib/public/linux

SRCDS_INCLUDE_DIRS = $(HL2SDK_INCLUDE_DIRS) -I./include
SRCDS_CFLAGS = -m32 -Wno-unknown-pragmas -fPIC -msse -m32 -DGNU -DGNUC -DLINUX -D_LINUX -DPOSIX -std=gnu++17 $(SRCDS_INCLUDE_DIRS)
SRCDS_OUTPUT_DIR = target


RUST_TARGET_DIR = $(SRCDS_OUTPUT_DIR)/release

rust: $(RUST_TARGET_DIR)/libsrcds.so

$(RUST_TARGET_DIR)/libsrcds.so:
	cargo build --release


all: srcds


.PHONY: clean-submodules
clean-submodules:
	rm -rf $(HL2SDK_DIR)
	rm -rf .git/modules/$(HL2SDK_DIR)/index.lock
	git submodule status | cut -d ' ' -f 3 | xargs rm -rf
	git submodule update --init --recursive

.PHONY: clean
clean: clean-submodules

.PHONY: sdk-patches
sdk-patches: clean-submodules
	./apply-patches.sh ./patches/hl2sdk/css $(HL2SDK_DIR)

.PHONY: srcds
srcds: rust sdk-patches
	mkdir -p $(SRCDS_OUTPUT_DIR)
	g++ -c $(SRCDS_CFLAGS) src/srcds.cpp -o $(SRCDS_OUTPUT_DIR)/srcds.o
	g++ -c $(SRCDS_CFLAGS) src/plugin.cpp -o $(SRCDS_OUTPUT_DIR)/plugin.o
	g++ -c $(SRCDS_CFLAGS) src/plugin_bridge.cpp -o $(SRCDS_OUTPUT_DIR)/plugin_bridge.o

	g++ -m32 -shared -o $(SRCDS_OUTPUT_DIR)/libsrcds.so \
		$(SRCDS_OUTPUT_DIR)/srcds.o \
		$(SRCDS_OUTPUT_DIR)/plugin.o \
		$(SRCDS_OUTPUT_DIR)/plugin_bridge.o \
		$(HL2SDK_LINK_DIRS) \
		-L$(RUST_TARGET_DIR) -lsrcds \
		-ltier0_srv -lvstdlib_srv -ldl \
		-l:tier1_i486.a -l:mathlib_i486.a -l:libcurl.a
