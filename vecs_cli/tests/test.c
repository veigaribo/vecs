// This is meant to work with data from the `examples/test.vecs` file.
// TODO: Structure proper tests with proper requirements.

#include "vecs.h"
#include <stdio.h>

// Dyn arrays:

#define vecs_dyn_array_render _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6
#define vecs_dyn_array_render_init                                             \
  _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M4_initd41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_array_render_push                                             \
  _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M4_pushd41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_array_render_pop                                              \
  _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M3_popd41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_array_render_swap_remove                                      \
  _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_Mb_swap_removed41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_array_render_destroy                                          \
  _VECSs9_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M7_destroyd41d8cd98f00b204e9800998ecf8427e

void test_dyn_array() {
  struct vecs_component_render r1 = {.texture = 1};
  struct vecs_component_render r2 = {.texture = 2};
  struct vecs_component_render r3 = {.texture = 3};
  struct vecs_component_render r4 = {.texture = 4};
  struct vecs_component_render r5 = {.texture = 5};

  struct vecs_dyn_array_render arr;
  vecs_dyn_array_render_init(&arr, 0);
  printf("#r: %zu\n", arr.len);

  vecs_dyn_array_render_push(&arr, r1);
  vecs_dyn_array_render_push(&arr, r2);
  vecs_dyn_array_render_push(&arr, r3);
  vecs_dyn_array_render_swap_remove(&arr, 1);
  vecs_dyn_array_render_push(&arr, r4);
  vecs_dyn_array_render_push(&arr, r5);
  printf("#r: %zu\n", arr.len);

  size_t len = arr.len;
  for (size_t i = 0; i < len; ++i) {
    struct vecs_component_render r = vecs_dyn_array_render_pop(&arr);
    printf("{%zu}. texture: %d\n", i, r.texture);
  }

  printf("#r: %zu\n", arr.len);
  vecs_dyn_array_render_destroy(&arr);
}

// Dyn queues:

#define vecs_dyn_queue_mouse_click                                             \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138
#define vecs_dyn_queue_mouse_click_init                                        \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138_M4_initd41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_queue_mouse_click_enqueue                                     \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138_M7_enqueued41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_queue_mouse_click_enqueue                                     \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138_M7_enqueued41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_queue_mouse_click_dequeue                                     \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138_M7_dequeued41d8cd98f00b204e9800998ecf8427e
#define vecs_dyn_queue_mouse_click_destroy                                     \
  _VECSs9_dyn_queue056d765f5aaba614610625e0c4d57138_M7_destroyd41d8cd98f00b204e9800998ecf8427e

void test_dyn_queue() {
  struct vecs_event_mouse_click ev1 = {.x = 0.1, .y = 0.9, .button = 1};
  struct vecs_event_mouse_click ev2 = {.x = 0.2, .y = 0.8, .button = 2};
  struct vecs_event_mouse_click ev3 = {.x = 0.3, .y = 0.7, .button = 3};
  struct vecs_event_mouse_click ev4 = {.x = 0.4, .y = 0.6, .button = 4};

  struct vecs_dyn_queue_mouse_click q;
  vecs_dyn_queue_mouse_click_init(&q, 0);
  printf("#ev: %zu\n", q.len);

  vecs_dyn_queue_mouse_click_enqueue(&q, ev1);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev2);
  vecs_dyn_queue_mouse_click_dequeue(&q);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev3);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev4);
  printf("#ev: %zu\n", q.len);

  size_t len = q.len;
  for (size_t i = 0; i < len; ++i) {
    struct vecs_event_mouse_click ev = vecs_dyn_queue_mouse_click_dequeue(&q);
    printf("{%zu}. x: %f, y: %f, btn: %d\n", i, ev.x, ev.y, ev.button);
  }

  printf("#ev: %zu\n", q.len);
  vecs_dyn_queue_mouse_click_destroy(&q);
}

// Sparse dyn arrays:

#define vecs_sparse_dyn_array_render                                           \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6
#define vecs_sparse_dyn_array_render_init                                      \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M4_initd41d8cd98f00b204e9800998ecf8427e
#define vecs_sparse_dyn_array_render_push                                      \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M4_pushd41d8cd98f00b204e9800998ecf8427e
#define vecs_sparse_dyn_array_render_get                                       \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M3_getd41d8cd98f00b204e9800998ecf8427e
#define vecs_sparse_dyn_array_render_remove                                    \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M6_removed41d8cd98f00b204e9800998ecf8427e
#define vecs_sparse_dyn_array_render_destroy                                   \
  _VECSs10_sparse_dyn_array617b255d8c9a0e7c4b64e9a69b89c5a6_M7_destroyd41d8cd98f00b204e9800998ecf8427e

