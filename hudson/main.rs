#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate definitions;
extern crate bincode;

mod error;
mod commands;

use clap::{App, AppSettings, Arg, SubCommand};
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
    let matches = App::new("hudson")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("The bakervm toolkit")
        .subcommand(SubCommand::with_name("stock").about("Generate the default image"),)
        .subcommand(
            SubCommand::with_name("compile")
                .arg(Arg::with_name("input").index(1).required(true).help("Sets the source file to use."),)
                .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE")
                .help("Sets the destination file.")),
        )
        .get_matches();


    match matches.subcommand() {
        ("compile", Some(sub_match)) => commands::compile(sub_match)?,
        ("stock", Some(sub_match)) => commands::stock(sub_match)?,
        _ => {}
    }

    Ok(())
}
