const KEYWORDS: &'static [&'static str] = &["let", "true", "false"];
const OPERATORS: &'static [&'static str] = &["=", "+", "-", "*", "/"];

#[derive(Clone, Debug)]
pub enum Keyword {
    Let,
    Nothing,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Equals,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Nothing,
}

#[derive(Clone, Debug)]
pub enum Token {
    Boolean { value: bool },
    String { value: String },
    Variable { identifier: String },
    Number { value: i64 },
    Keyword { kw: Keyword },
    Operator { op: Operator },
    Nothing,
}

#[derive(Debug)]
pub struct Tokenizer {
    pub tokens: Vec<Token>,
    is_parsing_string: bool,
    temp_string: String,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: vec![],
            is_parsing_string: false,
            temp_string: String::new(),
        }
    }

    pub fn add_token(&mut self, token: String) {
        let is_keyword = KEYWORDS.contains(&&token.as_str());
        let is_operator = OPERATORS.contains(&&token.as_str());
        let is_number = token.parse::<i64>().is_ok();
        let is_string_start = token.starts_with('"');
        let is_string_end = token.ends_with('"');

        if self.is_parsing_string && is_string_end {
            self.temp_string.push_str(" ");
            self.temp_string.push_str(token.trim_end_matches('"'));

            let token = Token::String {
                value: self.temp_string.clone(),
            };

            self.temp_string = String::new();
            self.is_parsing_string = false;

            self.tokens.push(token);

            return;
        }

        if self.is_parsing_string {
            self.temp_string.push_str(" ");
            self.temp_string.push_str(&token);

            return;
        }

        let tok = match token {
            _ if is_keyword => Tokenizer::match_keyword(token),
            _ if is_operator => Tokenizer::match_operator(token),
            _ if is_number => Token::Number {
                value: token.parse::<i64>().unwrap(),
            },
            _ if is_string_start => {
                if is_string_end {
                    let string = token.trim_start_matches('"').trim_end_matches('"');
                    Token::String {
                        value: string.to_string(),
                    }
                } else {
                    self.is_parsing_string = true;

                    self.temp_string.push_str(token.trim_start_matches('"'));

                    return;
                }
            }
            _ => Token::Variable { identifier: token },
        };

        self.tokens.push(tok);
    }

    fn match_keyword(token: String) -> Token {
        let kw = match token.as_str() {
            "let" => Keyword::Let,
            "true" => return Token::Boolean { value: true },
            "false" => return Token::Boolean { value: false },
            _ => Keyword::Nothing,
        };

        Token::Keyword { kw }
    }

    fn match_operator(token: String) -> Token {
        let op = match token.as_str() {
            "=" => Operator::Equals,
            "+" => Operator::Plus,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => Operator::Nothing,
        };

        Token::Operator { op }
    }
}
