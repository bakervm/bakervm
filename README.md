# bakerVM [![Crates.io][crate-image]][crate-link] [![Build Status][travis-image]][travis-link] [![Docs.rs][docs-image]][docs-link]
A virtual machine for building and running retro games

![A screenshot of the bakerVM][screenshot]

## Introduction
The bakerVM is a virtual machine that executes bakerVM bytecode.

A builder-like compiler backend for the vm can be found in `definitions/image_builder.rs`. Since this project doesn't ship a sensible compiler yet, I recommend using it to create some form of assembler :grin:.

The executable images of the vm are encoded and decoded using the [bincode crate][bincode]

**NOTE THAT THIS SOFTWARE IS STILL UNDER HEAVY DEVELOPMENT AND IN NO WAY STABLE OR COMPLETE.**

**I HIGHLY RECOMMEND TO NOT USE IT IN PRODUCTION!**

[crate-image]: https://img.shields.io/crates/v/bakervm.svg
[crate-link]: https://crates.io/crates/bakervm
[travis-image]: https://travis-ci.org/bakervm/bakervm.svg?branch=master
[travis-link]: https://travis-ci.org/bakervm/bakervm
[docs-image]: https://docs.rs/bakervm/badge.svg
[docs-link]: https://docs.rs/bakervm
[screenshot]: https://raw.githubusercontent.com/bakervm/bakervm/master/screenshot.png
[bincode]: https://crates.io/crates/bincode
