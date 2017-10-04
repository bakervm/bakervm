extern crate rmp_serde;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate core;
extern crate sdl2;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod vm;
mod io;

use clap::{App, Arg};
use core::Program;
use core::error::*;
use core::typedef::*;
use rmp_serde::Deserializer;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Barrier, mpsc};

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = App::new("bakerVM")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("A virtual machine for building and running retro games")
        .arg(Arg::with_name("input").index(1).help(
            "Sets the image file to use. Uses a standard image if nothing is specified.",
        ))
        .arg(
            Arg::with_name("scale").short("s").long("scale").takes_value(true).value_name("SCALE").help(
                "Sets the scale for the display. If not specified, the default scale set by the image will be used.",
            ),
        )
        .get_matches();

    let program: Program = if let Some(input) = matches.value_of("input") {
        let mut file = File::open(input).chain_err(|| "unable to open file")?;
        let mut buf: ImageData = ImageData::new();
        file.read_to_end(&mut buf).chain_err(|| "unable to read from file")?;

        let mut de = Deserializer::new(&buf[..]);

        Deserialize::deserialize(&mut de).chain_err(|| "unable to decode image file")?
    } else {
        let program_data = include_bytes!("stock.img");

        let mut de = Deserializer::new(&program_data[..]);

        Deserialize::deserialize(&mut de).chain_err(|| "unable to decode image file")?
    };

    let mut config = program.config.clone();

    if let Some(scale) = matches.value_of("scale") {
        config.display.default_scale = scale.parse().chain_err(|| "unable to parse scale")?;
    }

    if config.display.default_scale < 1.0 {
        bail!("Display scale can't be less than 1");
    }

    let (vm_sender, outer_receiver) = mpsc::sync_channel(1);
    let (outer_sender, vm_receiver) = mpsc::channel();

    let barrier = Arc::new(Barrier::new(2));

    let vm_handle = vm::start(program, vm_sender, vm_receiver, barrier.clone());

    io::start(outer_receiver, outer_sender, config, barrier.clone())?;

    if let Err(err) = vm_handle.join() {
        bail!("unable to join: {:?}", err);
    }

    Ok(())
}
