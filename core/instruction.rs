//! The instructions, the VM is able to interpret.

use signal::Signal;
use target::Target;
use type_t::Type;
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

    Cast(Target, Type),

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),
    Dup(Target),

    Call(Address),
    Ret,

    Halt,
    Pause,
    Nop,
    Sig(Signal),
}
