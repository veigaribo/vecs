use crate::{
  parse::data::str::Span,
  resolve::{
    ResolveMeta,
    cst::{Struct, StructBuilder, StructFieldBuilder},
    result::{ResolveError, ResolveResult},
    values::{Value, ValueKind},
  },
};

pub fn resolve_struct<
  'src,
  F: Fn(&'src str, Span<'src>) -> ResolveResult<'src, ()>,
>(
  meta: ResolveMeta<'src, '_, '_>,
  cdr: &[Value<'src>],
  validate_name: F,
) -> ResolveResult<'src, Struct<'src>> {
  let mut s = StructBuilder::default();
  let maybe_value = cdr.get(0);
  s.span(meta.span);

  if let Some(value) = maybe_value {
    if let ValueKind::Symbol(name) = value.kind {
      validate_name(name, value.span)?;
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
        let mut field = StructFieldBuilder::default();

        for value in values {
          if let ValueKind::Application(ref values) = value.kind {
            // Will contain both type and name.
            let mut name_segments = Vec::<&'src str>::new();

            for value in values {
              if let ValueKind::Symbol(segment) = value.kind {
                name_segments.push(segment);
              } else {
                return Err(ResolveError::new(
                  value.span,
                  format!(
                    "struct field must contain only symbols. instead found {}",
                    value,
                  ),
                ));
              }
            }

            if name_segments.is_empty() {
              return Err(ResolveError::new(
                value.span,
                "empty struct field not allowed",
              ));
            }

            let typ = Vec::from(&name_segments[0..name_segments.len() - 1]);
            let name = name_segments[name_segments.len() - 1];

            field.typ(typ);
            field.name(name);

            s.add_field(field.build().expect(&format!(
              "failed to build struct field. this is a bug.\n{}",
              meta.ast
            )));
          } else {
            panic!(
              "malformed ast: root expression is not an application. this is a bug.\n{}",
              meta.ast,
            );
          }
        }
      } else {
        return Err(ResolveError::new(
          value.span,
          format!("body of component should be a list. instead it's {}", value),
        ));
      }

      if let Some(extra) = cdr.get(2) {
        return Err(ResolveError::new(
          meta.span,
          format!("unexpected value {}", extra),
        ));
      }
    } else {
      return Err(ResolveError::new(
        meta.span,
        "component is missing its body",
      ));
    }
  } else {
    return Err(ResolveError::new(
      meta.span,
      "a component tag must be followed by the component name",
    ));
  }

  Ok(s.build().expect(&format!(
    "failed to build component. this is a bug.\n{}",
    meta.ast
  )))
}
