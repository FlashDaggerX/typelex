use crate::lexer::{LexError, LexResult, Lexer, Tokenize};

#[derive(PartialEq, Debug)]
pub struct Pipe;

impl<'src> Tokenize<'src> for Pipe {
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<Self> {
        lexer.lex_if(|c| c == &'|');

        match lexer.lexeme() {
            "|" => Ok(Self),
            _ => Err(LexError::One),
        }
    }
}
