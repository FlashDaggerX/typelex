use crate::lexer::{Element, Done, LexResult};

use super::Tokenize;

impl<'src> Tokenize<'src> for Element<'src, u32> {
    type NextToken = Done;

    fn tokenize(self) -> LexResult<'src, Self::NextToken> {
        Ok((self.0, Done))
    }
}
