use std::fmt::Display;

use crate::generate::generics::common::{
  GenericElement, StructName, method_name, struct_name,
};

pub struct DynQueue<T: GenericElement> {
  pub element_t: T,
}

impl<T: GenericElement> DynQueue<T> {
  pub fn new(element_t: T) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> DynQueueHeader<'a, T> {
    DynQueueHeader(self)
  }

  pub fn imple<'a>(&'a self) -> DynQueueImpl<'a, T> {
    DynQueueImpl(self)
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("dyn_queue"; self.element_t)
  }
}

pub struct DynQueueHeader<'a, T: GenericElement>(&'a DynQueue<T>);
pub struct DynQueueImpl<'a, T: GenericElement>(&'a DynQueue<T>);

impl<'a, T: GenericElement> Display for DynQueueHeader<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "// Dynamic queue of `{element_t}`.\n",
        "typedef struct {{\n",
        "  {element_t} *items;\n",
        "  uint32_t len;\n",
        "  uint32_t cap;\n",
        "  uint32_t head;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self, uint32_t cap);\n",
        "void {method_grow}({self_t} *self);\n",
        "void {method_fit}({self_t} *self, uint32_t len);\n",
        "void {method_enqueue}({self_t} *self, {element_t} value);\n",
        "{element_t} {method_dequeue}({self_t} *self);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      element_t = element_t,
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_enqueue = method_name!(&self_t, "enqueue"),
      method_dequeue = method_name!(&self_t, "dequeue"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

impl<'a, T: GenericElement> Display for DynQueueImpl<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "// Dynamic queue of `{element_t}`.\n",
        "void {method_init}({self_t} *self, uint32_t cap) {{\n",
        "  if (cap > 0) {{\n",
        "    self->items = malloc(cap * sizeof({element_t}));\n",
        "  }} else {{\n",
        "    self->items = NULL;\n",
        "  }}\n",
        "  self->len = 0;\n",
        "  self->cap = cap;\n",
        "  self->head = 0;\n",
        "}}\n",
        "\n",
        "void {method_grow}({self_t} *self) {{\n",
        "  uint32_t new_cap = self->cap + (self->cap >> 1) + 1;\n",
        "  {element_t} *new_items = malloc(sizeof({element_t}) * new_cap);\n",
        "\n",
        "  for (uint32_t i = 0; i < self->len; ++i) {{\n",
        "    uint32_t j = (self->head + i) % self->cap;\n",
        "    new_items[i] = self->items[j];\n",
        "  }}\n",
        "\n",
        "  free(self->items);\n",
        "  self->items = new_items;\n",
        "  self->cap = new_cap;\n",
        "  self->head = 0;\n",
        "}}\n",
        "\n",
        "void {method_fit}({self_t} *self, uint32_t len) {{\n",
        "  while (self->cap < len)\n",
        "    {method_grow}(self);\n",
        "}}\n",
        "\n",
        "void {method_enqueue}({self_t} *self, {element_t} value) {{\n",
        "  uint32_t new_len = self->len + 1;\n",
        "  {method_fit}(self, new_len);\n",
        "  uint32_t j = (self->head + self->len) % self->cap;\n",
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
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_enqueue = method_name!(&self_t, "enqueue"),
      method_dequeue = method_name!(&self_t, "dequeue"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}
