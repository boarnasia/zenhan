SHELL 	:= /bin/sh
ARCH 	:= $(shell uname -m)

ifeq ($(ARCH), x86_64)
    TARGET := x86_64-pc-windows-gnu
else
    TARGET := i686-pc-windows-gnu
endif

BINARY_NAME := zenhan.exe
INSTALL_DIR := $(HOME)/.local/bin

.PHONY: all win64 win32 clean test

all: build

build:
	@echo "Detected ARCH: $(ARCH)"
	@echo "Building for $(TARGET)..."
	cargo build --release --target $(TARGET)

install:
	@echo "Installing to $(INSTALL_DIR)..."
	mkdir -p $(INSTALL_DIR)
	install -m 755 target/$(TARGET)/release/$(BINARY_NAME) $(INSTALL_DIR)
	@echo "Installation complete."

uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Uninstalled."

buildAll:
	@echo "Building for both architectures..."
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target i686-pc-windows-gnu
	install -m 755 target/x86_64-pc-windows-gnu/release/$(BINARY_NAME) ./zenhan-x86_64.exe
	install -m 755 target/i686-pc-windows-gnu/release/$(BINARY_NAME) ./zenhan-i686.exe

clean:
	cargo clean
	rm ./zenhan-*.exe
