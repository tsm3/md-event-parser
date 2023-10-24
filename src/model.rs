#![allow(unused)]

use chrono::{Datelike, NaiveDate, NaiveTime};
use std::{error::Error, fmt};

#[derive(Debug, Clone, Default)]
pub struct EventParseError {
  desc: String
}

impl fmt::Display for EventParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Oh no can't parse that.")
  }
}

type Result<T> = std::result::Result<T, EventParseError>;


#[derive(Default, Debug)]
pub struct EventModel {
  date: NaiveDate, // Make this just a datetime, mandatory
  time: Option<NaiveTime>, // If None, all day
  place: String, // Should this be mandatory? yuh just empty string if None
  title: String, // This is mandatory, but just a String
}

impl EventModel {
  const DATEFMT: &str = "%d %b %Y";

  pub fn new(
    date: String,
    time: Option<String>,
    place: Option<String>,
    title: String
  ) -> Result<EventModel> {
    let date_struct = match NaiveDate::parse_from_str(&date, EventModel::DATEFMT) {
      Ok(d) => d,
      Err(_) => return Err(EventParseError::default()),
    };

    /* time is hard bc I don't want to always do hour:minute I wanna leave it as like, 6 PM instead of 6:00 PM */

    let p = match place {
      Some(s) => s,
      None => "".to_owned(),
    };
      
    Ok(EventModel { date: date_struct, title: title, place: p, ..Default::default()})
  }

  pub fn with_date(date_str: String) -> Result<EventModel> {
    let date_struct = match NaiveDate::parse_from_str(&date_str, EventModel::DATEFMT) {
      Ok(d) => d,
      Err(_) => return Err(EventParseError::default()),
    };
    Ok(EventModel{
      date:date_struct,
      ..Default::default()
    })

  }

}




#[cfg(test)]
mod tests {
  use crate::model::*;

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
        "15 Feb 2023".to_string(),
        None, 
        None, 
        "Test Title".to_string());
      dbg!(em);
    }

    #[test]
    fn test_with_date() {
      let em = EventModel::with_date("15 Feb 2023".to_owned());
      // dbg!(em);
      assert!(em.is_ok());
    }
    
    #[test]
    fn test_parse_date() {
      let bruh = NaiveDate::parse_from_str("15-Feb-2023", "%d-%b-%Y");
      // dbg!(bruh);
      assert!(bruh.is_ok());
    }

    #[test]
    fn test_parse_time() {
      let bruh = NaiveTime::parse_from_str("6:00 PM", "%I:%M %P");
      dbg!(bruh);
    }

}