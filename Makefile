# --- Configuration Variables ---
PACKAGE      := led-blinker
TARGET       := thumbv6m-none-eabi
BUILD_DIR    := target/$(TARGET)/release
ELF_BINARY   := $(BUILD_DIR)/$(PACKAGE)

# Crucial Rust compilation and memory mapping flags for RP2040
export RUSTFLAGS := -C linker=flip-link -C link-arg=--nmagic -C link-arg=-Tlink.x -C link-arg=-Tlink-rp.x -C link-arg=-Tdefmt.x

.PHONY: all build flash clean check-memory

all: build

build: check-memory
	cargo build --release --package $(PACKAGE) --target $(TARGET)

flash: build
	picotool load -u -x -v -t elf $(ELF_BINARY)

check-memory:
	@if [ ! -f memory.x ]; then \
		cp led/blinker/memory.x ./memory.x 2>/dev/null || (echo "❌ Failed: Please create a 'memory.x' file in this root directory first." && exit 1); \
	fi

clean:
	cargo clean

