/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use std::fmt::Debug;

#[derive(Debug)]
struct Lexer<I: Iterator<Item = char>> {
    stream: I,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> From<I> for Lexer<I> {
    fn from(stream: I) -> Self {
        Lexer {
            stream,
            line: 1,
            col: 0,
        }
    }
}

type Element<I, E> = (Lexer<I>, E);

#[derive(Debug)]
struct LexError;

type LexResult<I, E> = Result<Element<I, E>, LexError>;

trait Tokenize<I: Iterator<Item = char>> {
    type NextToken;

    fn tokenize(self, lexer: Lexer<I>) -> LexResult<I, Self::NextToken>;
}

#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Expr {
    lhs: u32,
    rhs: u32,
}

impl<I: Iterator<Item = char>> Tokenize<I> for Empty {
    type NextToken = Expr;

    fn tokenize(self, mut lexer: Lexer<I>) -> LexResult<I, Self::NextToken> {
        let mut iter = &mut lexer.stream;

        let lhs = iter.next().ok_or(LexError)?.to_digit(10).ok_or(LexError)?;
        let rhs = iter.next().ok_or(LexError)?.to_digit(10).ok_or(LexError)?;

        Ok((lexer, Expr { lhs, rhs }))
    }
}

fn main() {
    let lexer = Lexer::from("012".chars());
    let start = Empty.tokenize(lexer);
    println!("{start:?}");
}
