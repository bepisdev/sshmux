CARGO := $(shell which cargo)
BIN := sshmux
PREFIX := /usr/local

sshmux:
	$(CARGO) build --release
	cp ./target/release/sshmux ./$(BIN)

install:
	install ./$(BIN) $(PREFIX)/bin/$(BIN)
