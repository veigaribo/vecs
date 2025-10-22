#![feature(assert_matches)]

mod cli;
mod generate;
mod parse;

use std::{fs, io::stdout};

use bumpalo::Bump;
use clap::Parser as _;

use crate::{
  cli::Cli,
  generate::generate,
  parse::{data::src::ParseSrc, parse, strip_comments},
};

fn main() {
  let arena = Bump::new();
  let cli = Cli::parse();

  let mut src_str = fs::read_to_string(&cli.source).expect("error reading file");
  strip_comments(&arena, &mut src_str);

  let src = ParseSrc::new(Some(&cli.source), &src_str);
  let success = parse(&arena, src).expect("parsing error");

  let mut out = stdout();
  generate(success.value, &mut out).expect("error generating output");
}
