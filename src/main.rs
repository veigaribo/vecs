mod cli;
mod parse;

use std::fs;

use bumpalo::Bump;
use clap::Parser as _;

use crate::{
  cli::Cli,
  parse::{data::src::ParseSrc, parse, strip_comments},
};

fn main() {
  let arena = Bump::new();
  let cli = Cli::parse();

  let mut src_str = fs::read_to_string(&cli.source).expect("error reading file");
  strip_comments(&arena, &mut src_str);

  let src = ParseSrc::new(Some(&cli.source), &src_str);
  let success = parse(&arena, src).expect("parsing error");

  println!("{:?}", success.value);
}
