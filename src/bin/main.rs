/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use ptypes::{lexer::new_lexer, token::Tokenize};

fn main() {
    let lexer = new_lexer("012");

    let lexer = lexer.tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
    let lexer = lexer.unwrap().tokenize();
    println!("{lexer:?}");
}
