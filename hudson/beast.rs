use core::error::*;
use core::typedef::*;
use std::path::Path;

const _GRAMMAR: &str = include_str!("../src/beast.pest");

#[derive(Parser)]
#[grammar = "beast.pest"]
pub struct BeastParser;

pub fn compile(file_name: String) -> Result<ImageData> {
    let path = Path::new(&file_name)
        .canonicalize()
        .chain_err(|| "unable to canonicalize path")?;

    bail!("Beast compiler not implemented yet");
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
