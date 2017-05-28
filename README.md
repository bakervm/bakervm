# bakerVM ![Crates.io][1] ![Build Status][2] ![Docs.rs][3]
A virtual machine for building and running retro games

![A screenshot of the bakerVM][4]

## Introduction
The bakerVM is a virtual machine that executes bakerVM bytecode.

A builder-like compiler backend for the vm can be found in `definitions/image_builder.rs`. Since this project doesn't ship a sensible compiler yet, I recommend using it to create some form of assembler :grin:.

The executable images of the vm are encoded and decoded using the [bincode crate][5]

**NOTE THAT THIS SOFTWARE IS STILL UNDER HEAVY DEVELOPMENT AND IN NO WAY STABLE OR COMPLETE.**

**I HIGHLY RECOMMEND TO NOT USE IT IN PRODUCTION!**

[1]: https://img.shields.io/crates/v/bakervm.svg "https://crates.io/crates/bakervm"
[2]: https://travis-ci.org/bakervm/bakervm.svg?branch=master "https://travis-ci.org/bakervm/bakervm"
[3]: https://docs.rs/bakervm/badge.svg "https://docs.rs/bakervm"
[4]: https://raw.githubusercontent.com/bakervm/bakervm/master/screenshot.png
[5]: https://crates.io/crates/bincode
