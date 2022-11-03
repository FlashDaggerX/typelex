/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use std::{fmt::Debug, iter::Peekable, str::Chars};

#[derive(Debug)]
enum LexError {
    ONE,
    TWO,
    THREE,
    FOUR,
}

trait Streamable<'a, I: Iterator<Item = char> + 'a> {
    fn stream(&'a mut self) -> &'a mut Peekable<I>;
}

trait SkipSpaces<'a, I: Iterator<Item = char> + 'a>: Streamable<'a, I> {
    fn skip_spaces(&mut self);
}

impl<'a, I, S> SkipSpaces<'a, I> for S
where
    I: Iterator<Item = char> + 'a,
    S: Streamable<'a, I>,
{
    fn skip_spaces(&mut self) {
        loop {
            let peek = {
                let peek = self.stream().peek().cloned();
                if peek.is_none() {
                    break;
                }
                peek.unwrap()
            };

            match peek {
                ' ' => {
                    self.stream().next();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

trait Lex<'a, I: Iterator<Item = char> + 'a>: Streamable<'a, I> {
    type Expected;

    fn lex(self) -> Result<Self::Expected, LexError>;
}

#[derive(Debug)]
struct Empty<'a> {
    stream: Peekable<Chars<'a>>,
}

impl<'a> Streamable<'a, Chars<'a>> for Empty<'a> {
    fn stream(&'a mut self) -> &'a mut Peekable<Chars<'a>> {
        &mut self.stream
    }
}

impl<'a> Empty<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            stream: input.chars().peekable(),
        }
    }
}

#[derive(Debug)]
struct Expr<'a> {
    stream: Peekable<Chars<'a>>,
    lhs: u32,
    rhs: u32,
}

impl<'a> Streamable<'a, Chars<'a>> for Expr<'a> {
    fn stream(&'a mut self) -> &'a mut Peekable<Chars<'a>> {
        &mut self.stream
    }
}

impl<'a, I: Iterator<Item = char> + 'a> Lex<'a, I> for Empty<'a>
where
    Empty<'a>: Streamable<'a, I>,
{
    type Expected = Expr<'a>;

    fn lex(mut self) -> Result<Self::Expected, LexError> {
        let lhs = self
            .stream()
            .next()
            .ok_or(LexError::ONE)?
            .to_digit(10)
            .ok_or(LexError::TWO)?;

        let rhs = self
            .stream()
            .next()
            .ok_or(LexError::THREE)?
            .to_digit(10)
            .ok_or(LexError::FOUR)?;

        Ok(Expr {
            stream: self.stream,
            lhs,
            rhs,
        })
    }
}

fn main() {
    let lex = Empty::new("012");

    println!("{:?}", lex.lex());
}
