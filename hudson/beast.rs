use core::error::*;
use core::typedef::*;
use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const _GRAMMAR: &str = include_str!("../src/beast.pest");

#[derive(Parser)]
#[grammar = "beast.pest"]
pub struct BeastParser;

pub fn compile(path: PathBuf) -> Result<ImageData> {
    let mut file = File::open(path).chain_err(|| "unable to open file")?;

    let mut buf = String::new();

    file.read_to_string(&mut buf)
        .chain_err(|| "unable to read file")?;

    let parser_res = BeastParser::parse(Rule::file, &buf);

    if let Err(err) = parser_res {
        bail!("\nError parsing file:\n{}\n", err);
    }

    bail!("Beast compiler is not implemented yet!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn simple() {
        BeastParser::parse(
            Rule::file,
            include_str!("../examples/beast/simple/simple.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn signals() {
        BeastParser::parse(
            Rule::file,
            include_str!("../examples/beast/signals/basic.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn else_if() {
        BeastParser::parse(
            Rule::file,
            include_str!("../examples/beast/simple/else_if.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }
}
