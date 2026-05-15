#![allow(irrefutable_let_patterns)]
#![feature(assert_matches)]
#![feature(iter_intersperse)]

mod cli;
mod generate;
mod parse;
mod resolve;

use std::{
  fs::{self, File, OpenOptions},
  io::{self, Write, stdout},
  path::PathBuf,
  str::FromStr,
};

use clap::Parser as _;
use generate::{generate_header, generate_impl};

use crate::{
  cli::Cli,
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

  let dest = PathBuf::from_str(&cli.dest).expect("failed to parse output directory");

  if !dest.is_dir() {
    panic!("dest ({}) should be a directory!", dest.display());
  }

  let open_for_write = |filename: &str| -> io::Result<File> {
    OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(dest.join(filename))
  };

  let mut h_out_file: Box<dyn Write> = if cli.h_output == "-" {
    Box::new(stdout())
  } else {
    Box::new(
      open_for_write(&cli.h_output)
        .expect(&format!("failed to open header output `{}`", &cli.h_output)),
    )
  };

  let mut c_out_file: Box<dyn Write> = if cli.c_output == "-" {
    Box::new(stdout())
  } else {
    Box::new(open_for_write(&cli.c_output).expect(&format!(
      "failed to open implementation output `{}`",
      &cli.c_output
    )))
  };

  generate_header(&cst, &mut h_out_file).expect("error generating header output");
  generate_impl(&cst, &cli.h_output, &mut c_out_file)
    .expect("error generating implementation output");
}
