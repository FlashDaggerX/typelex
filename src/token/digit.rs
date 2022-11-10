use crate::lexer::{LexError, LexResult, Lexer, Tokenize};

#[derive(Debug)]
pub struct Digit(usize);

impl<'src> Tokenize<'src> for Digit {
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<'src, Self> {
        while lexer.advance_if(|c| c.is_digit(10)).is_some() {}

        match lexer.lexeme().parse::<usize>() {
            Ok(digit) => Ok(Self(digit)),
            Err(_) => Err(LexError::Two),
        }
    }
}
