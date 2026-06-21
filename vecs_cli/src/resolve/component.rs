use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::{TypeName, TypeNameBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

pub fn resolve_component<'src>(
  meta: ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, TypeName<'src>> {
  let mut s = TypeNameBuilder::default();
  s.span(meta.span);
  s.type_components(vec![]);

  let maybe_value = values.pop_front();

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if meta.cst.components.contains_key(name) {
        let previous = meta.cst.components.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "duplicated component name '{}'. previously defined at {}",
            name, previous.span
          ),
        ));
      }

      s.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("component name must be a symbol. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a component tag must be followed by the component name",
    ));
  }

  while let Some(value) = values.pop_front() {
    if let ValueKind::Symbol(name) = value.kind {
      s.add_type_component(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!(
          "component type must be a sequence of symbols. instead found {}",
          value
        ),
      ));
    }
  }

  Ok(s.build().expect(&format!(
    "failed to build component. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
  )))
}
