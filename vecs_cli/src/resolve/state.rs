use std::collections::VecDeque;

use crate::resolve::{
  ResolveMeta,
  cst::{State, StateBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
};

pub fn resolve_state<'src>(
  meta: ResolveMeta<'src, '_>,
  mut values: VecDeque<Value<'src>>,
) -> ResolveResult<'src, State<'src>> {
  let mut s = StateBuilder::default();
  let maybe_value = values.pop_front();
  s.span(meta.span);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      if name.eq_ignore_ascii_case("NONE") {
        return Err(ResolveError::new(
          value.span,
          format!(
            "a state named 'none' is always generated, and thus the name is reserved. if still want that state, please name it something else"
          ),
        ));
      }

      if meta.cst.states.contains_key(name) {
        let previous = meta.cst.states.get(name).unwrap();

        return Err(ResolveError::new(
          value.span,
          format!(
            "duplicated state name '{}'. previously defined at {}",
            name, previous.span
          ),
        ));
      }

      s.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("struct name must be a symbol. instead found {}", value),
      ));
    }

    let maybe_value = values.pop_front();

    let mut got_nodes = false;
    let mut got_systems = false;

    if let Some(value) = maybe_value {
      if let ValueKind::List(values) = value.kind {
        for value in values {
          if let ValueKind::Application(mut values) = value.kind {
            let car = values.pop_front().ok_or_else(|| {
              let name = s.name.unwrap();
              ResolveError::new(value.span, format!("body of state {} empty. should contain a list of systems and optionally a list of nodes", name))
            })?;

            if car.kind == ValueKind::Symbol("nodes") {
              if got_nodes {
                return Err(ResolveError::new(
                  meta.span,
                  "duplicated state nodes. expected only one",
                ));
              }

              got_nodes = true;
              let nodes = resolve_state_nodes(meta, values)?;

              for node in nodes {
                s.add_node(node);
              }
            } else if car.kind == ValueKind::Symbol("systems") {
              if got_systems {
                return Err(ResolveError::new(
                  meta.span,
                  "duplicated state systems. expected only one",
                ));
              }

              got_systems = true;
              let (systems, system_nodes) = resolve_state_systems(meta, values)?;

              // Add system nodes
              for node in system_nodes.iter() {
                s.add_node(node);
              }

              s.systems(systems);
            } else {
              return Err(ResolveError::new(
                car.span,
                format!("expected `nodes` or `systems`. instead found {}", car),
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
          format!("state body should be a list. instead found {}", value),
        ));
      }
    } else {
      return Err(ResolveError::new(value.span, "state is missing its body"));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a state tag must be followed by the state name",
    ));
  }

  if s.systems.is_none() {
    return Err(ResolveError::new(meta.span, "state is missing its systems"));
  }

  if s.nodes.is_none() {
    s.nodes(vec![]);
  }

  Ok(s.build().expect(&format!(
    "failed to build state. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
  )))
}

fn resolve_state_nodes<'src>(
  meta: ResolveMeta<'src, '_>,
  cdr: VecDeque<Value<'src>>,
) -> ResolveResult<'src, Vec<&'src str>> {
  let mut sn = Vec::<&'src str>::new();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::List(ref values) = value.kind {
      for value in values {
        if let ValueKind::Application(ref values) = value.kind {
          let maybe_value = values.get(0);

          if let Some(value) = maybe_value {
            if let ValueKind::Symbol(name) = value.kind {
              if meta.cst.nodes.contains_key(name) {
                sn.push(name);
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("node `{}` not found", name),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!("state node name must be a symbol. instead found {}", value,),
              ));
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              "node entry should start with the component name",
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
        format!(
          "nodes must be followed by the list of nodes. instead found {}",
          value,
        ),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "nodes must be followed by the list of nodes",
    ));
  }

  Ok(sn)
}

// Returns the list of systems and the list of nodes that derive from those systems.
fn resolve_state_systems<'src>(
  meta: ResolveMeta<'src, '_>,
  cdr: VecDeque<Value<'src>>,
) -> ResolveResult<'src, (Vec<Vec<&'src str>>, Vec<&'src str>)> {
  let mut ss = Vec::<Vec<&'src str>>::new();
  let mut sn = Vec::<&'src str>::new();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::List(ref values) = value.kind {
      for value in values {
        ss.push(vec![]);

        let len = ss.len();
        let s = &mut ss[len - 1];

        if let ValueKind::Application(ref values) = value.kind {
          if values.len() != 1 {
            return Err(ResolveError::new(
              value.span,
              format!("expected a single system list. instead found {}", value,),
            ));
          }

          let value = &values[0];

          if let ValueKind::List(ref values) = value.kind {
            for value in values {
              if let ValueKind::Application(ref values) = value.kind {
                if values.len() != 1 {
                  return Err(ResolveError::new(
                    value.span,
                    format!("expected a single system name. instead found {}", value,),
                  ));
                }

                let value = &values[0];

                if let ValueKind::Symbol(name) = value.kind {
                  let maybe_system = meta.cst.systems.get(name);
                  if let Some(system) = maybe_system {
                    s.push(name);

                    if let Some(node) = system.node {
                      sn.push(node);
                    }
                  } else {
                    return Err(ResolveError::new(
                      value.span,
                      format!("system `{}` not found", name),
                    ));
                  }
                } else {
                  return Err(ResolveError::new(
                    value.span,
                    format!(
                      "state system name must be a symbol. instead found {}",
                      value,
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
              format!(
                "state system list must be a list. instead found {} (maybe wrap it in another list?)",
                value,
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
        format!(
          "components must be followed by the list of components. instead found {}",
          value,
        ),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "components must be followed by the list of components",
    ));
  }

  Ok((ss, sn))
}
