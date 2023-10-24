
use std::fs::File;
use regex::Regex;

use std::env;
use std::io::prelude::*;

pub fn file_is_event(filestr: &str) -> bool {
  /* Need to, later, figure out how to only check the first like, 10 lines so I don't process entire,
  large files, since it'll always be at the beginning */
  let event_bool: bool = filestr.lines().any(|line| line.matches("Tags: #event").collect::<Vec<&str>>().len() > 0);
  event_bool
}

pub fn line_is_event(linestr: &&str) -> bool {
  /* For now, only really checking the beginning of the line for `- [ ] (.*) (.*) (.*)` */
  let reg: Regex = Regex::new(r#"- \[[ ,x]\] +\(.*\) +\(.*\) +\(.*\)"#).expect("Bruh");
  // let reg: Regex = Regex::new(r"- \[[ ,x]\] \(.*\)").unwrap();
  // println!("{}", linestr.trim());
  // dbg!(&reg);
  reg.captures(linestr.trim()).is_some()
}


#[cfg(test)]
mod tests {
  use crate::*;


  #[test]
  fn test_all() {
    let file_path: &str = "res/example.md";
    let mut filemd = match File::open(file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  let mut s = String::new();
  let contents = filemd.read_to_string(&mut s)
    .expect("Should have been able to find file");

  let event_bool: bool = file_is_event(&s);
  println!("{event_bool}");

  let thin: Vec<String> = s.lines().filter(line_is_event).map(|s| s.to_string()).collect();
  println!("{:#?}", thin);
  }
}