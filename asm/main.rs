#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate bakervm_definitions as definitions;
extern crate ieee754;

mod error;
mod compiler;

use std::fs::File;
use clap::{App, Arg};
use error::*;


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
    let matches = App::new("basm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("The assembler for the bakerVM")
        .arg(Arg::with_name("input")
            .index(1)
            .required(true)
            .help("Sets the source file to use."))
        .arg(Arg::with_name("output")
            .index(2)
            .required(true)
            .help("Sets the destination file."))
        .get_matches();

    let input_file_name = matches.value_of("input").unwrap_or("");

    let output_file_name = matches.value_of("output").unwrap_or("");

    let input_file = File::open(input_file_name).chain_err(|| "unable to open input file")?;

    compiler::compile(input_file, output_file_name).chain_err(|| "unable to compile file")?;

    Ok(())
}
