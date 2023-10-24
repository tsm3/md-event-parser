#![allow(unused)]

use std::env;
use std::{fs, fs::File};
use std::io::prelude::*;


use regex::Regex;

mod model;
use model::EventModel;

mod parsing;
use parsing::{file_is_event, line_is_event};
// use crate::lib::prelude::*;


fn main() {
  

  }
