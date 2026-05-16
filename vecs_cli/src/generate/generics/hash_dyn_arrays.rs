use std::fmt::Display;

use crate::generate::generics::{
  common::{function_name, method_name, whatever_name},
  dyn_arrays::DynArray,
};

use super::common::{GenericElement, StructName, Whatever, struct_name};

// Stores a (key, value) pair in the hash array.
pub struct HashDynArrayAux<K: GenericElement, V: GenericElement> {
  pub key_t: K,
  pub element_t: V,
}

impl<K: GenericElement, V: GenericElement> HashDynArrayAux<K, V> {
  pub fn new(key_t: K, element_t: V) -> Self {
    Self { key_t, element_t }
  }

  pub fn header<'a>(&'a self) -> HashDynArrayAuxHeader<'a, K, V> {
    HashDynArrayAuxHeader(self)
  }

  pub fn imple<'a>(&'a self) -> HashDynArrayAuxImpl {
    HashDynArrayAuxImpl()
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("hash_dyn_array_aux"; self.key_t, self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("hash_dyn_array_aux", self.key_t, self.element_t)
  }
}

pub struct HashDynArrayAuxHeader<'a, K: GenericElement, V: GenericElement>(
  &'a HashDynArrayAux<K, V>,
);

pub struct HashDynArrayAuxImpl();

impl<'a, K: GenericElement, V: GenericElement> Display
  for HashDynArrayAuxHeader<'a, K, V>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "typedef struct {whatever} {{\n",
        "  {key_t} key;\n",
        "  {element_t} value;\n",
        "}} {self_t};\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      key_t = key_t,
      element_t = element_t,
      self_t = self_t,
    )
  }
}

impl Display for HashDynArrayAuxImpl {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}

// TODO: Make configurable and/or dynamic.
const BUCKET_COUNT: usize = 32;

// The actual hash array.
pub struct HashDynArray<K: GenericElement, V: GenericElement> {
  pub key_t: K,
  pub element_t: V,
  pub aux: HashDynArrayAux<K, V>,
}

impl<K: GenericElement, V: GenericElement> HashDynArray<K, V> {
  pub fn new(key_t: K, element_t: V) -> Self {
    Self {
      key_t: key_t.clone(),
      element_t: element_t.clone(),
      aux: HashDynArrayAux::new(key_t.clone(), element_t.clone()),
    }
  }

  pub fn header<'a>(&'a self) -> HashDynArrayHeader<'a, K, V> {
    HashDynArrayHeader(self)
  }

  pub fn imple<'a>(&'a self) -> HashDynArrayImpl<'a, K, V> {
    HashDynArrayImpl(self)
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("hash_dyn_array"; self.key_t, self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("hash_dyn_array", self.key_t, self.element_t)
  }
}

pub struct HashDynArrayHeader<'a, K: GenericElement, V: GenericElement>(
  &'a HashDynArray<K, V>,
);
pub struct HashDynArrayImpl<'a, K: GenericElement, V: GenericElement>(
  &'a HashDynArray<K, V>,
);

impl<'a, K: GenericElement, V: GenericElement> Display
  for HashDynArrayHeader<'a, K, V>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let self_t = self.0.get_type();

    let aux = &self.0.aux;
    let aux_struct_t = aux.get_type();

    let aux_dyn_arr = DynArray::new(aux_struct_t.clone());
    let aux_dyn_arr_t = aux_dyn_arr.get_type();

    write!(
      f,
      concat!(
        "// Hash dynamic array of `{element_t}` with key `{key_t}`.\n",
        "\n",
        "{aux_header}\n",
        "\n",
        "{aux_dyn_arr_header}\n",
        "\n",
        "typedef struct {whatever} {{\n",
        "  {aux_dyn_arr_t} buckets[{BUCKET_COUNT}];\n",
        "  uint32_t len;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self);\n",
        "void {method_add}({self_t} *self, {key_t} key, {element_t} value);\n",
        "bool {method_get}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      key_t = key_t,
      element_t = element_t,
      aux_dyn_arr_t = aux_dyn_arr_t,
      self_t = self_t,
      aux_header = aux.header(),
      aux_dyn_arr_header = aux_dyn_arr.header(),
      BUCKET_COUNT = BUCKET_COUNT,
      method_init = method_name!(&self_t, "init"),
      method_add = method_name!(&self_t, "add"),
      method_get = method_name!(&self_t, "get"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

impl<'a, K: GenericElement, V: GenericElement> Display
  for HashDynArrayImpl<'a, K, V>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let self_t = self.0.get_type();

    let aux = &self.0.aux;
    let aux_t = aux.get_type();

    let aux_dyn_arr = DynArray::new(aux_t.clone());
    let aux_dyn_arr_t = aux_dyn_arr.get_type();

    let hash_fn_name = function_name!("hash"; key_t);
      let eq_fn_name = function_name!("eq"; key_t);

    write!(
      f,
      concat!(
        "// Hash dynamic array of `{element_t}` with key `{key_t}`.\n",
        "\n",
        "{aux_impl}",
        "\n",
        "{aux_dyn_arr_impl}\n",
        "\n",
        "void {method_init}({self_t} *self) {{\n",
        "  for (uint32_t i = 0; i < {BUCKET_COUNT}; ++i) {{\n",
        "    {aux_method_init}(&self->buckets[i], 0);\n",
        "  }}\n",
        "  self->len = 0;\n",
        "}}\n",
        "\n",
        "static {aux_dyn_arr_t} *{method_get_bucket}({self_t} *self, {key_t} key) {{\n",
        "  uint32_t bucket = {hash_fn_name}(key) % {BUCKET_COUNT};\n",
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
        "  for (uint32_t i = 0; i < bucket->len; ++i) {{\n",
        "    {aux_t} aux = bucket->items[i];\n",
        "    if ({eq_fn_name}(aux.key, key)) {{\n",
        "      {element_t} found = aux.value;\n",
        "      memcpy(result, &found, sizeof found);\n",
        "      return true;\n",
        "    }}\n",
        "  }}\n",
        "  return false;\n",
        "}}\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result) {{\n",
        "  {aux_dyn_arr_t} *bucket = {method_get_bucket}(self, key);\n",
        "  for (uint32_t i = 0; i < bucket->len; ++i) {{\n",
        "    {aux_t} aux = bucket->items[i];\n",
        "    if ({eq_fn_name}(aux.key, key)) {{\n",
        "      {aux_t} aux = {aux_method_swap_remove}(bucket, i);\n",
        "      --self->len;\n",
        "      memcpy(result, &aux.value, sizeof aux.value);\n",
        "      return true;\n",
        "    }}\n",
        "  }}\n",
        "  return false;\n",
        "}}\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  for (uint32_t i = 0; i < {BUCKET_COUNT}; ++i) {{\n",
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
      aux_method_init = method_name!(&aux_dyn_arr_t, "init"),
      aux_method_push = method_name!(&aux_dyn_arr_t, "push"),
      aux_method_swap_remove = method_name!(&aux_dyn_arr_t, "swap_remove"),
      aux_method_destroy = method_name!(&aux_dyn_arr_t, "destroy"),
      hash_fn_name = hash_fn_name,
      eq_fn_name = eq_fn_name,
      self_t = self_t,
      aux_impl = aux.imple(),
      aux_dyn_arr_impl = aux_dyn_arr.imple(),
      BUCKET_COUNT = BUCKET_COUNT,
      method_init = method_name!(&self_t, "init"),
      method_get_bucket = method_name!(&self_t, "get_bucket"),
      method_add = method_name!(&self_t, "add"),
      method_get = method_name!(&self_t, "get"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}
