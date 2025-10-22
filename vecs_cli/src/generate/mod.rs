use std::io;

use crate::parse::{components::Component, Parsed};

pub fn gen_component_struct_name<'str, W: io::Write>(
  component: &Component<'str>,
  w: &mut W,
) -> io::Result<()> {
  write!(w, "v_component_{}", component.name)?;
  Ok(())
}

pub fn generate<'str, W: io::Write>(data: Parsed<'str>, w: &mut W) -> io::Result<()> {
  let prelude = include_str!("prelude.c");

  write!(w, "{}\n", prelude)?;

  // Component structs.
  for component in data.components.iter() {
    write!(w, "struct ")?;
    gen_component_struct_name(&component, w)?;
    write!(w, " {{")?;

    for field in component.fields.iter() {
      write!(w, "{} {};", field.typ, field.name)?;
    }
    write!(w, "}};")?;
  }

  // Component union.
  write!(w, "union v_component {{")?;
  for component in data.components.iter() {
    write!(w, "struct ")?;
    gen_component_struct_name(&component, w)?;
    write!(w, " {};", component.name)?;
  }
  write!(w, "}};")?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{
    generate::generate,
    parse::{
      components::{Component, ComponentField},
      events::{Event, EventField},
      systems::System,
      Parsed,
    },
  };

  #[test]
  fn test_generate() {
    let data = Parsed {
      components: vec![
        Component::new(
          "transform",
          vec![
            ComponentField::new("double", "x"),
            ComponentField::new("double", "y"),
          ],
        ),
        Component::new("render", vec![ComponentField::new("texture_t", "texture")]),
      ],

      events: vec![Event::new(
        "mouse_click",
        vec![
          EventField::new("double", "x"),
          EventField::new("double", "y"),
          EventField::new("uint8_t", "button"),
        ],
      )],

      systems: vec![
        System::new("move", vec!["transform"]),
        System::new("render", vec!["transform", "render"]),
      ],
    };

    let mut generated = Vec::<u8>::new();
    generate(data, &mut generated).expect("generate error");

    let generated_string = unsafe { String::from_utf8_unchecked(generated) };
    let expected_string = format!(
      "{}\n{}",
      include_str!("prelude.c"),
      "\
      struct v_component_transform {\
        double x;\
        double y;\
      };\
      \
      struct v_component_render {\
        texture_t texture;\
      };\
      \
      union v_component {\
        struct v_component_transform transform;\
        struct v_component_render render;\
      };"
    );

    assert_eq!(&generated_string, &expected_string);
  }
}
