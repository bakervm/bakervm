#[derive(Debug)]
pub enum Expression {
    Symbol(String),
    List(Vec<Expression>),
}

pub type AST = Vec<Expression>;
