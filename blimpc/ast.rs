use definitions::Value;

#[derive(Debug)]
pub enum Expression {
    AtomicSymbol(String),
    List(Vec<Expression>),
    Literal(Value),
}

pub type AST = Vec<Expression>;
