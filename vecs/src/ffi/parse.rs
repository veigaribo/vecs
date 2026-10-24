use std::ffi::CString;

use crate::{
  ffi::{
    context::{FfiVecsContextDouble, undouble},
    src::FfiLocation,
  },
  parse::parse,
};

/// WARNING: If you got handed one of these, you must call `vecs_parse_error_free`
/// on it eventually.
#[repr(C)]
pub struct FfiParseError {
  pub message: *mut cty::c_uchar, // CString must be dropped
  pub message_len: cty::size_t,
  pub location: FfiLocation,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_parse_error_free(errp: *mut FfiParseError) {
  let err = match unsafe { errp.as_ref() } {
    Some(r) => r,
    None => {
      println!("NULL errp in vecs_parse_error_free");
      return;
    }
  };

  unsafe {
    let _ = CString::from_raw(err.message.cast());
  };
}

/// Will consume the ingested text.
///
/// Returns:
/// - +1 on success;
/// - 0 on unexpected failure (`errorp` will NOT be filled);
/// - -1 on parse error (`errorp` WILL be filled).
///
/// If you get a parse error, you must call `vecs_parse_error_free` on it
/// eventually.
///
/// `ctxp` should not be NULL.
/// `errorp` should not be NULL.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
/// This should only be called after a `vecs_context_ingest` on the same context.
#[must_use]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_parse(
  ctxp: *mut FfiVecsContextDouble,
  errorp: *mut FfiParseError,
) -> cty::c_int {
  let ctx = match unsafe { undouble(ctxp).as_mut() } {
    Some(r) => r,
    None => {
      println!("NULL ctx in vecs_context_parse");
      return 0;
    }
  };

  match ctx.src.take() {
    Some(src) => match parse(src) {
      Ok(success) => {
        ctx.ast = Some(success.value);
        return 1;
      }
      Err(err) => {
        let error = match unsafe { errorp.as_uninit_mut() } {
          Some(r) => r,
          None => {
            println!("NULL errorp in vecs_context_parse");
            return 0;
          }
        };

        let message = match CString::new(err.message.as_ref()) {
          Ok(message) => message.into_raw(),
          Err(err) => {
            println!("invalid string in vecs_context_parse: {}", err);
            return 0;
          }
        };

        error.write(FfiParseError {
          message: message.cast(),
          message_len: err.message.len(),
          location: err.location.into(),
        });

        return -1;
      }
    },
    None => {
      println!("called vecs_context_parse with no src");
      return 0;
    }
  }
}
