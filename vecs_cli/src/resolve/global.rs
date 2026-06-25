use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::{TypeName, TypeNameBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

pub fn resolve_global<'src>(
  meta: ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, TypeName<'src>> {
  let mut s = TypeNameBuilder::default();
  s.span(meta.span);
  s.type_components(vec![]);

  let maybe_value = values.pop_front();

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if meta.cst.globals.contains_key(name) {
        let previous = meta.cst.globals.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "duplicated global name '{}'. previously defined at {}",
            name, previous.span
          ),
        ));
      }

      s.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("global name must be a symbol. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a global tag must be followed by the global name",
    ));
  }

  if values.is_empty() {
    return Err(ResolveError::new(
      meta.span,
      format!("global {} is missing its type", s.name.unwrap()),
    ));
  }

  while let Some(value) = values.pop_front() {
    if let ValueKind::Symbol(name) = value.kind {
      s.add_type_component(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!(
          "global type must be a sequence of symbols. instead found {}",
          value
        ),
      ));
    }
  }

  Ok(s.build().expect(&format!(
    "failed to build global. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
  )))
}
