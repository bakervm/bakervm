extern crate core;
#[macro_use]
extern crate error_chain;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate regex;
#[macro_use]
extern crate structopt;

mod commands;
mod basm;
mod beast;
mod mnemonic;

// use clap::{App, AppSettings, Arg, SubCommand};
use commands::{Lang, PackingType};
use core::error::*;
use std::path::PathBuf;
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
#[structopt(name = "hudson", about = "The bakervm toolkit")]
enum Opt {
    #[structopt(name = "compile",
                about = "compiles a compatible source file into a bakerVM image", alias = "c")]
    Compile {
        #[structopt(short = "l")]
        lang: Option<Lang>,
        #[structopt(short = "o", parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    #[structopt(name = "pack", about = "Write texture functions from images", alias = "p")]
    Pack {
        #[structopt(long = "type", short = "t", value_name = "type")]
        packing_type: Option<PackingType>,
        #[structopt(short = "o", parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

fn run() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Compile {
            lang,
            input,
            output,
        } => commands::compile(lang, input, output)?,
        Opt::Pack {
            packing_type,
            input,
            output,
        } => commands::pack(packing_type, input, output)?,
    }

    Ok(())
}
