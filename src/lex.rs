use regex::Regex;
use crate::token;

pub struct LexAnalyser<'a> {
    pub code: Vec<&'a str>,
    comment: Regex,
    reserved: Regex,
    identifier: Regex,
    number: Regex,
    operator: Regex,
    punctuation: Regex,
    whitespace: Regex,
}

impl LexAnalyser<'_> {
    pub fn new(code: Vec<&'a str>) -> Self {
        LexAnalyser {
            code,
            comment: Regex::new(r"#.*").unwrap(),
            reserved: Regex::new(r"(int|double|char|float|if|while|for)").unwrap(),
            identifier: Regex::new(r"^[A-Z][a-z]*$").unwrap(),
            number: Regex::new(r"([-+]?\d+)|([-+]?\d*,\d+)").unwrap(),
            operator: Regex::new(r"[+-*/<>&|]+").unwrap(),
            punctuation: Regex::new(r"[(),;.]+").unwrap(),
            whitespace: Regex::new(r"\s+").unwrap(),
        }
    }

    fn tokenize(&self, position: u32, tag: token::TokenTag, name: &str) -> token::Token {
        token::Token::new(position, tag, Some(name.to_string()))
    }

    //fn token_list(&self) {
    //    separar em linhas, e p cada linha filtrar usando as regras e usar o tokenize p cada instancia

        for line in self.code {
            
        }
    //}

    //fn token_table(&self) -> Vec<Token> {
    //    let mut table: Vec<Token> = Vec::new();

    //    for line in self.code {
    //    }
    //}
}