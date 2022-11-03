/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use std::fmt::Debug;

#[derive(Debug)]
enum LexError {
    ONE,
    TWO,
    THREE,
    FOUR,
}

trait Lex<S>
where
    for<'a> &'a mut S: Iterator<Item = char>,
{
    type Expected;

    fn lex(self, stream: &mut S) -> Result<Self::Expected, LexError>;
}

#[derive(Debug)]
struct EmptyLexer;

impl EmptyLexer {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Expr {
    lhs: u32,
    rhs: u32,
}

impl<S> Lex<S> for EmptyLexer
where
    for<'a> &'a mut S: Iterator<Item = char>,
{
    type Expected = Expr;

    fn lex(self, mut stream: &mut S) -> Result<Self::Expected, LexError> {
        let lhs = stream
            .next()
            .ok_or(LexError::ONE)?
            .to_digit(10)
            .ok_or(LexError::TWO)?;

        let rhs = stream
            .next()
            .ok_or(LexError::THREE)?
            .to_digit(10)
            .ok_or(LexError::FOUR)?;

        Ok(Expr { lhs, rhs })
    }
}

fn main() {
    let mut stream = "12".chars();
    println!("{stream:?}");

    let lex = EmptyLexer::new();

    println!("{:?}", lex.lex(&mut stream));
}
