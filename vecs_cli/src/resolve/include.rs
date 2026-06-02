use std::collections::VecDeque;

use crate::{
  common::StringKind,
  resolve::{
    ResolveMeta,
    result::{ResolveError, ResolveResult},
    values::{Value, ValueKind},
  },
};

pub fn resolve_include<'src>(
  meta: ResolveMeta<'src, '_>,
  mut cdr: VecDeque<Value<'src>>,
) -> ResolveResult<'src, StringKind> {
  let maybe_value = cdr.pop_front();

  if !cdr.is_empty() {
    let extra_value = cdr.pop_front().unwrap();
    return Err(ResolveError::new(
      extra_value.span,
      format!("unexpected value {}", extra_value),
    ));
  }

  if let Some(value) = maybe_value {
    if let ValueKind::String(path) = value.kind {
      return Ok(path);
    } else {
      return Err(ResolveError::new(
        value.span,
        format!("include path must be a string. instead found {}", value),
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      format!(
        "an include tag must be followed by the include path, either in double quotes or angle brackets"
      ),
    ));
  }
}
