use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::{Node, NodeBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

pub fn resolve_node<'src>(
  meta: ResolveMeta<'src, '_>,
  values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, Node<'src>> {
  let mut n = NodeBuilder::default();
  let maybe_value = values.get(0);
  n.span(meta.span);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if meta.cst.nodes.contains_key(name) {
        let previous = meta.cst.nodes.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "duplicated node name '{}'. previously defined at {} (systems generate corresponding nodes)",
            name, previous.span
          ),
        ));
      }

      n.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("node name must be a symbol. instead found {}", value),
      ));
    }

    let maybe_value = values.get(1);

    if let Some(value) = maybe_value {
      if let ValueKind::List(ref values) = value.kind {
        for value in values {
          if let ValueKind::Application(ref values) = value.kind {
            if values.len() != 1 {
              return Err(ResolveError::new(
                value.span,
                format!(
                  "node param should be a single component name symbol. instead it's {}. maybe you forgot a semicolon?",
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
                  format!("component `{}` not found.", param),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!("node parameter must be a symbol. instead found {}", value),
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
          format!("body of node should be a list. instead it's {}", value),
        ));
      }

      if let Some(extra) = values.get(2) {
        return Err(ResolveError::new(
          meta.span,
          format!(
            "unexpected value in node: {} (maybe you're missing a semicolon?)",
            extra
          ),
        ));
      }
    } else {
      return Err(ResolveError::new(
        meta.span,
        format!("node {} is missing its body", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a node tag must be followed by the node name",
    ));
  }

  Ok(n.build().expect(&format!(
    "failed to build node. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
  )))
}
