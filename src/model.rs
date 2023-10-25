#![allow(unused)]

use chrono::{Datelike, NaiveDate, NaiveTime, ParseResult};
use std::{error::Error, fmt};
use serde::{Serialize, Deserialize};
use regex::Regex;

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

  fn base_parse_time(timestr: impl Into<String> + Copy + AsRef<str> + std::fmt::Display) -> ParseResult<NaiveTime> {
    /* Put in one place for ease of iteration */
    NaiveTime::parse_from_str(&timestr.into(), EventModel::TIMEFMT)
  }

  fn base_parse_date(datestr: impl Into<String> + Copy + AsRef<str> + std::fmt::Display) -> ParseResult<NaiveDate> {
    /* Put in one place for ease of iteration */
    NaiveDate::parse_from_str(&datestr.into(), EventModel::DATEFMT)
  }

  fn parse_time_tup(timestr: impl Into<String> + Copy + AsRef<str> + std::fmt::Display) -> (Result<NaiveTime>, Option<NaiveTime>) {
    /* Need to localize the complicated way I'm going to parse time */
    /** List of ways I might write time?
     * form a) 6 PM
     * 6:00 PM
     * 6-7 PM
     * 6:30-7PM
     * I'm going to have to figure out how to extract both start and end time from this?
     */
    
    // unimplemented!("Just not here yet");
    let time_reg_arr: [Regex; 3] = [
        Regex::new(r"^\d{1,2}:\d\d [A,P]M").unwrap(),
        Regex::new(r"^(\d{1,2}) ([A,P]M)").unwrap(),
        Regex::new(r"^(\d{1,2})((?::\d\d)|)((?:[A,P]M|))-(\d{1,2})((?::\d\d)|) ?([A,P]M)").unwrap(),
      ];

    if time_reg_arr[0].is_match(timestr.as_ref()) {
      println!("String {timestr} matches regex {:?}", time_reg_arr[0]);
      // Simple/well-formed case
      let start_time_struct = Self::base_parse_time(timestr).map_err(|e| EventParseError{desc: e.to_string()});
      return (start_time_struct, None);

    } else if let Some(mat) = time_reg_arr[1].captures(timestr.as_ref()) {
      println!("String {timestr} matches regex {:?}", time_reg_arr[1]);
      // This is single time with no `:\d\d`
      let mut modstr: String = String::new();
      // Pushes the number we matched on to the str
      modstr.push_str(mat.extract::<2>().1[0]);
      modstr.push_str(":00 ");
      // Pushes the AM or PM
      modstr.push_str(mat.extract::<2>().1[1]);
      let start_time_struct = Self::base_parse_time(&modstr).map_err(|e| EventParseError{desc:e.to_string()});
      return (start_time_struct, None);

    } else if let Some(mat) = time_reg_arr[2].captures(timestr.as_ref()) {
      println!("String {timestr} matches regex {:?}", time_reg_arr[2]);
      /**
       * This is the most complicated case, we have up to 5 capture groups.
       * I'm considering breaking this into two different cases like
       * 6-7 PM vs 6:30-7:30 PM
       * But what if I have a like 6-7:30 PM or 6:15-7 PM case?
       * Then it's weird bc do those belong to the no :00 case or the yes :00 case?
       */

      /**
       * Check for capture group 2 (idx 1) and 4 (idx 3), as these are the :\d\d groups
       * If they don't exist, push :00 to the string in their place
       * Then add cap group 5 (idx 4) to each string, then parse base time each
       * so I'll return (Some(NaiveTime(($1)($2 OR :00)($5))) , Some(NaiveTime(($3)($4 OR :00)($5))) )
       */

      let mut startstr: String = String::new();
      let mut endstr: String = String::new();
      let cap1 = mat.extract::<6>().1[0]; // Start Hour
      let cap2 = mat.extract::<6>().1[1]; // Start minute (or "")
      let cap3 = mat.extract::<6>().1[2]; // AM or PM for start
      let cap4 = mat.extract::<6>().1[3]; // End Hour
      let cap5 = mat.extract::<6>().1[4]; // End minute (or "")
      let cap6 = mat.extract::<6>().1[5]; // AM or PM for end

      startstr.push_str(cap1);
      if !cap2.is_empty() {
        startstr.push_str(cap2);
        startstr.push_str(" ");
      } else {
        startstr.push_str(":00 ");
      }

      if !cap3.is_empty() {
        startstr.push_str(cap3);
      } else {
        startstr.push_str(cap6)
      }

      endstr.push_str(cap4);
      if !cap5.is_empty() {
        endstr.push_str(cap5);
        endstr.push_str(" ");
      } else {
        endstr.push_str(":00 ");
      }
      endstr.push_str(cap6);
      
      let start_time_struct = Self::base_parse_time(&startstr).map_err(|e| EventParseError{desc:e.to_string()});
      let end_time_struct = Self::base_parse_time(&endstr).ok();

      return (start_time_struct, end_time_struct);

    } else {
      return (Err(EventParseError { desc: "".to_owned() }), None);
    }

    unreachable!();

    /**
     * If here, know is_err(), which means we can't just parse it as is, so try looking for a `-`, meaning there's a start and stop time
     * if no `-`, then check for a :, if there isn't one, then try parsing as if it's form a) 
     * so like, make a String, capture the number, append `:00` to it, put the appropriate AM/PM, try parsing again
     */

    let timestr: String = timestr.into();
    let constructed_str = String::new();

    
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

    #[test]
    fn test_parse_time_tup() {
      let time_str_vec = vec![
        "6:00 AM",
        "6 PM",
        "6:00-7:00 AM",
        "6:00AM-7:00 AM",
        "6:00AM-7:00PM",
        "6-7:00 PM",
        "6:30-7:00 AM",
        "6-7 AM",
        "6PM-7 AM",
      ];
      
      let mut time_struct_vec = vec![];
      for time_str in time_str_vec {
        let temp = EventModel::parse_time_tup(time_str);
        dbg!(&temp);
        assert!(temp.0.is_ok());
        time_struct_vec.push(temp);
      }
    }

    #[test]
    fn test_time_regex_arr() {
      /* I'm manually copy + pasting the regex arr bc I don't know how to make regex a const in EventModel */
      let time_reg_arr: [Regex; 3] = [
        Regex::new(r"^\d{1,2}:\d\d [A,P]M").unwrap(),
        Regex::new(r"^\d{1,2} [A,P]M").unwrap(),
        Regex::new(r"^\d{1,2}(:\d\d)?-\d{1,2}(:\d\d)? [A,P]M").unwrap(),
      ];
      let time1: String = "6:00 AM".to_owned();
      let time2: String = "6 PM".to_owned();
      let time3: String = "6:00-7:00 AM".to_owned();
      let time4: String = "6-7 AM".to_owned();

      let mut time1_results = Vec::new();
      for reg in &time_reg_arr {
        time1_results.push(reg.captures(&time1));
      }
      // dbg!(&time1_results);
      assert!(time1_results.iter().any(|o| o.is_some()));

      let mut time2_results = Vec::new();
      for reg in &time_reg_arr {
        time2_results.push(reg.captures(&time2));
      }
      // dbg!(&time2_results);
      assert!(time2_results.iter().any(|o| o.is_some()));

      let mut time3_results = Vec::new();
      for reg in &time_reg_arr {
        time3_results.push(reg.captures(&time3));
      }
      // dbg!(&time3_results);
      assert!(time3_results.iter().any(|o| o.is_some()));

      let mut time4_results = Vec::new();
      for reg in &time_reg_arr {
        time4_results.push(reg.captures(&time4));
      }
      // dbg!(&time4_results);
      assert!(time4_results.iter().any(|o| o.is_some()));

    }


}