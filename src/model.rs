#![allow(unused)]

use chrono::{Datelike, NaiveDate, NaiveTime, ParseResult};
use std::{error::Error, fmt};
use serde::{Serialize, Deserialize};

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

mod my_date_ser {
  use chrono::{DateTime, NaiveDate, NaiveTime};
  use serde::{self, Deserialize, Serializer, Deserializer};

use super::EventModel;

  const DATEFORMAT: &'static str = EventModel::DATEFMT;
  const TIMEFORMAT: &'static str = EventModel::TIMEFMT;

  // The signature of a serialize_with function must follow the pattern:
  //
  //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
  //    where
  //        S: Serializer
  //
  // although it may also be generic over the input types T.
  pub fn serialize_naive_date<S>(
      date: &NaiveDate,
      serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = format!("{}", date.format(DATEFORMAT));
      serializer.serialize_str(&s)
  }

  pub fn serialize_naive_date_opt<S>(
    date: &Option<NaiveDate>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match date {
      Some(date) => format!("{}", date.format(DATEFORMAT)),
      _ => unreachable!(),
    };
    serializer.serialize_str(&s)
}

pub fn serialize_naive_time_opt<S>(
  time: &Option<NaiveTime>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let s = match time {
    Some(time) => format!("{}", time.format(TIMEFORMAT)),
    _ => unreachable!(),
  };
  serializer.serialize_str(&s)
}


}

#[derive(Default, Debug, Serialize)]
pub struct EventModel {
  #[serde(serialize_with = "my_date_ser::serialize_naive_date")]
  start_date: NaiveDate, // Make this just a datetime, mandatory
  #[serde(serialize_with = "my_date_ser::serialize_naive_date_opt", skip_serializing_if = "Option::is_none")]
  end_date: Option<NaiveDate>, // Make this just a datetime, mandatory
  #[serde(serialize_with = "my_date_ser::serialize_naive_time_opt", skip_serializing_if = "Option::is_none")]
  start_time: Option<NaiveTime>, // If None, all day
  #[serde(serialize_with = "my_date_ser::serialize_naive_time_opt", skip_serializing_if = "Option::is_none")]
  end_time: Option<NaiveTime>, // If None, all day
  place: String, // Should this be mandatory? yuh just empty string if None
  title: String, // This is mandatory, but just a String
}

impl EventModel {
  const DATEFMT: &'static str = "%d %b %Y";
  const TIMEFMT: &'static str = "%I:%M %P";
  const TIMEFMT2: &'static str = "%I%M %P";

  pub fn new(
    start_date: String,
    end_date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    place: Option<String>,
    title: String
  ) -> Result<EventModel> {
    let date_struct = match NaiveDate::parse_from_str(&start_date, EventModel::DATEFMT) {
      Ok(d) => d,
      Err(_) => return Err(EventParseError::default()),
    };

    /* time is hard bc I don't want to always do hour:minute I wanna leave it as like, 6 PM instead of 6:00 PM */

    let p = match place {
      Some(s) => s,
      None => "".to_owned(),
    };
      
    Ok(EventModel { start_date: date_struct, title: title, place: p, ..Default::default()})
  }

  pub fn with_date(date_str: String) -> Result<EventModel> {
    let date_struct = match NaiveDate::parse_from_str(&date_str, EventModel::DATEFMT) {
      Ok(d) => d,
      Err(_) => return Err(EventParseError::default()),
    };
    Ok(EventModel{
      start_date:date_struct,
      ..Default::default()
    })
  }

  fn base_parse_time(timestr: impl Into<String>) -> ParseResult<NaiveTime> {
    /* Put in one place for ease of iteration */
    NaiveTime::parse_from_str(&timestr.into(), EventModel::TIMEFMT)
  }

  fn base_parse_date(datestr: impl Into<String>) -> ParseResult<NaiveDate> {
    /* Put in one place for ease of iteration */
    NaiveDate::parse_from_str(&datestr.into(), EventModel::DATEFMT)
  }

  fn parse_time_tup(timestr: impl Into<String>) -> (Result<NaiveTime>, Result<NaiveTime>) {
    /* Need to localize the complicated way I'm going to parse time */
    /** List of ways I might write time?
     * 6 PM
     * 6:00 PM
     * 6-7 PM
     * 6:30-7PM
     * I'm going to have to figure out how to extract both start and end time from this?
     */
    
    unimplemented!("Just not here yet");

    // if let Ok(time_struct) = 

    // (Err(EventParseError{..Default::default()}), Err(EventParseError{..Default::default()}))
  }

}




#[cfg(test)]
mod tests {
  use crate::model::*;

  #[test]
  fn default_cons() {
    // unimplemented!("Need to update this to NaiveDate");
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
      let bruh = EventModel::base_parse_date("3 Nov 2023");
      // dbg!(bruh);
      assert!(bruh.is_ok());
    }

    #[test]
    fn test_parse_time() {
      let bruh = EventModel::base_parse_time("6:00 PM");
      dbg!(bruh);
    }

    #[test]
    fn test_ser_naive_date() {
      let datestr: &'static str = "3 Nov 2023";
      let timestr: &'static str = "6:00 PM";
      
      let date = NaiveDate::parse_from_str(datestr, EventModel::DATEFMT);
      assert!(date.is_ok());
      let date = date.unwrap();

      let time = NaiveTime::parse_from_str(timestr, EventModel::TIMEFMT);
      assert!(time.is_ok());
      let time = time.unwrap();

      let em = EventModel{
        start_date: date,
        start_time: Some(time),
        ..Default::default()
    };

      let json = serde_json::to_string_pretty(&em).unwrap();
      println!("{}", json);
    }

    #[test]
    fn test_timefmt2() {
      let timestr: &'static str = "0600 PM"; // This does work
      // let timestr: &'static str = "600 PM"; // This doesn't work
      let date = NaiveTime::parse_from_str(timestr, EventModel::TIMEFMT2);
      assert!(date.is_ok());
      dbg!(date);
    }

}