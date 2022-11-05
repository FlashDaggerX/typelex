use std::{fmt::Debug, str::CharIndices};

use crate::token::Tokenize;

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    stream: CharIndices<'src>,
    curr: usize,
    last: usize,
    line: usize,
    col: usize,
}

impl<'src> From<&'src str> for Lexer<'src> {
    fn from(source: &'src str) -> Self {
        Self {
            source,
            stream: source.char_indices(),
            curr: 0,
            last: 0,
            line: 1,
            col: 0,
        }
    }
}

impl<'src> Lexer<'src> {
    pub(crate) fn accept(&mut self) {
        self.last = self.curr;
    }
}

pub fn new_lexer<'src>(source: &'src str) -> Element<'src, Empty> {
    (Lexer::from(source), Empty)
}

impl<'src> Iterator for Lexer<'src> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.stream.next();
        self.col += 1;

        if let Some((idx, _)) = c {
            self.curr = idx;
        }

        match c {
            Some((_, '\n')) => {
                self.line += 1;
                self.col = 0;
                self.next()
            }
            Some((_, ' ')) => self.next(),
            _ => c,
        }
    }
}

#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub struct Done;

pub type Element<'src, E> = (Lexer<'src>, E);

impl<'src> Tokenize<'src> for Element<'src, Done> {
    type NextToken = Done;

    fn tokenize(self) -> LexResult<'src, Self::NextToken> {
        Err(LexError)
    }
}

#[derive(Debug)]
pub struct LexError;

pub type LexResult<'src, E> = Result<Element<'src, E>, LexError>;
