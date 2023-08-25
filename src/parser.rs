use std::process::exit;

use crate::{
    peeker::Peeker,
    tokenizer::{KeywordType, Token},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub curr_idx: usize,
}

impl Peeker<Token> for Parser {
    fn peek(&self, offset: usize) -> Option<Token> {
        if self.curr_idx + offset >= self.tokens.len() {
            return None;
        }
        let c = self.tokens[self.curr_idx + offset].clone();
        Some(c)
    }
    fn consume(&mut self, offset: usize) -> Option<Token> {
        let c = self.peek(offset);
        self.curr_idx += offset + 1;
        c
    }
}

pub fn parse(tokens: Vec<Token>) -> String {
    let mut parser = Parser {
        tokens,
        curr_idx: 0,
    };
    let mut assembly = "global _start\n_start:\n".to_string();
    while let Some(t) = parser.peek(0) {
        match t {
            Token::Keyword(_) => assembly.push_str(&parse_keyword_expression(&mut parser)),
            _ => {
                eprintln!("unhandled case in parser. Found unexpected token {:?}", t);
                exit(1);
            }
        }
    }
    assembly
}

fn parse_keyword_expression(parser: &mut Parser) -> String {
    let keyword_token = parser.peek(0).unwrap();
    match keyword_token {
        Token::Keyword(keyword_type) => match keyword_type {
            KeywordType::Exit => parse_keyword_exit(parser),
            _ => {
                eprintln!("Unhandled KeywordType '{:?}'", keyword_type);
                exit(1);
            }
        },
        _ => {
            eprintln!(
                "Expected 'Keyword' token type but found '{:?}'",
                keyword_token
            );
            exit(1);
        }
    }
}

fn parse_keyword_exit(parser: &mut Parser) -> String {
    parser.consume(0); // exit token

    let mut exit_assembly = String::new();
    if peek_matches_tokens(parser, vec![Token::IntLit(String::new()), Token::Semi]) {
        let int_value = get_int_lit(parser.consume(0).unwrap());
        parser.consume(0); // semi token
        exit_assembly.push_str("    mov rax, 60\n");
        exit_assembly.push_str("    mov rdi, ");
        exit_assembly.push_str(&int_value);
        exit_assembly.push('\n');
        exit_assembly.push_str("    syscall");
    } else {
        let actual = parser.peek(0);
        eprintln!("Expected integer literal, but found '{:?}'", actual)
    }
    exit_assembly
}

fn get_int_lit(int_lit_token: Token) -> String {
    match int_lit_token {
        Token::IntLit(x) => x,
        _ => {
            eprintln!("Expected integer literal");
            exit(1);
        }
    }
}

fn peek_matches_tokens(parser: &Parser, tokens: Vec<Token>) -> bool {
    tokens
        .iter()
        .enumerate()
        .all(|(offset, token)| peek_matches_token(parser, offset, token))
}

fn peek_matches_token(parser: &Parser, offset: usize, token: &Token) -> bool {
    match parser.peek(offset) {
        Some(t) => match t {
            token => true,
            _ => false,
        },
        None => false,
    }
}
