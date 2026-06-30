use std::fmt::Display;

use crate::generate::generics::common::{
  GenericElement, StructName, Whatever, function_name, method_name, struct_name,
  whatever_name,
};

// Entry of the skip list, containing the values of the next entry (the one to the
// right) and pointers to entries to the right and below.
pub struct SkipListEntry<K: GenericElement, V: GenericElement> {
  pub key_t: K,
  pub element_t: V,
}

impl<K: GenericElement, V: GenericElement> SkipListEntry<K, V> {
  pub fn new(key_t: K, element_t: V) -> Self {
    Self { key_t, element_t }
  }

  pub fn header<'a>(&'a self) -> SkipListEntryHeader<'a, K, V> {
    SkipListEntryHeader(self)
  }

  pub fn imple<'a>(&'a self) -> SkipListEntryImpl {
    SkipListEntryImpl()
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("skip_list_entry"; self.key_t, self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("skip_list_entry", self.key_t, self.element_t)
  }
}

pub struct SkipListEntryHeader<'a, K: GenericElement, V: GenericElement>(
  &'a SkipListEntry<K, V>,
);

pub struct SkipListEntryImpl();

impl<'a, K: GenericElement, V: GenericElement> Display
  for SkipListEntryHeader<'a, K, V>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let self_t = self.0.get_type();

    write!(
      f,
      concat!(
        "typedef struct {whatever} {{\n",
        "  struct {whatever} *right;\n",
        "  struct {whatever} *down;\n",
        "  {key_t} next_key;\n",
        "  {element_t} next_value;\n",
        "}} {self_t};\n",
        "\n",
      ),
      whatever = self.0.get_whatever(),
      self_t = self_t,
      key_t = self.0.key_t,
      element_t = self.0.element_t,
    )
  }
}

impl Display for SkipListEntryImpl {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}

// The actual skip list.
pub struct SkipList<K: GenericElement, V: GenericElement> {
  pub key_t: K,
  pub element_t: V,
  pub entry: SkipListEntry<K, V>,
}

impl<K: GenericElement, V: GenericElement> SkipList<K, V> {
  pub fn new(key_t: K, element_t: V) -> Self {
    Self {
      key_t: key_t.clone(),
      element_t: element_t.clone(),
      entry: SkipListEntry::new(key_t.clone(), element_t.clone()),
    }
  }

  pub fn header<'a>(&'a self) -> SkipListHeader<'a, K, V> {
    SkipListHeader(self)
  }

  pub fn imple<'a>(&'a self) -> SkipListImpl<'a, K, V> {
    SkipListImpl(self)
  }

  pub fn get_type<'a>(&'a self) -> StructName<'a> {
    struct_name!("skip_list"; self.key_t, self.element_t)
  }

  pub fn get_whatever<'a>(&'a self) -> Whatever {
    whatever_name!("skip_list", self.key_t, self.element_t)
  }
}

pub struct SkipListHeader<'a, K: GenericElement, V: GenericElement>(
  &'a SkipList<K, V>,
);
pub struct SkipListImpl<'a, K: GenericElement, V: GenericElement>(&'a SkipList<K, V>);
pub struct SkipListImplInit();

