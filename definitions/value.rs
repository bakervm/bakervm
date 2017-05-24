use std::ops::{Add, Div, Mul, Rem, Sub};
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Boolean(bool), // true | false
    Float(Float), // -1.33 | 0.23114 | 3.141 | ...
    Integer(Integer), // 12 | 42 | 1 | 0 | 24 | ...
    Symbol(String), // :hello | :test | :symbol | ...
    String(String), // "hello world" | "hello!" | "yellow \"blue\" or red" | ...
    Char(char), // 'a' | 'b' | 'c' | 'd' | ...
    Undefined, // The Undefined value symbolizes an internal error or a wrong use of the bytecode
}

impl Value {
    pub fn is_undefined(&self) -> bool {
        match self {
            &Value::Undefined => true,
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
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_float, rhs_string),),
                    _ => Value::Undefined,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer + rhs_integer),
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_integer, rhs_string),),
                    _ => Value::Undefined,
                }
            }

            Value::String(lhs_string) => {
                match rhs {
                    Value::Float(rhs_float) => Value::String(format!("{}{}", lhs_string, rhs_float),),
                    Value::Integer(rhs_integer) => Value::String(format!("{}{}", lhs_string, rhs_integer),),
                    Value::String(rhs_string) => Value::String(format!("{}{}", lhs_string, rhs_string),),
                    _ => Value::Undefined,
                }
            }
            _ => Value::Undefined,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Value::Float(lhs_float - rhs_float,),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Value::Integer(lhs_integer - rhs_integer)
            }
            _ => Value::Undefined,
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
                    _ => Value::Undefined,
                }
            }

            Value::Integer(lhs_integer) => {
                match rhs {
                    Value::Integer(rhs_integer) => Value::Integer(lhs_integer * rhs_integer),
                    _ => Value::Undefined,
                }
            }
            _ => Value::Undefined,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Value::Float(lhs_float / rhs_float,),
            _ => Value::Undefined,
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Value::Float(lhs_float % rhs_float,),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Value::Integer(lhs_integer % rhs_integer)
            }
            _ => Value::Undefined,
        }
    }
}
