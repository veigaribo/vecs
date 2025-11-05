use std::{collections::HashMap, slice};

use crate::resolve::{
  cst::{State, StateBuilder, StateComponent, StateComponentBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
  ResolveMeta,
};

pub fn resolve_state<'a>(
  meta: ResolveMeta<'a, '_, '_>,
  cdr: &[Value<'a>],
) -> ResolveResult<'a, State<'a>> {
  let mut s = StateBuilder::default();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      s.name(name);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("struct name must be a symbol. instead found {}", value),
      ));
    }

    let maybe_value = cdr.get(1);

    if let Some(value) = maybe_value {
      if let ValueKind::List(ref values) = value.kind {
        for value in values {
          if let ValueKind::List(ref values) = value.kind {
            let car = &values[0];
            let cdr = &values[1..];

            if car.kind == ValueKind::Symbol("components") {
              if s.components.is_some() {
                return Err(ResolveError::new(
                  meta.span,
                  "duplicated state components. expected only one",
                ));
              }

              let components = resolve_state_components(meta, cdr)?;
              s.components(components);
            } else if car.kind == ValueKind::Symbol("systems") {
              if s.systems.is_some() {
                return Err(ResolveError::new(
                  meta.span,
                  "duplicated state systems. expected only one",
                ));
              }

              let systems = resolve_state_systems(meta, cdr)?;
              s.systems(systems);
            } else {
              return Err(ResolveError::new(
                car.span,
                format!("expected `components` or `systems`. instead found {}", car),
              ));
            }
          } else {
            panic!(
              "malformed ast: root expression is not a list. this is a bug.\n{}",
              meta.ast,
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

  if s.components.is_none() {
    return Err(ResolveError::new(
      meta.span,
      "state is missing its components",
    ));
  }

  if s.systems.is_none() {
    return Err(ResolveError::new(meta.span, "state is missing its systems"));
  }

  Ok(s.build().expect(&format!(
    "failed to build state. this is a bug.\n{}",
    meta.ast
  )))
}

fn resolve_state_components<'a>(
  meta: ResolveMeta<'a, '_, '_>,
  cdr: &[Value<'a>],
) -> ResolveResult<'a, Vec<StateComponent<'a>>> {
  let mut ss = Vec::<StateComponent<'a>>::new();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::List(ref values) = value.kind {
      for value in values {
        let mut s = StateComponentBuilder::default();

        if let ValueKind::List(ref values) = value.kind {
          let maybe_value = values.get(0);

          if let Some(value) = maybe_value {
            if let ValueKind::Symbol(name) = value.kind {
              if meta.cst.components.contains_key(name) {
                s.name(name);
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!("component `{}` not found", name),
                ));
              }
            } else {
              return Err(ResolveError::new(
                value.span,
                format!(
                  "state component name must be a symbol. instead found {}",
                  value,
                ),
              ));
            }

            let maybe_value = values.get(1);

            if let Some(value) = maybe_value {
              let mut opts = resolve_options(meta.clone(), slice::from_ref(value))?;
              let maybe_value = opts.remove("max");

              if let Some(value) = maybe_value {
                if let ValueKind::Integer(max) = value.kind {
                  let as_u64: u64 = TryInto::<u64>::try_into(max).map_err(|_| {
                    ResolveError::new(
                      value.span,
                      format!("could not convert component max count ({}) to a 64-bit unsigned integer. expected to be able to.", max),
                    )
                  })?;

                  s.max(Some(as_u64));
                } else {
                  return Err(ResolveError::new(
                    value.span,
                    format!(
                      "max component count must be an integer. instead found {}",
                      value,
                    ),
                  ));
                }
              } else {
                s.max(None);
              }

              if !opts.is_empty() {
                let unknown_keys = opts
                  .keys()
                  .map(|key| *key)
                  .intersperse(", ")
                  .collect::<String>();

                return Err(ResolveError::new(
                  value.span,
                  format!("unknown component options: {}", unknown_keys),
                ));
              }

              if let Some(value) = values.get(2) {
                return Err(ResolveError::new(
                  value.span,
                  format!("unexpected value {}", value),
                ));
              }
            } else {
              s.max(None);
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              "components entry should start with the component name",
            ));
          }
        } else {
          panic!(
            "malformed ast: root expression is not a list. this is a bug.\n{}",
            meta.ast
          );
        }

        ss.push(s.build().expect(&format!(
          "failed to build state components. this is a bug.\n{}",
          meta.ast
        )));
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

  Ok(ss)
}

fn resolve_state_systems<'a>(
  meta: ResolveMeta<'a, '_, '_>,
  cdr: &[Value<'a>],
) -> ResolveResult<'a, Vec<Vec<&'a str>>> {
  let mut ss = Vec::<Vec<&'a str>>::new();
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::List(ref values) = value.kind {
      for value in values {
        ss.push(vec![]);

        let len = ss.len();
        let s = &mut ss[len - 1];

        if let ValueKind::List(ref values) = value.kind {
          if values.len() != 1 {
            return Err(ResolveError::new(
              value.span,
              format!("expected a single system list. instead found {}", value,),
            ));
          }

          let value = &values[0];

          if let ValueKind::List(ref values) = value.kind {
            for value in values {
              if let ValueKind::List(ref values) = value.kind {
                if values.len() != 1 {
                  return Err(ResolveError::new(
                    value.span,
                    format!("expected a single system name. instead found {}", value,),
                  ));
                }

                let value = &values[0];

                if let ValueKind::Symbol(name) = value.kind {
                  if meta.cst.systems.contains_key(name) {
                    s.push(name);
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
                  "malformed ast: root expression is not a list. this is a bug.\n{}",
                  meta.ast
                );
              }
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              format!("state system list must be a list. instead found {}", value,),
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

  Ok(ss)
}

fn resolve_options<'a, 'b>(
  meta: ResolveMeta<'a, '_, '_>,
  args: &'b [Value<'a>],
) -> ResolveResult<'a, HashMap<&'a str, &'b Value<'a>>> {
  let mut opts = HashMap::new();
  let arg1 = args.get(0);

  if let Some(arg1) = arg1 {
    if let ValueKind::List(ref inner) = arg1.kind {
      for app in inner {
        if let ValueKind::List(ref inner) = app.kind {
          let arg1 = inner.get(0);

          let name: &'a str;

          if let Some(arg1) = arg1 {
            if let ValueKind::Symbol(content) = arg1.kind {
              name = content;
            } else {
              return Err(ResolveError::new(
                arg1.span,
                format!("option name must be a symbol. instead found {}", arg1,),
              ));
            }

            let arg2 = inner.get(1);

            if let Some(arg2) = arg2 {
              opts.insert(name, arg2);
            } else {
              return Err(ResolveError::new(
                app.span,
                "option name should be followed by its value",
              ));
            }
          } else {
            return Err(ResolveError::new(
              app.span,
              "option entry should start with its name",
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
        arg1.span,
        format!("options must be a list. instead found {}", arg1,),
      ));
    }
  } else {
    return Err(ResolveError::new(meta.span, "options must be a list"));
  }

  Ok(opts)
}
