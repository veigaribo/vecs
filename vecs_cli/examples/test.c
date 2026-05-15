#include "vecs.h"
#include <stdio.h>

void move(struct vecs_engine *engine, struct vecs_node_move node,
          struct vecs_event_mouse_click event) {
  struct vecs_component_transform *t =
      vecs_state_main_node_move_get_transform(engine, node);

  printf(".x: %f, .y: %f, .btn: %d, .x: %f, .y: %f\n", event.x, event.y,
         event.button, t->x, t->y);
}

void render(struct vecs_engine *engine, struct vecs_node_render node,
            struct vecs_event_frame event) {
  struct vecs_component_transform *t =
      vecs_state_main_node_render_get_transform(engine, node);
  struct vecs_component_render *r =
      vecs_state_main_node_render_get_render(engine, node);

  printf(
      ".delta: %f, .runtime: %f, .frame: %lu, .x: %f, .y: %f, .texture: %d\n",
      event.delta, event.runtime, event.frame, t->x, t->y, r->texture);
}

int main() {
  struct vecs_engine e = {0};

  struct vecs_id ent1 = vecs_add_entity(&e);

  struct vecs_component_transform t1 = {.x = 1.5, .y = 3.0};
  vecs_main_add_component_transform(&e, ent1, t1);

  struct vecs_component_render r1 = {.texture = 6};
  vecs_main_add_component_render(&e, ent1, r1);
  printf("add render\n");

  struct vecs_event_mouse_click mc1 = {.x = 0.5, .y = 0.328, .button = 2};
  vecs_emit_mouse_click(&e, mc1);

  struct vecs_event_frame f1 = {.delta = 0.16, .runtime = 0.16, .frame = 1};
  vecs_emit_frame(&e, f1);

  printf("run\n");
  vecs_run_state_main(&e);

  vecs_emit_frame(&e, f1);
  vecs_main_remove_component_render(&e, ent1);
  printf("remove render\n");
  printf("run\n");
  vecs_run_state_main(&e);

  vecs_emit_frame(&e, f1);
  vecs_main_add_component_render(&e, ent1, r1);
  printf("add render\n");
  printf("run\n");
  vecs_run_state_main(&e);
}
