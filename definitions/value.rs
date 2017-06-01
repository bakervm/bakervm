use std::ops::{Add, Div, Mul, Rem, Sub};
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Boolean(bool), // true | false
    Float(Float), // -1.33 | 0.23114 | 3.141 | ...
    Integer(Integer), // 12 | 42 | 1 | 0 | 24 | ...
    Color(u8, u8, u8), // #FF00FF | #bd37b3 | ...
    Char(char), // 'a' | 'b' | 'c' | 'd' | ...
    Undefined, // The Undefined value symbolizes an internal error or a wrong use of the bytecode
}

impl Default for Value {
    fn default() -> Self {
        Value::Undefined
    }
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
        match (self, rhs) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Value::Float(lhs_float + rhs_float,),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Value::Integer(lhs_integer + rhs_integer)
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
        match (self, rhs) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Value::Float(lhs_float * rhs_float,),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Value::Integer(lhs_integer * rhs_integer)
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
