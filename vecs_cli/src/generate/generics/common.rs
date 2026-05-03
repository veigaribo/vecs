use md5;
use std::fmt::Display;

fn mangle_generics(ctx: &mut md5::Context, generics: &[&str]) {
  if generics.is_empty() {
    return;
  }

  let mut iter = generics.iter();
  let head = iter.next().unwrap();
  ctx.consume(head);

  for generic in iter {
    ctx.consume(" ");
    ctx.consume(generic);
  }
}

// Represents the name of a generic struct with name `name` parameterized on
// `generics`. Names of methods may be constructed from it.
#[derive(Debug, Clone)]
pub struct StructName<'a> {
  name: &'a str,
  generics: Vec<&'a str>,
}

impl<'a> StructName<'a> {
  pub fn new(name: &'a str, generics: Vec<&'a str>) -> Self {
    Self { name, generics }
  }

  // Prefixes the name with `struct`, so it can be used as a type name in C.
  pub fn get_type_name(&self) -> String {
    format!("struct {}", self)
  }

  pub fn gmethod(&'a self, name: &'a str, generics: Vec<&'a str>) -> MethodName<'a> {
    MethodName {
      strukt: self,
      name,
      generics,
    }
  }

  // If you want generics, use `gmethod` instead (historical reasons).
  pub fn method(&'a self, name: &'a str) -> MethodName<'a> {
    self.gmethod(name, vec![])
  }
}

impl<'a> Display for StructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut md5_ctx = md5::Context::new();
    mangle_generics(&mut md5_ctx, &self.generics);

    let digest = md5_ctx.finalize();
    write!(f, "_VECSs{:x}_{}{:x}", self.name.len(), self.name, digest)?;

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct MethodName<'a> {
  strukt: &'a StructName<'a>,
  name: &'a str,
  generics: Vec<&'a str>,
}

impl<'a> Display for MethodName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut md5_ctx = md5::Context::new();
    mangle_generics(&mut md5_ctx, &self.generics);

    let digest = md5_ctx.finalize();
    write!(
      f,
      "{}_M{:x}_{}{:x}",
      self.strukt,
      self.name.len(),
      self.name,
      digest
    )?;
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct FunctionName<'a> {
  name: &'a str,
  generics: Vec<&'a str>,
}

impl<'a> Display for FunctionName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut md5_ctx = md5::Context::new();
    mangle_generics(&mut md5_ctx, &self.generics);

    let digest = md5_ctx.finalize();
    write!(f, "_VECSf{:x}_{}{:x}", self.name.len(), self.name, digest)?;

    Ok(())
  }
}

impl<'a> FunctionName<'a> {
  pub fn new(name: &'a str, generics: Vec<&'a str>) -> Self {
    Self { name, generics }
  }
}
