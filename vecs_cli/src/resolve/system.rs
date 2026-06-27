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
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, (System<'src>, Option<Node<'src>>)> {
  let mut s = SystemBuilder::default();
  let mut n = NodeBuilder::default();
  let maybe_value = values.pop_front();
  s.span(meta.span);
  n.span(meta.span);

  s.event("frame");
  s.node(None);

  if let Some(ref value) = maybe_value {
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
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("system name must be a symbol. instead found {}", value),
      ));
    }

    let maybe_value = values.front();

    if let Some(value) = maybe_value {
      match value.kind {
        // Should be `on <event>`
        ValueKind::Symbol(_) => {
          (values, s) = resolve_system_event(&meta, values, s)?;

          if !values.is_empty() {
            (values, s, n) = resolve_system_node(&meta, values, s, n)?;
          }
        }
        // Straight to components
        ValueKind::List(_) => {
          (values, s, n) = resolve_system_node(&meta, values, s, n)?;
        }
        _ => {
          return Err(ResolveError::new(
            meta.span,
            format!(
              "expected either the system event (`on <event>`) or the parameter list, but found neither"
            ),
          ));
        }
      }
    }

    if let Some(extra) = values.pop_front() {
      return Err(ResolveError::new(
        meta.span,
        format!(
          "unexpected value in system: {} (maybe you're missing a semicolon?)",
          extra
        ),
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
      "failed to build system {:?} ({:?}). this is a bug. run with VECS_DEBUG_AST set to dump the AST",
      s.name, s.span,
    )),
    s.node.unwrap().map(|_| n.build().expect(&format!(
      "failed to build node {:?} ({:?}). this is a bug. run with VECS_DEBUG_AST set to dump the AST",
      n.name, n.span,
    ))),
  ))
}

pub fn resolve_system_event<'src>(
  meta: &ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
  mut s: SystemBuilder<'src>,
) -> ResolveResult<'src, (VecDeque<Value<'src>>, SystemBuilder<'src>)> {
  let value = values.pop_front().unwrap();

  if let ValueKind::Symbol(ref on) = value.kind {
    // `on <event>`
    if *on == "on" {
      let maybe_value = values.pop_front();

      if let Some(value) = maybe_value {
        if let ValueKind::Symbol(event) = value.kind {
          if meta.cst.events.contains_key(event) {
            s.event(event);
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
  }

  Ok((values, s))
}

pub fn resolve_system_node<'src>(
  meta: &ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
  mut s: SystemBuilder<'src>,
  mut n: NodeBuilder<'src>,
) -> ResolveResult<
  'src,
  (
    VecDeque<Value<'src>>,
    SystemBuilder<'src>,
    NodeBuilder<'src>,
  ),
> {
  let value = values.pop_front().unwrap();

  if let ValueKind::List(ref values) = value.kind {
    n.init_components();
    s.node(n.name);

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
            format!("system parameter must be a symbol. instead found {}", value),
          ));
        }
      } else {
        panic!(
          "malformed ast: root expression is not an application. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
        );
      }
    }
  }

  Ok((values, s, n))
}
