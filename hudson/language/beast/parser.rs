const _GRAMMAR: &str = include_str!("../../../src/beast.pest");

#[derive(Parser)]
#[grammar = "beast.pest"]
pub struct BeastParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn simple() {
        BeastParser::parse(
            Rule::file,
            include_str!("../../../examples/beast/simple/simple.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn signals() {
        BeastParser::parse(
            Rule::file,
            include_str!("../../../examples/beast/signals/basic.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn else_if() {
        BeastParser::parse(
            Rule::file,
            include_str!("../../../examples/beast/simple/else_if.beast"),
        ).unwrap_or_else(|e| panic!("{}", e));
    }
}
