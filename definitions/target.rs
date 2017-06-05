//! A Target is an abstract representation of a memory section inside the VM

use regex::Regex;
use std::str::FromStr;
use typedef::*;

lazy_static! {
    static ref VALUEINDEX_RE: Regex = Regex::new(r"^\$vi\((\d+)\)$").unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Framebuffer,
    ValueIndex(Address),
    Stack,
}

impl FromStr for Target {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if VALUEINDEX_RE.is_match(s) {
            let value = VALUEINDEX_RE.captures_iter(s).next().unwrap();
            let index: Address = value[1].parse().unwrap();

            Ok(Target::ValueIndex(index))
        } else if s == "$fb" {
            Ok(Target::Framebuffer)
        } else if s == "$st" {
            Ok(Target::Stack)
        } else {
            Err("unable to parse target")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_index_regex() {
        let input = "$vi(123)";

        if !VALUEINDEX_RE.is_match(input) {
            panic!("input doesn't match a value index literal");
        } else {
            let value = VALUEINDEX_RE.captures_iter(input).next().unwrap();
            let index: Address = value[1].parse().unwrap();
            assert_eq!(index, 123);
        }
    }
}
