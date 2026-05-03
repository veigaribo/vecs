use std::fmt::Display;

use super::common::StructName;
use super::dyn_arrays::DynArray;

pub struct SparseDynArray {
  pub element_t: String,
}

impl SparseDynArray {
  pub fn new(element_t: String) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> SparseDynArrayHeader<'a> {
    SparseDynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> SparseDynArrayImpl<'a> {
    SparseDynArrayImpl(self)
  }

  pub fn get_name<'a>(&'a self) -> StructName<'a> {
    StructName::new("sparse_dyn_array", vec![self.element_t.as_str()])
  }
}

pub struct SparseDynArrayHeader<'a>(&'a SparseDynArray);
pub struct SparseDynArrayImpl<'a>(&'a SparseDynArray);

impl<'a> Display for SparseDynArrayHeader<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    let element_dyn_arr_t =
      DynArray::new(element_t.clone()).get_name().get_type_name();

    let size_t_dyn_arr_t = DynArray::new("size_t".to_string())
      .get_name()
      .get_type_name();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "struct {struct_name} {{\n",
        "  {element_dyn_arr_t} items;\n",
        "  {size_t_dyn_arr_t} holes;\n",
        "\n",
        "  // Every time an item is added, its corresponding gen is incremented.\n",
        "  // The gen is used to differentiate items that happened to be put on\n",
        "  // the same index. A pair of index and gen is thus necessary to address\n",
        "  // any particular item.\n",
        "  {size_t_dyn_arr_t} gens;\n",
        "\n",
        "  size_t len;\n",
        "}};\n",
        "\n",
        "void {method_init}({self_t} *self, size_t cap);\n",
        "void {method_grow}({self_t} *self);\n",
        "void {method_fit}({self_t} *self, size_t len);\n",
        "{element_t} *{method_get}({self_t} *self, size_t index, size_t gen);\n",
        "void {method_push}({self_t} *self, {element_t} value, size_t *index, size_t *gen);\n",
        "{element_t} {method_remove}({self_t} *self, size_t index);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      struct_name = struct_name,
      element_t = element_t,
      element_dyn_arr_t = element_dyn_arr_t,
      size_t_dyn_arr_t = size_t_dyn_arr_t,
      self_t = self_t,
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_get = struct_name.method("get"),
      method_push = struct_name.method("push"),
      method_remove = struct_name.method("remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}

impl<'a> Display for SparseDynArrayImpl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    let element_dyn_arr = DynArray::new(element_t.clone());
    let element_dyn_arr_name = element_dyn_arr.get_name();

    let size_t = "size_t".to_string();
    let size_t_dyn_arr = DynArray::new(size_t);
    let size_t_dyn_arr_name = size_t_dyn_arr.get_name();
    let size_t_dyn_arr_t = size_t_dyn_arr_name.get_type_name();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "void {method_init}({self_t} *self, size_t cap) {{\n",
        "  {element_method_init}(&self->items, cap);\n",
        "  {size_t_method_init}(&self->gens, cap);\n",
        "  self->holes = ({size_t_dyn_arr_t}){{0}};\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
        "void {method_grow}({self_t} *self) {{\n",
        "  {element_method_grow}(&self->items);\n",
        "  {size_t_method_grow}(&self->gens);\n",
        "}}\n",
        "\n",
        "void {method_fit}({self_t} *self, size_t len) {{\n",
        "  {element_method_fit}(&self->items, len);\n",
        "  {size_t_method_fit}(&self->gens, len);\n",
        "}}\n",
        "\n",
        "{element_t} *{method_get}({self_t} *self, size_t index, size_t gen) {{\n",
        "  if (index >= self->items.len)\n",
        "    return NULL;\n",
        "\n",
        "  for (size_t i = 0; i < self->holes.len; ++i) {{\n",
        "    if (self->holes.items[i] == index) {{\n",
        "      return NULL;\n",
        "    }}\n",
        "  }}\n",
        "\n",
        "  size_t found_gen = self->gens.items[index];\n",
        "  if (found_gen == gen) {{\n",
        "    return &self->items.items[index];\n",
        "  }}\n",
        "\n",
        "  return NULL;\n",
        "}}\n",
        "\n",
        "void {method_push}({self_t} *self, {element_t} value, size_t *index, size_t *gen) {{\n",
        "  if (self->holes.len > 0) {{\n",
        "    size_t hole = {size_t_method_pop}(&self->holes);\n",
        "    *index = hole;\n",
        "    self->items.items[hole] = value;\n",
        "    self->gens.items[hole] += 1;\n",
        "    self->len += 1;\n",
        "    *gen = self->gens.items[hole];\n",
        "  }} else {{\n",
        "    if (self->gens.len == self->items.len) {{\n",
        "      *index = self->items.len;\n",
        "      {size_t_method_push}(&self->gens, 0);\n",
        "      {element_method_push}(&self->items, value);\n",
        "      self->len += 1;\n",
        "      *gen = 0;\n",
        "    }} else {{\n",
        "      *index = self->items.len;\n",
        "      self->gens.items[self->items.len] += 1;\n",
        "      {element_method_push}(&self->items, value);\n",
        "      self->len += 1;\n",
        "      *gen = self->gens.items[self->items.len];\n",
        "    }}\n",
        "  }}\n",
        "}}\n",
        "\n",
        "{element_t} {method_remove}({self_t} *self, size_t index) {{\n",
        "  if (index == self->items.len - 1) {{\n",
        "    self->len -= 1;\n",
        "    return {element_method_pop}(&self->items);\n",
        "  }} else {{\n",
        "    self->len -= 1;\n",
        "    {size_t_method_push}(&self->holes, index);\n",
        "    return self->items.items[index];\n",
        "  }}\n",
        "}}\n",
        "\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  {element_method_destroy}(&self->items);\n",
        "  {size_t_method_destroy}(&self->holes);\n",
        "  {size_t_method_destroy}(&self->gens);\n",
        "}}\n",
        "\n",
      ),
      element_t = element_t,
      element_method_init = element_dyn_arr_name.method("init"),
      element_method_grow = element_dyn_arr_name.method("grow"),
      element_method_fit = element_dyn_arr_name.method("fit"),
      element_method_push = element_dyn_arr_name.method("push"),
      element_method_pop = element_dyn_arr_name.method("pop"),
      element_method_destroy = element_dyn_arr_name.method("destroy"),
      size_t_dyn_arr_t = size_t_dyn_arr_t,
      size_t_method_init = size_t_dyn_arr_name.method("init"),
      size_t_method_grow = size_t_dyn_arr_name.method("grow"),
      size_t_method_fit = size_t_dyn_arr_name.method("fit"),
      size_t_method_push = size_t_dyn_arr_name.method("push"),
      size_t_method_pop = size_t_dyn_arr_name.method("pop"),
      size_t_method_destroy = size_t_dyn_arr_name.method("destroy"),
      self_t = self_t,
      method_init = struct_name.method("init"),
      method_grow = struct_name.method("grow"),
      method_fit = struct_name.method("fit"),
      method_get = struct_name.method("get"),
      method_push = struct_name.method("push"),
      method_remove = struct_name.method("remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}