void test_sparse_dyn_array() {
  struct vecs_component_render r1 = {.texture = 1};
  struct vecs_component_render r2 = {.texture = 2};
  struct vecs_component_render r3 = {.texture = 3};
  struct vecs_component_render r4 = {.texture = 4};
  struct vecs_component_render r5 = {.texture = 5};

  struct vecs_sparse_dyn_array_render arr;
  vecs_sparse_dyn_array_render_init(&arr, 0);
  printf("#r: %zu\n", arr.len);

  size_t ri[5] = {0}, rg[5] = {0};
  vecs_sparse_dyn_array_render_push(&arr, r1, &ri[0], &rg[0]);
  vecs_sparse_dyn_array_render_push(&arr, r2, &ri[1], &rg[1]);
  vecs_sparse_dyn_array_render_push(&arr, r3, &ri[2], &rg[2]);
  vecs_sparse_dyn_array_render_remove(&arr, ri[1]);
  vecs_sparse_dyn_array_render_push(&arr, r4, &ri[3], &rg[3]);
  vecs_sparse_dyn_array_render_push(&arr, r5, &ri[4], &rg[4]);
  vecs_sparse_dyn_array_render_remove(&arr, ri[4]);
  printf("#r: %zu\n", arr.len);

  for (size_t i = 0; i < 5; ++i) {
    struct vecs_component_render *r =
        vecs_sparse_dyn_array_render_get(&arr, ri[i], rg[i]);

    if (r != NULL) {
      printf("{%zu/%zu}. texture: %d\n", ri[i], rg[i], r->texture);
    } else {
      printf("{%zu/%zu}. null\n", ri[i], rg[i]);
    }
  }

  printf("#r: %zu\n", arr.len);
  vecs_sparse_dyn_array_render_destroy(&arr);
}

// Hash dyn arrays:

#define vecs_hash_dyn_array_id_pair                                            \
  _VECSs12_hash_dyn_array_aux229ae29ca35818fa89308d8ed0a1aa92
#define vecs_hash_dyn_array_id                                                 \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92
#define vecs_hash_dyn_array_id_init                                            \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92_M4_initd41d8cd98f00b204e9800998ecf8427e
#define vecs_hash_dyn_array_id_add                                             \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92_M3_addd41d8cd98f00b204e9800998ecf8427e
#define vecs_hash_dyn_array_id_get                                             \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92_M3_getd41d8cd98f00b204e9800998ecf8427e
#define vecs_hash_dyn_array_id_remove                                          \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92_M6_removed41d8cd98f00b204e9800998ecf8427e
#define vecs_hash_dyn_array_id_destroy                                         \
  _VECSse_hash_dyn_array229ae29ca35818fa89308d8ed0a1aa92_M7_destroyd41d8cd98f00b204e9800998ecf8427e

void test_hash_dyn_array() {
  struct vecs_sparse_array_id c1 = {.index = 0, .gen = 4};
  struct vecs_sparse_array_id c2 = {.index = 1, .gen = 3};
  struct vecs_sparse_array_id c3 = {.index = 2, .gen = 2};
  struct vecs_sparse_array_id c4 = {.index = 3, .gen = 1};
  struct vecs_sparse_array_id c5 = {.index = 4, .gen = 0};

  struct vecs_hash_dyn_array_id map;
  vecs_hash_dyn_array_id_init(&map);
  printf("#r: %zu\n", map.len);

  struct vecs_sparse_array_id rc2, rc5;
  vecs_hash_dyn_array_id_add(&map, 0, c1);
  vecs_hash_dyn_array_id_add(&map, 1, c2);
  vecs_hash_dyn_array_id_add(&map, 2, c3);
  vecs_hash_dyn_array_id_remove(&map, 1, &rc2);
  vecs_hash_dyn_array_id_add(&map, 3, c4);
  vecs_hash_dyn_array_id_add(&map, 4, c5);
  vecs_hash_dyn_array_id_remove(&map, 4, &rc5);
  printf("#r: %zu\n", map.len);

  for (size_t i = 0; i < 5; ++i) {
    struct vecs_sparse_array_id c;
    bool found = vecs_hash_dyn_array_id_get(&map, i, &c);

    if (found) {
      printf("{%zu}. index: %zu, gen: %zu\n", i, c.index, c.gen);
    } else {
      printf("{%zu}. null\n", i);
    }
  }

  printf("#r: %zu\n", map.len);
  vecs_hash_dyn_array_id_destroy(&map);
}

int main() {
  printf("# Dyn arrays:\n");
  test_dyn_array();
  printf("\n");
  printf("# Dyn queues:\n");
  test_dyn_queue();
  printf("\n");
  printf("# Sparse dyn arrays:\n");
  test_sparse_dyn_array();
  printf("\n");
  printf("# Hash dyn arrays:\n");
  test_hash_dyn_array();
}
