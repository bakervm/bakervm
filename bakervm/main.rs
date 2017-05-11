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

    let input = matches.value_of("input").unwrap_or("").to_string();

    let mut file = File::open(input).chain_err(|| "unable to open file")?;
    let mut buf: ImageData = ImageData::new();
    file.read_to_end(&mut buf).chain_err(|| "unable to read from file")?;

    let program: Program = bincode::deserialize(&buf[..]).unwrap();

    let mut vm = VM::new();

    vm.exec(program).chain_err(|| "unable to exec program")?;

    Ok(())
}
