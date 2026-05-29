use std::fmt::Display;

use crate::generate::generics::common::{method_name, whatever_name};
use crate::generate::generics::dyn_queue::DynQueue;

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

    let uint32_dyn_arr = DynArray::new("uint32_t");
    let uint32_dyn_queue = DynQueue::new("uint32_t");

    let gen_dyn_arr_t = uint32_dyn_arr.get_type();
    let hole_indices_dyn_queue_t = uint32_dyn_queue.get_type();

    let uint64_dyn_arr = DynArray::new("uint64_t");
    let holes_dyn_arr_t = uint64_dyn_arr.get_type();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "typedef struct {whatever} {{\n",
        "  {element_dyn_arr_t} items;\n",
        "\n",
        "  // The kth bit being set means there is a hole at index k. There will be\n",
        "  // one bit for each used index in the array.\n",
        "  {holes_dyn_arr_t} holes;\n",
        "  // Since holes is an array, this contains the indices in that array\n",
        "  // where a hole is to be found. There will be one item per hole.\n",
        "  {hole_indices_dyn_queue_t} hole_indices;\n",
        "\n",
        "  // Every time an item is added, its corresponding gen is incremented.\n",
        "  // The gen is used to differentiate items that happened to be put on\n",
        "  // the same index. A pair of index and gen is thus necessary to address\n",
        "  // any particular item.\n",
        "  {gen_dyn_arr_t} gens;\n",
        "\n",
        "  uint32_t len;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self, uint32_t cap);\n",
        "bool {method_is_hole}({self_t} *self, uint32_t index);\n",
        "{element_t} *{method_get}({self_t} *self, uint32_t index, uint32_t gen);\n",
        "{element_t} *{method_get_unchecked}({self_t} *self, uint32_t index);\n",
        "{element_t} *{method_push}({self_t} *self, {element_t} value, uint32_t *index, uint32_t *gen);\n",
        "bool {method_remove}({self_t} *self, uint32_t index, uint32_t gen, {element_t} *result);\n",
        "bool {method_remove_unchecked}({self_t} *self, uint32_t index, {element_t} *result);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      element_t = element_t,
      element_dyn_arr_t = element_dyn_arr_t,
      gen_dyn_arr_t = gen_dyn_arr_t,
      holes_dyn_arr_t = holes_dyn_arr_t,
      hole_indices_dyn_queue_t = hole_indices_dyn_queue_t,
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_is_hole = method_name!(&self_t, "is_hole"),
      method_get = method_name!(&self_t, "get"),
      method_get_unchecked = method_name!(&self_t, "get_unchecked"),
      method_push = method_name!(&self_t, "push"),
      method_remove = method_name!(&self_t, "remove"),
      method_remove_unchecked = method_name!(&self_t, "remove_unchecked"),
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

    let uint32_dyn_arr = DynArray::new("uint32_t");
    let uint32_dyn_queue = DynQueue::new("uint32_t");

    let gen_dyn_arr_t = uint32_dyn_arr.get_type();
    let hole_indices_dyn_queue_t = uint32_dyn_queue.get_type();

    let holes_dyn_arr = DynArray::new("uint64_t");
    let holes_dyn_arr_t = holes_dyn_arr.get_type();

    write!(
      f,
      concat!(
        "// Sparse dynamic array of `{element_t}`.\n",
        "void {method_init}({self_t} *self, uint32_t cap) {{\n",
        "  {element_method_init}(&self->items, cap);\n",
        "  {holes_method_init}(&self->holes, cap);\n",
        "  {hole_indices_method_init}(&self->hole_indices, cap);\n",
        "  {gen_method_init}(&self->gens, cap);\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
        "void {method_add_holes}({self_t} *self, uint32_t len) {{\n",
        "  uint32_t old_len = self->holes.len;\n",
        "  uint32_t new_len = (len / 64) + 1;\n",
        "\n",
        "  for (uint32_t i = old_len; i < new_len; ++i) {{\n",
        "    {holes_method_push}(&self->holes, 0);\n",
        "  }}\n",
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
        "bool {method_is_hole}({self_t} *self, uint32_t index) {{\n",
        "  if (index >= self->items.len)\n",
        "    return true;\n",
        "\n",
        "  uint32_t holes_i = index / 64;\n",
        "  uint32_t holes_j = index % 64;\n",
        "  uint32_t holes_bitfield = self->holes.items[holes_i];\n",
        "  return (holes_bitfield & (1 << holes_j)) > 0;\n",
        "}}\n",
        "\n",
        "{element_t} *{method_get_unchecked}({self_t} *self, uint32_t index) {{\n",
        "  if ({method_is_hole}(self, index))\n",
        "    return NULL;\n",
        "\n",
        "  return &self->items.items[index];\n",
        "}}\n",
        "\n",
        "{element_t} *{method_push}({self_t} *self, {element_t} value, uint32_t *index, uint32_t *gen) {{\n",
        "  self->len += 1;\n",
        "  if (self->hole_indices.len > 0) {{\n",
        "    uint32_t hole_i = {hole_indices_method_dequeue}(&self->hole_indices);\n",
        "    uint32_t hole_bitmap = self->holes.items[hole_i];\n",
        "    uint32_t hole_j = __builtin_clz(hole_bitmap);\n",
        "    uint32_t hole = hole_i * 64 + hole_j;\n",
        "    *index = hole;\n",
        "\n",
        "    self->holes.items[hole_i] = hole_bitmap & ~(1 << hole_j);\n",
        "    self->items.items[hole] = value;\n",
        "    self->gens.items[hole] += 1;\n",
        "\n",
        "    *gen = self->gens.items[hole];\n",
        "    return &self->items.items[hole];\n",
        "  }} else {{\n",
        "    {element_t} *result;\n",
        "    {method_add_holes}(self, self->items.len + 1);\n",
        "\n",
        "    if (self->gens.len == self->items.len) {{\n",
        "      *index = self->items.len;\n",
        "      {gen_method_push}(&self->gens, 0);\n",
        "      {element_method_push}(&self->items, value);\n",
        "      *gen = 0;\n",
        "      result = &self->items.items[*index];\n",
        "    }} else {{\n",
        "      *index = self->items.len;\n",
        "      self->gens.items[self->items.len] += 1;\n",
        "      {element_method_push}(&self->items, value);\n",
        "      *gen = self->gens.items[self->items.len];\n",
        "      result = &self->items.items[*index];\n",
        "    }}\n",
        "    return result;\n",
        "  }}\n",
        "}}\n",
        "\n",
        "bool {method_remove}({self_t} *self, uint32_t index, uint32_t gen, {element_t} *result) {{\n",
        "  uint32_t found_gen = self->gens.items[index];\n",
        "  if (found_gen == gen) {{\n",
        "    return {method_remove_unchecked}(self, index, result);\n",
        "  }}\n",
        "\n",
        "  return false;\n",
        "}}\n",
        "\n",
        "bool {method_remove_unchecked}({self_t} *self, uint32_t index, {element_t} *result) {{\n",
        "  if ({method_is_hole}(self, index)) {{\n",
        "    return false;\n",
        "  }}\n",
        "  if (index == self->items.len - 1) {{\n",
        "    *result = {element_method_pop}(&self->items);\n",
        "  }} else {{\n",
        "    uint32_t hole_i = index / 64;\n",
        "    uint32_t hole_j = index % 64;\n",
        "    self->holes.items[hole_i] |= 1 << hole_j;\n",
        "    {hole_indices_method_enqueue}(&self->hole_indices, hole_i);\n",
        "    *result = self->items.items[index];\n",
        "  }}\n",
        "  self->len -= 1;\n",
        "  return true;\n",
        "}}\n",
        "\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  {element_method_destroy}(&self->items);\n",
        "  {holes_method_destroy}(&self->holes);\n",
        "  {hole_indices_method_destroy}(&self->hole_indices);\n",
        "  {gen_method_destroy}(&self->gens);\n",
        "}}\n",
        "\n",
      ),
      element_t = element_t,
      element_method_init = method_name!(&element_dyn_arr_t, "init"),
      element_method_push = method_name!(&element_dyn_arr_t, "push"),
      element_method_pop = method_name!(&element_dyn_arr_t, "pop"),
      element_method_destroy = method_name!(&element_dyn_arr_t, "destroy"),
      gen_method_init = method_name!(&gen_dyn_arr_t, "init"),
      gen_method_push = method_name!(&gen_dyn_arr_t, "push"),
      gen_method_destroy = method_name!(&gen_dyn_arr_t, "destroy"),
      holes_method_init = method_name!(&holes_dyn_arr_t, "init"),
      holes_method_push = method_name!(&holes_dyn_arr_t, "push"),
      holes_method_destroy = method_name!(&holes_dyn_arr_t, "destroy"),
      hole_indices_method_init = method_name!(&hole_indices_dyn_queue_t, "init"),
      hole_indices_method_enqueue =
        method_name!(&hole_indices_dyn_queue_t, "enqueue"),
      hole_indices_method_dequeue =
        method_name!(&hole_indices_dyn_queue_t, "dequeue"),
      hole_indices_method_destroy =
        method_name!(&hole_indices_dyn_queue_t, "destroy"),
      self_t = self_t,
      method_init = method_name!(&self_t, "init"),
      method_add_holes = method_name!(&self_t, "add_holes"),
      method_is_hole = method_name!(&self_t, "is_hole"),
      method_get = method_name!(&self_t, "get"),
      method_get_unchecked = method_name!(&self_t, "get_unchecked"),
      method_push = method_name!(&self_t, "push"),
      method_remove = method_name!(&self_t, "remove"),
      method_remove_unchecked = method_name!(&self_t, "remove_unchecked"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}
