use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::Struct,
  result::{ResolveError, ResolveResult},
  strukt::resolve_struct,
  values::Value,
};

pub fn resolve_event<'src>(
  meta: ResolveMeta<'src, '_>,
  cdr: VecDeque<Value<'src>>,
) -> ResolveResult<'src, Struct<'src>> {
  resolve_struct(meta, cdr, |name, span| {
    if meta.cst.events.contains_key(name) {
      let previous = meta.cst.events.get(name).unwrap();

      return Err(ResolveError::new(
        span,
        format!(
          "duplicated event name '{}'. previously defined at {}",
          name, previous.span
        ),
      ));
    }

    Ok(())
  })
}
