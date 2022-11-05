/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use std::fmt::Debug;

#[derive(Debug)]
struct Lexer<I> {
    stream: I,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.stream.next();
        self.col += 1;

        match c {
            Some('\n') => {
                self.line += 1;
                self.col = 0;
                self.next()
            }
            Some(' ') => self.next(),
            _ => c,
        }
    }
}

type Element<I, E> = (Lexer<I>, E);

fn new_lexer<I: Iterator<Item = char>>(stream: I) -> Element<I, Empty> {
    (
        Lexer {
            stream,
            line: 1,
            col: 0,
        },
        Empty,
    )
}

#[derive(Debug)]
struct LexError;

type LexResult<I, E> = Result<Element<I, E>, LexError>;

trait Streamable<I: Iterator<Item = char>> {
    fn stream(&mut self) -> &mut Lexer<I>;
}

impl<I: Iterator<Item = char>, E> Streamable<I> for Element<I, E> {
    fn stream(&mut self) -> &mut Lexer<I> {
        &mut self.0
    }
}

trait Tokenize<I: Iterator<Item = char>>: Streamable<I> {
    type NextToken;

    fn tokenize(self) -> LexResult<I, Self::NextToken>;
}

#[derive(Debug)]
struct Done;

impl<I: Iterator<Item = char>> Tokenize<I> for Element<I, Done> {
    type NextToken = Done;

    fn tokenize(self) -> LexResult<I, Self::NextToken> {
        Err(LexError)
    }
}

#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Expr {
    lhs: u32,
    rhs: u32,
}

impl<I: Iterator<Item = char>> Tokenize<I> for Element<I, Empty> {
    type NextToken = Expr;

    fn tokenize(mut self) -> LexResult<I, Self::NextToken> {
        let iter = &mut self.stream();

        let lhs = iter.next().ok_or(LexError)?.to_digit(10).ok_or(LexError)?;
        let rhs = iter.next().ok_or(LexError)?.to_digit(10).ok_or(LexError)?;

        Ok((self.0, Expr { lhs, rhs }))
    }
}

impl<I: Iterator<Item = char>> Tokenize<I> for Element<I, Expr> {
    type NextToken = u32;

    fn tokenize(mut self) -> LexResult<I, Self::NextToken> {
        let iter = &mut self.stream();

        let num = iter.next().ok_or(LexError)?.to_digit(10).ok_or(LexError)?;

        Ok((self.0, num))
    }
}

impl<I: Iterator<Item = char>> Tokenize<I> for Element<I, u32> {
    type NextToken = Done;

    fn tokenize(self) -> LexResult<I, Self::NextToken> {
        Ok((self.0, Done))
    }
}

fn main() {
    let lexer = new_lexer("\n 0 1    2".chars());
    println!("{lexer:?}");

    let lexer = lexer.tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
}
