#[derive(Debug)]
pub struct Scanner {
    pub text: Vec<String>,
}

impl Scanner {
    pub fn new(text: String) -> Scanner {
        let splitted = text
            .split(|c| c == ' ' || c == '\n' || c == ';' || c == ',')
            .map(|x| x.to_string())
            .filter(|token| !token.is_empty())
            .collect::<Vec<String>>();

        Scanner { text: splitted }
    }
}
