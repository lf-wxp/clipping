use chrono::prelude::*;

use crate::traits::Markdown;

#[derive(Debug, PartialEq)]
pub struct Clipping {
  date_time: DateTime<Utc>,
  position: String,
  clipping: String,
  mark: Option<String>,
}

impl Clipping {
  pub fn new(
    clipping: String,
    position: String,
    date_time: DateTime<Utc>,
    mark: Option<String>,
  ) -> Clipping {
    Clipping {
      date_time,
      position,
      clipping,
      mark,
    }
  }
}

impl Markdown for Clipping {
  fn to_markdown(&self) -> String {
    format!(
      "> &emsp; \n> {:}\n> \n> <p align=\"right\"> {:} </p>\n> &emsp;\n",
      self.clipping,
      self.date_time.format("%Y/%m/%d %H:%M:%S")
    )
  }
}
