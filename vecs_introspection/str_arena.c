#include "str_arena.h"
#include <assert.h>
#include <stdalign.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>

static void str_arena_chunk_init(str_arena_chunk_t *self) {
  uint8_t *start = malloc(ARENA_CHUNK_SIZE);
  self->start = start;
  self->end = start;
  self->used = 0;
  self->next = NULL;
}

void str_arena_init(str_arena_t *self) {
  str_arena_chunk_t *chunk = malloc(sizeof(str_arena_chunk_t));
  str_arena_chunk_init(chunk);
  self->start_chunk = chunk;
  self->end_chunk = chunk;
}

static str_arena_chunk_t *arena_grow(str_arena_t *self) {
  str_arena_chunk_t *chunk = malloc(sizeof(str_arena_chunk_t));
  str_arena_chunk_init(chunk);
  self->end_chunk = chunk;
  return chunk;
}

static const uint8_t HIGHEST_BIT = (1 << 7);

// Little endian
static void store_header(uint8_t *dst, uint16_t value) {
  dst[0] = value & 0x00FF;
  dst[1] = (value & 0xFF00) >> 8;

  // We add this bit that would never occur in an ASCII character to prevent
  // trying to interpret a character as a header (we don't need the full 16
  // bits so it's fine)
  dst[1] |= HIGHEST_BIT;
}

static uint16_t read_header(uint8_t *header) {
  uint8_t low = header[0];
  uint8_t high = header[1];
  assert((high & HIGHEST_BIT) != 0);

  return (((uint16_t)(high & ~HIGHEST_BIT)) << 8) | low;
}

char *str_arena_alloc(str_arena_t *self, size_t size) {
  assert(size <= ARENA_CHUNK_SIZE);
  str_arena_chunk_t *chunk = self->end_chunk;

  if (chunk->used + size + 2 > ARENA_CHUNK_SIZE) {
    chunk = arena_grow(self);
  }

  uint8_t *header = chunk->end;
  char *spot = (char *)(header + 2);

  store_header(header, chunk->used);
  chunk->used += size + 2;
  chunk->end += size + 2;

  return spot;
}

char *str_arena_push_nt(str_arena_t *self, const char *str) {
  str_arena_chunk_t *chunk = self->end_chunk;
  size_t remaining = ARENA_CHUNK_SIZE - chunk->used - 2;

  uint8_t *header = chunk->end;
  char *spot = (char *)(header + 2);

try_fit:
  for (size_t i = 0; i < remaining; ++i) {
    spot[i] = str[i];

    if (str[i] == '\0') {
      store_header(header, chunk->used);
      chunk->used += i + 1 + 2; // + null + header
      chunk->end += i + 1 + 2;  // + null + header
      return spot;
    }
  }

  chunk = arena_grow(self);
  remaining = ARENA_CHUNK_SIZE - sizeof(*header);
  header = chunk->start;
  spot = (char *)(header + 2);
  goto try_fit;
}

#define AsNumber(X) ((uintptr_t)X)

void str_arena_pop(str_arena_t *self, const char *str) {
  assert(str != NULL && AsNumber(str) > 2);
  uint16_t header = read_header((uint8_t *)str - 2);
  assert(header < ARENA_CHUNK_SIZE);

  str_arena_chunk_t *current = self->start_chunk;
  while (!(AsNumber(str) > AsNumber(current->start) &&
           AsNumber(str) < AsNumber(current->end))) {
    current = current->next;
    assert(current != NULL);
  }

  uint16_t diff = current->used - header;
  assert(diff < ARENA_CHUNK_SIZE);

  current->used = header;
  current->end -= diff;
}

static void arena_chunk_destroy(str_arena_chunk_t *self) {
  if (self->start != NULL) {
    free(self->start);
  }

  self->start = self->end = 0;
}

void str_arena_destroy(str_arena_t *self) {
  if (self->start_chunk == NULL) {
    return;
  }

  str_arena_chunk_t *current = self->start_chunk, *next;

  do {
    next = current->next;
    arena_chunk_destroy(current);
    free(current);
  } while (next != NULL);
}
