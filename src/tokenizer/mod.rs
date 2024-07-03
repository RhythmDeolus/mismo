pub mod token;

use std::collections::HashMap;

use token::{Token, TokenType};

pub struct Tokenizer<'a> {
    pub pointer: usize,
    pub contents: &'a Vec<char>,
    pub line_no: usize,
    pub col_no: usize,
}


impl<'a> Tokenizer<'a> {
    pub fn create(contents: &'a Vec<char>) -> Self {
        Tokenizer {
            pointer: 0,
            line_no: 1,
            col_no: 1,
            contents,
        }
    }

    fn advance(&mut self) {
        if self.is_eof() {
            return;
        }
        self.col_no += 1;
        self.pointer += 1;
    }

    // TODO Look for a better way to write it
    fn match_char(&mut self, c: char) -> bool {
        let actual = self.peek();
        match actual {
            Some(val) => {
                if val == c {
                    self.advance();
                    return true;
                }
                false
            }
            None => false,
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        Some(self.contents[self.pointer])

    }

    fn is_eof(&self) -> bool {
        self.contents.len() <= self.pointer || self.contents[self.pointer] == '\0'
    }

    fn _peek_ahead(&mut self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        if self.contents.len() > self.pointer  ||
            self.contents[self.pointer] == '\0' {
            return None;
        }
        Some(self.contents[self.pointer + 1])
    }

    fn get_token(&mut self, t_type: TokenType, i: usize) -> Token {
        let mut literal: Vec<char> = vec!['a'; self.pointer - i];
        println!("i: {}, p: {}", i, self.pointer);
        println!("len: {}", literal.len());
        literal.copy_from_slice(&self.contents[i..self.pointer]);
        Token {
            t_type,
            line_no: self.line_no,
            literal: literal.into_iter().collect(),
            col_no: i,
        }
    }
    
    fn check(&mut self, c: char) -> bool { // match but not consume
        let t = self.peek();
        match t {
            None => false,
            Some(x) => x == c,
        }
    }

    fn string(&mut self, c: char) -> Token{
        let i = self.pointer;
        while !self.is_eof() && !self.check(c) {
            if self.check('\n') {
                panic!("Unexpected line feed during string parsing");
            }
            self.advance();
        }
        let t = self.get_token(TokenType::String, i);
        if !self.is_eof() { self.advance() }
        t
    }

    fn is_numeric(&mut self) -> bool {
        match self.peek() {
            None => false,
            Some(x) => x.is_numeric(),
        }
    }

    fn is_alphabetic(&mut self) -> bool {
        match self.peek() {
            None => false,
            Some(x) => x.is_alphabetic(),
        }
    }
    
    fn is_alphanumeric(&mut self) -> bool {
        match self.peek() {
            None => false,
            Some(x) => x.is_alphanumeric(),
        }
    }

    fn number(&mut self) -> Token {
        let i = self.pointer;
        while self.is_numeric() {
            self.advance();
        }
        if self.match_char('.') { 
            while self.is_numeric() {
                self.advance();
            }
        }
        self.get_token(TokenType::Number, i)
    }

    fn identifier(&mut self) -> Token {
        let i = self.pointer;
        if !self.is_alphabetic() {
            panic!("Error while parsing identifer");
        }
        while self.is_alphanumeric() {
            self.advance();
        }
        self.get_token(TokenType::Identifer, i)
    }
    
    fn comment(&mut self) -> Token {
        let i = self.pointer;
        while !self.is_eof() && !self.check('\n') {
            self.advance();
        }
        self.get_token(TokenType::Comment, i)
    }

    pub fn next_token(&mut self, keymap: &HashMap<&'static str, TokenType>) -> Option<Token> {
        while self.check(' ') || self.check('\t') || self.check('\n') {
            if self.check('\n') {
                self.col_no = 1;
                self.line_no += 1;
            }
            self.advance();
        }
        let i = self.pointer;
        if self.match_char('+'){
            if self.match_char('+') {
                return Some(self.get_token(TokenType::PlusPlus, i));
            } else if self.match_char('=') {
                return Some(self.get_token(TokenType::PlusEqual, i));
            }
            Some(self.get_token(TokenType::Plus, i))
        } else if self.match_char('-') {
            if self.match_char('-') {
                return Some(self.get_token(TokenType::MinusMinus, i));
            } else if self.match_char('=') {
                return Some(self.get_token(TokenType::MinusEqual, i));
            }
            return Some(self.get_token(TokenType::Minus, i));
        } else if self.match_char('*') {
            if self.match_char('=') {
                return Some(self.get_token(TokenType::MulEqual, i));
            }
            return Some(self.get_token(TokenType::Mul, i));
        } else if self.match_char('/') {
            if self.match_char('/') {
                return Some(self.comment());
            } else if self.match_char('=') {
                return Some(self.get_token(TokenType::SlashEqual, i));
            }
            return Some(self.get_token(TokenType::Slash, i));
        } else if self.match_char('!') {
            if self.match_char('=') {
                return Some(self.get_token(TokenType::BangEqual, i));
            }
            return Some(self.get_token(TokenType::Bang, i));
        } else if self.match_char('(') {
            return Some(self.get_token(TokenType::OpenParen, i));
        } else if self.match_char(')') {
            return Some(self.get_token(TokenType::CloseParen, i));
        } else if self.match_char('[') {
            return Some(self.get_token(TokenType::OpenSquare, i));
        } else if self.match_char(']') {
            return Some(self.get_token(TokenType::CloseSquare, i));
        } else if self.match_char('{') {
            return Some(self.get_token(TokenType::OpenCurly, i));
        } else if self.match_char('}') {
            return Some(self.get_token(TokenType::CloseCurly, i));
        } else if self.match_char('<') {
            if self.match_char('=') {
                return Some(self.get_token(TokenType::LessEqual, i));
            }
            return Some(self.get_token(TokenType::Less, i));
        } else if self.match_char('>') {
            if self.match_char('=') {
                return Some(self.get_token(TokenType::GreatEqual, i));
            }
            return Some(self.get_token(TokenType::Greater, i));
        } else if self.match_char('.') {
            return Some(self.get_token(TokenType::Dot, i));
        } else if self.match_char(',') {
            return Some(self.get_token(TokenType::Comma, i));
        } else if self.match_char('=') {
            if self.match_char('=') {
                return Some(self.get_token(TokenType::EqualEqual, i));
            }
            return Some(self.get_token(TokenType::Equal, i));
        } else if self.match_char('"') {
            return Some(self.string('"'));
        } else if self.match_char('\'') {
            return Some(self.string('\''));
        } else if self.is_numeric() {
            return Some(self.number());
        } else if self.is_alphabetic() {
            let mut identifier = self.identifier();
            match keymap.get(&identifier.literal[..]) {
                None => Some(identifier),
                Some(x) => {
                    identifier.t_type = x.clone();
                    Some(identifier)
                }
            }
        } else if self.match_char(';') {
            Some(self.get_token(TokenType::Semicolon, i))
        } else {
            return None
        }
    }
}
