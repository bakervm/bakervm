#[derive(Debug)]
pub enum Expression {
    Symbol(String),
    List(Vec<Expression>),
}
