extern crate clap;

mod vm;
mod program;
mod instruction;

use clap::{Arg, App};

fn main() {
    let matches = App::new("bakerVM")
        .version("0.0.1")
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("A virtual machine for classic point-and-click adventure games")
        .arg(Arg::with_name("input")
            .index(1)
            .help("Sets the input file to use"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
}