impl<'a, K: GenericElement, V: GenericElement> Display for SkipListHeader<'a, K, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let self_t = self.0.get_type();

    let entry = &self.0.entry;
    let entry_t = entry.get_type();

    write!(
      f,
      concat!(
        "// Skip list of `{element_t}` with key `{key_t}`.\n",
        "\n",
        "{entry_header}\n",
        "\n",
        "typedef struct {whatever} {{\n",
        "  // This can't be in-place because it's self-referential.\n",
        "  {entry_t} *header;\n",
        "  uint8_t height;\n",
        "}} {self_t};\n",
        "\n",
        "void {method_init}({self_t} *self);\n",
        "void {method_add}({self_t} *self, {key_t} key, {element_t} value);\n",
        "bool {method_get}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result);\n",
        "void {method_destroy}({self_t} *self);\n",
        "\n",
      ),
      entry_header = entry.header(),
      whatever = self.0.get_whatever(),
      key_t = key_t,
      element_t = element_t,
      self_t = self_t,
      entry_t = entry_t,
      method_init = method_name!(&self_t, "init"),
      method_add = method_name!(&self_t, "add"),
      method_get = method_name!(&self_t, "get"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

const MAX_HEIGHT: usize = 8;

impl<'a, K: GenericElement, V: GenericElement> Display for SkipListImpl<'a, K, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let key_t = &self.0.key_t;
    let element_t = &self.0.element_t;

    let self_t = self.0.get_type();

    let entry = &self.0.entry;
    let entry_t = entry.get_type();

    let max_key_fn_name = function_name!("max"; key_t);
    let key_eq_fn_name = function_name!("eq"; key_t);
    let key_cmp_fn_name = function_name!("cmp"; key_t);

    write!(
      f,
      concat!(
        "// Skip list of `{element_t}` with key `{key_t}`.\n",
        "{entry_impl}",
        "\n",
        "void {method_init}({self_t} *self) {{\n",
        "  const size_t size = {MAX_HEIGHT} * sizeof self->header[0];\n",
        "  self->header = ({entry_t} *)malloc(size);\n",
        "  self->height = 1;\n",
        "\n",
        "  self->header[0].next_key = {max_key_fn_name}();\n",
        "  self->header[0].next_value = 0;\n",
        "  self->header[0].right = NULL;\n",
        "  self->header[0].down = NULL;\n",
        "\n",
        "  for (uint8_t i = 1; i < {MAX_HEIGHT}; ++i) {{\n",
        "    self->header[i].next_key = {max_key_fn_name}();\n",
        "    self->header[i].next_value = 0;\n",
        "    self->header[i].right = NULL;\n",
        "    self->header[i].down = &self->header[i - 1];\n",
        "  }}\n",
        "}}\n",
        "\n",
        "void {method_add}({self_t} *self, {key_t} key, {element_t} value) {{\n",
        "  uint8_t ceiling_index = self->height - 1;\n",
        "  uint8_t level = ceiling_index;\n",
        "\n",
        "  {entry_t} *rightmosts[{MAX_HEIGHT}];\n",
        "  {entry_t} *current = &self->header[ceiling_index];\n",
        "\n",
        "  do {{\n",
        "    while ({key_cmp_fn_name}(current->next_key, key) < 0) {{\n",
        "      current = current->right;\n",
        "    }}\n",
        "\n",
        "    if ({key_eq_fn_name}(current->next_key, key)) {{\n",
        "      current->next_value = value;\n",
        "    }}\n",
        "\n",
        "    rightmosts[level] = current;\n",
        "    current = current->down;\n",
        "    --level;\n",
        "  }} while (current != NULL);\n",
        "\n",
        "  uint8_t block_height = skiplist_random_level();\n",
        "  {entry_t} *block =\n",
        "      ({entry_t} *)malloc(block_height * sizeof({entry_t}));\n",
        "\n",
        "  if (block_height > self->height) {{\n",
        "    uint8_t old_height = self->height;\n",
        "    self->height = block_height;\n",
        "\n",
        "    for (size_t i = old_height; i < block_height; ++i) {{\n",
        "      block[i].next_key = self->header[i].next_key;\n",
        "      block[i].next_value = self->header[i].next_value;\n",
        "      block[i].right = self->header[i].right;\n",
        "      block[i].down = &block[i - 1];\n",
        "\n",
        "      self->header[i].right = &block[i];\n",
        "      self->header[i].next_key = key;\n",
        "      self->header[i].next_value = value;\n",
        "    }}\n",
        "\n",
        "    for (uint8_t i = old_height - 1; i > 0; --i) {{\n",
        "      block[i].next_key = rightmosts[i]->next_key;\n",
        "      block[i].next_value = rightmosts[i]->next_value;\n",
        "      block[i].right = rightmosts[i]->right;\n",
        "      block[i].down = &block[i - 1];\n",
        "\n",
        "      rightmosts[i]->right = &block[i];\n",
        "      rightmosts[i]->next_key = key;\n",
        "      rightmosts[i]->next_value = value;\n",
        "    }}\n",
        "  }} else {{\n",
        "    for (uint8_t i = block_height - 1; i > 0; --i) {{\n",
        "      block[i].next_key = rightmosts[i]->next_key;\n",
        "      block[i].next_value = rightmosts[i]->next_value;\n",
        "      block[i].right = rightmosts[i]->right;\n",
        "      block[i].down = &block[i - 1];\n",
        "\n",
        "      rightmosts[i]->right = &block[i];\n",
        "      rightmosts[i]->next_key = key;\n",
        "      rightmosts[i]->next_value = value;\n",
        "    }}\n",
        "  }}\n",
        "\n",
        "  block[0].next_key = rightmosts[0]->next_key;\n",
        "  block[0].next_value = rightmosts[0]->next_value;\n",
        "  block[0].right = rightmosts[0]->right;\n",
        "  block[0].down = NULL;\n",
        "\n",
        "  rightmosts[0]->right = &block[0];\n",
        "  rightmosts[0]->next_key = key;\n",
        "  rightmosts[0]->next_value = value;\n",
        "}}\n",
        "bool {method_get}({self_t} *self, {key_t} key, {element_t} *result) {{\n",
        "  {entry_t} *current = &self->header[self->height - 1];\n",
        "\n",
        "  do {{\n",
        "    while ({key_cmp_fn_name}(current->next_key, key) < 0) {{\n",
        "      current = current->right;\n",
        "    }}\n",
        "\n",
        "    if ({key_eq_fn_name}(current->next_key, key)) {{\n",
        "      *result = current->next_value;\n",
        "      return true;\n",
        "    }}\n",
        "\n",
        "    current = current->down;\n",
        "  }} while (current != NULL);\n",
        "\n",
        "  return false;\n",
        "}}\n",
        "bool {method_remove}({self_t} *self, {key_t} key, {element_t} *result) {{\n",
        "  if (self->height == 0 || {key_eq_fn_name}(key, {max_key_fn_name}())) {{\n",
        "    return false;\n",
        "  }}\n",
        "\n",
        "  {entry_t} *current = &self->header[self->height - 1];\n",
        "\n",
        "  do {{\n",
        "    {element_t} next_value = current->next_value;\n",
        "\n",
        "    while ({key_cmp_fn_name}(current->next_key, key) < 0) {{\n",
        "      current = current->right;\n",
        "    }}\n",
        "\n",
        "    {entry_t} *next = current->right;\n",
        "    if ({key_eq_fn_name}(current->next_key, key)) {{\n",
        "      current->next_key = next->next_key;\n",
        "      current->next_value = next->next_value;\n",
        "      current->right = next->right;\n",
        "\n",
        "      if (current->down != NULL) {{\n",
        "        current = current->down;\n",
        "      }} else {{\n",
        "        *result = next_value;\n",
        "        free(next);\n",
        "        return true;\n",
        "      }}\n",
        "    }} else {{\n",
        "      current = current->down;\n",
        "    }}\n",
        "  }} while (current != NULL);\n",
        "\n",
        "  return false;\n",
        "}}\n",
        "void {method_destroy}({self_t} *self) {{\n",
        "  {entry_t} *header = self->header, *current = header[0].right;\n",
        "\n",
        "  while (current != NULL) {{\n",
        "    {entry_t} *to_remove = current;\n",
        "    current = current->right;\n",
        "    free(to_remove);\n",
        "  }}\n",
        "\n",
        "  free(header);\n",
        "}}\n",
        "\n",
      ),
      key_t = key_t,
      element_t = element_t,
      entry_t = entry_t,
      self_t = self_t,
      max_key_fn_name = max_key_fn_name,
      key_eq_fn_name = key_eq_fn_name,
      key_cmp_fn_name = key_cmp_fn_name,
      entry_impl = entry.imple(),
      MAX_HEIGHT = MAX_HEIGHT,
      method_init = method_name!(&self_t, "init"),
      method_add = method_name!(&self_t, "add"),
      method_get = method_name!(&self_t, "get"),
      method_remove = method_name!(&self_t, "remove"),
      method_destroy = method_name!(&self_t, "destroy"),
    )
  }
}

