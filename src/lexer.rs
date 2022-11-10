use core::panic;
use std::{fmt::Debug, iter::Peekable, str::Chars};

const NEWLINE: char = '\n';
const SPACE: char = ' ';

#[derive(Debug)]
pub struct Token<E> {
    pub line: usize,
    pub column: usize,
    pub token: E,
}

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    fn advance(&mut self) {
        self.column += 1;
    }
}

/// Keeps last position of acceptance along with the current
/// position within the token stream.
#[derive(Debug)]
struct LexemeCursor {
    last: usize,
    curr: usize,
}

impl LexemeCursor {
    fn new_lexeme(&mut self) {
        self.last = self.curr;
    }

    fn advance(&mut self) {
        self.curr += 1;
    }
}

#[derive(Debug)]
pub enum LexError {
    One,
    Two,
    Three,
    Four,
}

pub type LexResult<'src, E> = Result<E, LexError>;

pub trait Tokenize<'src>
where
    Self: Sized,
{
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<'src, Self>;
}

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    stream: Peekable<Chars<'src>>,
    cursor: LexemeCursor,
    position: Position,
}

impl<'src> From<&'src str> for Lexer<'src> {
    fn from(source: &'src str) -> Self {
        Self {
            source,
            stream: source.chars().peekable(),
            cursor: LexemeCursor { curr: 0, last: 0 },
            position: Position { line: 1, column: 0 },
        }
    }
}

impl<'src> Lexer<'src> {
    pub fn consume<E: Tokenize<'src>>(&mut self) -> LexResult<'src, Token<E>> {
        let token = E::tokenize(self)?;
        Ok(Token {
            line: self.position.line,
            column: self.position.column,
            token,
        })
    }

    pub fn accept<E: Tokenize<'src>>(&mut self) -> Token<E> {
        if let Ok(elem) = self.consume::<E>() {
            self.cursor.new_lexeme();
            elem
        } else {
            panic!("Element is not acceptable!");
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        let c = self.stream.peek()?;
        self.cursor.advance();
        Some(c)
    }

    pub fn advance_over_whitespace(&mut self) -> Option<char> {
        let c = self.stream.next()?;

        self.cursor.advance();
        self.position.advance();

        match c {
            NEWLINE => {
                self.position.newline();
                self.advance_over_whitespace()
            }
            SPACE => self.advance_over_whitespace(),
            _ => Some(c),
        }
    }

    pub fn advance_if<F: FnOnce(&char) -> bool>(&mut self, expected: F) -> Option<char> {
        self.stream.next_if(|c| {
            self.cursor.advance();
            self.position.advance();

            println!("POS {:?}", self.position);

            if c == &NEWLINE {
                self.position.newline();
            }

            expected(c)
        })
    }

    pub fn lexeme(&self) -> &'src str {
        self.source.get(self.cursor.last..self.cursor.curr).unwrap()
    }
}
