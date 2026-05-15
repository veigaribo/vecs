use std::fmt::Display;

use super::common::StructName;

pub struct DynArray {
  pub element_t: String,
}

impl DynArray {
  pub fn new(element_t: String) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> DynArrayHeader<'a> {
    DynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> DynArrayImpl<'a> {
    DynArrayImpl(self)
  }

  pub fn get_name<'a>(&'a self) -> StructName<'a> {
    StructName::new("dyn_array", vec![self.element_t.as_str()])
  }
}

pub struct DynArrayHeader<'a>(&'a DynArray);
pub struct DynArrayImpl<'a>(&'a DynArray);

impl<'a> Display for DynArrayHeader<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    write!(
      f,
      concat!(
        "// Dynamic array of `{element_t}`.\n",
        "struct {struct_name} {{\n",
        "  {element_t} *items;\n",
        "  uint32_t len;\n",
        "  uint32_t cap;\n",
        "}};\n",
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
      struct_name = struct_name,
      element_t = element_t,
      self_t = self_t,
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_push = struct_name.method("push"),
      method_pop = struct_name.method("pop"),
      method_swap_remove = struct_name.method("swap_remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}

impl<'a> Display for DynArrayImpl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    write!(
      f,
      concat!(
        "// Dynamic array of `{element_t}`.\n",
        "void {method_init}({self_t} *self, uint32_t cap) {{\n",
        "  self->items = malloc(cap * sizeof({element_t}));\n",
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
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_push = struct_name.method("push"),
      method_pop = struct_name.method("pop"),
      method_swap_remove = struct_name.method("swap_remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}
