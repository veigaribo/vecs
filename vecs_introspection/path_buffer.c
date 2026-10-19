#include "path_buffer.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void path_buffer_init(path_buffer_t *self) {
  self->head = NULL;
  self->length = 0;
  self->capacity = 0;
}

static void path_buffer_grow(path_buffer_t *self) {
  if (self->capacity == 0) {
    self->capacity = 2;
    self->head = malloc(self->capacity * sizeof(path_entry_t));
  } else {
    self->capacity = self->capacity + (self->capacity >> 1);
    self->head = realloc(self->head, self->capacity * sizeof(path_entry_t));
  }
}

static void path_buffer_push(path_buffer_t *self, path_entry_t entry) {
  uint32_t new_length = self->length + 1;

  if (new_length > self->capacity) {
    path_buffer_grow(self);
  }

  self->head[self->length] = entry;
  self->length = new_length;
}

void path_buffer_push_leaf_str(path_buffer_t *self, const char *name) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_LEAF,
                                        .data_type = PATH_ENTRY_STR,
                                        .str = (char *)name});
}

void path_buffer_push_value_str(path_buffer_t *self, const char *name) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_VALUE,
                                        .data_type = PATH_ENTRY_STR,
                                        .str = (char *)name});
}

void path_buffer_push_ptr_str(path_buffer_t *self, const char *name) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_PTR,
                                        .data_type = PATH_ENTRY_STR,
                                        .str = (char *)name});
}

void path_buffer_push_indexed_str(path_buffer_t *self, const char *name) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_INDEXED,
                                        .data_type = PATH_ENTRY_STR,
                                        .str = (char *)name});
}

void path_buffer_push_leaf_int(path_buffer_t *self, size_t integer) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_LEAF,
                                        .data_type = PATH_ENTRY_INT,
                                        .integer = integer});
}

void path_buffer_push_value_int(path_buffer_t *self, size_t integer) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_VALUE,
                                        .data_type = PATH_ENTRY_INT,
                                        .integer = integer});
}

void path_buffer_push_ptr_int(path_buffer_t *self, size_t integer) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_PTR,
                                        .data_type = PATH_ENTRY_INT,
                                        .integer = integer});
}

void path_buffer_push_indexed_int(path_buffer_t *self, size_t integer) {
  path_buffer_push(self, (path_entry_t){.conn_type = PATH_ENTRY_INDEXED,
                                        .data_type = PATH_ENTRY_INT,
                                        .integer = integer});
}

void path_buffer_pop(path_buffer_t *self) {
  path_buffer_truncate(self, self->length - 1);
}

void path_buffer_truncate(path_buffer_t *self, uint32_t length) {
  assert(length <= self->length);
  self->length = length;
}

static inline void path_conn_pre_dump(FILE *file, path_entry_conn_type_t type) {
  switch (type) {
  case PATH_ENTRY_LEAF:
    assert(false);
    break;
  case PATH_ENTRY_VALUE:
    fprintf(file, ".");
    break;
  case PATH_ENTRY_PTR:
    fprintf(file, "->");
    break;
  case PATH_ENTRY_INDEXED:
    fprintf(file, "[");
    break;
  }
}

static inline void path_conn_post_dump(FILE *file,
                                       path_entry_conn_type_t type) {
  switch (type) {
  case PATH_ENTRY_INDEXED:
    fprintf(file, "]");
    break;
  case PATH_ENTRY_LEAF:
  case PATH_ENTRY_VALUE:
  case PATH_ENTRY_PTR:;
  }
}

static inline void path_entry_data_dump(FILE *file, path_entry_t entry) {
  switch (entry.data_type) {
  case PATH_ENTRY_STR:
    fprintf(file, "%s", entry.str);
    break;
  case PATH_ENTRY_INT:
    fprintf(file, "%zu", entry.integer);
    break;
  }
}

void path_buffer_dump(FILE *file, path_buffer_t *self) {
  if (self->length == 0)
    return;

  path_entry_data_dump(file, self->head[0]);

  for (uint32_t i = 1; i < self->length; ++i) {
    path_entry_conn_type_t pre_conn = self->head[i - 1].conn_type;
    path_conn_pre_dump(file, pre_conn);
    path_entry_data_dump(file, self->head[i]);
    path_conn_post_dump(file, pre_conn);
  }
}

void path_buffer_destroy(path_buffer_t *self) {
  if (self->head != NULL) {
    free(self->head);
  }
}
