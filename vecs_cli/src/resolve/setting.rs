use crate::{
  parse::data::str::Span,
  resolve::{
    result::{ResolveError, ResolveResult},
    values::{Value, ValueKind},
  },
};

use super::cst::Cst;

// Modifies the CST directly because that's the easiest thing to do.
pub fn resolve_setting<'src>(
  span: Span<'src>,
  cdr: &[Value<'src>],
  cst: &mut Cst,
) -> ResolveResult<'src, ()> {
  let setting_name: &'src str;
  let maybe_value = cdr.get(0);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      setting_name = name
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("setting name must be a symbol. instead found {}", value),
      ));
    }

    if cdr.len() > 2 {
      return Err(ResolveError::new(
        span,
        format!(
          "extraneous value under setting {}. maybe you forgot a semicolon?",
          setting_name,
        ),
      ));
    }

    let maybe_value = cdr.get(1);

    if let Some(value) = maybe_value {
      match setting_name {
        "default_component_max" => {
          if let ValueKind::Integer(integer) = value.kind {
            match TryInto::<usize>::try_into(integer) {
              Ok(max) => cst.settings.default_component_max = max,
              Err(_) => {
                return Err(ResolveError::new(
                  value.span,
                  format!(
                    "{} is not a valid count or would be way too large (bounds: [{}, {}])",
                    integer, usize::MIN, usize::MAX,
                  ),
                ));
              }
            }
          } else {
            return Err(ResolveError::new(
              value.span,
              format!(
                "value for `default_component_max` should be an integer. instead found {}",
                value,
              ),
            ));
          }
        }
        other => {
          return Err(ResolveError::new(
            value.span,
            format!("unrecognized setting {}", other,),
          ));
        }
      }
    } else {
      return Err(ResolveError::new(
        span,
        format!("setting {} is missing its body", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      span,
      "a setting tag must be followed by the setting name",
    ));
  }

  Ok(())
}
