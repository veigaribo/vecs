#include "generate.h"
#include "path_buffer.h"
#include "sparse/expression.h"
#include "sparse/symbol.h"
#include "sparse/token.h"
#include "str_arena.h"
#include <stdio.h>

// State: Each generator will have a copy of this handy.

typedef struct {
  // Indent level of the code the generator generates.
  // Used to make the generated code pretty.
  uint32_t generator_indent;
  // Indent level of the data being displayed by the generated code.
  // Used to make the final result legible.
  uint32_t generated_indent;

  // Used to store strings. Particularly for use in `path`.
  str_arena_t str_arena;

  // If inside an struct, union or array, will be the path to it from the root
  // e.g. "state->prefix."
  path_buffer_t path;
} generator_state_t;

static void state_init(generator_state_t *state) {
  state->generated_indent = 0;
  state->generator_indent = 0;
  path_buffer_init(&state->path);
  str_arena_init(&state->str_arena);
}

static void state_destroy(generator_state_t *state) {
  path_buffer_destroy(&state->path);
  str_arena_destroy(&state->str_arena);
}

// Utility functions.

static struct symbol *maybe_get_base_type(struct symbol *symbol) {
  if (symbol->type == SYM_NODE) {
    symbol = symbol->ctype.base_type;
  }

  return symbol;
}

static void print_generator_indent(FILE *file, generator_state_t *state) {
  for (uint32_t i = 0; i < state->generator_indent; ++i) {
    fprintf(file, "  ");
  }
}

static void print_generated_indent(FILE *file, generator_state_t *state) {
  if (state->generated_indent == 0) {
    return;
  }

  // This is the only function that prints its own preceding whitespace, which
  // is weird, but oh so very handy...
  print_generator_indent(file, state);

  fprintf(file, "printf(\"");
  for (uint32_t i = 0; i < state->generated_indent; ++i) {
    fprintf(file, "  ");
  }
  fprintf(file, "\");\n");
}

// Wrappers around path buffers and the string stack arena.

// If you know the name of the symbol (and have it properly stored!), this will
// only push its path to the path buffer. Later pop it using `path_buffer_pop`.
static void path_buffer_push_symbol_str(struct symbol *symbol,
                                        generator_state_t *state, char *name) {
  if (is_struct_type(symbol)) {
    path_buffer_push_value_str(&state->path, name);
  } else if (is_union_type(symbol)) {
    path_buffer_push_value_str(&state->path, name);
  } else if (is_array_type(symbol)) {
    path_buffer_push_indexed_str(&state->path, name);
  } else if (is_enum_type(symbol)) {
    path_buffer_push_leaf_str(&state->path, name);
  } else if (is_ptr_type(symbol)) {
    path_buffer_push_ptr_str(&state->path, name);
  } else if (is_int_type(symbol)) {
    path_buffer_push_leaf_str(&state->path, name);
  } else if (is_float_type(symbol)) {
    path_buffer_push_leaf_str(&state->path, name);
  } else {
    path_buffer_push_leaf_str(&state->path, name);
  }
}

// If you want to refer to a member as an integer (i.e. you are an array), this
// will only push its path to the path buffer. Later pop it using
// `path_buffer_pop`.
static void path_buffer_push_symbol_int(struct symbol *symbol,
                                        generator_state_t *state,
                                        size_t integer) {
  if (is_struct_type(symbol)) {
    path_buffer_push_value_int(&state->path, integer);
  } else if (is_union_type(symbol)) {
    path_buffer_push_value_int(&state->path, integer);
  } else if (is_array_type(symbol)) {
    path_buffer_push_indexed_int(&state->path, integer);
  } else if (is_enum_type(symbol)) {
    path_buffer_push_leaf_int(&state->path, integer);
  } else if (is_ptr_type(symbol)) {
    path_buffer_push_ptr_int(&state->path, integer);
  } else if (is_int_type(symbol)) {
    path_buffer_push_leaf_int(&state->path, integer);
  } else if (is_float_type(symbol)) {
    path_buffer_push_leaf_int(&state->path, integer);
  } else {
    path_buffer_push_leaf_int(&state->path, integer);
  }
}

