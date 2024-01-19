//! # The scripting system of Mutemaanpa-game
//!
//! The scripting system of mutemaanpa composes character dialogues and interacts with the environment.
//!
//! The scripting system manages four elements:
//! - text: what everyone says and the sequence/condition of them.
//! - audio: character voices and background music
//! - animation: scripting system does **not** design the animation. It simply says there is one or not.
//! - effects: when side effects are accessed or mutated, it will invoke rust functions to do them.
//!
//! The format resembles S-expressions and is processes by an interpreter as the result.
//!
//! The language:
//!
//! <atomics> := symbol
//!            | name
//!            | text
//!            | number
//!
//! <expr> :=  atomics
//!         | (name <expr> ...)
//!
//! <scripts> := <comments>
//!            | <expr>
//!
//! <comments> := ;; .... \n
//!
//! # examples
//!
//! ;; prologue
//! "Hello"
//! "world"
//!
//! ;; condition
//!
//! "hello"
//! (cond
//!     [(> 1 0) "world"]
//!     [else "hello"]
//! )

use std::str::Chars;

use tracing::info;

#[derive(Debug)]
pub struct Lexer<'a> {
    script: Chars<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Comment(String),
}

const EOF: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(script: Chars) -> Lexer {
        Lexer { script }
    }

    pub fn parse_script(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        self.eat_while(predicates::is_whitespace);
        while !self.is_eof() {
            tokens.push(self.parse_token());
            self.eat_while(predicates::is_whitespace);
        }
        tokens
    }

    pub fn parse_token(&mut self) -> Token {
        match self.peek() {
            ';' => {
                info!("parsed comment");
                self.consume().unwrap();
                Token::Comment(self.eat_while_build(predicates::not_clrf))
            }
            '"' => {
                info!("parsed string");
                self.consume().unwrap();
                let s = self.eat_while_build(predicates::not_quote);
                assert_eq!(self.consume().unwrap(), '\"');
                Token::String(s)
            }
            c  => panic!("unexpected token: {}", c),
        }
    }

    fn peek(&self) -> char {
        self.script.clone().next().unwrap_or(EOF)
    }

    fn is_eof(&self) -> bool {
        self.script.as_str().is_empty()
    }

    fn consume(&mut self) -> Option<char> {
        self.script.next()
    }

    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.peek()) && !self.is_eof() {
            self.consume();
        }
    }

    fn eat_while_build(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut s = String::from("");
        while predicate(self.peek()) && !self.is_eof() {
            s.push(self.consume().unwrap());
        }
        s
    }
}

mod predicates {
    macro_rules! define_predicate_not {
        ($l:literal, $i: ident) => {
            pub fn $i(c: char) -> bool {
                !$l.contains(c)
            }
        };
    }

    define_predicate_not!("\r\n", not_clrf);
    define_predicate_not!("\"", not_quote);

    pub fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils;

    use super::*;

    #[test]
    fn test_lexer() {
        tests_utils::logging_init();
        let script = r#"
            ; this is a comment
            "hello"
            "world"
        "#;
        let mut lexer = Lexer::new(script.chars());
        let tokens = lexer.parse_script();
        assert_eq!(
            tokens,
            vec![
                Token::Comment(" this is a comment".to_string()),
                Token::String("hello".to_string()),
                Token::String("world".to_string()),
            ]
        );
    }
}