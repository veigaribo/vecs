// This is meant to work with data from the `examples/test.vecs` file.
// TODO: Structure proper tests with proper requirements.

#include "vecs.h"
#include <stdio.h>

// Dyn arrays:

#define vecs_dyn_array_render_t _VECSs9_dyn_array8e730cc2bae447f9_t
#define vecs_dyn_array_render_init                                             \
  _VECSs9_dyn_array8e730cc2bae447f9_t_M4_init0000000000000000
#define vecs_dyn_array_render_push                                             \
  _VECSs9_dyn_array8e730cc2bae447f9_t_M4_push0000000000000000
#define vecs_dyn_array_render_pop                                              \
  _VECSs9_dyn_array8e730cc2bae447f9_t_M3_pop0000000000000000
#define vecs_dyn_array_render_swap_remove                                      \
  _VECSs9_dyn_array8e730cc2bae447f9_t_Mb_swap_remove0000000000000000
#define vecs_dyn_array_render_destroy                                          \
  _VECSs9_dyn_array8e730cc2bae447f9_t_M7_destroy0000000000000000

void test_dyn_array() {
  vecs_component_render_t r1 = {.texture = 1};
  vecs_component_render_t r2 = {.texture = 2};
  vecs_component_render_t r3 = {.texture = 3};
  vecs_component_render_t r4 = {.texture = 4};
  vecs_component_render_t r5 = {.texture = 5};

  vecs_dyn_array_render_t arr;
  vecs_dyn_array_render_init(&arr, 0);
  printf("#r: %d\n", arr.len);

  vecs_dyn_array_render_push(&arr, r1);
  vecs_dyn_array_render_push(&arr, r2);
  vecs_dyn_array_render_push(&arr, r3);
  vecs_dyn_array_render_swap_remove(&arr, 1);
  vecs_dyn_array_render_push(&arr, r4);
  vecs_dyn_array_render_push(&arr, r5);
  printf("#r: %d\n", arr.len);

  size_t len = arr.len;
  for (size_t i = 0; i < len; ++i) {
    vecs_component_render_t r = vecs_dyn_array_render_pop(&arr);
    printf("{%zu}. texture: %d\n", i, r.texture);
  }

  printf("#r: %d\n", arr.len);
  vecs_dyn_array_render_destroy(&arr);
}

// Dyn queues:

#define vecs_dyn_queue_mouse_click_t _VECSs9_dyn_queue738a6f05d1537108_t
#define vecs_dyn_queue_mouse_click_init                                        \
  _VECSs9_dyn_queue738a6f05d1537108_t_M4_init0000000000000000
#define vecs_dyn_queue_mouse_click_enqueue                                     \
  _VECSs9_dyn_queue738a6f05d1537108_t_M7_enqueue0000000000000000
#define vecs_dyn_queue_mouse_click_dequeue                                     \
  _VECSs9_dyn_queue738a6f05d1537108_t_M7_dequeue0000000000000000
#define vecs_dyn_queue_mouse_click_destroy                                     \
  _VECSs9_dyn_queue738a6f05d1537108_t_M7_destroy0000000000000000

void test_dyn_queue() {
  vecs_event_mouse_click_t ev1 = {.x = 0.1, .y = 0.9, .button = 1};
  vecs_event_mouse_click_t ev2 = {.x = 0.2, .y = 0.8, .button = 2};
  vecs_event_mouse_click_t ev3 = {.x = 0.3, .y = 0.7, .button = 3};
  vecs_event_mouse_click_t ev4 = {.x = 0.4, .y = 0.6, .button = 4};

  vecs_dyn_queue_mouse_click_t q;
  vecs_dyn_queue_mouse_click_init(&q, 0);
  printf("#ev: %d\n", q.len);

  vecs_dyn_queue_mouse_click_enqueue(&q, ev1);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev2);
  vecs_dyn_queue_mouse_click_dequeue(&q);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev3);
  vecs_dyn_queue_mouse_click_enqueue(&q, ev4);
  printf("#ev: %d\n", q.len);

  size_t len = q.len;
  for (size_t i = 0; i < len; ++i) {
    vecs_event_mouse_click_t ev = vecs_dyn_queue_mouse_click_dequeue(&q);
    printf("{%zu}. x: %f, y: %f, btn: %d\n", i, ev.x, ev.y, ev.button);
  }

  printf("#ev: %d\n", q.len);
  vecs_dyn_queue_mouse_click_destroy(&q);
}

// Sparse dyn arrays:

#define vecs_sparse_dyn_array_render_t                                         \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t
#define vecs_sparse_dyn_array_render_init                                      \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t_M4_init0000000000000000
#define vecs_sparse_dyn_array_render_push                                      \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t_M4_push0000000000000000
#define vecs_sparse_dyn_array_render_get                                       \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t_M3_get0000000000000000
#define vecs_sparse_dyn_array_render_remove                                    \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t_M6_remove0000000000000000
#define vecs_sparse_dyn_array_render_destroy                                   \
  _VECSs10_sparse_dyn_array8e730cc2bae447f9_t_M7_destroy0000000000000000

