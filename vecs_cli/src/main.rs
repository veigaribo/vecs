#![allow(irrefutable_let_patterns)]
#![feature(assert_matches)]
#![feature(iter_intersperse)]

mod cli;
mod generate;
mod parse;
mod resolve;

use std::{fs, io::stdout};

use clap::Parser as _;

use crate::{
  cli::Cli,
  generate::generate,
  parse::{data::src::ParseSrc, parse, strip_comments},
  resolve::resolve,
};

fn main() {
  let cli = Cli::parse();

  let mut src_str = fs::read_to_string(&cli.source).expect("error reading file");
  strip_comments(&mut src_str);

  let src = ParseSrc::new(Some(&cli.source), &src_str);
  let ast = parse(src).expect("parsing error").value;
  let cst = resolve(ast).expect("resolving error");

  let mut out = stdout();
  generate(cst, &mut out).expect("error generating output");
}
