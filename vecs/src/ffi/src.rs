use core::ptr;
use core::slice;

use crate::parse::data::src::{Location, Span};

#[repr(C)]
pub struct FfiString {
  pub str: *const cty::c_uchar,
  pub length: cty::size_t,
}

impl FfiString {
  pub fn new(str: *const cty::c_uchar, length: cty::size_t) -> Self {
    Self { str, length }
  }

  pub fn null() -> Self {
    Self::new(ptr::null(), 0)
  }

  pub unsafe fn to_str_unchecked(&self) -> Option<&str> {
    if self.str.is_null() {
      return None;
    }

    unsafe {
      let slice: &[u8] = slice::from_raw_parts(self.str, self.length);
      Some(str::from_utf8_unchecked(slice))
    }
  }

  pub fn from_str(s: &str) -> Self {
    Self::new(s.as_ptr(), s.len())
  }
}

impl From<&str> for FfiString {
  fn from(value: &str) -> Self {
    FfiString::from_str(value)
  }
}

impl Default for FfiString {
  fn default() -> Self {
    FfiString::null()
  }
}

#[repr(C)]
#[derive(Default)]
pub struct FfiLocation {
  pub filename: *const cty::c_uchar,
  pub byte_offset: cty::size_t,
  pub line: cty::size_t,
  pub column: cty::size_t,
}

impl<'a> From<Location<'a>> for FfiLocation {
  fn from(value: Location<'a>) -> Self {
    let filename = value.file.map(|f| f.as_ptr()).unwrap_or(ptr::null());

    FfiLocation {
      filename,
      byte_offset: value.byte_offset,
      line: value.line,
      column: value.column,
    }
  }
}

#[repr(C)]
#[derive(Default)]
pub struct FfiSpan {
  pub filename: *const cty::c_uchar,
  pub start_byte_offset: cty::size_t,
  pub start_line: cty::size_t,
  pub start_column: cty::size_t,
  pub end_byte_offset: cty::size_t,
  pub end_line: cty::size_t,
  pub end_column: cty::size_t,
}

impl<'a> From<Span<'a>> for FfiSpan {
  fn from(value: Span<'a>) -> Self {
    let filename = value.file.map(|f| f.as_ptr()).unwrap_or(ptr::null());

    FfiSpan {
      filename,
      start_byte_offset: value.start_byte_offset,
      start_line: value.start_line,
      start_column: value.start_column,
      end_byte_offset: value.start_byte_offset,
      end_line: value.start_line,
      end_column: value.start_column,
    }
  }
}
