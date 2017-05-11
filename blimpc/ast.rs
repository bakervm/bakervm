#[derive(Debug)]
pub enum AST {
    Symbol(String),
    List(Vec<AST>),
}