// This will push the name of the symbol to the path buffer for you. Later pop
// it using `path_buffer_pop_symbol`.
static char *path_buffer_push_symbol(struct symbol *symbol,
                                     generator_state_t *state) {
  char *name = (char *)show_ident(symbol->ident);
  name = str_arena_push_nt(&state->str_arena, name);
  path_buffer_push_symbol_str(symbol, state, name);

  return name;
}

// Undoes what `path_buffer_push_symbol` does. Do not use if you did not use
// `path_buffer_push_symbol` specifically.
static void path_buffer_pop_symbol(char *name, generator_state_t *state) {
  path_buffer_pop(&state->path);
  str_arena_pop(&state->str_arena, name);
}

// Generators proper.

static void generate_dump_member(FILE *file, struct symbol *symbol,
                                 generator_state_t *state);

// TODO: Use the appropriate format specifier
static void generate_dump_int(FILE *file, struct symbol *symbol,
                              generator_state_t *state) {
  print_generator_indent(file, state);
  fprintf(file, "printf(\"%%d\\n\", ");
  path_buffer_dump(file, &state->path);
  fprintf(file, ");\n");
}

static void generate_dump_float(FILE *file, struct symbol *symbol,
                                generator_state_t *state) {
  print_generator_indent(file, state);
  fprintf(file, "printf(\"%%f\\n\", ");
  path_buffer_dump(file, &state->path);
  fprintf(file, ");\n");
}

static void generate_dump_ptr(FILE *file, struct symbol *symbol,
                              generator_state_t *state) {
  print_generator_indent(file, state);
  fprintf(file, "printf(\"%%p\\n\", ");
  path_buffer_dump(file, &state->path);
  fprintf(file, ");\n");
}

static void generate_dump_struct(FILE *file, struct symbol *symbol,
                                 generator_state_t *state) {
  struct symbol *base_type = maybe_get_base_type(symbol);

  print_generator_indent(file, state);
  fprintf(file, "printf(\"struct %s {\\n\");\n", show_ident(base_type->ident));

  struct symbol *member;
  FOR_EACH_PTR(base_type->symbol_list, member) {
    char *member_name = path_buffer_push_symbol(member, state);
    ++state->generated_indent;

    print_generated_indent(file, state);
    print_generator_indent(file, state);
    fprintf(file, "printf(\"%s: \");\n", member_name);
    generate_dump_member(file, member, state);

    --state->generated_indent;
    path_buffer_pop_symbol(member_name, state);
  }
  END_FOR_EACH_PTR(member);

  print_generated_indent(file, state);
  print_generator_indent(file, state);
  fprintf(file, "printf(\"}\\n\");\n");
}

static void generate_dump_array(FILE *file, struct symbol *symbol,
                                generator_state_t *state) {
  struct symbol *base_type = maybe_get_base_type(symbol);
  struct expression *size_expr = base_type->array_size;

  // If the array size is undetermined at compile time, treat is as a pointer.
  if (!expr_truth_value(size_expr)) {
    generate_dump_ptr(file, symbol, state);
  }

  print_generator_indent(file, state);
  fprintf(file, "printf(\"%s {\\n\");\n", show_typename(base_type));

  long long size_const = get_expression_value(size_expr);

  print_generator_indent(file, state);
  fprintf(file, "for (size_t i = 0; i < %lld; ++i) {\n", size_const);

  struct symbol *el = base_type->ctype.base_type;
  path_buffer_push_symbol_str(el, state, "i");

  ++state->generator_indent;
  ++state->generated_indent;

  print_generated_indent(file, state);
  generate_dump_member(file, base_type->ctype.base_type, state);

  --state->generated_indent;
  --state->generator_indent;

  path_buffer_pop(&state->path);

  print_generator_indent(file, state);
  fprintf(file, "}\n");

  print_generated_indent(file, state);
  print_generator_indent(file, state);
  fprintf(file, "printf(\"}\\n\");\n");
}

