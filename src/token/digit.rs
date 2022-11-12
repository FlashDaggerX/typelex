use crate::lexer::{LexError, LexResult, Lexer, Tokenize};

#[derive(PartialEq, Debug)]
pub struct Digit(pub usize);

impl<'src> Tokenize<'src> for Digit {
    fn tokenize(lexer: &mut Lexer<'src>) -> LexResult<Self> {
        lexer.lex_if(|c| c.is_digit(10));

        match lexer.lexeme().parse::<usize>() {
            Ok(digit) => Ok(Self(digit)),
            Err(_) => Err(LexError::Two),
        }
    }
}
