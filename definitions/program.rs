use std::ops::{Add, Div, Mul, Rem, Sub};
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(Number),
    String(String),
    Nil,
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
        match (self, rhs) {
            (Value::Number(lhs_num), Value::Number(rhs_num)) => Value::Number(lhs_num + rhs_num),
            (Value::String(lhs_string), Value::String(rhs_string)) => {
                Value::String(lhs_string + rhs_string.as_str())
            }

            (Value::Number(lhs_num), Value::String(rhs_string)) => {
                Value::String(format!("{}{}", lhs_num, rhs_string))
            }
            (Value::String(lhs_string), Value::Number(rhs_num)) => {
                Value::String(format!("{}{}", lhs_string, rhs_num))
            }

            _ => Value::Nil,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_num), Value::Number(rhs_num)) => Value::Number(lhs_num - rhs_num),
            _ => Value::Nil,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_num), Value::Number(rhs_num)) => Value::Number(lhs_num * rhs_num),

            (Value::Number(lhs_num), Value::String(rhs_string)) => {
                let mut result_string = String::new();

                for _ in 0..(lhs_num as i64) {
                    result_string += rhs_string.as_str();
                }

                Value::String(result_string)
            }
            (Value::String(lhs_string), Value::Number(rhs_num)) => {
                let mut result_string = String::new();

                for _ in 0..(rhs_num as i64) {
                    result_string += lhs_string.as_str();
                }

                Value::String(result_string)
            }

            _ => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_num), Value::Number(rhs_num)) => Value::Number(lhs_num / rhs_num),
            _ => Value::Nil,
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_num), Value::Number(rhs_num)) => Value::Number(lhs_num % rhs_num),
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
