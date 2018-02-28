use core::error::*;
use core::typedef::*;
use pest::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const _GRAMMAR: &str = include_str!("../src/beast.pest");

#[derive(Parser)]
#[grammar = "beast.pest"]
pub struct BeastParser;

pub fn compile(file_name: String) -> Result<ImageData> {
    let orig_path = Path::new(&file_name)
        .canonicalize()
        .chain_err(|| "unable to canonicalize path")?;

    let mut file = File::open(orig_path).chain_err(|| "unable to open file")?;

    let mut buf = String::new();

    file.read_to_string(&mut buf)
        .chain_err(|| "unable to read file")?;

    let parser_res = BeastParser::parse(Rule::file, &buf);

    if let Err(err) = parser_res {
        bail!("{}", err);
    }

    bail!("Beast compiler is not implemented yet!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use pest::Token;
    use pest::iterators::Pairs;

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
