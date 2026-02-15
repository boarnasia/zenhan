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

clean:
	cargo clean
