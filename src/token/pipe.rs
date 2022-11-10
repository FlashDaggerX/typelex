use crate::lexer::{LexError, LexResult, Lexer, Tokenize};

pub struct Pipe;

impl<'src> Tokenize<'src> for Pipe {
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<'src, Self> {
        match lexer.advance_over_whitespace() {
            Some('|') => Ok(Self),
            _ => Err(LexError::Four),
        }
    }
}
