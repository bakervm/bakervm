use ast::Expression;
use definitions::program::Value;
use definitions::typedef::*;
use error::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn compile(file: File) -> ImageData {
    unimplemented!()
}

#[derive(PartialEq, Clone, Debug)]
enum Token {
    OpenBrace,
    ClosedBrace,
    Symbol(String),
    Literal(Value),
}

struct Tokenizer {
    string_flag: bool,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { string_flag: false }
    }

    fn tokenize(&mut self, file: File) -> CompilationResult<Vec<Token>> {
        let reader = BufReader::new(file);

        let mut tokenstream = Vec::new();

        let mut word: Vec<char> = Vec::new();

        for line in reader.lines() {
            let line = line.chain_err(|| "decoding line failed")?;

            for character in line.chars() {
                print!("{:?}", character);

                if !(character.is_alphanumeric() || self.string_flag || word.is_empty()) {
                    let sym: String = word.iter().cloned().collect();
                    tokenstream.push(Token::Symbol(sym));
                    word.clear();
                }

                if self.string_flag && character != '"' {
                    word.push(character);
                    continue;
                }

                match character {
                    '(' => tokenstream.push(Token::OpenBrace),
                    ')' => tokenstream.push(Token::ClosedBrace),
                    '"' => self.string_flag = !self.string_flag,
                    chr if chr.is_whitespace() => continue,
                    chr if chr.is_alphanumeric() => word.push(chr),
                    chr => bail!("weird thing you got there: {:?}", chr),
                }
            }
        }

        Ok(tokenstream)
    }
}


struct LispParser {
    tokens: Vec<Token>,
    counter: usize,
}

impl LispParser {
    pub fn new() -> LispParser {
        LispParser {
            tokens: Vec::new(),
            counter: 0,
        }
    }

    pub fn parse(&mut self, tokenstream: Vec<Token>) -> CompilationResult<Vec<Expression>> {
        self.tokens = tokenstream;
        self.counter = 0;

        let mut program: Vec<Expression> = Vec::new();

        while let Some(token) = self.tokens.get(self.counter).cloned() {
            let ast = match token {
                Token::Symbol(inner) => Expression::Symbol(inner),
                Token::OpenBrace => self.list().chain_err(|| "unable to parse list")?,
                x => {
                    bail!(
                        "expected one of (Token::Symbol(_), Token::OpenBrace). Got {:?}",
                        x
                    )
                }
            };

            program.push(ast);
        }

        Ok(program)
    }

    fn advance_counter(&mut self) {
        self.counter += 1;
    }

    fn symbol(&mut self) -> CompilationResult<Expression> {
        if let Some(Token::Symbol(inner)) = self.tokens.get(self.counter).cloned() {
            self.advance_counter();
            Ok(Expression::Symbol(inner.clone()))
        } else {
            bail!("expected symbol");
        }
    }

    fn match_symbol(&mut self) -> bool {
        match self.tokens.get(self.counter) {
            Some(&Token::Symbol(_)) => true,
            _ => false,
        }
    }

    fn list(&mut self) -> CompilationResult<Expression> {
        let mut ast: Vec<Expression> = Vec::new();

        if self.match_token(Token::OpenBrace) {
            self.advance_counter();

            while let Some(_) = self.tokens.get(self.counter).cloned() {
                if self.match_token(Token::OpenBrace) {
                    ast.push(self.list().chain_err(|| "unable to parse list")?);
                } else if self.match_symbol() {
                    ast.push(self.symbol().chain_err(|| "unable to parse symbol")?);
                } else {
                    break;
                }
            }

            if self.match_token(Token::ClosedBrace) {
                self.advance_counter();
            } else {
                bail!("expected Token::ClosedBrace");
            }
        }

        Ok(Expression::List(ast))
    }

    fn match_token(&mut self, token: Token) -> bool {
        self.tokens.get(self.counter) == Some(&token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parser() {
        let tokenstream = vec![
            Token::OpenBrace,
            Token::Symbol(String::from("defun")),
            Token::Symbol(String::from("defun")),
            Token::OpenBrace,
            Token::Symbol(String::from("defun")),
            Token::Symbol(String::from("defun")),
            Token::Symbol(String::from("defun")),
            Token::ClosedBrace,
            Token::ClosedBrace,
            Token::OpenBrace,
            Token::OpenBrace,
            Token::ClosedBrace,
            Token::ClosedBrace,
        ];

        let ast = LispParser::new().parse(tokenstream);

        println!("{:#?}", ast);
    }

    #[test]
    fn tokenizer() {
        let mut file = File::create("foo.txt").expect("shit");
        file.write_all(b"(defun hallo(x) \"blabl  v s !! ^#*^!*&$fsdf sdf ablabal\")").expect("shit");

        let mut file = File::open("foo.txt").expect("shit");

        let tokens = Tokenizer::new().tokenize(file);

        println!("{:#?}", tokens);

        fs::remove_file("foo.txt");
    }
}
