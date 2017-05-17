#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
extern crate definitions;
extern crate bincode;

mod error;
mod compiler;
mod ast;

use clap::{App, Arg};
use error::*;
use std::fs::File;


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

fn run() -> CompilationResult<()> {
    let matches = App::new("blimpc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("The blimp compiler")
        .arg(Arg::with_name("input").index(1).required(true).help("Sets the source file to use."),)
        .arg(Arg::with_name("output").index(2).required(true).help("Sets the destination file."),)
        .get_matches();

    let input_file_name = matches.value_of("input").unwrap_or("");

    let output_file_name = matches.value_of("output").unwrap_or("");

    let input_file = File::open(input_file_name).chain_err(|| "unable to open input file")?;

    compiler::compile(input_file).chain_err(|| "unable to compile file")?;

    Ok(())
}
