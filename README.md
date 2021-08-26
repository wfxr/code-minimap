<h1 align="center">🛰 code-minimap</h1>
<p align="center">
    <em>A high performance code minimap render.</em>
</p>

<p align="center">
    <a href="https://github.com/wfxr/code-minimap/actions?query=workflow%3ACICD">
        <img src="https://github.com/wfxr/code-minimap/workflows/CICD/badge.svg" alt="CICD"/>
    </a>
    <img src="https://img.shields.io/crates/l/code-minimap.svg" alt="License"/>
    <a href="https://crates.io/crates/code-minimap">
        <img src="https://img.shields.io/crates/v/code-minimap.svg?colorB=319e8c" alt="Version">
    </a>
    <a href="https://github.com/wfxr/code-minimap/releases">
        <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX%20|%20Win%20|%20ARM-orange.svg" alt="Platform"/>
    </a>
</p>

This tool is for generating text minimaps at 🚀 speed.
You can use it to implement IDE-like minimap plugin for a terminal text editor,
[minimap.vim](https://github.com/wfxr/minimap.vim) for example.

### Features

* Small and *fast* (see [benchmarks](#benchmark) below).
* Little constant memory usage.
* Freely zoom.
* [Multi platforms](https://github.com/wfxr/code-minimap/releases) support.

### Usage

```
$ code-minimap src/core.rs -H 0.6 -V 0.5
⣿⣿⣿⣿⣿⠿⠛⠓⠒⠒⠂
⣉⣿⣿⣿⣟⣛⣛⣛⠒⠒⠂
⠀⠉⣿⣿⣿⣿⠭⠭⠭⠭⠤⠤⠤⠤⠤
⠀⠉⠛⠻⢿⣿⣿⣿⣿⣶⣶⣶⣒⣒⣒⣒⣒⣒⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀
⠀⣀⣶⣾⣿⣿⣿⣿⣭⣭⣭⣤⣤⣤⣤⣤⠤⠤⠤⠤⠤
⣿⣿⣿⣶⡒⠒⠒⠒
⣿⣿⣶⣶⣶⣶⣶⣶⣤⣤⣤⣤⣤⣤⣤⣤⣄
⣭⣭⣭⣭⠭⠭⠭⠭⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
⣿⣿⣿⣿⣧⣤⣤⣤⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀
⣛⣿⣿⣿⣟⣛⣒⣒⠂
⣀⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣋⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀
⠀⣤⣭⣽⣿⣷⣶⣶⣶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠒⠒⠒⠒⠒
⠀⠶⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠓⠒⠒⠒⠒⠒
⣉⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⡛⠛⠛⠛⠛
⠒⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⡄
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇
⠄⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠃
```

Run `code-minimap --help` to view detailed usage.

### Installation

#### On Windows

You can install `code-minimap` with [scoop](https://github.com/lukesampson/scoop):

```
scoop bucket add extras
scoop install code-minimap
```

#### On Arch Linux

`code-minimap` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/code-minimap):

```
yay -S code-minimap
```

#### On macOS

You can install `code-minimap` with Homebrew:

```
brew install code-minimap
```

or with MacPorts:

```
sudo port install code-minimap
```

#### From binaries

Prebuilt versions of `code-minimap` for various architectures are available at [Github release page](https://github.com/wfxr/code-minimap/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

#### From source

`code-minimap` is also published on [crates.io](https://crates.io). If you have Rust toolchains (1.40 or above) installed you can use `cargo` to install it from source:

```
cargo install --locked code-minimap
```

If you want the latest version, clone this repository and run `cargo build --release`.

### Benchmark

- [src/core.rs](https://github.com/wfxr/code-minimap/blob/v0.3.0/src/core.rs):

```
$ hyperfine -w 10 'code-minimap src/core.rs'
Benchmark #1: code-minimap src/core.rs
  Time (mean ± σ):       0.2 ms ±   0.1 ms    [User: 0.4 ms, System: 0.3 ms]
  Range (min … max):     0.2 ms …   1.1 ms    1560 runs
```
**79** lines, **4K** size, **0.2ms**.

---------------------------------

- [all rust code from rust-1.46.0](https://github.com/rust-lang/rust/archive/1.46.0.tar.gz):

```
$ fd -t f -e rs -x cat "{}" >> /tmp/all-in-one.rs
$ hyperfine -w 10 'code-minimap /tmp/all-in-one.rs'
Benchmark #1: code-minimap /tmp/all-in-one.rs
  Time (mean ± σ):     322.7 ms ±   4.5 ms    [User: 298.7 ms, System: 23.8 ms]
  Range (min … max):   318.5 ms … 334.1 ms    10 runs
```
**1,153,225** lines, **37M** size, **323ms**.

---------------------------------

- [huge random file]():
```
$ base64 /dev/urandom | head -10000000 > huge.txt
$ hyperfine -w 1 'code-minimap huge.txt'
Benchmark #1: code-minimap huge.txt
  Time (mean ± σ):      2.941 s ±  0.034 s    [User: 2.618 s, System: 0.321 s]
  Range (min … max):    2.919 s …  3.028 s    10 runs

```
 **10,000,000** lines, **735M** size, **2.9s**.

---------------------------------

*Test environment:*

```
Binary version: 0.3.0
OS: Arch Linux x86_64
Kernel: 5.8.10-arch1-1
CPU: Intel i9-9900K (16) @ 5.000GHz
```

### Related Project

[minimap.vim](https://github.com/wfxr/minimap.vim): Blazing fast minimap for vim.

### License

`code-minimap` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
