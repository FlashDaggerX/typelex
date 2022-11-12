use core::panic;
use std::{fmt::Debug, iter::Peekable, str::Chars};

const NEWLINE: char = '\n';

#[derive(PartialEq, Debug)]
pub struct Token<E: PartialEq> {
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

pub type LexResult<E> = Result<E, LexError>;

pub trait Tokenize<'src>
where
    Self: Sized,
{
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<Self>;
}

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    stream: Peekable<Chars<'src>>,
    cursor: LexemeCursor,
    position: Position,
}

impl<'src> Lexer<'src> {
    pub fn new<T: AsRef<str> + ?Sized>(source: &'src T) -> Self {
        let source = source.as_ref();
        Self {
            source,
            stream: source.chars().peekable(),
            cursor: LexemeCursor { curr: 0, last: 0 },
            position: Position { line: 1, column: 0 },
        }
    }
}

impl<'src> Lexer<'src> {
    pub fn consume<E: Tokenize<'src> + PartialEq>(&mut self) -> LexResult<Token<E>> {
        let token = E::tokenize(self)?;
        self.cursor.new_lexeme();
        Ok(Token {
            line: self.position.line,
            column: self.position.column,
            token,
        })
    }

    pub fn accept<E: Tokenize<'src> + PartialEq>(&mut self) -> Token<E> {
        if let Ok(elem) = self.consume::<E>() {
            elem
        } else {
            panic!("Element is not acceptable!");
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        let c = self.stream.peek()?;
        Some(c)
    }

    pub fn advance_if(&mut self, expected: impl Fn(&char) -> bool) -> Option<&char> {
        loop {
            let item = self.stream.next_if(&expected);
            match item {
                Some(c) => {
                    self.cursor.advance();
                    if c == NEWLINE {
                        // Why `new_lexeme` here? You can't have the same token type
                        // on two different lines (unless it's a multiline string, which
                        // is a special case.)
                        self.position.newline();
                        self.cursor.new_lexeme();
                    } else {
                        self.position.advance();
                    }
                }
                None => break,
            }
        }
        self.peek()
    }

    pub fn skip_whitespace(&mut self) -> Option<&char> {
        self.advance_if(|c| c.is_whitespace())
    }

    pub fn start_lexeme(&mut self) {
        self.cursor.new_lexeme();
    }

    pub fn lex_if(&mut self, expected: impl Fn(&char) -> bool) {
        self.skip_whitespace();
        self.start_lexeme();
        self.advance_if(expected);
    }

    pub fn lexeme(&self) -> &'src str {
        match self.source.get(self.cursor.last..self.cursor.curr) {
            Some(tslice) => tslice,
            None => panic!(
                "Error obtaining lexeme (slice is {:?})",
                self.cursor.last..self.cursor.curr,
            ),
        }
    }
}
