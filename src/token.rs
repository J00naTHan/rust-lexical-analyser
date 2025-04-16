use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TokenTag {
    Reserved,
    Identifier,
    Number,
    Operator,
    Punctuation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub position: u32,
    pub tag: TokenTag,
    pub name: Option<String>,
}

impl Token {
    pub fn new(position: u32, tag: TokenTag, name: Option<String>) -> Self {
        Token { position, tag, name }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}, {}", self.tag, self.position)
    }
}
