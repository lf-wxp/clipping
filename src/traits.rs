use std::path::Path;

pub trait Markdown {
  fn to_markdown(&self) -> String;
  fn to_file(&self, _: Option<&Path>) {}
}
