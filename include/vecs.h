#ifndef VECS_VECS_H
#define VECS_VECS_H

#include <stdbool.h>
#include <stdint.h>

typedef struct vecs_engine vecs_engine_t;

// Index and gen in one struct. Used for permanent storage of entities and
// component references.
typedef struct vecs_id {
  uint32_t index;
  uint32_t gen;
} vecs_id_t;

// Used to temporarily refer to entities and components that have not yet
// been persisted, and so do not yet posses a permanent `vecs_id_t`.
typedef struct vecs_tmp_id {
  uint32_t index;
} vecs_tmp_id_t;

static const vecs_id_t vecs_id_invalid = {.index = UINT32_MAX,
                                          .gen = UINT32_MAX};

inline static bool vecs_id_is_invalid(vecs_id_t id) {
  return id.index == UINT32_MAX && id.gen == UINT32_MAX;
}

inline static vecs_id_t vecs_id_max() { return vecs_id_invalid; }

inline static bool vecs_id_eq(vecs_id_t a, vecs_id_t b) {
  return a.index == b.index && a.gen == b.gen;
}

inline static uint8_t vecs_id_cmp(vecs_id_t a, vecs_id_t b) {
  uint64_t a64 = ((uint64_t)a.index << 32) | a.gen;
  uint64_t b64 = ((uint64_t)b.index << 32) | b.gen;
  return a64 == b64 ? 0 : a64 < b64 ? -1 : 1;
}

#endif // !VECS_VECS_H
