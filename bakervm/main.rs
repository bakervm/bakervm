extern crate bincode;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate definitions;
extern crate sdl2;
extern crate rand;

mod vm;
mod error;
mod io;

use clap::{App, Arg};
use definitions::Value;
use definitions::config::DisplayResolution;
use definitions::image_builder::ImageBuilder;
use definitions::interrupt::{ExternalInterrupt, InternalInterrupt};
use definitions::program::Program;
use definitions::target::Target;
use definitions::typedef::*;
use error::*;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;

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

fn run() -> VMResult<()> {
    let matches = App::new("bakerVM")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("A virtual machine for building and running retro games")
        .arg(
            Arg::with_name("input")
                .index(1)
                .help("Sets the image file to use. Uses a standard image if nothing is specified.")
        )
        .get_matches();

    let program: Program = if let Some(input) = matches.value_of("input") {
        let mut file = File::open(input).chain_err(|| "unable to open file")?;
        let mut buf: ImageData = ImageData::new();
        file.read_to_end(&mut buf).chain_err(|| "unable to read from file")?;

        bincode::deserialize(&buf[..]).chain_err(|| "unable to decode image file")?
    } else {
        let mut builder = ImageBuilder::new();

        let res_def = DisplayResolution::default();

        let max = res_def.width * res_def.height;

        for x in 0..max {
            builder.push(Target::Framebuffer(x), Value::Color(0, 0, 0));
        }

        builder.int(InternalInterrupt::FlushFramebuffer);

        // Triangle
        builder.push(Target::Framebuffer(161), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(322), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(483), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(644), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(803), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(962), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(1121), Value::Color(0xFF, 0xFF, 0xFF));

        // Underscore
        builder.push(Target::Framebuffer(1125), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(1126), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(1127), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(1128), Value::Color(0xFF, 0xFF, 0xFF));
        builder.push(Target::Framebuffer(1129), Value::Color(0xFF, 0xFF, 0xFF));

        builder.int(InternalInterrupt::FlushFramebuffer);

        builder.jmp(max + 1);
        builder.gen_program()
    };

    let vm_config = program.config.clone();

    let (vm_sender, outer_receiver) = mpsc::sync_channel::<Frame>(1);
    let (outer_sender, vm_receiver) = mpsc::channel::<ExternalInterrupt>();

    let vm_handle = vm::start(program, vm_sender, vm_receiver);

    io::start(outer_receiver, outer_sender, vm_config)?;

    if let Err(err) = vm_handle.join() {
        bail!("unable to join: {:?}", err);
    }

    Ok(())
}
