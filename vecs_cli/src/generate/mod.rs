mod common;
mod generics;
mod header;
mod imple;
mod masks;

use std::io;

use header::Header;
use imple::Impl;

use crate::resolve::cst::Cst;

pub fn generate_header<W: io::Write>(data: &Cst, w: &mut W) -> io::Result<()> {
  let h = Header { data };
  write!(w, "{}", h)
}

pub fn generate_impl<W: io::Write>(
  data: &Cst,
  header_name: &str,
  w: &mut W,
) -> io::Result<()> {
  let c = Impl { data, header_name };
  write!(w, "{}", c)
}
