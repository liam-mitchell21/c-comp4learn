//this is my attempt at building a c compiler
//this is written in rust


use std::env;
use std::fs;

fn main() {
	//take command line argument as filename
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("Compiling {} ...", filename);
	
	//get tokens
	println!("The file is {}", lex(filename));
}

//lexer, takes filename as input and returns list of tokens
fn lex(f: &String) -> String {
	let contents = fs::read_to_string(f)
		.expect("ERROR, something went wrong reading the file");
		
}
