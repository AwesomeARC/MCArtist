<p align="center"><img src="./assets/logo.png" alt="logo" height="100px"></p>
<p align="center">Image to Minecraft blocks converter</p>

# Introduction

MCArtist is a Minecraft pixel art generator witten in Rust. Given an image, it converts the image to a matrix of Minecraft blocks as per the width and height speicified by the user, where each block closely resembles each pixel of the image. Each block texture has been carefully hand-picked to make the block database as extensive and versatile as possible. Translucent blocks such as different kinds of glass are supported too, optionally.

Currently, its output format is in the form of of `/setblock` commands. There are plans to add more kinds of output formats in the future.

# Installation

## Precompiled Binaries

Binary releases of MCArtist can be found on the [releases page](https://github.com/AwesomeARC/MCArtist/releases).

## Compiling From Source

MCArtist makes use of the Rust build system.

The [`cargo`](https://github.com/rust-lang/cargo/) tool makes compilation and installation very straightforward.

**Note:** You need to have a Rust toolchain installed on your system before proceeding. The recommended way to install a Rust toolchain is through [`rustup`](https://github.com/rust-lang/rustup). For the official Rustup installation script, please visit [https://rustup.rs/](https://rustup.rs/).

1. Clone the repository

```
$ git clone https://github.com/AwesomeARC/MCArtist.git
```

2. Navigate into the directory

```
$ cd MCArtist
```

3. Use `cargo` to build the binary

```
$ cargo build --release
```

3. Use `cargo` to install the binary

```
$ cargo install --path .
```

The binary will, by default, get installed to `$HOME/.cargo/bin/`. Please make sure this directory is in your `$PATH`. Although Rustup's installation script usually takes care of this, you are advised to confirm that your shell's `$PATH` contains this directory.
