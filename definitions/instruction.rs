use interrupt::InternalInterrupt;
use target::Target;
use typedef::*;
use value::{Type, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Instruction {
    Add(Target, Target),
    Sub(Target, Target),
    Div(Target, Target),
    Mul(Target, Target),
    Rem(Target, Target),

    Cmp(Target, Target),
    Jmp(Address),
    JmpLt(Address),
    JmpGt(Address),
    JmpEq(Address),
    JmpLtEq(Address),
    JmpGtEq(Address),

    Cast(Target, Type),

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),

    Call(Address),
    Ret,

    Halt,
    Pause,
    Int(InternalInterrupt),
}
