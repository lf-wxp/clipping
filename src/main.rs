use clap::Parser;

mod args;
mod parse;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  let cli = args::Cli::parse();
  match cli.command {
    args::Commands::Parse(args) => {
      parse::parse(args.path)?;
    },
    args::Commands::Generate(args) => {
      parse::to_file(args.path);
    },
  };
  Ok(())
}
