#![allow(unused)]

use std::env;
use std::{fs, fs::File};
use std::io::prelude::*;

use crate::parsing::file_is_event;

mod parsing {
  use std::fs::File;

  pub fn file_is_event(filestr: &String) -> bool {
    /* Need to, later, figure out how to only check the first like, 10 lines so I don't process entire,
    large files, since it'll always be at the beginning */
    let event_bool: bool = filestr.lines().any(|line| line.matches("Tags: #event").collect::<Vec<&str>>().len() > 0);
    event_bool
  }
}

fn main() {
  let file_path: &str = "res/example.md";
  // println!("{:#?}", env::current_dir().unwrap());
  // Open the path in read-only mode, returns `io::Result<File>`
  let mut filemd = match fs::File::open(file_path) {
    Err(why) => panic!("couldn't open {}: {}", file_path, why),
    Ok(file) => file,
  };

  let mut s = String::new();
  let contents = filemd.read_to_string(&mut s)
    .expect("Should have been able to find file");

  let event_bool: bool = file_is_event(&s);
  println!("{event_bool}")


  }
