//this is my attempt at building a c compiler
//this is written in rust
extern crate regex;
#[macro_use]
extern crate lazy_static;
mod rxtokens;

use std::env;
use std::fs;
use regex::Regex;
use rxtokens::*;

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
  
  let matches: Vec<_> = TOKENS.matches(&contents).into_iter().collect();
  
  for m in matches {
    println!("regex is {:?}", TKHASHMAP.get(&m));
    let cmd = TKHASHMAP.get(&m);
    let rex = Regex::new(cmd.unwrap()).unwrap();
    for caps in rex.captures_iter(&contents) {
      println!("capture is {:?}", caps); 
    }
  }
}
