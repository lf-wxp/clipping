use chrono::prelude::*;
use nom::{
  self,
  bytes::complete::{tag, take_till, take_until, take_until1},
  sequence::{delimited, preceded, tuple},
  IResult,
};
use nom_unicode::{
  complete::{alpha0, digit0, space0},
  is_alphabetic, is_numeric,
};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

use crate::{book::Book, clipping::Clipping, shelf::BookShelf, traits::Markdown};

pub type UError = Box<dyn std::error::Error>;
pub type UResult<T> = std::result::Result<T, UError>;

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines())
}

fn extract_bracket_content(line: &str) -> IResult<&str, &str> {
  let (line, content) = delimited(tag("("), take_until(")"), tag(")"))(line.trim())?;
  if !line.is_empty() {
    return extract_bracket_content(line);
  }
  Ok((line, content))
}

fn extract_recursive_bracket_content(line: &str) -> IResult<&str, &str> {
  delimited(tag("("), take_until("("), tag("("))(line.trim())
}

fn parse_author(line: &str) -> IResult<&str, &str> {
  extract_bracket_content(line).or_else(|_| extract_recursive_bracket_content(line))
}

fn parse_book<'a>(line: &'a str) -> Result<Book, nom::Err<nom::error::Error<&'a str>>> {
  let (line, title) = take_until1("(")(line.trim())?;
  let (_, author) = parse_author(line)?;
  Ok(Book::new(title.to_owned(), author.to_owned()))
}

fn get_number(line: &str) -> Result<(&str, u32), nom::Err<nom::error::Error<&str>>> {
  let (remain, number) = preceded(take_till(is_numeric), digit0)(line)?;
  Ok((remain, number.to_owned().parse::<u32>().unwrap()))
}

fn parse_date_time(line: &str) -> Result<DateTime<Utc>, nom::Err<nom::error::Error<&str>>> {
  let (line, year) = get_number(line)?;
  let (line, month) = get_number(line)?;
  let (line, day) = get_number(line)?;
  let (line, (_, _, am_of_pm)) = tuple((alpha0, space0, alpha0))(line)?;
  let (line, hour) = get_number(line)?;
  let (line, minute) = get_number(line)?;
  let (_, second) = get_number(line)?;
  let time_offset = if am_of_pm == "上午" { 0 } else { 12 };
  let hour = {
    let h = hour + time_offset;
    if h >= 24 {
      h - 24
    } else {
      h
    }
  };
  Ok(
    Utc
      .with_ymd_and_hms(year as i32, month, day, hour, minute, second)
      .unwrap(),
  )
}

fn parse_position_date_time(
  line: &str,
) -> Result<(String, DateTime<Utc>), nom::Err<nom::error::Error<&str>>> {
  let (remain, position) = preceded(take_until("#"), take_till(is_alphabetic))(line)?;
  let date_time = parse_date_time(remain)?;
  Ok((position.to_owned(), date_time))
}

fn parse_lines(
  lines: &[String],
) -> Result<(Book, Clipping), nom::Err<nom::error::Error<&str>>> {
  let book = parse_book(&lines[0])?;
  let (position, date_time) = parse_position_date_time(&lines[1])?;
  let clipping = Clipping::new(lines[2].clone(), position, date_time, None);
  Ok((book, clipping))
}

pub fn parse(path: PathBuf) -> UResult<BookShelf> {
  let lines = read_lines(path)?;
  let mut book_shelf = BookShelf::new();
  lines
    .filter(|line| {
      if let Ok(line) = line {
        return !line.is_empty();
      }
      false
    })
    .fold(vec![Vec::new()], |mut acc, x| {
      if let Ok(x) = x {
        if x == "==========" {
          acc.push(Vec::new());
        } else {
          acc.last_mut().unwrap().push(x);
        }
      }
      acc
    })
    .iter()
    .filter(|x| !x.is_empty())
    .filter_map(|x| parse_lines(x).ok())
    .for_each(|x| {
      let (book, clipping) = x;
      book_shelf.add_book_and_clipping(book, clipping);
    });
  Ok(book_shelf)
}

pub fn to_file(path: PathBuf) {
  let book_shelf = parse(path).unwrap();
  book_shelf.to_file(None);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_parse_book() -> UResult<()> {
    let parsed_book = parse_book("乌合之众:大众心理研究 (社会学经典名著) (古斯塔夫·勒宠)")?;
    let book = Book::new(
      "乌合之众:大众心理研究".to_owned(),
      "古斯塔夫·勒宠".to_owned(),
    );
    assert_eq!(book, parsed_book);
    Ok(())
  }
  #[test]
  pub fn test_parse_author() -> UResult<()> {
    let (_, line1) = parse_author("(社会学经典名著) (古斯塔夫·勒宠)")?;
    let (_, line2) = parse_author("(知乎「盐」系列) (采铜)")?;
    let (_, line3) = parse_author("(万维钢(同人于野))")?;
    assert_eq!(line1, "古斯塔夫·勒宠");
    assert_eq!(line2, "采铜");
    assert_eq!(line3, "万维钢");
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
    let position = "#116-119";
    let date_time = Utc.with_ymd_and_hms(2015, 2, 14, 15, 21, 3).unwrap();
    let (parsed_position, parsed_date_time) = parse_position_date_time(line)?;
    assert_eq!(position, parsed_position);
    assert_eq!(date_time, parsed_date_time);
    Ok(())
  }
}
