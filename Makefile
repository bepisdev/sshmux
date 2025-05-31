CARGO := $(shell which cargo)
BIN := sshmux
PREFIX ?= /usr/local
TARGET := target/release/$(BIN)
BUILD := ./$(BIN)

.PHONY: all sshmux install check clean

all: sshmux

# Run pre-check, then build
sshmux: check
	@echo "Building $(BIN)..."
	$(CARGO) build --release
	cp $(TARGET) $(BUILD)

# Pre-run sanity check
check:
	@echo "Checking TOML config format..."
	$(CARGO) run -- --check-config

# Install to /usr/local/bin or $PREFIX/bin
install: sshmux
	@echo "Installing $(BIN) to $(PREFIX)/bin/$(BIN)..."
	install -d $(PREFIX)/bin
	install -m 755 $(BUILD) $(PREFIX)/bin/$(BIN)

# Clean target and local copy
clean:
	$(CARGO) clean
	rm -f $(BUILD)
