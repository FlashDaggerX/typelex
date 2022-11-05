use crate::lexer::{Element, Empty, LexError, LexResult};

use super::{Streamable, Tokenize};

#[derive(Debug)]
pub struct Expr {
    lhs: u32,
    rhs: u32,
}

impl<'src> Tokenize<'src> for Element<'src, Empty> {
    type NextToken = Expr;

    fn tokenize(mut self) -> LexResult<'src, Self::NextToken> {
        let iter = &mut self.stream();

        let lhs = iter
            .next()
            .ok_or(LexError)?
            .1
            .to_digit(10)
            .ok_or(LexError)?;
        let rhs = iter
            .next()
            .ok_or(LexError)?
            .1
            .to_digit(10)
            .ok_or(LexError)?;

        self.accept(Expr { lhs, rhs })
    }
}

impl<'src> Tokenize<'src> for Element<'src, Expr> {
    type NextToken = u32;

    fn tokenize(mut self) -> LexResult<'src, Self::NextToken> {
        let iter = &mut self.stream();

        let num = iter
            .next()
            .ok_or(LexError)?
            .1
            .to_digit(10)
            .ok_or(LexError)?;

        self.accept(num)
    }
}
