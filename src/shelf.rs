use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use crate::book::Book;
use crate::clipping::Clipping;
use crate::traits::Markdown;

#[derive(Debug, PartialEq)]
pub struct BookShelf {
  books: Vec<Book>,
}

impl BookShelf {
  pub fn new() -> Self {
    BookShelf { books: vec![] }
  }

  pub fn add_book_and_clipping(&mut self, mut book: Book, clipping: Clipping) {
    if let Some(i) = self.books.iter().position(|x| x.is_identical(&book)) {
      self.books[i].add_clipping(clipping)
    } else {
      book.add_clipping(clipping);
      self.books.push(book);
    }
  }

  pub fn to_content(&self) -> String {
    let content = self
      .books
      .iter()
      .map(|x| format!("- [{0}](./{0}.md)", x.get_title()))
      .collect::<Vec<String>>()
      .join("\n");
    format!("# Clipping \r\r{}", content)
  }

  pub fn to_summary(&self) -> String {
    let content = self
      .books
      .iter()
      .map(|x| format!("- [{0}](./clipping/{0}.md)", x.get_title()))
      .collect::<Vec<String>>()
      .join("\n  ");
    format!("- [Clipping](./clipping/index.md) \r  {}", content)
  }

  pub fn to_content_file(&self, path: &Path) {
    let path = path.join(format!("{:}.md", "index"));
    let mut file = File::create(path).unwrap();
    write!(file, "{}", self.to_content()).unwrap();
  }

  pub fn to_summary_file(&self, path: &Path) {
    let path = path.join(format!("{:}.md", "summary"));
    let mut file = File::create(path).unwrap();
    write!(file, "{}", self.to_summary()).unwrap();
  }
}

impl Markdown for BookShelf {
  fn to_markdown(&self) -> String {
    self
      .books
      .iter()
      .map(|x| x.to_markdown())
      .collect::<Vec<String>>()
      .join("\r")
  }

  fn to_file(&self, path: Option<&Path>) {
    let output_path = match path {
      Some(path) => path,
      None => Path::new("./output"),
    };
    create_dir_all(output_path).unwrap();
    self.to_content_file(output_path);
    self.to_summary_file(output_path);
    self.books.iter().for_each(|x| x.to_file(Some(output_path)));
  }
}
