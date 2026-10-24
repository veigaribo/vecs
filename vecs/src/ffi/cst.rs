use std::ptr;

use crate::{
  ffi::{
    context::{FfiVecsContextDouble, undouble},
    src::{FfiSpan, FfiString},
  },
  resolve::cst::{Component, TypeName},
};

/// Will return NULL if the component was not found (or there was an error).
///
/// `ctxp` should not be NULL.
/// `name` should not contain NULL.
/// This MUST ONLY be called after a `vecs_context_init` on the same context.
/// This should only be called after a `vecs_context_resolve` on the same context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_context_cst_get_component<'a>(
  ctxp: *const FfiVecsContextDouble,
  name: FfiString,
) -> *const Component<'a> {
  let ctx = match unsafe { undouble(ctxp.cast_mut()).as_ref() } {
    Some(r) => r,
    None => {
      println!("NULL ctxp in vecs_context_init");
      return ptr::null();
    }
  };

  let name = match unsafe { name.to_str_unchecked() } {
    Some(s) => s,
    None => {
      println!("NULL name in vecs_context_cst_get_component");
      return ptr::null();
    }
  };

  match &ctx.cst {
    Some(cst) => match cst.components.get(name) {
      Some(component) => component,
      None => ptr::null(),
    },
    None => {
      println!("called vecs_context_generate_header with no cst");
      return ptr::null();
    }
  }
}

macro_rules! vecs_component_getter {
  ($fn_name:ident, $extract:expr, $typ:ty) => {
    /// `componentp` should not be NULL.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn $fn_name<'a>(componentp: *const Component<'a>) -> $typ {
      let component = match unsafe { componentp.as_ref() } {
        Some(r) => r,
        None => {
          println!(concat!("NULL componentp in ", stringify!($fn_name)));
          return core::default::Default::default();
        }
      };

      $extract(component)
    }
  };

  (test $fn_name:ident, $res:expr, $typ:ty) => {
    /// `componentp` should not be NULL.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn $fn_name<'a>(componentp: *const Component<'a>) -> $typ {
      let component = match unsafe { componentp.as_ref() } {
        Some(r) => r,
        None => {
          println!(concat!("NULL componentp in ", stringify!($fn_name)));
          return core::default::Default::default();
        }
      };

      $res
    }
  };
}

vecs_component_getter!(
  vecs_component_get_span,
  |component: &Component<'_>| component.span().into(),
  FfiSpan
);

vecs_component_getter!(
  vecs_component_get_name,
  |component: &Component<'_>| component.name().into(),
  FfiString
);

/// `componentp` should not be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_component_get_type<'a>(
  componentp: *const Component<'a>,
) -> *const TypeName<'a> {
  let component = match unsafe { componentp.as_ref() } {
    Some(r) => r,
    None => {
      println!("NULL componentp in vecs_component_get_type");
      return core::default::Default::default();
    }
  };

  // This doesn't work with the macro because the closure does its usual thing
  // of messing lifetimes up
  &component.typ as *const TypeName<'a>
}

#[repr(C)]
pub struct FfiTypeIter {
  // arr: *const &'a str,
  arr: *const cty::c_void,
  length: usize,
  index: usize,
}

impl FfiTypeIter {
  pub fn new(v: &Vec<&str>) -> Self {
    FfiTypeIter {
      arr: v.as_slice().as_ptr().cast(),
      length: v.len(),
      index: 0,
    }
  }

  pub fn is_done(&self) -> bool {
    self.index >= self.length
  }

  pub fn next(&mut self) {
    assert!(!self.is_done());
    self.index += 1;
  }

  pub unsafe fn get(&self) -> FfiString {
    if self.arr.is_null() {
      return FfiString::null();
    }

    assert!(!self.is_done());
    let ptr = self.arr.cast::<&str>();
    let s = unsafe { *ptr.add(self.index) };
    FfiString::from_str(s)
  }
}

impl Default for FfiTypeIter {
  fn default() -> Self {
    Self {
      arr: ptr::null(),
      length: 0,
      index: 0,
    }
  }
}

/// `typp` should not be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_type_iter<'a>(
  typp: *const TypeName<'a>,
) -> FfiTypeIter {
  let typ = match unsafe { typp.as_ref() } {
    Some(r) => r,
    None => {
      println!(concat!("NULL componentp in vecs_component_type_iter"));
      return FfiTypeIter::default();
    }
  };

  FfiTypeIter::new(&typ.type_components)
}

/// `iterp` should not be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_type_iter_is_done(
  iterp: *const FfiTypeIter,
) -> cty::c_int {
  let iter = match unsafe { iterp.as_ref() } {
    Some(r) => r,
    None => {
      println!(concat!("NULL iterp in vecs_component_type_iter_is_done"));
      return 0;
    }
  };

  iter.is_done().into()
}

/// `iterp` should not be NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_type_iter_next(iterp: *mut FfiTypeIter) {
  let iter = match unsafe { iterp.as_mut() } {
    Some(r) => r,
    None => {
      println!(concat!("NULL iterp in vecs_component_type_iter_next"));
      return;
    }
  };

  iter.next();
}

/// `iterp` should not be NULL.
/// `iterp` MUST NOT be finished (vecs_component_type_iter_is_done).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vecs_type_iter_get(iterp: *const FfiTypeIter) -> FfiString {
  let iter = match unsafe { iterp.as_ref() } {
    Some(r) => r,
    None => {
      println!(concat!("NULL iterp in vecs_component_type_iter_next"));
      return FfiString::null();
    }
  };

  unsafe { iter.get() }
}

vecs_component_getter!(
  vecs_component_get_mask_i,
  |component: &Component<'_>| component.mask_i,
  cty::uint16_t
);

vecs_component_getter!(
  vecs_component_get_mask_j,
  |component: &Component<'_>| component.mask_j,
  cty::uint8_t
);
