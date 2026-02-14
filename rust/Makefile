.PHONY: all win64 win32 clean

all: win64 win32

win64:
	cargo build --release --target x86_64-pc-windows-gnu
	@echo
	@echo == Built exe: =============================================
	@echo
	@ls target/x86_64-pc-windows-gnu/release/zenhan.exe
	@echo

win32:
	cargo build --release --target i686-pc-windows-gnu
	@echo
	@echo == Built exe: =============================================
	@echo
	@ls target/i686-pc-windows-gnu/release/zenhan.exe
	@echo

clean:
	cargo clean
