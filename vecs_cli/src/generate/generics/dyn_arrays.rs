use std::fmt::Display;

use crate::generate::generics::common::{
  Whatever, method_name, struct_name, whatever_name,
};

use super::common::{GenericElement, StructName};

pub struct DynArray<T: GenericElement> {
  pub element_t: T,
}

impl<T: GenericElement> DynArray<T> {
  pub fn new(element_t: T) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> DynArrayHeader<'a, T> {
    DynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> DynArrayImpl<'a, T> {
    DynArrayImpl(self)
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("dyn_array"; self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("dyn_array", self.element_t)
  }
}

pub struct DynArrayHeader<'a, T: GenericElement>(&'a DynArray<T>);
pub struct DynArrayImpl<'a, T: GenericElement>(&'a DynArray<T>);

impl<'a, T: GenericElement> Display for DynArrayHeader<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "// Dynamic array of `{element_t}`.\n",
        "typedef struct {whatever} {{\n",
        "  {element_t} *items;\n",
        "  uint32_t len;\n",
        "  uint32_t cap;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self, uint32_t cap);\n",
        "void {method_grow}({self_t} *self);\n",
        "void {method_fit}({self_t} *self, uint32_t len);\n",
        "uint32_t {method_push}({self_t} *self, {element_t} value);\n",
        "{element_t} {method_pop}({self_t} *self);\n",
        "{element_t} {method_swap_remove}({self_t} *self, uint32_t index);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      element_t = element_t,
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_push = method_name!(&self_t, "push"),
      method_pop = method_name!(&self_t, "pop"),
      method_swap_remove = method_name!(&self_t, "swap_remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

impl<'a, T: GenericElement> Display for DynArrayImpl<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "// Dynamic array of `{element_t}`.\n",
        "void {method_init}({self_t} *self, uint32_t cap) {{\n",
        "  if (cap > 0) {{\n",
        "    self->items = malloc(cap * sizeof({element_t}));\n",
        "  }} else {{\n",
        "    self->items = NULL;\n",
        "  }}\n",
        "  self->len = 0;\n",
        "  self->cap = cap;\n",
        "}}\n",
        "\n",
        "void {method_grow}({self_t} *self) {{\n",
        "  uint32_t new_cap = self->cap + (self->cap >> 1) + 1;\n",
        "  self->items = realloc(self->items, sizeof({element_t}) * new_cap);\n",
        "  self->cap = new_cap;\n",
        "}}\n",
        "\n",
        "void {method_fit}({self_t} *self, uint32_t len) {{\n",
        "  while (self->cap < len)\n",
        "    {method_grow}(self);\n",
        "}}\n",
        "\n",
        "uint32_t {method_push}({self_t} *self, {element_t} value) {{\n",
        "  uint32_t new_len = self->len + 1;\n",
        "  {method_fit}(self, new_len);\n",
        "  self->items[self->len] = value;\n",
        "  self->len = new_len;\n",
        "  return new_len - 1;\n",
        "}}\n",
        "\n",
        "{element_t} {method_pop}({self_t} *self) {{\n",
        "  {element_t} result = self->items[self->len - 1];\n",
        "  self->len -= 1;\n",
        "  return result;\n",
        "}}\n",
        "\n",
        "{element_t} {method_swap_remove}({self_t} *self, uint32_t index) {{\n",
        "  {element_t} result = self->items[index];\n",
        "  self->items[index] = self->items[self->len - 1];\n",
        "  self->len -= 1;\n",
        "  return result;\n",
        "}}\n",
        "\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  if (self->items != NULL)\n",
        "    free(self->items);\n",
        "\n",
        "  self->items = NULL;\n",
        "  self->len = 0;\n",
        "  self->cap = 0;\n",
        "}}\n",
      ),
      element_t = element_t,
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_push = method_name!(&self_t, "push"),
      method_pop = method_name!(&self_t, "pop"),
      method_swap_remove = method_name!(&self_t, "swap_remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}
