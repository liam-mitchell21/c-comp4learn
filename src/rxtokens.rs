use regex::RegexSet; 
use regex::Regex;

lazy_static! {
  static ref TOKENS : RegexSet = RegexSet::new(&[
    r"\{",          //0
    r"\}",          //1
    r"\(",          //2
    r"\)",          //3
    r";",           //4
    r"[a-zA-Z]\w*", //5 
    r"[0-9]+",      //6
  ]).unwrap();
  
  static ref TK1 : Regex = Regex::new(r"
  \{
  ").unwrap();
}  

