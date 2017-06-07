use definitions::{InternalInterrupt, Target, Type, Value};
#[derive(Clone)]
pub enum Mnemonic {
    Add(Target, Target),
    Sub(Target, Target),
    Div(Target, Target),
    Mul(Target, Target),
    Rem(Target, Target),

    Cmp(Target, Target),
    Jmp(String),
    JmpLt(String),
    JmpGt(String),
    JmpEq(String),
    JmpLtEq(String),
    JmpGtEq(String),

    Cast(Target, Type),

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),
    Dup(Target),

    Call(String),
    Ret,

    Halt,
    Pause,
    Nop,
    Int(InternalInterrupt),
}
