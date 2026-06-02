use std::collections::VecDeque;

use crate::{
  parse::data::str::Span,
  resolve::{result::ResolveResult, values::Value},
};

use super::cst::Cst;

// Modifies the CST directly because that's the easiest thing to do.
pub fn resolve_setting<'src>(
  _span: Span<'src>,
  _cdr: VecDeque<Value<'src>>,
  _cst: &mut Cst,
) -> ResolveResult<'src, ()> {
  // There are currently no settings.

  // let setting_name: &'src str;
  // let maybe_value = cdr.get(0);

  // if let Some(value) = maybe_value {
  //   if let ValueKind::Symbol(name) = value.kind {
  //     setting_name = name
  //   } else {
  //     return Err(ResolveError::new(
  //       value.span,
  //       format!("setting name must be a symbol. instead found {}", value),
  //     ));
  //   }

  //   if cdr.len() > 2 {
  //     return Err(ResolveError::new(
  //       span,
  //       format!(
  //         "extraneous value under setting {}. maybe you forgot a semicolon?",
  //         setting_name,
  //       ),
  //     ));
  //   }

  //   let maybe_value = cdr.get(1);

  //   if let Some(value) = maybe_value {
  //     match setting_name {
  //       other => {
  //         return Err(ResolveError::new(
  //           value.span,
  //           format!("unrecognized setting {}", other,),
  //         ));
  //       }
  //     }
  //   } else {
  //     return Err(ResolveError::new(
  //       span,
  //       format!("setting {} is missing its body", value),
  //     ));
  //   }
  // } else {
  //   return Err(ResolveError::new(
  //     span,
  //     "a setting tag must be followed by the setting name",
  //   ));
  // }

  Ok(())
}
