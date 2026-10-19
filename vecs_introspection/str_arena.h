#ifndef VECS_INTROSPECTION_STR_ARENA
#define VECS_INTROSPECTION_STR_ARENA

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

// This arena is used to store null-terminated strings. We store a header before
// each string that contains the offset from `chunk.start` of the previous
// string. This allows us to remove them from the end as well, which makes it
// behave as a stack.

// The strings are stored in a linked list of chunks. I understand that better
// alternatives would require OS-specific code.

#define ARENA_CHUNK_SIZE 4096

typedef struct str_arena_chunk {
  uint8_t *start;
  uint8_t *end;

  uint16_t used;
  struct str_arena_chunk *next;
} str_arena_chunk_t;

// `used` can't exceed 15 bits. See the static functions `store_header` and
// `read_header`.
static_assert((1 << 15) > ARENA_CHUNK_SIZE,
              "`chunk->used` cannot contain ARENA_CHUNK_SIZE");

typedef struct {
  str_arena_chunk_t *start_chunk;
  str_arena_chunk_t *end_chunk;
} str_arena_t;

void str_arena_init(str_arena_t *self);
char *str_arena_push_nt(str_arena_t *self, const char *str);
char *str_arena_alloc(str_arena_t *self,
                      size_t size); // Size *including* the null terminator
void str_arena_pop(str_arena_t *self, const char *str);
void str_arena_destroy(str_arena_t *self);

#endif // !VECS_INTROSPECTION_STR_ARENA
