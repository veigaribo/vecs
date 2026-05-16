use std::{fmt::Display, hash::Hash};

macro_rules! hash_internal {
  ($hasher:ident) => {};
  ($hasher:ident; $generic:expr) => {
    std::hash::Hash::hash(&$generic, &mut $hasher);
  };
  ($hasher:ident; $generic:expr, $($rest:tt)*) => {
    std::hash::Hash::hash(&$generic, &mut $hasher);
    std::hash::Hash::hash(&0xFF, &mut $hasher);
    crate::generate::generics::common::hash_internal!($hasher; $($rest)*);
  };
}

pub(crate) use hash_internal;

// Represents the name of a generic struct with name `name` parameterized on
// `generics`. Names of methods may be constructed from it.
#[derive(Debug, Clone, Hash)]
pub struct StructName<'a> {
  pub name: &'a str,
  pub hash: u64,
}

impl<'a> Display for StructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "_VECSs{:x}_{}{:016x}_t",
      self.name.len(),
      self.name,
      self.hash,
    )
  }
}

macro_rules! struct_name {
  ($name:expr) => {{
    crate::generate::generics::common::StructName { name: $name, hash: 0 }
  }};
  ($name:expr; $($tt:tt)*) => {{
    let mut hasher = std::hash::DefaultHasher::new();
    crate::generate::generics::common::hash_internal!(hasher; $($tt)*);
    crate::generate::generics::common::StructName { name: $name, hash: std::hash::Hasher::finish(&hasher) }
  }};
}

pub(crate) use struct_name;

#[derive(Debug, Clone, Hash)]
pub struct MethodName<'a> {
  pub strukt: &'a StructName<'a>,
  pub name: &'a str,
  pub hash: u64,
}

impl<'a> Display for MethodName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}_M{:x}_{}{:016x}",
      self.strukt,
      self.name.len(),
      self.name,
      self.hash,
    )
  }
}

macro_rules! method_name {
  ($parent:expr, $name:expr) => {{
    crate::generate::generics::common::MethodName { strukt: $parent, name: $name, hash: 0 }
  }};
  ($parent:expr, $name:expr; $($tt:tt)*) => {{
    let mut hasher = std::hash::DefaultHasher::new();
    crate::generate::generics::common::hash_internal!(hasher; $($tt)*);
    crate::generate::generics::common::MethodName { strukt: $parent, name: $name, hash: std::hash::Hasher::finish(&hasher) }
  }};
}

pub(crate) use method_name;

#[derive(Debug, Clone, Hash)]
pub struct FunctionName<'a> {
  pub name: &'a str,
  pub hash: u64,
}

impl<'a> Display for FunctionName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "_VECSf{:x}_{}{:016x}",
      self.name.len(),
      self.name,
      self.hash
    )
  }
}

macro_rules! function_name {
  ($name:expr) => {{
    crate::generate::generics::common::FunctionName { name: $name, hash: 0 }
  }};
  ($name:expr; $($tt:tt)*) => {{
    let mut hasher = std::hash::DefaultHasher::new();
    crate::generate::generics::common::hash_internal!(hasher; $($tt)*);
    crate::generate::generics::common::FunctionName { name: $name, hash: std::hash::Hasher::finish(&hasher) }
  }};
}

pub(crate) use function_name;

/// Used where something's name doesn't matter.
/// Two Whatevers with equal hashes will format to equal strings.
#[derive(Debug, Clone, Hash)]
pub struct Whatever {
  pub hash: u64,
}

impl Display for Whatever {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "_VECSw_{:x}", self.hash)
  }
}

macro_rules! whatever_name {
  ($($tt:tt)*) => {{
    let mut hasher = std::hash::DefaultHasher::new();
    crate::generate::generics::common::hash_internal!(hasher; $($tt)*);
    crate::generate::generics::common::Whatever { hash: std::hash::Hasher::finish(&hasher) }
  }};
}

pub(crate) use whatever_name;

// Clone for composing e.g. maybe A<T> requires B<T>;
// Display generates the output;
// Hash used for name mangling.
pub trait GenericElement = Clone + Display + Hash;
