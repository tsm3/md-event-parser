#![allow(unused)]

use chrono::{Datelike, NaiveDate};
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct EventParseError;
impl fmt::Display for EventParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Oh no can't parse that.")
  }
}

type Result<T> = std::result::Result<T, EventParseError>;


#[derive(Default, Debug)]
pub struct EventModel {
  date: NaiveDate, // Make this just a datetime, mandatory
  time: Option<String>, // If None, all day
  place: Option<String>, // Should this be mandatory?
  title: String, // This is mandatory, but just a String
}

impl EventModel {
  pub fn new(
    date: Option<String>,
    time: Option<String>,
    place: Option<String>,
    title: String
  ) -> Result<EventModel> {
    let date_struct = match date {
      Some(datestr) => {
        NaiveDate::parse_from_str(&datestr, "%d %b %Y").unwrap()
      }
      None => return Err(EventParseError),
    };
    Ok(EventModel { date: date_struct, title, ..Default::default()})
  }
}


#[cfg(test)]
mod tests {
  use crate::model::EventModel;
  use chrono::{Datelike, NaiveDate};


  #[test]
  fn default_cons() {
    unimplemented!("Need to update this to NaiveDate");
    let em = EventModel{
      // date: Some("Hi".to_string()),
      ..Default::default()
    };
    dbg!(em);
  }

    #[test]
    fn test_new() {
      let em = EventModel::new(
        Some("15 Feb 2023".to_string()), None, None, "Test Title".to_string());
      dbg!(em);
    }
    
    #[test]
    fn test_parse() {
      let bruh = NaiveDate::parse_from_str("15-Feb-2023", "%d-%b-%Y");
      // let bruh = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d");
      // dbg!(bruh);
      assert!(bruh.is_ok());
    }

}