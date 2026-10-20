use std::collections::{HashMap, VecDeque};

use crate::resolve::{
  ResolveMeta,
  cst::ComponentMethod,
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

pub fn resolve_method<'src>(
  meta: ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, (&'src str, HashMap<ComponentMethod, &'src str>)> {
  let c: &'src str;
  let mut method_map: HashMap<ComponentMethod, &'src str> = HashMap::new();

  let maybe_value = values.pop_front();

  if let Some(ref value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if !meta.cst.components.contains_key(name) {
        return Err(ResolveError::new(
          value.span,
          format!("component `{}` not found.", name),
        ));
      }

      c = name;
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("component name must be a symbol. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a method tag must be followed by the component name",
    ));
  }

  let maybe_value = values.pop_front();

  if let Some(value) = maybe_value {
    if let ValueKind::List(mut pairs) = value.kind {
      while let Some(value) = pairs.pop_front() {
        let app_value_span = value.span;
        if let ValueKind::Application(mut values) = value.kind {
          if let Some(ref value) = values.pop_front() {
            if let ValueKind::Symbol(method_name) = value.kind {
              let method = match ComponentMethod::try_from(method_name) {
                Ok(m) => m,
                Err(_) => {
                  return Err(ResolveError::new(
                    value.span,
                    format!("method `{}` not recognized.", method_name),
                  ));
                }
              };

              if let Some(ref value) = values.pop_front() {
                if let ValueKind::Symbol(fn_name) = value.kind {
                  method_map.insert(method, fn_name);
                } else {
                  return Err(ResolveError::new(
                    value.span,
                    format!(
                      "method function name should be a symbol. instead found {}",
                      value
                    ),
                  ));
                }
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("method missing its function name."),
                ));
              }

              if let Some(extra) = values.pop_front() {
                return Err(ResolveError::new(
                  app_value_span,
                  format!(
                    "unexpected value in method: {} (maybe you're missing a comma?)",
                    extra
                  ),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!("method name should be a symbol. instead found {}", value),
              ));
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              format!("missing the method name."),
            ));
          }
        } else {
          panic!(
            "malformed ast: root expression is not an application. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
          );
        }
      }
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("method body should be a list. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(meta.span, "method is missing its body"));
  }

  Ok((c, method_map))
}
