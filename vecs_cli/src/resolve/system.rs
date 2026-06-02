use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::{System, SystemBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

use super::cst::{Node, NodeBuilder};

pub fn resolve_system<'src>(
  meta: ResolveMeta<'src, '_>,
  values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, (System<'src>, Node<'src>)> {
  let mut s = SystemBuilder::default();
  let mut n = NodeBuilder::default();
  let maybe_value = values.get(0);
  s.span(meta.span);
  n.span(meta.span);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if meta.cst.systems.contains_key(name) {
        let previous = meta.cst.systems.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "duplicated system name '{}'. previously defined at {}",
            name, previous.span
          ),
        ));
      }

      if meta.cst.nodes.contains_key(name) {
        let previous = meta.cst.nodes.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "system name conflicts with a node: '{}', previously defined at {} (systems generate a corresponding node with the same name)",
            name, previous.span
          ),
        ));
      }

      s.name(name);
      n.name(name);
      s.node(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("system name must be a symbol. instead found {}", value),
      ));
    }

    let mut value_cursor = 1;
    let maybe_value = values.get(value_cursor);

    if let Some(value) = maybe_value {
      if let ValueKind::Symbol(ref on) = value.kind {
        // `on <event>`
        if *on == "on" {
          value_cursor += 1;
          let maybe_value = values.get(value_cursor);

          if let Some(value) = maybe_value {
            if let ValueKind::Symbol(event) = value.kind {
              if meta.cst.events.contains_key(event) {
                s.event(event);
                value_cursor += 1;
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("event `{}` not found", event),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!("expected an event name. instead found {}", value),
              ));
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              format!("system missing its event name"),
            ));
          }
        } else {
          return Err(ResolveError::new(
            value.span,
            format!("expected an `on` symbol. instead found {}", value),
          ));
        }
      } else {
        // Implied `frame` event
        s.event("frame");
      }

      let maybe_values = values.get(value_cursor);
      if let Some(values) = maybe_values {
        if let ValueKind::List(ref values) = values.kind {
          n.init_components();

          for value in values {
            if let ValueKind::Application(ref values) = value.kind {
              if values.len() != 1 {
                return Err(ResolveError::new(
                  value.span,
                  format!(
                    "system param should be a single component name symbol. instead it's {}. maybe you forgot a semicolon?",
                    value,
                  ),
                ));
              }

              let value = &values[0];

              if let ValueKind::Symbol(param) = value.kind {
                if meta.cst.components.contains_key(param) {
                  n.add_component(param);
                } else {
                  return Err(ResolveError::new(
                    value.span,
                    format!("component `{}` not found", param),
                  ));
                }
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!(
                    "system parameter must be a symbol. instead found {}",
                    value
                  ),
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
            format!("expected a list of parameters. instead found {}", values),
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
        format!(
          "expected either the system event (`on <event>`) or the parameter list, but found neither"
        ),
      ));
    }

    if let Some(extra) = values.get(value_cursor + 1) {
      return Err(ResolveError::new(
        meta.span,
        format!("unexpected value {}", extra),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a system tag must be followed by the system name",
    ));
  }

  Ok((
    s.build().expect(&format!(
      "failed to build system. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
    )),
    n.build().expect(&format!(
      "failed to build node. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
    )),
  ))
}
