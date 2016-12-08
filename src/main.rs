extern crate clap;

mod vm;
mod program;
mod instruction;

use clap::{Arg, App};
use program::Program;

fn main() {
    let matches = App::new("bakerVM")
        .version("0.0.1")
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("A virtual machine for classic point-and-click adventure games")
        .arg(Arg::with_name("input")
            .index(1)
            .help("Sets the image file to use. Uses a standard image if not specified."))
        .get_matches();

    let input = matches.value_of("input").unwrap_or("").to_string();

    let program = if input.is_empty() {
        Program::new()
    } else {
        program::decode(input)
    };

    let mut vm = vm::VM::new(program, 0);

    vm.exec();
}
