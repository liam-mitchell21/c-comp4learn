use regex::RegexSet; 
use regex::Regex;
use std::collections::HashMap;
extern crate lazy_static;

lazy_static! {
  pub static ref TOKENS : RegexSet = RegexSet::new(&[
    r"\{",          //0
    r"\}",          //1
    r"\(",          //2
    r"\)",          //3
    r";",           //4
    r"\[a-zA-Z]\w*", //5 
    r"\[0-9]+",      //6
  ]).unwrap();
  
  //pub static ref TK0 : Regex = Regex::new(r"\{").unwrap();
  //pub static ref TK1 : Regex = Regex::new(r"\}").unwrap();
  //pub static ref TK2 : Regex = Regex::new(r"\(").unwrap();
  //pub static ref TK3 : Regex = Regex::new(r"\)").unwrap();
  //pub static ref TK4 : Regex = Regex::new(r";").unwrap();
  //pub static ref TK5 : Regex = Regex::new(r"\[a-zA-Z]\w*").unwrap();
  //pub static ref TK6 : Regex = Regex::new(r"\[0-9]+").unwrap();

  pub static ref TKHASHMAP: HashMap<usize, &'static str > = {
    let mut m = HashMap::new();
      m.insert(0 as usize, r"\{" );
      m.insert(1 as usize, r"\}" );
      m.insert(2 as usize, r"\(" );
      m.insert(3 as usize, r"\)" );
      m.insert(4 as usize, r";" );
      m.insert(5 as usize, r"\[a-zA-Z]\w*" );
      m.insert(6 as usize, r"\[0-9]+" );
      m
  };
}  

