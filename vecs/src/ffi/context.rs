use crate::{
  parse::{ast::Ast, data::src::ParseSrc},
  resolve::cst::Cst,
};

#[repr(C)]
pub struct FfiContext<'src> {
  pub src: Option<ParseSrc<'src>>,
  pub ast: Option<Ast<'src>>,
  pub cst: Option<Cst<'src>>,
}

// We expose this instead of the struct we actually use to avoid having to define
// what everything inside FfiVecsContext is while still allowing for the type
// to be statically allocated.
#[repr(C, align(8))]
pub struct FfiVecsContextDouble {
  pub _pad: [u8; 448],
}

// We make sure FfiVecsContext and FfiVecsContextDouble have the same dimensions.
const _: () = {
  assert!(
    core::mem::align_of::<FfiVecsContextDouble>()
      == core::mem::align_of::<FfiContext>()
  );
  const _: () = assert!(
    core::mem::align_of::<FfiVecsContextDouble>()
      == core::mem::align_of::<FfiContext>()
  );
};

pub fn undouble<'src>(doublep: *mut FfiVecsContextDouble) -> *mut FfiContext<'src> {
  doublep.cast::<FfiContext>()
}

/// Call this before using a `vecs_context_t` for anything.
/// `ctxp` should not be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_init(ctxp: *mut FfiVecsContextDouble) {
  let ctx = match unsafe { undouble(ctxp).as_uninit_mut() } {
    Some(r) => r,
    None => {
      println!("NULL ctxp in vecs_context_init");
      return;
    }
  };

  ctx.write(FfiContext {
    src: None,
    ast: None,
    cst: None,
  });
}

/// Call this after you're done with a `vecs_context_t`.
///
/// `ctxp` should not be NULL.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_destroy(ctxp: *mut FfiVecsContextDouble) {
  if ctxp.is_null() {
    println!("NULL ctxp in vecs_context_destroy");
    return;
  };

  unsafe { std::ptr::drop_in_place(ctxp.cast::<FfiContext>()) };
}
