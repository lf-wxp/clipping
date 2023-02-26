use nom::{
  self,
  bytes::complete::{tag, take_until, take_until1},
  sequence::delimited,
  IResult,
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
}

impl Book {
  fn new(title: &str, author: &str) -> Book {
    Book {
      title: title.trim().to_owned(),
      author: author.trim().to_owned(),
    }
  }
}

#[derive(Debug)]
struct DateTime {
  time: String,
  date: String,
}
#[derive(Debug)]
struct Clipping {
  book: Book,
  datetime: DateTime,
  position: String,
  clipping: String,
  mark: Option<String>,
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines())
}

pub fn extrack_bracket_content(line: &str) -> IResult<&str, &str> {
  let (line, content) = delimited(tag("("), take_until(")"), tag(")"))(line.trim())?;
  if !line.is_empty() {
    return extrack_bracket_content(line);
  }
  Ok((line, content))
}

pub fn parse_book(line: &str) -> Result<Book, nom::Err<nom::error::Error<&str>>> {
  let (line, title) = take_until1("(")(line.trim())?;
  let (_, author) = extrack_bracket_content(line)?;
  Ok(Book::new(title, author))
}

pub fn parse(path: PathBuf) -> Result<(), io::Error> {
  let lines = read_lines(path)?;
  let mut line_vec: Vec<String> = vec![];
  lines.for_each(|line| {
    if let Ok(line) = line {
      if line == "==========" {
        return;
      };
      if !line.is_empty() {
        line_vec.push(line);
      }
      if line_vec.len() == 3 {
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
    let book = Book::new("乌合之众:大众心理研究", "古斯塔夫·勒宠");

    assert_eq!(book, parsed_book);
    Ok(())
  }

}
