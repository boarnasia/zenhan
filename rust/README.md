# zenhan -- Rust version

Prerequisites (for Cross-Compilation from Linux)

To build this project for Windows from a Linux/WSL environment, you need the following:

```bash
# for 64-bit Windows
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
make win64

# for 32-bit Windows
sudo apt install gcc-mingw-w64-i686
rustup target add i686-pc-windows-gnu
make win32
```

**Note**

Turning IME OFF works reliably.  
Turning IME ON is unstable.

In many cases, if you turn IME off using the keyboard (e.g.,
the Hankaku/Zenkaku key) and then run `zenhan 1`, IME will not turn on.
However, if IME is already ON and you run `zenhan 0` followed by `zenhan 1`,
IME can be turned ON as expected.

Since turning IME OFF is sufficient for practical use, this issue is
currently left unresolved.
