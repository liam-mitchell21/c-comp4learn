//this is my attempt at building a c compiler
//this is written in rust
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
use regex::RegexSet;

fn main() {
	//take command line argument as filename
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	println!("Compiling {} ...", filename);
	
	//get tokens
	lex(filename);
}

//lexer, takes filename as input and returns list of tokens
fn lex(f: &String) {
	//get file
	let contents = fs::read_to_string(f)
		.expect("ERROR, something went wrong reading the file");

	//def regexps
	lazy_static! {
		static ref TOKENS : RegexSet = RegexSet::new(&[
			r"\{",
			r"\}",
			r"\(",
			r"\)",
			r";",
			r"[a-zA-Z]\w*",
			r"[0-9]+",
		]).unwrap();
	}
	
	let matches: Vec<_> = TOKENS.matches(&contents).into_iter().collect();
	assert_eq!(matches, vec![0,1,2,3,4,5,6]);
        for m in matches {
                println!("{}", m);
        }

}
