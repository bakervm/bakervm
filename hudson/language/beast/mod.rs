mod parser;
mod compiler;

use self::compiler::BeastCompiler;
use self::parser::{BeastParser, Rule};
use core::error::*;
use core::typedef::*;
use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn compile(path: PathBuf) -> Result<ImageData> {
    let mut file = File::open(path).chain_err(|| "unable to open file")?;

    let mut buf = String::new();

    file.read_to_string(&mut buf)
        .chain_err(|| "unable to read file")?;

    let parser_res = BeastParser::parse(Rule::file, &buf);

    if let Err(err) = parser_res {
        bail!("\nError parsing file:\n{}\n", err);
    }

    BeastCompiler::compile(parser_res.unwrap())
}
