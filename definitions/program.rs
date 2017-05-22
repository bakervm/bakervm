use typedef::*;
use value::Value;

pub const PREAMBLE: &str = "BAKERVM";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Register(usize),
    Stack(usize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Instruction {
    Add(Target, Target),
    Sub(Target, Target),
    Div(Target, Target),
    Mul(Target, Target),
    Mod(Target, Target),

    Cmp(Target, Target),
    Jmp(Address),
    JmpLt(Address),
    JmpGt(Address),
    JmpEq(Address),

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),

    Call(Address),
    Ret,

    Halt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub preamble: String,
    pub version: String,
    pub instructions: Vec<Instruction>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            preamble: String::from(PREAMBLE),
            version: String::from(env!("CARGO_PKG_VERSION")),
            instructions: Vec::new(),
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
