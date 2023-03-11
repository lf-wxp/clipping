use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Parse(ParseArgs),
  Generate(ParseArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ParseArgs {
  #[clap(value_parser)]
  pub path: PathBuf,
}
