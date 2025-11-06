use crate::resolve::{
  cst::{System, SystemBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
  ResolveMeta,
};

pub fn resolve_system<'src>(
  meta: ResolveMeta<'src, '_, '_>,
  cdr: &[Value<'src>],
) -> ResolveResult<'src, System<'src>> {
  let mut s = SystemBuilder::default();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      s.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("system name must be a symbol. instead found {}", value),
      ));
    }

    let maybe_value = cdr.get(1);

    if let Some(value) = maybe_value {
      if let ValueKind::List(ref values) = value.kind {
        for value in values {
          if let ValueKind::Application(ref values) = value.kind {
            if values.len() != 1 {
              return Err(ResolveError::new(
                value.span,
                format!(
                  "system param should be a single component name symbol. instead it's {}",
                  value,
                ),
              ));
            }

            let value = &values[0];

            if let ValueKind::Symbol(param) = value.kind {
              if meta.cst.components.contains_key(param) {
                s.add_param(param);
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("component `{}` not found.", param),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!("system parameter must be a symbol. instead found {}", value),
              ));
            }
          } else {
            panic!(
              "malformed ast: root expression is not an application. this is a bug.\n{}",
              meta.ast,
            );
          }
        }
      } else {
        return Err(ResolveError::new(
          value.span,
          format!("body of system should be a list. instead it's {}", value),
        ));
      }

      if let Some(extra) = cdr.get(2) {
        return Err(ResolveError::new(
          meta.span,
          format!("unexpected value {}", extra),
        ));
      }
    } else {
      return Err(ResolveError::new(
        meta.span,
        format!("system {} is missing its body", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a system tag must be followed by the system name",
    ));
  }

  Ok(s.build().expect(&format!(
    "failed to build system. this is a bug.\n{}",
    meta.ast
  )))
}