void test_sparse_dyn_array() {
  vecs_component_render_t r1 = {.texture = 1};
  vecs_component_render_t r2 = {.texture = 2};
  vecs_component_render_t r3 = {.texture = 3};
  vecs_component_render_t r4 = {.texture = 4};
  vecs_component_render_t r5 = {.texture = 5};

  vecs_sparse_dyn_array_render_t arr;
  vecs_sparse_dyn_array_render_init(&arr, 0);
  printf("#r: %d\n", arr.len);

  uint32_t ri[5] = {0}, rg[5] = {0};
  vecs_sparse_dyn_array_render_push(&arr, r1, &ri[0], &rg[0]);
  vecs_sparse_dyn_array_render_push(&arr, r2, &ri[1], &rg[1]);
  vecs_sparse_dyn_array_render_push(&arr, r3, &ri[2], &rg[2]);
  vecs_sparse_dyn_array_render_remove(&arr, ri[1]);
  vecs_sparse_dyn_array_render_push(&arr, r4, &ri[3], &rg[3]);
  vecs_sparse_dyn_array_render_push(&arr, r5, &ri[4], &rg[4]);
  vecs_sparse_dyn_array_render_remove(&arr, ri[4]);
  printf("#r: %d\n", arr.len);

  for (size_t i = 0; i < 5; ++i) {
    vecs_component_render_t *r =
        vecs_sparse_dyn_array_render_get(&arr, ri[i], rg[i]);

    if (r != NULL) {
      printf("{%d/%d}. texture: %d\n", ri[i], rg[i], r->texture);
    } else {
      printf("{%d/%d}. null\n", ri[i], rg[i]);
    }
  }

  printf("#r: %d\n", arr.len);
  vecs_sparse_dyn_array_render_destroy(&arr);
}

// Hash dyn arrays:

#define vecs_hash_dyn_array_id_pair_t                                          \
  _VECSs12_hash_dyn_array_aux37f1b91012c3886a_t
#define vecs_hash_dyn_array_id_t _VECSse_hash_dyn_array37f1b91012c3886a_t
#define vecs_hash_dyn_array_id_init                                            \
  _VECSse_hash_dyn_array37f1b91012c3886a_t_M4_init0000000000000000
#define vecs_hash_dyn_array_id_add                                             \
  _VECSse_hash_dyn_array37f1b91012c3886a_t_M3_add0000000000000000
#define vecs_hash_dyn_array_id_get                                             \
  _VECSse_hash_dyn_array37f1b91012c3886a_t_M3_get0000000000000000
#define vecs_hash_dyn_array_id_remove                                          \
  _VECSse_hash_dyn_array37f1b91012c3886a_t_M6_remove0000000000000000
#define vecs_hash_dyn_array_id_destroy                                         \
  _VECSse_hash_dyn_array37f1b91012c3886a_t_M7_destroy0000000000000000

void test_hash_dyn_array() {
  vecs_id_t c[] = {
      {.index = 0, .gen = 4}, {.index = 1, .gen = 3}, {.index = 2, .gen = 2},
      {.index = 3, .gen = 1}, {.index = 4, .gen = 0},
  };

  vecs_hash_dyn_array_id_t map;
  vecs_hash_dyn_array_id_init(&map);
  printf("#r: %d\n", map.len);

  uint32_t rc2, rc5;
  vecs_hash_dyn_array_id_add(&map, c[0], 0);
  vecs_hash_dyn_array_id_add(&map, c[1], 1);
  vecs_hash_dyn_array_id_add(&map, c[2], 2);
  vecs_hash_dyn_array_id_remove(&map, c[1], &rc2);
  vecs_hash_dyn_array_id_add(&map, c[3], 3);
  vecs_hash_dyn_array_id_add(&map, c[4], 4);
  vecs_hash_dyn_array_id_remove(&map, c[4], &rc5);
  printf("#r: %d\n", map.len);

  for (size_t i = 0; i < 5; ++i) {
    uint32_t v;
    bool found = vecs_hash_dyn_array_id_get(&map, c[i], &v);

    if (found) {
      printf("{%zu}. v: %d\n", i, v);
    } else {
      printf("{%zu}. null\n", i);
    }
  }

  printf("#r: %d\n", map.len);
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

void move(vecs_engine_t *engine, vecs_node_move_t node,
          vecs_event_mouse_click_t event) {}

void render(vecs_engine_t *engine, vecs_node_render_t node,
            vecs_event_frame_t event) {}

void click(vecs_engine_t *engine, vecs_node_click_t node,
            vecs_event_mouse_click_t event) {}
