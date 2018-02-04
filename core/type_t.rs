use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Size,
    Boolean,
    Float,
    Integer,
    Color,
    Char,
}

impl FromStr for Type {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "size" => Ok(Type::Size),
            "bool" => Ok(Type::Boolean),
            "float" => Ok(Type::Float),
            "int" => Ok(Type::Integer),
            "color" => Ok(Type::Color),
            "char" => Ok(Type::Char),
            _ => Err("unable to parse type"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Type::Size => write!(f, "size"),
            &Type::Boolean => write!(f, "bool"),
            &Type::Float => write!(f, "float"),
            &Type::Integer => write!(f, "int"),
            &Type::Color => write!(f, "color"),
            &Type::Char => write!(f, "char"),
        }
    }
}
