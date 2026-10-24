use crate::{
  ffi::{
    context::{FfiVecsContextDouble, undouble},
    src::FfiString,
  },
  parse::data::src::ParseSrc,
};

/// Necessary step before parsing. The effects of this function will be undone when
/// parsing is attempted, so you may need to invoke it multiple times.
/// `datap` should contain the Vecs file contents to be parsed. `filenamep` is
/// there only to show up in error messages -- nothing else.
///
/// `ctxp` should not be NULL.
/// `filename` may be NULL.
/// `datap` should not be NULL.
/// `filename`, if present, and `datap` MUST be valid UTF-8.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_ingest(
  ctxp: *mut FfiVecsContextDouble,
  filename: FfiString, // For usage in error messages only.
  data: FfiString,
) -> cty::c_int {
  let ctx = match unsafe { undouble(ctxp).as_mut() } {
    Some(r) => r,
    None => {
      println!("NULL ctxp in vecs_context_ingest");
      return 0;
    }
  };

  let file = unsafe { filename.to_str_unchecked() };

  let data_string = match unsafe { data.to_str_unchecked() } {
    Some(s) => s,
    None => {
      println!("NULL data in vecs_context_ingest");
      return 0;
    }
  };

  ctx.src = Some(ParseSrc::new(file, data_string));
  return 1;
}
