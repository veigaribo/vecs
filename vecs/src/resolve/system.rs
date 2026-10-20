use std::collections::VecDeque;

use crate::{
  parse::data::str::Span,
  resolve::{
    ResolveMeta,
    cst::{System, SystemBuilder},
    result::{ResolveError, ResolveResult},
    values::{Value, ValueKind},
  },
};

use super::cst::{Node, NodeBuilder};

#[derive(Debug, Clone)]
pub enum SystemNode<'src> {
  None,
  Inline(Node<'src>),
  Named(&'src str),
}

pub fn resolve_system<'src>(
  meta: ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, (System<'src>, Option<Node<'src>>)> {
  let mut s = SystemBuilder::default();
  let mut system_node: SystemNode = SystemNode::None;
  let name_span: Span<'src>;

  s.span(meta.span);
  s.node(None);

  if let Some(ref value) = values.pop_front() {
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

      s.name(name);
      name_span = value.span;
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("system name must be a symbol. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a system tag must be followed by the system name",
    ));
  }

  let system_name = s.name.unwrap();

  while let Some(value) = values.pop_front() {
    if value.is_symbol_ignore_ascii_case("on") {
      if s.event.is_some() {
        return Err(ResolveError::new(
          meta.span,
          format!("event specified twice for system {}", system_name),
        ));
      }

      (values, s) = resolve_system_event(&meta, value.span, values, s)?;
    } else if value.is_symbol_ignore_ascii_case("for") {
      if !matches!(system_node, SystemNode::None) {
        return Err(ResolveError::new(
          meta.span,
          format!("node specified twice for system {}", system_name),
        ));
      }

      (values, s, system_node) =
        resolve_system_node(&meta, name_span, value.span, values, s)?;
    } else {
      return Err(ResolveError::new(
        meta.span,
        format!(
          "expected either the system event (`on <event>`) or the system node (either `for <node>` or `for {{ <components...> }}`), but found neither"
        ),
      ));
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

  let node = match system_node {
    SystemNode::None => {
      s.node(None);
      None
    }
    SystemNode::Inline(node) => {
      s.node(Some(node.name));
      Some(node)
    }
    SystemNode::Named(node_name) => {
      s.node(Some(node_name));
      None
    }
  };

  if s.event.is_none() {
    // TODO: Error!
    s.event("frame");
  }

  Ok((
    s.build().expect(&format!(
      "failed to build system {:?} ({:?}). this is a bug. run with VECS_DEBUG_AST set to dump the AST",
      s.name, s.span,
    )),
    node,
  ))
}

pub fn resolve_system_event<'src>(
  meta: &ResolveMeta<'src, '_>,
  pre_span: Span<'src>, // Span of the `on` symbol
  mut values: VecDeque<Value<'src>>,
  mut s: SystemBuilder<'src>,
) -> ResolveResult<'src, (VecDeque<Value<'src>>, SystemBuilder<'src>)> {
  if let Some(value) = values.pop_front() {
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
      pre_span,
      format!("system missing its event name"),
    ));
  }

  Ok((values, s))
}

pub fn resolve_system_node<'src>(
  meta: &ResolveMeta<'src, '_>,
  name_span: Span<'src>, // Span of the system name
  pre_span: Span<'src>,  // Span of the `for` symbol
  mut values: VecDeque<Value<'src>>,
  s: SystemBuilder<'src>,
) -> ResolveResult<'src, (VecDeque<Value<'src>>, SystemBuilder<'src>, SystemNode<'src>)>
{
  if let Some(value) = values.pop_front() {
    if let ValueKind::Symbol(node) = value.kind {
      // Must be named.
      if meta.cst.nodes.contains_key(node) {
        return Ok((values, s, SystemNode::Named(node)));
      } else {
        return Err(ResolveError::new(
          value.span,
          format!("node `{}` not found", node),
        ));
      }
    } else if let ValueKind::List(mut values) = value.kind {
      // Must be inline.
      let system_name = s.name.unwrap();

      if meta.cst.nodes.contains_key(system_name) {
        let previous = meta.cst.nodes.get(system_name).unwrap();

        return Err(ResolveError::new(
          name_span,
          format!(
            "duplicated node named '{}' (the same as its system). previously defined at {}",
            system_name, previous.span
          ),
        ));
      }

      let mut n = NodeBuilder::default();
      n.span(value.span);
      n.name(system_name);
      n.init_components();

      while let Some(value) = values.pop_front() {
        if let ValueKind::Application(ref values) = value.kind {
          if values.len() != 1 {
            return Err(ResolveError::new(
              value.span,
              format!(
                "system param should be a single component name symbol. instead it's {}. maybe you forgot a comma?",
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

      let s_name = s.name;
      let s_span = s.span;
      return Ok((values, s, SystemNode::Inline(n.build().expect(&format!("failed to build inline node {:?} ({:?}). this is a bug. run with VECS_DEBUG_AST set to dump the AST", s_name, s_span)))));
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("expected an event name. instead found {}", value),
      ));
    }
  } else {
    // We got the `for` symbol so this must be a mistake.
    return Err(ResolveError::new(
      pre_span,
      format!("non-singleton system missing its node name"),
    ));
  }
}
