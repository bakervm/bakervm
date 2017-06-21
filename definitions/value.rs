//! The value and type definitions

use error::*;
use regex::Regex;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::result;
use std::str::FromStr;
use type_t::Type;
use typedef::*;

lazy_static! {
    static ref ADDRESS_RE: Regex = Regex::new(r"^@(\d+)$").unwrap();
    static ref BOOLEAN_RE: Regex = Regex::new(r"^true|false$").unwrap();
    static ref FLOAT_RE: Regex = Regex::new(r"^(-?\d+)?\.[0-9]+$").unwrap();
    static ref INTEGER_RE: Regex = Regex::new(r"^(-?\d+)?$").unwrap();
    static ref COLOR_RE: Regex = Regex::new(r"^#([0-9abcdefABCDEF]{6})$").unwrap();
    static ref CHAR_RE: Regex = Regex::new(r"^'(.)'$").unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Address(Address), // @12 | @54 | @0 | @1 | ...
    Boolean(bool), // true | false
    Float(Float), // -1.33 | 0.23114 | 3.141 | ...
    Integer(Integer), // 12 | 42 | 1 | 0 | 24 | ...
    Color(u8, u8, u8), // #FF00FF | #bd37b3 | ...
    Char(char), // 'a' | 'b' | 'c' | 'd' | ...
}

impl Value {
    pub fn get_type(&self) -> Type {
        match *self {
            Value::Address(..) => Type::Address,
            Value::Boolean(..) => Type::Boolean,
            Value::Float(..) => Type::Float,
            Value::Integer(..) => Type::Integer,
            Value::Color(..) => Type::Color,
            Value::Char(..) => Type::Char,
        }
    }

    pub fn is_a(&self, val_type: &Type) -> bool {
        &self.get_type() == val_type
    }

    pub fn convert_to(&self, val_type: &Type) -> Self {
        match *self {
            Value::Address(addr) => Self::address_to(addr, val_type),
            Value::Boolean(boolean) => Value::Boolean(boolean),
            Value::Float(float) => Self::float_to(float, val_type),
            Value::Integer(integer) => Self::integer_to(integer, val_type),
            Value::Color(r, g, b) => Self::color_to((r, g, b), val_type),
            Value::Char(character) => Self::char_to(character, val_type),
        }
    }

    fn address_to(addr: Address, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => Value::Integer(addr as Integer),
            Type::Float => Value::Float(addr as Float),
            Type::Color => {
                let addr = addr as u32;

                let r = (addr >> 16) as u8;
                let g = (addr >> 8) as u8;
                let b = addr as u8;

                Value::Color(r, g, b)
            }
            Type::Char => {
                let addr = addr as u8;

                Value::Char(addr as char)
            }
            _ => Value::Address(addr),
        }
    }

    fn float_to(float: Float, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => Value::Integer(float.round() as Integer),
            Type::Address => Value::Address(float.round() as Address),
            _ => Value::Float(float),
        }
    }

    fn integer_to(integer: Integer, val_type: &Type) -> Value {
        match *val_type {
            Type::Address => Value::Address(integer as Address),
            Type::Float => Value::Float(integer as Float),
            Type::Color => {
                let integer = integer as u32;

                let r = (integer >> 16) as u8;
                let g = (integer >> 8) as u8;
                let b = integer as u8;

                Value::Color(r, g, b)
            }
            Type::Char => {
                let integer = integer.abs() as u8;

                Value::Char(integer as char)
            }
            _ => Value::Integer(integer),
        }
    }

    fn color_to((r, g, b): Color, val_type: &Type) -> Value {
        match *val_type {
            Type::Integer => {
                let integer: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

                Value::Integer(integer as Integer)
            }
            Type::Address => {
                let addr: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

                Value::Address(addr as Address)
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

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if ADDRESS_RE.is_match(s) {
            let address_cap = ADDRESS_RE.captures_iter(s).next().unwrap();

            Ok(Value::Address(address_cap[1].parse().unwrap()))
        } else if BOOLEAN_RE.is_match(s) {
            Ok(Value::Boolean(s.parse().unwrap()))
        } else if FLOAT_RE.is_match(s) {
            Ok(Value::Float(s.parse().unwrap()))
        } else if INTEGER_RE.is_match(s) {
            Ok(Value::Integer(s.parse().unwrap()))
        } else if COLOR_RE.is_match(s) {
            let color = COLOR_RE.captures_iter(s).next().unwrap();
            let uint: u32 = u32::from_str_radix(&color[1], 16).unwrap();
            let r = (uint >> 16) as u8;
            let g = (uint >> 8) as u8;
            let b = uint as u8;

            Ok(Value::Color(r, g, b))
        } else if CHAR_RE.is_match(s) {
            let character = CHAR_RE.captures_iter(s).next().unwrap();
            let real_char: char = character[1].chars().next().unwrap();
            Ok(Value::Char(real_char))
        } else {
            Err("failed to parse value")
        }
    }
}

impl Add for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Ok(Value::Float(lhs_float + rhs_float,),),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Ok(Value::Integer(lhs_integer.wrapping_add(rhs_integer)))
            }
            (Value::Address(lhs_addr), Value::Address(rhs_addr)) => {
                Ok(Value::Address(lhs_addr.wrapping_add(rhs_addr)))
            }
            _ => bail!("unable to add values {:?} and {:?}", self, rhs),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Ok(Value::Float(lhs_float - rhs_float,),),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Ok(Value::Integer(lhs_integer.wrapping_sub(rhs_integer)))
            }
            (Value::Address(lhs_addr), Value::Address(rhs_addr)) => {
                Ok(Value::Address(lhs_addr.wrapping_sub(rhs_addr)))
            }
            _ => bail!("unable to subtract values {:?} and {:?}", self, rhs),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Ok(Value::Float(lhs_float * rhs_float,),),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Ok(Value::Integer(lhs_integer.wrapping_mul(rhs_integer)))
            }
            (Value::Address(lhs_addr), Value::Address(rhs_addr)) => {
                Ok(Value::Address(lhs_addr.wrapping_mul(rhs_addr)))
            }
            _ => bail!("unable to multiply values {:?} and {:?}", self, rhs),
        }
    }
}

