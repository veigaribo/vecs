use std::fmt::Display;

use crate::generate::generics::common::{method_name, whatever_name};

use super::common::{GenericElement, StructName, Whatever, struct_name};
use super::dyn_arrays::DynArray;

pub struct SparseDynArray<T: GenericElement> {
  pub element_t: T,
}

impl<T: GenericElement> SparseDynArray<T> {
  pub fn new(element_t: T) -> Self {
    Self { element_t }
  }

  pub fn header<'a>(&'a self) -> SparseDynArrayHeader<'a, T> {
    SparseDynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> SparseDynArrayImpl<'a, T> {
    SparseDynArrayImpl(self)
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("sparse_dyn_array"; self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("sparse_dyn_array", self.element_t)
  }
}

pub struct SparseDynArrayHeader<'a, T: GenericElement>(&'a SparseDynArray<T>);
pub struct SparseDynArrayImpl<'a, T: GenericElement>(&'a SparseDynArray<T>);

impl<'a, T: GenericElement> Display for SparseDynArrayHeader<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    let element_dyn_arr = DynArray::new(element_t.clone());
    let element_dyn_arr_t = element_dyn_arr.get_type();

    let uint32_t_dyn_arr_t = DynArray::new("uint32_t");
    let uint32_t_dyn_arr_t = uint32_t_dyn_arr_t.get_type();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "typedef struct {whatever} {{\n",
        "  {element_dyn_arr_t} items;\n",
        "  {uint32_t_dyn_arr_t} holes;\n",
        "\n",
        "  // Every time an item is added, its corresponding gen is incremented.\n",
        "  // The gen is used to differentiate items that happened to be put on\n",
        "  // the same index. A pair of index and gen is thus necessary to address\n",
        "  // any particular item.\n",
        "  {uint32_t_dyn_arr_t} gens;\n",
        "\n",
        "  uint32_t len;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self, uint32_t cap);\n",
        "void {method_grow}({self_t} *self);\n",
        "void {method_fit}({self_t} *self, uint32_t len);\n",
        "{element_t} *{method_get}({self_t} *self, uint32_t index, uint32_t gen);\n",
        "{element_t} *{method_get_unchecked}({self_t} *self, uint32_t index);\n",
        "{element_t} *{method_push}({self_t} *self, {element_t} value, uint32_t *index, uint32_t *gen);\n",
        "{element_t} {method_remove}({self_t} *self, uint32_t index);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      element_t = element_t,
      element_dyn_arr_t = element_dyn_arr_t,
      uint32_t_dyn_arr_t = uint32_t_dyn_arr_t,
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_get = method_name!(&self_t, "get"),
      method_get_unchecked = method_name!(&self_t, "get_unchecked"),
      method_push = method_name!(&self_t, "push"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

impl<'a, T: GenericElement> Display for SparseDynArrayImpl<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let element_t = &self.0.element_t;
    let self_t = self.0.get_type();

    let element_dyn_arr = DynArray::new(element_t.clone());
    let element_dyn_arr_t = element_dyn_arr.get_type();

    let uint32_t_dyn_arr = DynArray::new("uint32_t");
    let uint32_t_dyn_arr_t = uint32_t_dyn_arr.get_type();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "void {method_init}({self_t} *self, uint32_t cap) {{\n",
        "  {element_method_init}(&self->items, cap);\n",
        "  {uint32_t_method_init}(&self->gens, cap);\n",
        "  self->holes = ({uint32_t_dyn_arr_t}){{0}};\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
        "void {method_grow}({self_t} *self) {{\n",
        "  {element_method_grow}(&self->items);\n",
        "  {uint32_t_method_grow}(&self->gens);\n",
        "}}\n",
        "\n",
        "void {method_fit}({self_t} *self, uint32_t len) {{\n",
        "  {element_method_fit}(&self->items, len);\n",
        "  {uint32_t_method_fit}(&self->gens, len);\n",
        "}}\n",
        "\n",
        "{element_t} *{method_get}({self_t} *self, uint32_t index, uint32_t gen) {{\n",
        "  uint32_t found_gen = self->gens.items[index];\n",
        "  if (found_gen == gen) {{\n",
        "    return {method_get_unchecked}(self, index);\n",
        "  }}\n",
        "\n",
        "  return NULL;\n",
        "}}\n",
        "\n",
        "{element_t} *{method_get_unchecked}({self_t} *self, uint32_t index) {{\n",
        "  if (index >= self->items.len)\n",
        "    return NULL;\n",
        "\n",
        "  for (size_t i = 0; i < self->holes.len; ++i) {{\n",
        "    if (self->holes.items[i] == index) {{\n",
        "      return NULL;\n",
        "    }}\n",
        "  }}\n",
        "\n",
        "  return &self->items.items[index];\n",
        "}}\n",
        "\n",
        "{element_t} *{method_push}({self_t} *self, {element_t} value, uint32_t *index, uint32_t *gen) {{\n",
        "  if (self->holes.len > 0) {{\n",
        "    uint32_t hole = {uint32_t_method_pop}(&self->holes);\n",
        "    *index = hole;\n",
        "    self->items.items[hole] = value;\n",
        "    self->gens.items[hole] += 1;\n",
        "    self->len += 1;\n",
        "    *gen = self->gens.items[hole];\n",
        "    return &self->items.items[*index];\n",
        "  }} else {{\n",
        "    if (self->gens.len == self->items.len) {{\n",
        "      *index = self->items.len;\n",
        "      {uint32_t_method_push}(&self->gens, 0);\n",
        "      {element_method_push}(&self->items, value);\n",
        "      self->len += 1;\n",
        "      *gen = 0;\n",
        "      return &self->items.items[*index];\n",
        "    }} else {{\n",
        "      *index = self->items.len;\n",
        "      self->gens.items[self->items.len] += 1;\n",
        "      {element_method_push}(&self->items, value);\n",
        "      self->len += 1;\n",
        "      *gen = self->gens.items[self->items.len];\n",
        "      return &self->items.items[*index];\n",
        "    }}\n",
        "  }}\n",
        "}}\n",
        "\n",
        "{element_t} {method_remove}({self_t} *self, uint32_t index) {{\n",
        "  if (index == self->items.len - 1) {{\n",
        "    self->len -= 1;\n",
        "    return {element_method_pop}(&self->items);\n",
        "  }} else {{\n",
        "    self->len -= 1;\n",
        "    {uint32_t_method_push}(&self->holes, index);\n",
        "    return self->items.items[index];\n",
        "  }}\n",
        "}}\n",
        "\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  {element_method_destroy}(&self->items);\n",
        "  {uint32_t_method_destroy}(&self->holes);\n",
        "  {uint32_t_method_destroy}(&self->gens);\n",
        "}}\n",
        "\n",
      ),
      element_t = element_t,
      element_method_init = method_name!(&element_dyn_arr_t, "init"),
      element_method_grow = method_name!(&element_dyn_arr_t, "grow"),
      element_method_fit = method_name!(&element_dyn_arr_t, "fit"),
      element_method_push = method_name!(&element_dyn_arr_t, "push"),
      element_method_pop = method_name!(&element_dyn_arr_t, "pop"),
      element_method_destroy = method_name!(&element_dyn_arr_t, "destroy"),
      uint32_t_dyn_arr_t = uint32_t_dyn_arr_t,
      uint32_t_method_init = method_name!(&uint32_t_dyn_arr_t, "init"),
      uint32_t_method_grow = method_name!(&uint32_t_dyn_arr_t, "grow"),
      uint32_t_method_fit = method_name!(&uint32_t_dyn_arr_t, "fit"),
      uint32_t_method_push = method_name!(&uint32_t_dyn_arr_t, "push"),
      uint32_t_method_pop = method_name!(&uint32_t_dyn_arr_t, "pop"),
      uint32_t_method_destroy = method_name!(&uint32_t_dyn_arr_t, "destroy"),
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_grow = method_name!(&self_t, "grow"),
      method_fit = method_name!(&self_t, "fit"),
      method_get = method_name!(&self_t, "get"),
      method_get_unchecked = method_name!(&self_t, "get_unchecked"),
      method_push = method_name!(&self_t, "push"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}
