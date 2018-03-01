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
use commands::Lang;
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
                about = "compiles a *.basm or *.beast file into a bakerVM image (default: beast)",
                alias = "c")]
    Compile {
        #[structopt(short = "l")]
        lang: Option<Lang>,
        #[structopt(short = "o", parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

fn run() -> Result<()> {
    // let matches = App::new("hudson")
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author("Julian Laubstein <contact@julianlaubstein.de>")
    //     .setting(AppSettings::SubcommandRequiredElseHelp)
    //     .about("The bakervm toolkit")
    // .subcommand(SubCommand::with_name("stock").about("Generate the default
    // image"),)     .subcommand(
    // SubCommand::with_name("pack").about("Write texture functions from
    // images")
    // .arg(Arg::with_name("input").index(1).required(true).help("Sets the source
    // file to use."),)             .arg(Arg::with_name("output")
    //                 .short("o")
    //                 .long("output")
    //                 .takes_value(true)
    //                 .value_name("FILE")
    //                 .help("Sets the destination file."))
    //             .arg(Arg::with_name("type")
    //                 .short("t")
    //                 .long("type")
    //                 .takes_value(true)
    //                 .value_name("TYPE")
    //                 .validator(|input| {
    //                     let options = vec!["static", "dynamic-pos"];
    //                     if !options.contains(&input.as_str()) {
    //                         Err(format!("value has to be one of {:?}", options))
    //                     } else {
    //                         Ok(())
    //                     }
    //                 })
    //                 .help("Sets the packing type")),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("compile")
    // .arg(Arg::with_name("input").index(1).required(true).help("Sets
    // the source file to use."),)             .arg(Arg::with_name("output")
    //                 .short("o")
    //                 .long("output")
    //                 .takes_value(true)
    //                 .value_name("FILE")
    //                 .help("Sets the destination file."))
    //
    // .arg(Arg::with_name("lang").long("lang").short("l").takes_value(true).
    // help("compile the specified language (default: Beast)"))     )
    //     .get_matches();
    //
    //
    // match matches.subcommand() {
    //     ("compile", Some(sub_match)) => commands::compile(sub_match)?,
    //     ("pack", Some(sub_match)) => commands::pack(sub_match)?,
    //     _ => {}
    // }

    let opt = Opt::from_args();

    match opt {
        Opt::Compile {
            lang,
            input,
            output,
        } => commands::compile(lang, input, output)?,
    }

    Ok(())
}
