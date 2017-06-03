mod mnemonic;
use definitions::typedef::*;
use regex::Regex;

lazy_static! {
    static ref BOOLEAN_RE: Regex = Regex::new("^true|false$").unwrap();
    static ref FLOAT_RE: Regex = Regex::new("^[0-9]+\\.[0-9]+$").unwrap();
    static ref INTEGER_RE: Regex = Regex::new("^[0-9]+$").unwrap();
    static ref COLOR_RE: Regex = Regex::new("^#([0-9abcdefABCDEF]{3}|[0-9abcdefABCDEF]{6})$").unwrap();
    static ref CHAR_RE: Regex = Regex::new("^'(.)'$").unwrap();
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
            let float: Integer = input_integer.parse().unwrap();
            assert_eq!(float, 23);
        }

        let input_non_integer = "42.76543";

        if INTEGER_RE.is_match(input_non_integer) {
            panic!("input does match an integer, but shouldn't");
        }
    }

    #[test]
    fn color_regex() {
        let input_colors = vec!["#fff", "#dea", "#123456", "#AB84E3", "#e732e8", "#975677"];
        let output_values = vec![0xfff, 0xdea, 0x123456, 0xAB84E3, 0xe732e8, 0x975677];

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
}
