use std::ops::{Add, Div, Mul, Rem, Sub};
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Boolean,
    Float,
    Integer,
    Color,
    Char,
    Undefined,
}

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

    pub fn get_type(&self) -> Type {
        match *self {
            Value::Boolean(..) => Type::Boolean,
            Value::Float(..) => Type::Float,
            Value::Integer(..) => Type::Integer,
            Value::Color(..) => Type::Color,
            Value::Char(..) => Type::Char,
            _ => Type::Undefined,
        }
    }

    pub fn is_a(&self, val_type: &Type) -> bool {
        &self.get_type() == val_type
    }

    pub fn convert_to(&self, val_type: &Type) -> Self {
        match *self {
            Value::Boolean(boolean) => Value::Boolean(boolean),
            Value::Float(float) => Self::float_to(float, val_type),
            Value::Integer(integer) => Self::integer_to(integer, val_type),
            Value::Color(r, g, b) => Self::color_to((r, g, b), val_type),
            Value::Char(character) => Self::char_to(character, val_type),
            _ => Value::Undefined,
        }
    }

    fn float_to(float: Float, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => Value::Integer(float as Integer),
            _ => Value::Float(float),
        }
    }

    fn integer_to(integer: Integer, val_type: &Type) -> Value {
        match *val_type {
            Type::Float => Value::Float(integer as Float),
            Type::Color => {
                let integer = integer as u32;

                let r = (integer >> 16) as u8;
                let g = (integer >> 8) as u8;
                let b = integer as u8;

                Value::Color(r, g, b)
            }
            Type::Char => {
                let integer = integer as u8;

                Value::Char(integer as char)
            }
            _ => Value::Integer(integer),
        }
    }

    fn color_to((r, g, b): Color, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => {
                let mut integer: u32 = (r as u32) << 16;
                integer |= (g as u32) << 8;
                integer |= b as u32;

                Value::Integer(integer as Integer)
            }
            _ => Value::Color(r, g, b),
        }
    }

    fn char_to(character: char, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => Value::Integer(character as Integer),
            _ => Value::Char(character),
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

    #[test]
    fn conversion() {
        let initial_color = Value::Integer(0xFF4422).convert_to(&Type::Color);

        assert_eq!(
            Value::Integer(0xFF4422),
            initial_color.convert_to(&Type::Integer)
        );
    }
}
