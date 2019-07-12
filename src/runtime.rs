use super::tokenize::*;
use log::info;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Clone, Debug)]
enum Types {
    String(String),
    Number(i64),
    Boolean(bool),
    Null,
}

impl Add for Types {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
            (Types::Number(a), Types::Number(b)) => Types::Number(a + b),
            (a, b) => {
                let a_string = format!("{:?}", a);
                let b_string = format!("{:?}", b);

                let a_type = a_string.split("(").collect::<Vec<&str>>();
                let b_type = b_string.split("(").collect::<Vec<&str>>();

                panic!(
                    "Cannot add types {} and {}",
                    a_type.first().unwrap(),
                    b_type.first().unwrap()
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Executor {
    pub tokens: Vec<Token>,
    variables: HashMap<String, Types>,
}

impl Executor {
    pub fn new(tokens: Vec<Token>) -> Executor {
        Executor {
            tokens,
            variables: HashMap::new(),
        }
    }

    fn parse_value(&mut self, value: &Token) -> Types {
        let var_val = match value {
            Token::Number { value: e } => Types::Number(*e),
            Token::String { value: text } => Types::String(text.to_string()),
            Token::Variable { identifier: ident } => self
                .variables
                .get(ident)
                .expect(&format!("Variable not found {}", ident))
                .clone(),
            Token::Boolean { value: b } => Types::Boolean(*b),
            _ => Types::Null,
        };

        if let Token::Operator { op: Operator::Plus } =
            self.tokens.first().unwrap_or(&Token::Nothing)
        {
            &self.tokens.remove(0);

            let next_token = &self.tokens.remove(0);

            let to_add = self.parse_value(next_token);

            var_val + to_add
        } else {
            var_val
        }
    }

    pub fn execute(&mut self) {
        while !self.tokens.is_empty() {
            let next_token = &self.tokens.remove(0);

            match next_token {
                Token::Keyword { kw: Keyword::Let } => {
                    let ident = &self.tokens.remove(0);
                    match ident {
                        Token::Variable { identifier: ident } => {
                            let equals = &self.tokens.remove(0);

                            match equals {
                                Token::Operator {
                                    op: Operator::Equals,
                                } => {
                                    let value = &self.tokens.remove(0);
                                    let var_val = self.parse_value(value);

                                    self.variables.insert(ident.to_string(), var_val);
                                }
                                _ => panic!("Equals expected"),
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        info!("{:?}", self);
    }
}
