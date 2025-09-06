#![cfg_attr(feature = "lax", allow(unused))]
#![feature(allocator_api)]

use std::fmt::Write;

use bumpalo::{Bump, collections::Vec};

use crate::parse::{
  comments::parse_comment,
  data::{src::ParseSrc, str::Span},
};

mod parse;

fn main() {}
