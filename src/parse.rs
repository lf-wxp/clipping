use chrono::prelude::*;
use nom::{
  self,
  bytes::complete::{tag, take_till, take_until, take_until1},
  sequence::{delimited, preceded, tuple},
  IResult,
};
use nom_unicode::{
  complete::{alpha0, digit0, space0},
  is_numeric,
  is_alphabetic
};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

pub type UError = Box<dyn std::error::Error>;
pub type UResult<T> = std::result::Result<T, UError>;
#[derive(Debug, PartialEq)]
pub struct Book {
  title: String,
  author: String,
  clipping: Vec<Clipping>,
}

impl Book {
  fn new(title: String, author: String) -> Book {
    Book {
      title: title.trim().to_owned(),
      author: author.trim().to_owned(),
      clipping: vec![],
    }
  }
  fn add_clipping(&mut self, clipping: Clipping) {
    self.clipping.push(clipping);
  }
}

#[derive(Debug, PartialEq)]
pub struct Clipping {
  date_time: DateTime<Utc>,
  position: String,
  clipping: String,
  mark: Option<String>,
}

impl Clipping {
  fn new(
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

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines())
}

pub fn extract_bracket_content(line: &str) -> IResult<&str, &str> {
  let (line, content) = delimited(tag("("), take_until(")"), tag(")"))(line.trim())?;
  if !line.is_empty() {
    return extract_bracket_content(line);
  }
  Ok((line, content))
}

pub fn parse_book<'a>(line: &'a str) -> Result<Book, nom::Err<nom::error::Error<&'a str>>> {
  let (line, title) = take_until1("(")(line.trim())?;
  let (_, author) = extract_bracket_content(line)?;
  Ok(Book::new(title.to_owned(), author.to_owned()))
}

pub fn get_number(line: &str) -> Result<(&str, u32), nom::Err<nom::error::Error<&str>>> {
  let (remain, number) = preceded(take_till(is_numeric), digit0)(line)?;
  Ok((remain, number.to_owned().parse::<u32>().unwrap()))
}

pub fn parse_date_time(line: &str) -> Result<DateTime<Utc>, nom::Err<nom::error::Error<&str>>> {
  let (line, year) = get_number(line)?;
  let (line, month) = get_number(line)?;
  let (line, day) = get_number(line)?;
  let (line, (_, _, am_of_pm)) = tuple((alpha0, space0, alpha0))(line)?;
  let (line, hour) = get_number(line)?;
  let (line, minute) = get_number(line)?;
  let (_, second) = get_number(line)?;
  let time_offset = if am_of_pm == "上午" { 0 } else { 12 };
  Ok(
    Utc
      .with_ymd_and_hms(year as i32, month, day, hour + time_offset, minute, second)
      .unwrap(),
  )
}

pub fn parse_clipping(lines: Vec<&str>) -> Result<Clipping, nom::Err<nom::error::Error<&str>>> {
  let (remain, position) = preceded(take_till(is_numeric), take_till(is_alphabetic))(lines[0])?;
  let date_time = parse_date_time(remain)?;
  Ok(Clipping::new(lines[1].to_owned(), position.to_owned(), date_time, None))
}

pub fn parse_position_date_time<'a>(line: &'a str) -> Result<(String, DateTime<Utc>), nom::Err<nom::error::Error<&'a str>>> {
  let (remain, position) = preceded(take_till(is_numeric), take_till(is_alphabetic))(line)?;
  let date_time = parse_date_time(remain)?;
  Ok((position.to_owned(), date_time))
}

pub fn parse_lines<'a>(lines: &'a Vec<String>, books: &'a mut Vec<Book>) -> Result<(), nom::Err<nom::error::Error<&'a str>>> {
  let book = parse_book(&lines[0])?;
  let (position, date_time) = parse_position_date_time(&lines[1])?;
  let clipping = Clipping::new(lines[2].clone(), position, date_time, None);
  Ok(())
}

pub fn parse(path: PathBuf) -> UResult<()> {
  let lines = read_lines(path)?;
  let mut line_vec: Vec<String> = vec![];
  let mut books: Vec<Book> = vec![];
  lines.for_each(|line| {
    if let Ok(line) = line {
      if line == "==========" {
        return;
      };
      if !line.is_empty() {
        line_vec.push(line);
      }
      if line_vec.len() == 3 {
        parse_lines(&line_vec, &mut books);
        line_vec.clear();
      }
    }
  });
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_parse_book() -> UResult<()> {
    let parsed_book = parse_book("乌合之众:大众心理研究 (社会学经典名著) (古斯塔夫·勒宠)")?;
    let book = Book::new("乌合之众:大众心理研究".to_owned(), "古斯塔夫·勒宠".to_owned());
    assert_eq!(book, parsed_book);
    Ok(())
  }

  #[test]
  pub fn test_parse_date_time() -> UResult<()> {
    let parsed_date_time = parse_date_time("添加于 2015年2月14日星期六 下午3:21:03")?;
    let date_time = Utc.with_ymd_and_hms(2015, 2, 14, 15, 21, 3).unwrap();
    assert_eq!(date_time, parsed_date_time);
    Ok(())
  }

  #[test]
  pub fn test_parse_position_date_time() -> UResult<()> {
    let line = "- 您在位置 #116-119的标注 | 添加于 2015年2月14日星期六 下午3:21:03";
    let position = "116-119";
    let date_time =  Utc.with_ymd_and_hms(2015, 2, 14, 15, 21, 3).unwrap();
    let (parsed_position, parsed_date_time) = parse_position_date_time(line)?;
    assert_eq!(position, parsed_position);
    assert_eq!(date_time, parsed_date_time);
    Ok(())
  }
}
