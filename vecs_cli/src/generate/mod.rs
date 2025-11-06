use std::io;

use crate::resolve::cst::{Cst, Struct};

pub fn gen_component_struct_name<'src, W: io::Write>(
  component: &Struct<'src>,
  w: &mut W,
) -> io::Result<()> {
  write!(w, "v_component_{}", component.name)?;
  Ok(())
}

pub fn generate<'src, W: io::Write>(data: Cst<'src>, w: &mut W) -> io::Result<()> {
  let prelude = include_str!("prelude.c");

  write!(w, "{}\n", prelude)?;

  // Component structs.
  for component in data.components.values() {
    write!(w, "struct ")?;
    gen_component_struct_name(&component, w)?;
    write!(w, " {{\n")?;

    for field in component.fields.iter() {
      for typ_segment in field.typ.iter() {
        write!(w, "  {} ", typ_segment)?;
      }

      write!(w, "{};\n", field.name)?;
    }
    write!(w, "}};\n\n")?;
  }

  write!(w, "\n")?;

  // Component union.
  write!(w, "union v_component {{\n")?;
  for component in data.components.values() {
    write!(w, "  struct ")?;
    gen_component_struct_name(&component, w)?;
    write!(w, " {};\n", component.name)?;
  }
  write!(w, "}};\n")?;

  // TODO: Hook up systems.
  write!(w, "\n")?;
  for system in data.systems.values() {
    write!(w, "// system {}(", system.name)?;

    for param in system.params.iter() {
      write!(w, "{},", param)?;
    }

    write!(w, ")\n")?;
  }

  // TODO: Define states.
  write!(w, "\n")?;
  for state in data.states.iter() {
    write!(w, "// state {} ", state.name)?;
    write!(w, "components = (")?;

    for component in state.components.iter() {
      write!(w, "{}", component.name)?;

      if let Some(max) = component.max {
        write!(w, " max {}", max)?;
      }

      write!(w, ",")?;
    }

    write!(w, ") systems = (")?;

    for (i, systems) in state.systems.iter().enumerate() {
      write!(w, "{} => (", i)?;

      for system in systems {
        write!(w, "{},", system)?;
      }

      write!(w, "),")?;
    }

    write!(w, ")\n")?;
  }

  Ok(())
}
