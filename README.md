# bakerVM [![Crates.io][crate-image]][crate-link] [![Build Status][travis-image]][travis-link] [![Docs.rs][docs-image]][docs-link]
A virtual machine for building and running retro games

![The logo of the bakerVM][logo]

## Introduction

![A screenshot of the bakerVM][screenshot]

The bakerVM is a virtual machine that executes bakerVM bytecode.

A builder-like compiler backend for the vm can be found in `definitions/image_builder.rs`.

The executable images of the vm are encoded and decoded using the [bincode crate][bincode]

**NOTE THAT THIS SOFTWARE IS STILL UNDER HEAVY DEVELOPMENT AND IN NO WAY STABLE OR COMPLETE.**

## Installation

### Install sdl2

On Ubuntu:
```shell
sudo apt install libsdl2-dev
```

On macOS:
```shell
brew install sdl2
```

The toolchain doesn't support Windows *yet*

### Install the toolchain

To get the newest version of the bakerVM toolchain, first you have to install Rust. The Project is currently tracking stable Rust. After you installed Rust and Cargo correctly, install the toolchain using the following command:
```shell
cargo install bakervm
```
If you already installed an older version you have to *force* the installation:
```shell
cargo install bakervm -f
```

After the installation, you should have the following binaries installed: `bakervm` and `hudson`.

`hudson` is the bakervm toolkit. It is currently only able to compile `*.basm` files.
```
hudson compile --basm path/to/main.basm
```
`bakervm` is the VM itself. On startup it loads the stock image by default. But you can specify any bakerVM image:
```shell
bakervm path/to/my/image/game.img
```

[crate-image]: https://img.shields.io/crates/v/bakervm.svg
[crate-link]: https://crates.io/crates/bakervm
[travis-image]: https://travis-ci.org/bakervm/bakervm.svg?branch=master
[travis-link]: https://travis-ci.org/bakervm/bakervm
[docs-image]: https://docs.rs/bakervm/badge.svg
[docs-link]: https://docs.rs/bakervm
[screenshot]: https://raw.githubusercontent.com/bakervm/bakervm/master/screenshot.png
[logo]: https://raw.githubusercontent.com/bakervm/bakervm/master/logo.png
[bincode]: https://crates.io/crates/bincode
