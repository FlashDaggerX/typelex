pub mod lexer;
pub mod token;

#[cfg(test)]
mod test {
    use crate::{
        lexer::{Lexer, Token},
        token::{digit::Digit, pipe::Pipe},
    };

    fn token<E: PartialEq>(line: usize, column: usize, token: E) -> Token<E> {
        Token {
            line,
            column,
            token,
        }
    }

    #[test]
    fn digit_and_string() {
        let mut lexer = Lexer::new(" 23414\n | 4324");

        let tok = token(1, 6, Digit(23414));
        assert_eq!(tok, lexer.accept::<Digit>());

        let tok = token(2, 2, Pipe);
        assert_eq!(tok, lexer.accept::<Pipe>());

        let tok = token(2, 7, Digit(4324));
        assert_eq!(tok, lexer.accept::<Digit>());
    }
}
