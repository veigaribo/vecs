#include "vecs.h"
#include <stdio.h>

void move(vecs_engine_t *engine, vecs_node_move_t node,
          vecs_event_mouse_click_t event) {
  vecs_component_transform_t *t = vecs_node_move_get_transform(engine, node);

  printf(".x: %f, .y: %f, .btn: %d, .x: %f, .y: %f\n", event.x, event.y,
         event.button, t->x, t->y);

  vecs_node_something_array_t somethings = vecs_nodes_something(engine);

  for (size_t i = 0; i < somethings.len; ++i) {
    vecs_node_something_t something = somethings.items[i];
    vecs_component_render_t *r =
        vecs_node_something_get_render(engine, something);
    printf("something .texture: %d\n", r->texture);
  }
}

void render(vecs_engine_t *engine, vecs_node_render_t node,
            vecs_event_frame_t event) {
  vecs_component_transform_t *t = vecs_node_render_get_transform(engine, node);
  vecs_component_render_t *r = vecs_node_render_get_render(engine, node);

  printf(
      ".delta: %f, .runtime: %f, .frame: %lu, .x: %f, .y: %f, .texture: %d\n",
      event.delta, event.runtime, event.frame, t->x, t->y, r->texture);
}

void click(vecs_engine_t *engine, vecs_node_click_t node,
           vecs_event_mouse_click_t event) {
  vecs_component_layout_t *l = vecs_node_click_get_layout(engine, node);
  vecs_component_render_t *r = vecs_node_click_get_render(engine, node);

  printf(".x: %f, .y: %f, .btn: %d, .mode: %d, .texture: %d\n", event.x,
         event.x, event.button, l->mode, r->texture);
}

int main() {
  vecs_engine_t e;
  vecs_init(&e);

  vecs_id_t ent1 = vecs_add_entity(&e);

  vecs_component_transform_t t1 = {.x = 1.5, .y = 3.0};
  vecs_main_add_component_transform(&e, ent1, t1);

  vecs_main_disable_component_transform(&e, ent1);
  vecs_main_enable_component_transform(&e, ent1);

  vecs_component_render_t r1 = {.texture = 6};
  vecs_main_add_component_render(&e, ent1, r1);

  vecs_component_layout_t l1 = {.mode = 4};
  vecs_main_add_component_layout(&e, ent1, l1);

  vecs_event_mouse_click_t mc1 = {.x = 0.5, .y = 0.328, .button = 2};
  vecs_emit_mouse_click(&e, mc1);

  vecs_event_frame_t f1 = {.delta = 0.16, .runtime = 0.16, .frame = 1};
  vecs_emit_frame(&e, f1);

  printf("  run\n");
  vecs_run_state_main(&e);

  printf("  remove render\n");
  vecs_emit_mouse_click(&e, mc1);
  vecs_emit_frame(&e, f1);
  vecs_main_remove_component_render(&e, ent1);
  vecs_run_state_main(&e);

  printf("  add render\n");
  vecs_emit_mouse_click(&e, mc1);
  vecs_emit_frame(&e, f1);
  vecs_main_add_component_render(&e, ent1, r1);
  vecs_run_state_main(&e);

  printf("  main -> menu\n");
  vecs_state_main_to_menu(&e);

  vecs_emit_mouse_click(&e, mc1);
  vecs_emit_frame(&e, f1);
  vecs_run_state_menu(&e);

  printf("  menu -> main\n");
  vecs_state_menu_to_main(&e);

  vecs_emit_mouse_click(&e, mc1);
  vecs_emit_frame(&e, f1);
  vecs_run_state_main(&e);

  vecs_destroy(&e);
}
