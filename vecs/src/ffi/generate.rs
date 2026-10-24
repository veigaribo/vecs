use std::io;

use crate::{
  ffi::{
    context::{FfiVecsContextDouble, undouble},
    src::FfiString,
  },
  generate::{generate_header, generate_impl},
};

#[repr(C)]
pub struct CFileWriter(*mut libc::FILE);

impl io::Write for CFileWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    assert!(!self.0.is_null());
    let result = unsafe { libc::fwrite(buf.as_ptr().cast(), 1, buf.len(), self.0) };

    if result == 0 {
      if unsafe { libc::ferror(self.0) } != 0 {
        return Err(io::Error::new(
          io::ErrorKind::WriteZero,
          format!("fwrite wrote only {} bytes", result),
        ));
      }
    }

    Ok(result)
  }

  fn flush(&mut self) -> io::Result<()> {
    assert!(!self.0.is_null());

    let result = unsafe { libc::fflush(self.0) };

    if result == 0 {
      Ok(())
    } else {
      Err(io::Error::last_os_error())
    }
  }
}

/// Will NOT consume the CST (or anything else).
///
/// Returns:
/// - +1 on success;
/// - 0 on unexpected failure.
///
/// `ctxp` should not be NULL.
/// `outp` should not be NULL.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
/// This should only be called after a `vecs_context_resolve` on the same context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_generate_header(
  ctxp: *mut FfiVecsContextDouble,
  outp: *mut libc::FILE,
) -> cty::c_int {
  let ctx = match unsafe { undouble(ctxp).as_mut() } {
    Some(r) => r,
    None => {
      println!("NULL ctx in vecs_context_generate_header");
      return 0;
    }
  };

  if outp.is_null() {
    println!("NULL ctx in vecs_context_generate_header");
    return 0;
  };

  let mut writer = CFileWriter(outp);

  match &ctx.cst {
    Some(cst) => match generate_header(cst, &mut writer) {
      Ok(_) => {
        return 1;
      }
      Err(err) => {
        println!("failed to generate: {}", err);
        return 0;
      }
    },
    None => {
      println!("called vecs_context_generate_header with no cst");
      return 0;
    }
  }
}

/// Will NOT consume the CST (or anything else).
///
/// Returns:
/// - +1 on success;
/// - 0 on unexpected failure.
///
/// `ctxp` should not be NULL.
/// `header_name` should not contain NULL.
/// `outp` should not be NULL.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
/// This should only be called after a `vecs_context_resolve` on the same context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_generate_impl(
  ctxp: *mut FfiVecsContextDouble,
  header_name: FfiString,
  outp: *mut libc::FILE,
) -> cty::c_int {
  let ctx = match unsafe { undouble(ctxp).as_mut() } {
    Some(r) => r,
    None => {
      println!("NULL ctx in vecs_context_generate_impl");
      return 0;
    }
  };

  if outp.is_null() {
    println!("NULL outp in vecs_context_generate_impl");
    return 0;
  };

  let header_name = match unsafe { header_name.to_str_unchecked() } {
    Some(s) => s,
    None => {
      println!("NULL header_name in vecs_context_generate_impl");
      return 0;
    }
  };

  let mut writer = CFileWriter(outp);

  match &ctx.cst {
    Some(cst) => match generate_impl(cst, header_name, &mut writer) {
      Ok(_) => {
        return 1;
      }
      Err(err) => {
        println!("failed to generate: {}", err);
        return 0;
      }
    },
    None => {
      println!("called vecs_context_generate_header with no cst");
      return 0;
    }
  }
}
