use std::slice;

use crate::parse::strip_comments;

/// Gets rid of comments for further processing. The comments will be replaced by
/// whitespace, so source positions (line X column Y) will be preserved.
///
/// `datap` should not be NULL.
/// `datap` MUST point to valid UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_strip_comments(
  datap: *mut cty::c_uchar,
  length: cty::size_t,
) {
  let data = match unsafe { datap.as_mut() } {
    Some(r) => r,
    None => {
      println!("NULL datap in vecs_strip_comments_checked");
      return;
    }
  };

  let string = unsafe {
    // Assume user gave us valid data
    let slice: &mut [u8] = slice::from_raw_parts_mut(data, length);
    str::from_utf8_unchecked_mut(slice)
  };

  strip_comments(string);
}
