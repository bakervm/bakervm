use core::{Signal, Target, Type, Value};
use std::fmt::{self, Display, Formatter};

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

    Halt,
    Pause,
    Nop,
    Sig(Signal),
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Mnemonic::Add(ref dest, ref src) => write!(f, "add {}, {}", dest, src),
            &Mnemonic::Sub(ref dest, ref src) => write!(f, "sub {}, {}", dest, src),
            &Mnemonic::Div(ref dest, ref src) => write!(f, "div {}, {}", dest, src),
            &Mnemonic::Mul(ref dest, ref src) => write!(f, "mul {}, {}", dest, src),
            &Mnemonic::Rem(ref dest, ref src) => write!(f, "rem {}, {}", dest, src),

            &Mnemonic::Cmp(ref target_a, ref target_b) => write!(f, "cmp {}, {}", target_a, target_b),
            &Mnemonic::Jmp(ref label) => write!(f, "jmp {}", label),
            &Mnemonic::JmpLt(ref label) => write!(f, "jmplt {}", label),
            &Mnemonic::JmpGt(ref label) => write!(f, "jmpgt {}", label),
            &Mnemonic::JmpEq(ref label) => write!(f, "jmpeq {}", label),
            &Mnemonic::JmpLtEq(ref label) => write!(f, "jmplteq {}", label),
            &Mnemonic::JmpGtEq(ref label) => write!(f, "jmpgteq {}", label),

            &Mnemonic::Cast(ref target, ref type_val) => write!(f, "cast {}, {}", target, type_val),

            &Mnemonic::Push(ref target, ref value) => write!(f, "push {}, {}", target, value),
            &Mnemonic::Mov(ref dest, ref src) => write!(f, "mov {}, {}", dest, src),
            &Mnemonic::Swp(ref target_a, ref target_b) => write!(f, "swp {}, {}", target_a, target_b),
            &Mnemonic::Dup(ref target) => write!(f, "dup {}", target),

            &Mnemonic::Call(ref label) => write!(f, "call {}", label),
            &Mnemonic::Ret => write!(f, "ret"),

            &Mnemonic::Halt => write!(f, "halt"),
            &Mnemonic::Pause => write!(f, "pause"),
            &Mnemonic::Nop => write!(f, "nop"),
            &Mnemonic::Sig(ref signal) => write!(f, "sig {}", signal),
        }
    }
}