impl Display for SkipListImplInit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // <https://github.com/veigaribo/skiplist-height>
    write!(
      f,
      concat!(
        "static inline uint64_t rotl(const uint64_t x, int k) {{\n",
        "  return (x << k) | (x >> (64 - k));\n",
        "}}\n",
        "\n",
        "uint64_t s[4] = {{2611686018427387905, 8305843009213693953, 9152921504606846977, 476460752303423488}};\n",
        "\n",
        "// xoshiro256+\n",
        "static inline uint64_t rng_next(void) {{\n",
        "  const uint64_t result = s[0] + s[3];\n",
        "  const uint64_t t = s[1] << 17;\n",
        "\n",
        "  s[2] ^= s[0];\n",
        "  s[3] ^= s[1];\n",
        "  s[1] ^= s[2];\n",
        "  s[0] ^= s[3];\n",
        "\n",
        "  s[2] ^= t;\n",
        "  s[3] = rotl(s[3], 45);\n",
        "\n",
        "  return result;\n",
        "}}\n",
        "\n",
        "#define VECS_FIX_RANGE (6)\n",
        "#define VECS_FIX_SCALE (32 - VECS_FIX_RANGE)\n",
        "\n",
        "#define VECS_FIX_MUL(a, b) (((a) >> 13) * ((b) >> 13))\n",
        "#define VECS_FIX_ONE (1 << VECS_FIX_SCALE)\n",
        "\n",
        "static inline uint8_t leading_zero_count(uint32_t v) {{\n",
        "  if (v == 0) {{\n",
        "    return 32;\n",
        "  }}\n",
        "\n",
        "  return __builtin_clz(v);\n",
        "}}\n",
        "\n",
        "static inline uint32_t fix_log2_approx(uint32_t fix_v) {{\n",
        "  uint32_t integral = (leading_zero_count(fix_v << VECS_FIX_RANGE));\n",
        "  uint32_t fix_integral = integral << VECS_FIX_SCALE;\n",
        "\n",
        "  uint32_t fix_mantissa =\n",
        "      ~(fix_v << (integral + 1)) & (0b00000011111111111111111111111111);\n",
        "\n",
        "  return fix_integral | fix_mantissa;\n",
        "}}\n",
        "\n",
        "static inline uint8_t skiplist_random_level() {{\n",
        "  uint32_t fix_random = rng_next();\n",
        "  fix_random = fix_random >> VECS_FIX_RANGE;\n",
        "  uint32_t fix_log_random = fix_log2_approx(fix_random);\n",
        "\n",
        "  uint32_t fix_result = VECS_FIX_ONE + VECS_FIX_MUL(fix_log_random, 0b00000010010011011000100010101001);\n",
        "  uint8_t result = (uint8_t)(fix_result >> VECS_FIX_SCALE);\n",
        "\n",
        "  return (result < {MAX_HEIGHT}) ? result : {MAX_HEIGHT};\n",
        "}}\n",
      ),
      MAX_HEIGHT = MAX_HEIGHT,
    )
  }
}
