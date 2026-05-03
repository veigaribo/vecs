use std::fmt::Display;

use super::common::StructName;

pub struct DynQueue {
  pub element_t: String,
}

impl DynQueue {
  pub fn new(element_t: String) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> DynQueueHeader<'a> {
    DynQueueHeader(self)
  }

  pub fn imple<'a>(&'a self) -> DynQueueImpl<'a> {
    DynQueueImpl(self)
  }

  pub fn get_name<'a>(&'a self) -> StructName<'a> {
    StructName::new("dyn_queue", vec![self.element_t.as_str()])
  }
}

pub struct DynQueueHeader<'a>(&'a DynQueue);
pub struct DynQueueImpl<'a>(&'a DynQueue);

impl<'a> Display for DynQueueHeader<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    write!(
      f,
      concat!(
        "// Dynamic queue of `{element_t}`.\n",
        "struct {struct_name} {{\n",
        "  {element_t} *items;\n",
        "  size_t len;\n",
        "  size_t cap;\n",
        "  size_t head;\n",
        "}};\n",
        "\n",
        "void {method_init}({self_t} *self, size_t cap);\n",
        "void {method_grow}({self_t} *self);\n",
        "void {method_fit}({self_t} *self, size_t len);\n",
        "void {method_enqueue}({self_t} *self, {element_t} value);\n",
        "{element_t} {method_dequeue}({self_t} *self);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      struct_name = struct_name,
      element_t = element_t,
      self_t = self_t,
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_enqueue = struct_name.method("enqueue"),
      method_dequeue = struct_name.method("dequeue"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}

impl<'a> Display for DynQueueImpl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    write!(
      f,
      concat!(
        "// Dynamic queue of `{element_t}`.\n",
        "void {method_init}({self_t} *self, size_t cap) {{\n",
        "  self->items = malloc(cap * sizeof({self_t}));\n",
        "  self->len = 0;\n",
        "  self->cap = cap;\n",
        "  self->head = 0;\n",
        "}}\n",
        "\n",
        "void {method_grow}({self_t} *self) {{\n",
        "  size_t new_cap = self->cap + (self->cap >> 1) + 1;\n",
        "  {element_t} *new_items = malloc(sizeof({element_t}) * new_cap);\n",
        "\n",
        "  for (size_t i = 0; i < self->len; ++i) {{\n",
        "    size_t j = (self->head + i) % self->cap;\n",
        "    new_items[i] = self->items[j];\n",
        "  }}\n",
        "\n",
        "  free(self->items);\n",
        "  self->items = new_items;\n",
        "  self->cap = new_cap;\n",
        "  self->head = 0;\n",
        "}}\n",
        "\n",
        "void {method_fit}({self_t} *self, size_t len) {{\n",
        "  while (self->cap < len)\n",
        "    {method_grow}(self);\n",
        "}}\n",
        "\n",
        "void {method_enqueue}({self_t} *self, {element_t} value) {{\n",
        "  size_t new_len = self->len + 1;\n",
        "  {method_fit}(self, new_len);\n",
        "  size_t j = (self->head + self->len) % self->cap;\n",
        "  self->items[j] = value;\n",
        "  self->len = new_len;\n",
        "}}\n",
        "\n",
        "{element_t} {method_dequeue}({self_t} *self) {{\n",
        "  {element_t} result = self->items[self->head];\n",
        "  self->head = (self->head + 1) % self->cap;\n",
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
        "  self->head = 0;\n",
        "}}\n",
        "\n",
      ),
      element_t = element_t,
      self_t = self_t,
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_enqueue = struct_name.method("enqueue"),
      method_dequeue = struct_name.method("dequeue"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}
