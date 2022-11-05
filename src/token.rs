use crate::lexer::{Element, LexResult, Lexer};

pub mod expr;
pub mod num;

pub trait Streamable<'src>
where
    Self: Sized,
{
    fn take_stream(self) -> Lexer<'src>;
    fn stream(&mut self) -> &mut Lexer;

    fn accept<E>(self, rep: E) -> LexResult<'src, E> {
        let mut stream = self.take_stream();
        stream.accept();
        Ok((stream, rep))
    }
}

impl<'src, E> Streamable<'src> for Element<'src, E> {
    fn take_stream(self) -> Lexer<'src> {
        self.0
    }

    fn stream(&mut self) -> &'src mut Lexer {
        &mut self.0
    }
}

pub trait Tokenize<'src>: Streamable<'src> {
    type NextToken;

    fn tokenize(self) -> LexResult<'src, Self::NextToken>;
}
