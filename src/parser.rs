use std::process::exit;

use crate::tokenizer::{KeywordType, Token};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub curr_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            curr_idx: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.curr_idx + offset).cloned()
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.peek(0)?;
        self.curr_idx += 1;
        Some(token)
    }

    fn parse_keyword_expression(&mut self) -> Result<String, &'static str> {
        match self.consume() {
            Some(Token::Keyword(KeywordType::Exit)) => self.parse_keyword_exit(),
            _ => Err("Unhandled KeywordType"),
        }
    }

    fn parse_keyword_exit(&mut self) -> Result<String, &'static str> {
        let int_value = match self.consume() {
            Some(Token::IntLit(x)) => x,
            _ => return Err("Expected integer literal"),
        };
        if self.consume() != Some(Token::Semi) {
            return Err("Expected semi token");
        }
        Ok(format!(
            "    mov rax, 60\n    mov rdi, {}\n    syscall",
            int_value
        ))
    }

    pub fn parse(&mut self) -> Result<String, &'static str> {
        let mut assembly = "global _start\n_start:\n".to_string();
        while let Some(t) = self.peek(0) {
            match t {
                Token::Keyword(_) => assembly.push_str(&self.parse_keyword_expression().unwrap()),
                _ => {
                    eprintln!("unhandled case in parser. Found unexpected token {:?}", t);
                    exit(1);
                }
            }
        }
        Ok(assembly)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<String, &'static str> {
    Parser::new(tokens).parse()
}
