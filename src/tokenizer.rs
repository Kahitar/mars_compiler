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

trait Seeker<T> {
    fn seek(&mut self, offset: usize) -> Option<T>;
    fn consume(&mut self, offset: usize) -> Option<T>;
}

impl Seeker<char> for Tokenizer {
    fn seek(&mut self, offset: usize) -> Option<char> {
        if self.curr_idx + offset >= self.source.len() {
            println!("Reached end of source");
            return None;
        }
        let c = self.source[self.curr_idx + offset];
        Some(c)
    }

    fn consume(&mut self, offset: usize) -> Option<char> {
        let c = self.seek(offset);
        self.curr_idx += offset + 1;
        c
    }
}

pub fn tokenize_iden(tokenizer: &mut Tokenizer) -> Token {
    let mut iden = String::new();
    while tokenizer.seek(0).is_some_and(|c| c.is_alphanumeric()) {
        write!(&mut iden, "{}", tokenizer.consume(0).unwrap()).unwrap();
    }

    if KeywordType::is_keyword(&iden) {
        Token::Keyword(KeywordType::to_keyword(&iden).unwrap())
    } else {
        Token::Iden(iden)
    }
}

pub fn tokenize_int_lit(tokenizer: &mut Tokenizer) -> Token {
    let mut int_lit = String::new();
    while tokenizer.seek(0).is_some_and(|c| c.is_numeric()) {
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
    while tokenizer.seek(0).is_some() {
        let seeked = tokenizer.seek(0);
        if seeked.is_some_and(|c| c.is_alphabetic()) {
            tokens.push(tokenize_iden(&mut tokenizer));
        } else if seeked.is_some_and(|c| c.is_numeric()) {
            tokens.push(tokenize_int_lit(&mut tokenizer));
        } else if seeked.is_some_and(|x| x.is_whitespace()) {
            tokenizer.consume(0);
        } else if seeked.is_some_and(|c| c == ';') {
            tokens.push(Token::Semi);
            tokenizer.consume(0);
        } else if seeked.is_none() {
            println!("Found end of source.");
            break;
        } else {
            eprintln!(
                "unhandled case in tokenizer. Found unexpected character `{}`",
                seeked.unwrap()
            );
            exit(1);
        }
    }
    tokens
}
