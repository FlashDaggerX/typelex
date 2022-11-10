/*
E -> E "+" T | E "-" T | T
T -> T "*" F | T "/" F | F
F -> (E) | I
I -> "a" | "b" | "c"
*/

use typelex::{lexer::Lexer, token::digit::Digit};

fn main() {
	let mut lexer = Lexer::from("23414 312 |4 324 23 4 ");
	println!("{:?}", lexer.accept::<Digit>());
	println!("{:?}", lexer.consume::<Digit>());

}
