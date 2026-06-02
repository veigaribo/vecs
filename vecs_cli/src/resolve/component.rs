use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::Struct,
  result::{ResolveError, ResolveResult},
  strukt::resolve_struct,
  values::Value,
};

pub fn resolve_component<'src>(
  meta: ResolveMeta<'src, '_>,
  cdr: VecDeque<Value<'src>>,
) -> ResolveResult<'src, Struct<'src>> {
  resolve_struct(meta, cdr, |name, span| {
    if meta.cst.components.contains_key(name) {
      let previous = meta.cst.components.get(name).unwrap();

      return Err(ResolveError::new(
        span,
        format!(
          "duplicated component name '{}'. previously defined at {}",
          name, previous.span
        ),
      ));
    }

    Ok(())
  })
}
