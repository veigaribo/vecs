use std::fmt::Display;

use crate::generate::generics::{common::FunctionName, dyn_arrays::DynArray};

use super::common::StructName;

// TODO: Make configurable and/or dynamic.
const BUCKET_COUNT: usize = 16;

pub struct HashDynArray {
  pub key_t: String,
  pub element_t: String,
}

impl HashDynArray {
  pub fn new(key_t: String, element_t: String) -> Self {
    Self { key_t, element_t }
  }

  pub fn header<'a>(&'a self) -> HashDynArrayHeader<'a> {
    HashDynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> HashDynArrayImpl<'a> {
    HashDynArrayImpl(self)
  }

  pub fn get_name<'a>(&'a self) -> StructName<'a> {
    StructName::new(
      "hash_dyn_array",
      vec![self.key_t.as_str(), self.element_t.as_str()],
    )
  }

  pub fn get_aux_struct_name<'a>(&'a self) -> StructName<'a> {
    // Stores a (key, value) pair.
    StructName::new(
      "hash_dyn_array_aux",
      vec![self.key_t.as_str(), self.element_t.as_str()],
    )
  }
}

pub struct HashDynArrayHeader<'a>(&'a HashDynArray);
pub struct HashDynArrayImpl<'a>(&'a HashDynArray);

impl<'a> Display for HashDynArrayHeader<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    let aux_struct_name = self.0.get_aux_struct_name();
    let aux_struct_t = aux_struct_name.get_type_name();

    let aux_dyn_arr = DynArray::new(aux_struct_t.clone());
    let aux_dyn_arr_t = aux_dyn_arr.get_name().get_type_name();

    write!(f, "{}\n", aux_dyn_arr.header())?;

    write!(
      f,
      concat!(
        "// Hash dynamic array of `{element_t}` with key `{key_t}`.\n",
        "\n",
        "struct {aux_struct_name} {{\n",
        "  {key_t} key;\n",
        "  {element_t} value;\n",
        "}};\n",
        "\n",
        "struct {struct_name} {{\n",
        "  {aux_dyn_arr_t} buckets[{BUCKET_COUNT}];\n",
        "  size_t len;\n",
        "}};\n",
        "\n",
        "void {method_init}({self_t} *self);\n",
        "void {method_add}({self_t} *self, {key_t} key, {element_t} value);\n",
        "bool {method_get}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      struct_name = struct_name,
      key_t = key_t,
      element_t = element_t,
      aux_struct_name = aux_struct_name,
      aux_dyn_arr_t = aux_dyn_arr_t,
      self_t = self_t,
      BUCKET_COUNT = BUCKET_COUNT,
      method_init = struct_name.method("init"),
      method_add = struct_name.method("add"),
      method_get = struct_name.method("get"),
      method_remove = struct_name.method("remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}

impl<'a> Display for HashDynArrayImpl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let struct_name = self.0.get_name();
    let self_t = struct_name.get_type_name();

    let aux_struct_name = self.0.get_aux_struct_name();
    let aux_t = aux_struct_name.get_type_name();

    let aux_dyn_arr = DynArray::new(aux_t.clone());
    let aux_dyn_arr_name = aux_dyn_arr.get_name();
    let aux_dyn_arr_t = aux_dyn_arr_name.get_type_name();

    let hash_fn_name = FunctionName::new("hash", vec![key_t.as_str()]);

    write!(f, "{}\n", aux_dyn_arr.imple())?;

    write!(
      f,
      concat!(
        "// Hash dynamic array of `{element_t}` with key `{key_t}`.\n",
        "\n",
        "void {method_init}({self_t} *self) {{\n",
        "  for (size_t i = 0; i < {BUCKET_COUNT}; ++i) {{\n",
        "    {aux_method_init}(&self->buckets[i], 0);\n",
        "  }}\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
        "static {aux_dyn_arr_t} *{method_get_bucket}({self_t} *self, {key_t} key) {{\n",
        "  size_t bucket = {hash_fn_name}(key) % {BUCKET_COUNT};\n",
        "  return &self->buckets[bucket];\n",
        "}}\n",
        "\n",
        "void {method_add}({self_t} *self, {key_t} key, {element_t} value) {{\n",
        "  {aux_dyn_arr_t} *bucket = {method_get_bucket}(self, key);\n",
        "  {aux_t} aux = {{.key = key, .value = value}};\n",
        "  {aux_method_push}(bucket, aux);\n",
        "  ++self->len;\n",
        "}}\n",
        "bool {method_get}({self_t} *self, {key_t} key, {element_t} *result) {{\n",
        "  {aux_dyn_arr_t} *bucket = {method_get_bucket}(self, key);\n",
        "  for (size_t i = 0; i < bucket->len; ++i) {{\n",
        "    {aux_t} aux = bucket->items[i];\n",
        "    if (aux.key == key) {{\n",
        "      {element_t} found = aux.value;\n",
        "      memcpy(result, &found, sizeof found);\n",
        "      return true;\n",
        "    }}\n",
        "  }}\n",
        "  return false;\n",
        "}}\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result) {{\n",
        "  {aux_dyn_arr_t} *bucket = {method_get_bucket}(self, key);\n",
        "  for (size_t i = 0; i < bucket->len; ++i) {{\n",
        "    {aux_t} aux = bucket->items[i];\n",
        "    if (aux.key == key) {{\n",
        "      {aux_t} aux = {aux_method_swap_remove}(bucket, i);\n",
        "      --self->len;\n",
        "      memcpy(result, &aux.value, sizeof aux.value);\n",
        "      return true;\n",
        "    }}\n",
        "  }}\n",
        "  return false;\n",
        "}}\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  for (size_t i = 0; i < {BUCKET_COUNT}; ++i) {{\n",
        "    {aux_method_destroy}(&self->buckets[i]);\n",
        "  }}\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
      ),
      key_t = key_t,
      element_t = element_t,
      aux_t = aux_t,
      aux_dyn_arr_t = aux_dyn_arr_t,
      aux_method_init = aux_dyn_arr_name.method("init"),
      aux_method_push = aux_dyn_arr_name.method("push"),
      aux_method_swap_remove = aux_dyn_arr_name.method("swap_remove"),
      aux_method_destroy = aux_dyn_arr_name.method("destroy"),
      hash_fn_name = hash_fn_name,
      self_t = self_t,
      BUCKET_COUNT = BUCKET_COUNT,
      method_init = struct_name.method("init"),
      method_get_bucket = struct_name.method("get_bucket"),
      method_add = struct_name.method("add"),
      method_get = struct_name.method("get"),
      method_remove = struct_name.method("remove"),
      method_destroy = struct_name.method("destroy"),
    )
  }
}
