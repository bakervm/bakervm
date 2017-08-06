#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate core;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate image;

mod commands;
mod basm;
mod mnemonic;

use clap::{App, AppSettings, Arg, SubCommand};
use core::error::*;

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
    let matches = App::new("hudson")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("The bakervm toolkit")
        .subcommand(SubCommand::with_name("stock").about(
            "Generate the default image",
        ))
        .subcommand(
            SubCommand::with_name("pack")
                .about("Write texture functions from images")
                .arg(Arg::with_name("input").index(1).required(true).help(
                    "Sets the source file to use.",
                ))
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Sets the destination file."),
                )
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .value_name("TYPE")
                        .validator(|input| {
                            let options = vec!["static", "dynamic-pos"];
                            if !options.contains(&input.as_str()) {
                                Err(format!("value has to be one of {:?}", options))
                            } else {
                                Ok(())
                            }
                        })
                        .help("Sets the packing type"),
                ),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .arg(Arg::with_name("input").index(1).required(true).help(
                    "Sets the source file to use.",
                ))
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Sets the destination file."),
                )
                .arg(Arg::with_name("basm").long("basm").help(
                    "compile the specified file as BASM",
                )),
        )
        .get_matches();


    match matches.subcommand() {
        ("compile", Some(sub_match)) => commands::compile(sub_match)?,
        ("pack", Some(sub_match)) => commands::pack(sub_match)?,
        _ => {}
    }

    Ok(())
}
