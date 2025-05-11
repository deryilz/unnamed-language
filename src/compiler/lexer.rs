// a lexer turns a string into tokens

use super::token::Token;
use super::token::TokenKind as K;

#[derive(Clone)]
pub struct Lexer<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Lexer<'a> {
        Lexer { string, index: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.string[self.index..].chars().next()
    }

    fn next_char(&mut self) {
        let next = self.peek_char().unwrap();
        self.index += next.len_utf8();
    }

    fn invalid(&self, start: usize) -> Token {
        Token::new(K::Invalid, start, self.string.len())
    }

    pub fn next(&mut self) -> Token {
        let start = self.index;
        let peek = match self.peek_char() {
            Some(c) => c,
            None => return Token::new(K::End, start, start),
        };

        let token = match peek {
            '\n' => Token::new(K::LineBreak, start, start + 1),
            '@' => Token::new(K::At, start, start + 1),
            ',' => Token::new(K::Comma, start, start + 1),
            '*' => Token::new(K::Times, start, start + 1),
            '(' => Token::new(K::ParenL, start, start + 1),
            ')' => Token::new(K::ParenR, start, start + 1),
            '[' => Token::new(K::SquareL, start, start + 1),
            ']' => Token::new(K::SquareR, start, start + 1),
            '{' => Token::new(K::CurlyL, start, start + 1),
            '}' => Token::new(K::CurlyR, start, start + 1),
            '.' => Token::new(K::Dot, start, start + 1),
            ' ' | '\r' | '\t' => self.from_whitespace(),
            '-' => self.from_dash(),
            '=' => self.from_equals(),
            '+' => self.from_plus(),
            '/' => self.from_slash(),
            '\'' => self.from_single_quote(),
            '"' => self.from_quote(),
            ':' => self.from_colon(),
            '0'..='9' => self.from_digit(),
            'A'..='Z' | 'a'..='z' | '_' => self.from_letter(),
            _ => self.invalid(start),
        };

        self.index = token.end;
        token
    }

    fn from_whitespace(&mut self) -> Token {
        let start = self.index;

        while let Some(' ' | '\r' | '\t') = self.peek_char() {
            self.next_char();
        }

        Token::new(K::WhiteSpace, start, self.index)
    }

    fn from_dash(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            Some('>') => Token::new(K::ThinArrow, start, self.index + 1),
            _ => Token::new(K::Minus, start, self.index),
        }
    }

    fn from_equals(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            Some('>') => Token::new(K::ThickArrow, start, self.index + 1),
            Some('=') => Token::new(K::DoubleEquals, start, self.index + 1),
            _ => Token::new(K::Equals, start, self.index),
        }
    }

    fn from_plus(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            Some('+') => Token::new(K::DoublePlus, start, self.index + 1),
            _ => Token::new(K::Plus, start, self.index),
        }
    }

    fn from_slash(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            Some('/') => {
                self.next_char();
                loop {
                    match self.peek_char() {
                        Some('\n') | None => break,
                        _ => self.next_char(),
                    };
                }
                Token::new(K::Comment, start, self.index)
            }
            Some('*') => {
                // TODO: nested comment support
                self.next_char();
                loop {
                    match self.peek_char() {
                        None => return self.invalid(start),
                        Some(c) => {
                            self.next_char();
                            if c == '*' && self.peek_char() == Some('/') {
                                break;
                            }
                        }
                    }
                }
                self.next_char(); // consume the ending slash
                Token::new(K::Comment, start, self.index)
            }
            _ => Token::new(K::Div, start, self.index),
        }
    }

    fn from_single_quote(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            // TODO: add escaping support (the single quote char)
            Some('\'') | None => return self.invalid(start),
            _ => self.next_char(),
        }

        match self.peek_char() {
            Some('\'') => Token::new(K::Char, start, self.index + 1),
            _ => self.invalid(start),
        }
    }

    fn from_quote(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        loop {
            match self.peek_char() {
                // TODO: add escaping support and interpolation
                Some('"') => break,
                Some(_) => self.next_char(),
                None => return self.invalid(start),
            }
        }

        Token::new(K::String, start, self.index + 1)
    }

    fn from_colon(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        match self.peek_char() {
            Some(':') => Token::new(K::DoubleColon, start, self.index + 1),
            _ => Token::new(K::Colon, start, self.index),
        }
    }

    fn from_digit(&mut self) -> Token {
        let start = self.index;
        self.next_char();

        let mut dot_index = None;
        loop {
            match self.peek_char() {
                Some('0'..='9' | '_') => self.next_char(),
                Some('.') => {
                    if dot_index.is_some() {
                        return self.invalid(start);
                    }
                    self.next_char();
                    dot_index = Some(self.index);
                }
                _ => break,
            }
        }

        match dot_index {
            // can't end with a dot
            Some(i) if i == self.index => self.invalid(start),
            Some(_) => Token::new(K::Float, start, self.index),
            None => Token::new(K::Int, start, self.index),
        }
    }

    fn from_letter(&mut self) -> Token {
        let start = self.index;

        while let Some('A'..='Z' | 'a'..='z' | '_') = self.peek_char() {
            self.next_char();
        }

        let string = &self.string[start..self.index];
        let kind = match string {
            "struct" => K::Struct,
            "union" => K::Union,
            "trait" => K::Trait,
            "or" => K::Or,
            "and" => K::And,
            _ => K::Identifier,
        };

        Token::new(kind, start, self.index)
    }
}