impl Div for Value {
    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Ok(Value::Float(lhs_float / rhs_float,),),
            _ => bail!("unable to divide values {:?} and {:?}", self, rhs),
        }
    }
}

impl Rem for Value {
    type Output = Result<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Float(lhs_float), Value::Float(rhs_float)) => Ok(Value::Float(lhs_float % rhs_float,),),
            (Value::Integer(lhs_integer), Value::Integer(rhs_integer)) => {
                Ok(Value::Integer(lhs_integer % rhs_integer))
            }
            (Value::Address(lhs_addr), Value::Address(rhs_addr)) => Ok(Value::Address(lhs_addr % rhs_addr,),),
            _ => {
                bail!(
                    "unable to calculate the remainder of values {:?} and {:?}",
                    self,
                    rhs
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_regex() {
        let input_boolean = "true";

        if !BOOLEAN_RE.is_match(input_boolean) {
            panic!("input doesn't match a boolean");
        } else {
            let boolean: bool = input_boolean.parse().unwrap();
            assert_eq!(boolean, true);
        }

        let input_boolean = "false";

        if !BOOLEAN_RE.is_match(input_boolean) {
            panic!("input doesn't match a boolean");
        } else {
            let boolean: bool = input_boolean.parse().unwrap();
            assert_eq!(boolean, false);
        }
    }

    #[test]
    fn float_regex() {
        let input_float = "23.123412";

        if !FLOAT_RE.is_match(input_float) {
            panic!("input doesn't match a float");
        } else {
            let float: Float = input_float.parse().unwrap();
            assert_eq!(float, 23.123412);
        }

        let input_float = "-23.123412";

        if !FLOAT_RE.is_match(input_float) {
            panic!("input doesn't match a float");
        } else {
            let float: Float = input_float.parse().unwrap();
            assert_eq!(float, -23.123412);
        }


        let input_non_float = "42";

        if FLOAT_RE.is_match(input_non_float) {
            panic!("input does match a float, but shouldn't");
        }
    }

    #[test]
    fn integer_regex() {
        let input_integer = "23";

        if !INTEGER_RE.is_match(input_integer) {
            panic!("input doesn't match an integer");
        } else {
            let integer: Integer = input_integer.parse().unwrap();
            assert_eq!(integer, 23);
        }

        let input_integer = "-23";

        if !INTEGER_RE.is_match(input_integer) {
            panic!("input doesn't match an integer");
        } else {
            let integer: Integer = input_integer.parse().unwrap();
            assert_eq!(integer, -23);
        }


        let input_non_integer = "42.76543";

        if INTEGER_RE.is_match(input_non_integer) {
            panic!("input does match an integer, but shouldn't");
        }
    }

    #[test]
    fn color_regex() {
        let input_colors = vec![
            "#ffffff",
            "#deafff",
            "#123456",
            "#AB84E3",
            "#e732e8",
            "#975677",
        ];
        let output_values = vec![0xffffff, 0xdeafff, 0x123456, 0xAB84E3, 0xe732e8, 0x975677];

        for i in 0..input_colors.len() {
            let current_color = input_colors[i].clone();
            let current_value = output_values[i].clone();

            if !COLOR_RE.is_match(current_color) {
                panic!("input doesn't match a color");
            } else {
                let color = COLOR_RE.captures_iter(current_color).next().unwrap();
                let uint: u32 = u32::from_str_radix(&color[1], 16).unwrap();
                assert_eq!(uint, current_value);
            }
        }
    }

    #[test]
    fn char_regex() {
        let input_chars = vec![
            "'a'",
            "'b'",
            "'c'",
            "'d'",
            "'e'",
            "'f'",
            "'g'",
            "'h'",
            "'i'",
        ];
        let output_chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

        for i in 0..input_chars.len() {
            let current_char = input_chars[i].clone();
            let current_value = output_chars[i].clone();

            if !CHAR_RE.is_match(current_char) {
                panic!("input doesn't match a char");
            } else {
                let character = CHAR_RE.captures_iter(current_char).next().unwrap();
                let real_char: char = character[1].chars().next().unwrap();
                assert_eq!(real_char, current_value);
            }
        }
    }

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
