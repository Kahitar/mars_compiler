use std::{fmt::Write, process::exit};

#[derive(Debug)]
pub enum Token {
    Keyword(KeywordType),
    IntLit(String),
    Semi,
    Iden(String),
}

#[derive(Debug)]
pub enum KeywordType {
    Exit,
}

impl KeywordType {
    fn is_keyword(word: &str) -> bool {
        KeywordType::to_keyword(word).is_some()
    }
    fn to_keyword(word: &str) -> Option<KeywordType> {
        match word {
            "exit" => Some(KeywordType::Exit),
            _ => None,
        }
    }
}

pub struct Tokenizer {
    pub source: Vec<char>,
    pub curr_idx: usize,
}

trait Peaker<T> {
    fn peak(&mut self, offset: usize) -> Option<T>;
    fn consume(&mut self, offset: usize) -> Option<T>;
}

impl Peaker<char> for Tokenizer {
    fn peak(&mut self, offset: usize) -> Option<char> {
        if self.curr_idx + offset >= self.source.len() {
            println!("Reached end of source");
            return None;
        }
        let c = self.source[self.curr_idx + offset];
        Some(c)
    }

    fn consume(&mut self, offset: usize) -> Option<char> {
        let c = self.peak(offset);
        self.curr_idx += offset + 1;
        c
    }
}

pub fn tokenize_iden(tokenizer: &mut Tokenizer) -> Token {
    let mut iden = String::new();
    while tokenizer.peak(0).is_some_and(|c| c.is_alphanumeric()) {
        iden.push(tokenizer.consume(0).unwrap());
    }

    if KeywordType::is_keyword(&iden) {
        Token::Keyword(KeywordType::to_keyword(&iden).unwrap())
    } else {
        Token::Iden(iden)
    }
}

pub fn tokenize_int_lit(tokenizer: &mut Tokenizer) -> Token {
    let mut int_lit = String::new();
    while tokenizer.peak(0).is_some_and(|c| c.is_numeric()) {
        write!(&mut int_lit, "{}", tokenizer.consume(0).unwrap()).unwrap();
    }
    Token::IntLit(int_lit)
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        source: src.chars().collect(),
        curr_idx: 0,
    };

    let mut tokens: Vec<Token> = vec![];
    while let Some(c) = tokenizer.peak(0) {
        match c {
            c if c.is_alphabetic() => tokens.push(tokenize_iden(&mut tokenizer)),
            c if c.is_numeric() => tokens.push(tokenize_int_lit(&mut tokenizer)),
            c if c.is_whitespace() => {
                tokenizer.consume(0);
            }
            ';' => {
                tokens.push(Token::Semi);
                tokenizer.consume(0);
            }
            _ => {
                eprintln!(
                    "unhandled case in tokenizer. Found unexpected character `{}`",
                    c
                );
                exit(1);
            }
        }
    }

    println!("Found end of source.");
    tokens
}
