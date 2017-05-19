use std::ops::{Add, Div, Mul, Rem, Sub};
use typedef::*;

pub const PREAMBLE: &str = "BAKERVM";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Boolean(bool), // true | false
    Float(Float), // -1.33 | 0.23114 | 3.141 | ...
    Integer(Integer), // 12 | 42 | 1 | 0 | 24 | ...
    Symbol(String), // :hello | :test | :symbol | ...
    String(String), // "hello world" | "hello!" | "yellow \"blue\" or red" | ...
    Char(char), // 'a' | 'b' | 'c' | 'd' | ...
    Nil, // nil
}

impl Value {
    pub fn is_nil(&self) -> bool {
        match self {
            &Value::Nil => true,
            _ => false,
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs_float) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float(lhs_float + rhs_float),
                    Value::Integer(rhs_integer) => Value::Float(lhs_float + (rhs_integer as Float)),
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_float, rhs_string),),
                    _ => Value::Nil,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float((lhs_integer as Float) + rhs_float),
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer + rhs_integer),
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_integer, rhs_string),),
                    _ => Value::Nil,
                }
            }

            Value::String(lhs_string) => {
                match rhs {
                    Value::Float(rhs_float) => Value::String(format!("{}{}", lhs_string, rhs_float),),
                    Value::Integer(rhs_integer) => Value::String(format!("{}{}", lhs_string, rhs_integer),),
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_string, rhs_string),),
                    _ => Value::Nil,
                }
            }
            _ => Value::Nil,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs_float) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float(lhs_float - rhs_float),
                    Value::Integer(rhs_integer) => Value::Float(lhs_float - (rhs_integer as Float)),
                    _ => Value::Nil,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float((lhs_integer as Float) - rhs_float),
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer - rhs_integer),
                    _ => Value::Nil,
                }
            }
            _ => Value::Nil,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs_float) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float(lhs_float * rhs_float),
                    Value::Integer(rhs_integer) => Value::Float(lhs_float * (rhs_integer as Float)),
                    _ => Value::Nil,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float((lhs_integer as Float) * rhs_float),
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer * rhs_integer),
                    _ => Value::Nil,
                }
            }
            _ => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs_float) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float(lhs_float / rhs_float),
                    Value::Integer(rhs_integer) => Value::Float(lhs_float / (rhs_integer as Float)),
                    _ => Value::Nil,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float((lhs_integer as Float) / rhs_float),
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer / rhs_integer),
                    _ => Value::Nil,
                }
            }
            _ => Value::Nil,
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs_float) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float(lhs_float % rhs_float),
                    Value::Integer(rhs_integer) => Value::Float(lhs_float % (rhs_integer as Float)),
                    _ => Value::Nil,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Float(rhs_float) => Value::Float((lhs_integer as Float) % rhs_float),
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer % rhs_integer),
                    _ => Value::Nil,
                }
            }
            _ => Value::Nil,
        }
    }
}

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
