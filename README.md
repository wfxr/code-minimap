<h1 align="center">🛰 code-minimap</h1>
<p align="center">
  <em>A high-performance code minimap generator for the terminal.</em>
</p>

<p align="center">
  <a href="https://github.com/wfxr/code-minimap/actions?query=workflow%3ACICD">
    <img src="https://github.com/wfxr/code-minimap/workflows/CICD/badge.svg" alt="CICD" />
  </a>
  <img src="https://img.shields.io/crates/l/code-minimap.svg" alt="License" />
  <a href="https://crates.io/crates/code-minimap">
    <img src="https://img.shields.io/crates/v/code-minimap.svg?colorB=319e8c" alt="Version" />
  </a>
  <a href="https://github.com/wfxr/code-minimap/releases">
    <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX%20|%20Win%20|%20ARM-orange.svg" alt="Platform" />
  </a>
</p>

`code-minimap` turns source code into a compact text minimap at terminal speed.
It is designed for CLI workflows, editor integrations, and tools such as
[minimap.vim](https://github.com/wfxr/minimap.vim).

## Why use it?

- Fast and lightweight.
- Works well in shell pipelines.
- Supports horizontal and vertical scaling.
- Can pad output to a fixed width for stable alignment.
- Reads UTF-8 strictly or with lossy decoding for messy input.

## Quick start

Render a file directly:

```console
$ code-minimap src/core.rs -H 0.6 -V 0.5
⣿⣿⣿⣿⣯⣭⣭⣍⣉⡀
⣉⣿⣿⣿⣿⣿⣿⣯⣀⣀⣀⣀⣀⡀
⠀⠉⠛⢛⣳⣶⣶⡶⠦⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
...
```

Use it in a pipeline:

```console
$ cat src/core.rs | code-minimap
```

See all options:

```console
$ code-minimap --help
```

## Installation

### Package managers

- **Arch Linux (AUR)**

  ```console
  $ yay -S code-minimap
  ```

- **Alpine Linux**

  ```console
  $ apk add code-minimap
  ```

- **Gentoo (GURU)**

  ```console
  $ emerge --ask code-minimap
  ```

- **Homebrew**

  ```console
  $ brew install code-minimap
  ```

- **MacPorts**

  ```console
  $ sudo port install code-minimap
  ```

- **Scoop**

  ```console
  $ scoop bucket add extras
  $ scoop install code-minimap
  ```

### Prebuilt binaries

Download a release from the
[GitHub releases page](https://github.com/wfxr/code-minimap/releases).

If you hit dynamic linking issues, try a `musl` build when one is available.

### From source

Install from crates.io:

```console
$ cargo install --locked code-minimap
```

Or build the latest development version:

```console
$ git clone https://github.com/wfxr/code-minimap.git
$ cd code-minimap
$ cargo build --release
```

## Common CLI usage

### Input sources

Read from a file:

```console
$ code-minimap path/to/file.rs
```

Read from standard input:

```console
$ cat path/to/file.rs | code-minimap
```

### Scaling

Use `-H` / `--horizontal-scale` to compress or stretch width, and `-V` /
`--vertical-scale` to control how many source lines are merged into each output row.

```console
$ code-minimap src/core.rs -H 0.6 -V 0.5
$ code-minimap src/core.rs --horizontal-scale 1.2 --vertical-scale 2.0
```

### Padding

Pad each rendered line to a fixed width when you need stable layout in an editor
panel, split view, or TUI.

```console
$ code-minimap src/core.rs --padding 40
```

### Encoding

By default, `code-minimap` uses `utf8-lossy`, which is a practical choice for
real-world text streams. Use strict UTF-8 mode when invalid input should be an
error.

```console
$ code-minimap input.txt --encoding utf8-lossy
$ code-minimap input.txt --encoding utf8
```

### Shell completions

Generate a completion script for your shell:

```console
$ code-minimap completion zsh
$ code-minimap completion fish
```

Pre-generated completion files are also checked into
[`completions/`](completions/).

## Library usage

Although the project is primarily used as a CLI, it can also be embedded as a
Rust library.

Write a minimap to a string:

```rust
fn main() -> std::io::Result<()> {
    let input = b"fn main() {\n    println!(\"hello\");\n}\n";
    let minimap = code_minimap::write_to_string(&input[..], 1.0, 1.0, None)?;
    println!("{}", minimap);
    Ok(())
}
```

The main entry points are:

- `code_minimap::print`
- `code_minimap::write`
- `code_minimap::write_to_string`

See `examples/simple.rs` and `examples/write_to_string.rs` for minimal working
examples.

## Benchmark

`code-minimap` is built for large inputs and interactive workflows. Historical
project benchmarks include:

- `src/core.rs`: about `0.2 ms`
- Rust `1.46.0` source tree concatenated into one file (`37M`, `1,153,225`
  lines): about `323 ms`
- Random text file (`735M`, `10,000,000` lines): about `2.9 s`

See the original benchmark commands in the project history if you want to rerun
or refresh the numbers on your machine.

## Development

Useful local commands:

```console
$ cargo build
$ cargo test
$ cargo fmt --all
$ cargo clippy --all-targets --all-features
```

To regenerate shell completions into the tracked `completions/` directory:

```console
$ SHELL_COMPLETIONS_DIR=completions cargo build
```

## Related project

- [minimap.vim](https://github.com/wfxr/minimap.vim) — a blazing fast minimap
  plugin for Vim/Neovim built on top of `code-minimap`.

## License

`code-minimap` is distributed under both the MIT License and the Apache License
2.0.

See [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) for
details.
