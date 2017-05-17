# bakerVM [![Build Status](https://travis-ci.org/bakervm/bakervm.svg?branch=master)](https://travis-ci.org/bakervm/bakervm)
A virtual machine for building and running retro games

## Introduction
The bakerVM is a virtual machine that executes bakerVM bytecode.

A builder-like compiler backend for the vm can be found in `definitions/image_builder.rs`. Since this project doesn't ship a sensible compiler yet, I recommend using it to create some form of assembler :grin:.

The executable images of the vm are encoded and decoded using the [bincode crate](https://crates.io/crates/bincode)
