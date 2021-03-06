use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Address,
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
            "addr" => Ok(Type::Address),
            "bool" => Ok(Type::Boolean),
            "float" => Ok(Type::Float),
            "int" => Ok(Type::Integer),
            "color" => Ok(Type::Color),
            "char" => Ok(Type::Char),
            _ => Err("unable to parse type"),
        }
    }
}
