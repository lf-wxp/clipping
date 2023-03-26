use std::path::Path;
use std::{fs::File, io::Write};

use crate::clipping::Clipping;
use crate::traits::Markdown;
use crate::parse::remove_whitespace;

#[derive(Debug, PartialEq)]
pub struct Book {
  title: String,
  author: String,
  clipping: Vec<Clipping>,
}

impl Book {
  pub fn new(title: String, author: String) -> Book {
    Book {
      title: remove_whitespace(title.trim()),
      author: author.trim().to_owned(),
      clipping: vec![],
    }
  }

  pub fn add_clipping(&mut self, clipping: Clipping) {
    self.clipping.push(clipping);
  }

  pub fn is_identical(&self, book: &Book) -> bool {
    self.author == book.author && self.title == book.title
  }

  pub fn get_title(&self) -> &str {
    &self.title
  }
}

impl Markdown for Book {
  fn to_markdown(&self) -> String {
    format!(
      "# {:} \nAuthor: `{:}` \n{:}",
      self.title,
      self.author,
      self
        .clipping
        .iter()
        .map(|x| x.to_markdown())
        .collect::<Vec<String>>()
        .join("\r"),
    )
  }

  fn to_file(&self, path: Option<&Path>) {
    if let Some(path) = path {
      let path = path.join(format!("{:}.md", self.title));
      let mut file = File::create(path).unwrap();
      write!(file, "{}", self.to_markdown()).unwrap();
    };
  }
}
