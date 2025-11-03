use crate::resolve::{
  cst::{System, SystemBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
  ResolveMeta,
};

pub fn resolve_system<'a>(
  meta: ResolveMeta<'a, '_, '_>,
  cdr: &[Value<'a>],
) -> ResolveResult<'a, System<'a>> {
  let mut s = SystemBuilder::default();
  let arg1 = cdr.get(0);

  if let Some(name) = arg1 {
    if let ValueKind::Symbol(name) = name.kind {
      s.name(name);
    } else {
      return Err(ResolveError::new(
        name.span,
        format!("system name must be a symbol. instead found {}", name),
      ));
    }

    let arg2 = cdr.get(1);

    if let Some(body) = arg2 {
      if let ValueKind::List(ref values) = body.kind {
        for value in values {
          if let ValueKind::List(ref app) = value.kind {
            if app.len() != 1 {
              return Err(ResolveError::new(
                body.span,
                format!(
                  "system param should be a single component name symbol. instead it's {}",
                  value,
                ),
              ));
            }

            let head = &app[0];

            if let ValueKind::Symbol(param) = head.kind {
              if meta.state.components.contains_key(param) {
                s.add_param(param);
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("component named {} not found.", param),
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
              "malformed ast: root expression is not a list. this is a bug.\n{}",
              meta.ast
            );
          }
        }
      } else {
        return Err(ResolveError::new(
          body.span,
          format!(
            "body of system {} should be a list. instead it's {}",
            name, body
          ),
        ));
      }
    } else {
      return Err(ResolveError::new(
        meta.span,
        format!("system {} is missing its body", name),
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
