use std::path::{Path};
use std::fs::create_dir_all;

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
    self.books.iter().for_each(|x| x.to_file(Some(&output_path)));
  }
}
