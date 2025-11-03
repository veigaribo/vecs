use crate::resolve::{
  cst::{Struct, StructBuilder, StructFieldBuilder},
  result::{ResolveError, ResolveResult},
  values::{Value, ValueKind},
  ResolveMeta,
};

pub fn resolve_struct<'a>(
  meta: ResolveMeta<'a, '_, '_>,
  cdr: &[Value<'a>],
) -> ResolveResult<'a, Struct<'a>> {
  let mut s = StructBuilder::default();
  let arg1 = cdr.get(0);

  if let Some(name) = arg1 {
    if let ValueKind::Symbol(name) = name.kind {
      s.name(name);
    } else {
      return Err(ResolveError::new(
        name.span,
        format!("struct name must be a symbol. instead found {}", name),
      ));
    }

    let arg2 = cdr.get(1);

    if let Some(body) = arg2 {
      if let ValueKind::List(ref values) = body.kind {
        let mut field = StructFieldBuilder::default();

        for value in values {
          if let ValueKind::List(ref segments) = value.kind {
            // Will contain both type and name.
            let mut name_segments = Vec::<&'a str>::new();

            for segment in segments {
              if let ValueKind::Symbol(segment) = segment.kind {
                name_segments.push(segment);
              } else {
                return Err(ResolveError::new(
                  segment.span,
                  format!(
                    "struct field must contain only symbols. instead found {}",
                    segment,
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
              "malformed ast: root expression is not a list. this is a bug.\n{}",
              meta.ast
            );
          }
        }
      } else {
        return Err(ResolveError::new(
          body.span,
          format!(
            "body of component {} should be a list. instead it's {}",
            name, body
          ),
        ));
      }
    } else {
      return Err(ResolveError::new(
        meta.span,
        format!("component {} is missing its body", name),
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
