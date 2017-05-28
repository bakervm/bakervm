use config::VMConfig;
use typedef::*;
use value::Value;

pub const PREAMBLE: &str = "BAKERVM";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interrupt {
    pub signal_id: usize,
    pub args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalInterrupt {
    FlushFramebuffer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Framebuffer(Address),
    ValueIndex(Address),
    Stack,
}

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
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub preamble: String,
    pub version: String,
    pub config: VMConfig,
    pub instructions: Vec<Instruction>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            preamble: String::from(PREAMBLE),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config: Default::default(),
            instructions: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison() {
        assert!(Value::Float(1.3) < Value::Float(1.4));
        assert!(Value::Float(3.1) > Value::Float(2.4));
        assert!(Value::Float(9.6) == Value::Float(9.6));

        assert!(Value::Integer(124) < Value::Integer(234));
        assert!(Value::Integer(4) > Value::Integer(1));
        assert!(Value::Integer(839) == Value::Integer(839));
    }
}
