EMBEDDED_DIR := embedded
WHAMM_CORE_WASM := $(EMBEDDED_DIR)/whamm_core.wasm
WASM_TARGET := wasm32-wasip1

# Targets for each build type
DEBUG_WASM := $(EMBEDDED_DIR)/debug/whamm_core.wasm
RELEASE_WASM := $(EMBEDDED_DIR)/release/whamm_core.wasm

# --------------------------------------
# Build main crate + embed whamm_core wasm
# --------------------------------------
all: debug

$(DEBUG_WASM):
	mkdir -p $(EMBEDDED_DIR)/debug
	rustup target add $(WASM_TARGET)
	cargo build -p whamm_core --target $(WASM_TARGET)
	cp target/$(WASM_TARGET)/debug/whamm_core.wasm $@
	@echo "✅ whamm_core.wasm updated for debug build"

$(RELEASE_WASM):
	mkdir -p $(EMBEDDED_DIR)/release
	rustup target add $(WASM_TARGET)
	cargo build -p whamm_core --target $(WASM_TARGET) --release
	cp target/$(WASM_TARGET)/release/whamm_core.wasm $@
	@echo "✅ whamm_core.wasm updated for release build"

# Main targets
debug: $(DEBUG_WASM)
	cargo build

release: $(RELEASE_WASM)
	cargo build --release

# Clean
clean:
	cargo clean
	rm -f $(WHAMM_CORE_WASM)
	@echo "🧹 Cleaned embedded/ whamm_core.wasm"
