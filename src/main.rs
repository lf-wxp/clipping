use clap::Parser;

mod args;
mod parse;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  let cli = args::Cli::parse();
  match cli.command {
    args::Commands::Parse(args) => {
      parse::parse_book("乌合之众:大众心理研究 (社会学经典名著) (古斯塔夫·勒宠)");
    },
  };
  Ok(())
}
