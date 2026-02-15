# zenhan -- Rust version

Switch the mode of input method editor from terminal.
This is a tool similar to im-select.

Original C version was created by [iuchim](https://github.com/iuchim).
I sincerely appreciate their work and inspiration.  
I made a Rust version based on their project.

- C version is here: [Context: https://github.com/iuchim/zenhan]

## 1. Usage

```shell
$ zenhan.exe -h
IME status controller

Usage: zenhan.exe [STATUS]

Arguments:
  [STATUS]  IME status (0: ON, 1: OFF)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 2. Setup

### 2.1. Windows 11

#### 2.1.1. Prerequisites

To build this project for Windows, you need the following:

- Any version of Visutal Studio with C++ Build tools

If don't work exe file in wsl environment, you need `.wslconfig` as well.

```config filename="$HOME.wslconfig"
[interop]
# **nessesary** for `zenhan.exe` to work in WSL
# this enables the interop between Windows and WSL, allowing you to run Windows
# executables from WSL.
enabled=true
```

#### 2.1.2. Install

Binary will be installed at `$HOME\.cargo\bin\zenhan.exe`

```powershell
git clone git@github.com:boarnasia/zenhan.git
cd zenhan\rust
cargo install --path .
```

#### 2.1.3. Uninstall

```powershell
cargo uninstall zenhan
```

### 2.2. WSL

#### 2.2.1. Prerequisites

To build this project for Windows from a WSL environment, you need the following:

```bash
# for 64-bit Windows
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu

# for 32-bit Windows
sudo apt install gcc-mingw-w64-i686
rustup target add i686-pc-windows-gnu
```

#### 2.2.2. Install

Binary will be installed at `~/.local/bin/zenhan.exe`

```bash
make install
```

#### 2.2.3. Uninstall

```bash
rm ~/.local/bin/zenhan.exe

or

make uninstall

```

## 3. Note

Turning IME OFF works reliably.  
Turning IME ON is unstable.

In many cases, if you turn IME off using the keyboard (e.g.,
the Hankaku/Zenkaku key) and then run `zenhan 1`, IME will not turn on.
However, if IME is already ON and you run `zenhan 0` followed by `zenhan 1`,
IME can be turned ON as expected.

Since turning IME OFF is sufficient for practical use, this issue is
currently left unresolved.
