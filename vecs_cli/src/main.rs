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

  let open_for_write = |filename: &str| -> io::Result<File> {
    OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(filename)
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
