use std::fmt::Display;

use derive_display_hash::DisplayHash;

use crate::generate::generics::common::GenericElement;

// A struct named $name that has one `name` field and implements Display with the
// provided format string, where the name is the first positional argument. Construct
// it with `$name::new(name)`.
macro_rules! format_struct {
  ($name:ident, $format:expr) => {
    #[derive(Debug, Clone, DisplayHash)]
    pub struct $name<T: Display> {
      pub name: T,
    }

    impl<T: Display> $name<T> {
      pub fn new(name: T) -> Self {
        Self { name: name }
      }
    }

    impl<T: Display> Display for $name<T> {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, $format, self.name)
      }
    }
  };
}

// Event:
format_struct!(EventStructName, "vecs_event_{}_t");

// Component:
format_struct!(ComponentStructName, "vecs_component_{}_t");

// Component deferred operations:
format_struct!(ComponentOpAddStructName, "vecs_op_add_component_{}_t");
format_struct!(
  ComponentOpAddTmpStructName,
  "vecs_op_tmp_add_component_{}_t"
);
format_struct!(ComponentOpUpdateStructName, "vecs_op_update_component_{}_t");

// Node:
format_struct!(NodeStructName, "vecs_node_{}_t");

/// Helper to generate an instance of all of the component-specific operations at once:
/// add, add_tmp & update.
// TODO: Is this really helpful?
#[derive(Clone)]
pub struct ComponentTmpOps<T: GenericElement> {
  pub add_t: ComponentOpAddStructName<T>,
  pub add_tmp_t: ComponentOpAddTmpStructName<T>,
  pub update_t: ComponentOpUpdateStructName<T>,
}

impl<T: GenericElement> ComponentTmpOps<T> {
  pub fn new(name: T) -> Self {
    Self {
      add_t: ComponentOpAddStructName::new(name.clone()),
      add_tmp_t: ComponentOpAddTmpStructName::new(name.clone()),
      update_t: ComponentOpUpdateStructName::new(name.clone()),
    }
  }
}
