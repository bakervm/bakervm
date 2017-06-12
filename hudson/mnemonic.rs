use definitions::{Signal, Target, Type, VMEventType, Value};

#[derive(Clone, Debug)]
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
    Rev(VMEventType, String),

    Halt,
    Pause,
    Nop,
    Int(Signal),
}
