use interrupt::InternalInterrupt;
use signal::Signal;
use target::Target;
use typedef::*;
use value::Value;

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

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),

    Call(Address),
    Ret,

    Halt,
    Int(InternalInterrupt),
    Ext(Signal, Address),
}