static void generate_dump_union(FILE *file, struct symbol *symbol,
                                generator_state_t *state) {
  struct symbol *base_type = maybe_get_base_type(symbol);

  print_generator_indent(file, state);
  fprintf(file, "printf(\"union %s {\\n\");\n", show_ident(base_type->ident));

  struct symbol *member;
  FOR_EACH_PTR(base_type->symbol_list, member) {
    char *member_name = path_buffer_push_symbol(member, state);
    ++state->generated_indent;

    print_generated_indent(file, state);
    print_generator_indent(file, state);
    fprintf(file, "printf(\"%s: \");\n", member_name);
    generate_dump_member(file, member, state);

    --state->generated_indent;
    path_buffer_pop_symbol(member_name, state);
  }
  END_FOR_EACH_PTR(member);

  print_generated_indent(file, state);
  print_generator_indent(file, state);
  fprintf(file, "printf(\"}\\n\");\n");
}

static void generate_dump_enum(FILE *file, struct symbol *symbol,
                               generator_state_t *state) {
  struct symbol *base_type = maybe_get_base_type(symbol);

  print_generator_indent(file, state);
  fprintf(file, "switch (");
  path_buffer_dump(file, &state->path);
  fprintf(file, ") {\n");

  struct symbol *member;
  FOR_EACH_PTR(base_type->symbol_list, member) {
    const char *candidate = show_ident(member->ident);
    print_generator_indent(file, state);
    fprintf(file, "case %s:\n", candidate);
    ++state->generator_indent;
    print_generator_indent(file, state);
    fprintf(file, "printf(\"%s\\n\");\n", candidate);
    print_generator_indent(file, state);
    fprintf(file, "break;\n");
    --state->generator_indent;
  }
  END_FOR_EACH_PTR(member);

  print_generator_indent(file, state);
  fprintf(file, "default:\n");
  ++state->generator_indent;
  print_generator_indent(file, state);
  fprintf(file, "printf(\"<invalid>\\n\");\n");
  --state->generator_indent;

  print_generator_indent(file, state);
  fprintf(file, "}\n");
}

static void generate_dump_member(FILE *file, struct symbol *symbol,
                                 generator_state_t *state) {
  if (is_struct_type(symbol)) {
    generate_dump_struct(file, symbol, state);
  } else if (is_union_type(symbol)) {
    generate_dump_union(file, symbol, state);
  } else if (is_array_type(symbol)) {
    generate_dump_array(file, symbol, state);
  } else if (is_enum_type(symbol)) {
    generate_dump_enum(file, symbol, state);
  } else if (is_ptr_type(symbol)) {
    generate_dump_ptr(file, symbol, state);
  } else if (is_int_type(symbol)) {
    generate_dump_int(file, symbol, state);
  } else if (is_float_type(symbol)) {
    generate_dump_int(file, symbol, state);
  } else {
    fprintf(file, "  printf(\"<%s>\\n\");\n", show_typename(symbol));
  }
}

void generate_impl_dump(FILE *file, const char *name, struct symbol *symbol) {
  if (symbol->type != SYM_NODE) {
    generator_state_t state;
    state_init(&state);

    fprintf(file, "void dump_%s(%s value) {\n", name, show_typename(symbol));

    path_buffer_push_value_str(&state.path, "value");
    ++state.generator_indent;

    generate_dump_member(file, symbol, &state);

    state_destroy(&state);
    fprintf(file, "}\n");
  } else {
    // if (symbol->namespace == NS_MACRO || symbol->namespace == NS_UNDEF)
    //   return;

    // printf("got symbol %s ; %s ; %d ; %s\n", get_type_name(symbol->type),
    //        show_ident(symbol->ident), is_type_type(symbol),
    //        show_ident(get_base_type(symbol)->ident));
  }
}

void generate_header_dump(FILE *file, const char *name, struct symbol *symbol) {
  if (symbol->type != SYM_NODE) {
    fprintf(file, "void dump_%s(%s value);\n", name, show_typename(symbol));
  } else {
    // if (symbol->namespace == NS_MACRO || symbol->namespace == NS_UNDEF)
    //   return;

    // printf("got symbol %s ; %s ; %d ; %s\n", get_type_name(symbol->type),
    //        show_ident(symbol->ident), is_type_type(symbol),
    //        show_ident(get_base_type(symbol)->ident));
  }
}
