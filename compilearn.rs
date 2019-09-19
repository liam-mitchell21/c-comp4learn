//this is my attempt at building a c compiler
//this is written in rust


use std:env;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
}

//fn lex() {
//
//}
