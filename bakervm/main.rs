extern crate bincode;

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate definitions;

mod vm;
mod error;

use clap::{App, Arg};
use definitions::program::Program;
use definitions::typedef::*;
use error::*;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use vm::VM;

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
        Program::default()
    };

    let display_resolution = program.config.display_resolution.clone();
    let display_scale = program.config.display_scale.clone();

    let (vm_sender, outer_receiver) = mpsc::channel::<Frame>();
    let (outer_sender, vm_receiver) = mpsc::channel::<Address>();

    thread::spawn(
        move || {
            VM::default().exec(program, vm_sender, vm_receiver).expect("unable to exec program");
        },
    );

    'main: loop {
        if let Ok(frame) = outer_receiver.recv() {
            for y in 0..display_resolution.height {
                for x in 0..display_resolution.width {
                    print!("{:?}", frame[(y * display_resolution.width) + x]);
                }
                println!();
            }
        } else {
            break 'main;
        }
    }

    Ok(())
}
