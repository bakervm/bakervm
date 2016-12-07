#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    HALT,
    IADD,
    ISUB,
    PRINT,
    PUSH(i64),
}
