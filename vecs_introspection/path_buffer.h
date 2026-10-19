#ifndef VECS_INTROSPECTION_PATH_BUFFER
#define VECS_INTROSPECTION_PATH_BUFFER

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

// Paths are the string sequences like `a.b->c[1][2].d`. The entries in this
// case would be (VALUE, `a`) -> (PTR, `b`) -> (INDEXED, `c`) -> (INDEXED, 1) ->
// (VALUE, 2) -> (LEAF, `d`).

// The conn type determines what kind of indexing the current entry supports. If
// I'm at `a.b` and I have conn type VALUE, children would become e.g. `a.b.c`.
// If instead I'm PTR, they would become `a.b->c`. Indexed would make `a.b[c]`.
// Important: the `conn` type of an entry will affect the relation of it and its
// children; not its parent.

typedef enum {
  PATH_ENTRY_LEAF,
  PATH_ENTRY_VALUE,   // a.b
  PATH_ENTRY_PTR,     // a->b
  PATH_ENTRY_INDEXED, // a[b]
} path_entry_conn_type_t;

// The data type of an entry will typically be `str`, except when indexind an
// array like `a.b[1]`.

typedef enum {
  PATH_ENTRY_STR, // a.b
  PATH_ENTRY_INT, // a[1]
} path_entry_data_type_t;

typedef struct {
  path_entry_conn_type_t conn_type;
  path_entry_data_type_t data_type;

  union {
    char *str;
    size_t integer;
  };
} path_entry_t;

typedef struct {
  path_entry_t *head;
  uint32_t length;
  uint32_t capacity;
} path_buffer_t;

void path_buffer_init(path_buffer_t *self);

void path_buffer_push_leaf_str(path_buffer_t *self, const char *name);
void path_buffer_push_value_str(path_buffer_t *self, const char *name);
void path_buffer_push_ptr_str(path_buffer_t *self, const char *name);
void path_buffer_push_indexed_str(path_buffer_t *self, const char *name);

void path_buffer_push_leaf_int(path_buffer_t *self, size_t integer);
void path_buffer_push_value_int(path_buffer_t *self, size_t integer);
void path_buffer_push_ptr_int(path_buffer_t *self, size_t integer);
void path_buffer_push_indexed_int(path_buffer_t *self, size_t integer);

void path_buffer_pop(path_buffer_t *self);
void path_buffer_truncate(path_buffer_t *self, uint32_t length);

void path_buffer_dump(FILE *file, path_buffer_t *self);
void path_buffer_destroy(path_buffer_t *self);

#endif // !VECS_INTROSPECTION_PATH_BUFFER
