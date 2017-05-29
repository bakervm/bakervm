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
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true)
                .value_name("SCALE")
                .help("Sets the scale for the display. If not specified, the default scale set by the image will be used."))
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
        builder.push(
            Target::Framebuffer(res_def.width + 1),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 2) + 2),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 3) + 3),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 4) + 4),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 5) + 3),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 6) + 2),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 1),
            Value::Color(0xFF, 0xFF, 0xFF),
        );

        // Underscore
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 5),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 6),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 7),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 8),
            Value::Color(0xFF, 0xFF, 0xFF),
        );
        builder.push(
            Target::Framebuffer((res_def.width * 7) + 9),
            Value::Color(0xFF, 0xFF, 0xFF),
        );

        builder.int(InternalInterrupt::FlushFramebuffer);

        builder.jmp(max + 1);
        builder.gen_program()
    };

    let mut vm_config = program.config.clone();

    if let Some(scale) = matches.value_of("scale") {
        vm_config.display.default_scale = scale.parse().chain_err(|| "unable to parse scale")?;
    }

    let (vm_sender, outer_receiver) = mpsc::sync_channel(1);
    let (outer_sender, vm_receiver) = mpsc::channel();

    let vm_handle = vm::start(program, vm_sender, vm_receiver);

    io::start(outer_receiver, outer_sender, vm_config)?;

    if let Err(err) = vm_handle.join() {
        bail!("unable to join: {:?}", err);
    }

    Ok(())
}
