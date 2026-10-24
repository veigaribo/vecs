#include "vecslib.h"
#include <limits.h>
#include <stdio.h>
#include <string.h>

#define VecsPrintfStringArgs(String)                                           \
  (int)((String).length > INT_MAX ? strlen("TOO LONG") : (String).length),     \
      (String).length > INT_MAX ? "TOO LONG" : ((char *)(String).str)

#define VecsTypeIter(Type, Var)                                                \
  for (vecs_type_iter_t Var = vecs_type_iter(Type);                            \
       !vecs_type_iter_is_done(&Var); vecs_type_iter_next(&Var))

char text[] =
    ""
    "include <GL/gl.h>;\n"
    "include \"components/collider.h\";\n"
    "include \"components/shepherd_anim_spin.h\";\n"
    "include \"components/transform.h\";\n"
    "include \"events/mouse_move.h\";\n"
    "include \"events/mouse_click.h\";\n"
    "include \"events/physics_frame.h\";\n"
    "include \"physics/movement.h\";\n"
    "include \"frame_arena.h\";\n"
    "\n"
    "// Global\n"
    "global movements movement_array_t;\n"
    "global walls wall_body_array_t;\n"
    "global sheep sheep_body_array_t;\n"
    "global frame_arena frame_arena_t;\n"
    "\n"
    "// Components\n"
    "component id vecs_id_t;\n"
    "component transform c_transform_t;\n"
    "component shepherd_anim_spin c_shepherd_anim_spin_t;\n"
    "\n"
    "methods id { clone clone_id };\n"
    "methods shepherd_anim_spin { clone clone_anim };\n"
    "\n"
    "component sheep_renderer;\n"
    "component shepherd_renderer;\n"
    "\n"
    "component shepherd;\n"
    "component sheep;\n"
    "component debug_sphere_collider;\n"
    "component debug_h_capsule_collider;\n"
    "\n"
    "// Events\n"
    "event physics_frame event_physics_frame_t;\n"
    "event mouse_move event_mouse_move_t;\n"
    "event mouse_click event_mouse_click_t;\n"
    "event render_only;\n"
    "\n"
    "// Systems\n"
    "system spawn on mouse_click;\n"
    "\n"
    "system sheep_process on physics_frame for { id, sheep, transform "
    "\n"
    "};\n"
    "system sheep_move on physics_frame for { sheep, transform };\n"
    "\n"
    "system shepherd_move on mouse_move for { shepherd, transform };\n"
    "system shepherd_anim_spin for { transform, shepherd_anim_spin "
    "};\n"
    "// system collide { id, collider, transform };\n"
    "\n"
    "// system render_sphere_collider_pre; // Debug\n"
    "// system render_sphere_collider { transform, collider, "
    "debug_sphere_collider }; // Debug\n"
    "// system render_h_capsule_collider_pre; // Debug\n"
    "// system render_h_capsule_collider { transform, collider, "
    "debug_h_capsule_collider }; // Debug\n"
    "\n"
    "system render_sheep_pre;\n"
    "system render_sheep for { transform, sheep_renderer };\n"
    "\n"
    "system render_shepherd_pre;\n"
    "system render_shepherd for { transform, shepherd_renderer };\n"
    "\n"
    "// Debug\n"
    "system render_shepherd_collider;\n"
    "system render_shepherd_collider_only on render_only;\n"
    "system render_sheep_collider;\n"
    "system render_sheep_collider_only on render_only;\n"
    "system render_wall_collider;\n"
    "system render_wall_collider_only on render_only;\n"
    "\n"
    "// States\n"
    "state playing {\n"
    "  systems {\n"
    "    { shepherd_move, sheep_process, spawn, shepherd_anim_spin "
    "},\n"
    "    { sheep_move },\n"
    "    // { collide },\n"
    "    {\n"
    "      render_sheep_collider, render_wall_collider,\n"
    "      render_sheep_collider_only, render_wall_collider_only,\n"
    "      render_shepherd_pre, render_shepherd,\n"
    "      // render_sheep_pre, render_sheep,\n"
    "    },\n"
    "  };\n"
    "};\n";

int main() {
  vecs_context_t ctx;
  vecs_context_init(&ctx);

  unsigned char *utext = (unsigned char *)text;

  // printf("---- START BEFORE ----\n");
  // printf("%s\n", text);
  // printf("---- END BEFORE ----\n");
  // printf("\n");

  vecs_strip_comments(utext, strlen(text));

  // printf("---- START AFTER ----\n");
  // printf("%s\n", text);
  // printf("---- END AFTER ----\n");
  // printf("\n");

  int result;
  result = vecs_context_ingest(&ctx, VecsNullString, VecsString(text));

  if (result == 0)
    return 1;

  vecs_parse_error_t parse_err;
  result = vecs_context_parse(&ctx, &parse_err);

  if (result == 0)
    return 1;

  if (result == -1) {
    printf("parse error %zu: %s\n", parse_err.location.line, parse_err.message);
    vecs_parse_error_free(&parse_err);
    return 1;
  }

  vecs_resolve_error_t resolve_err;
  result = vecs_context_resolve(&ctx, &resolve_err);

  if (result == 0)
    return 1;

  if (result == -1) {
    printf("resolve error %zu: %s\n", resolve_err.span.start_line,
           resolve_err.message);
    vecs_resolve_error_free(&resolve_err);
    return 1;
  }

  const vecs_component_t *c =
      vecs_context_cst_get_component(&ctx, VecsString("id"));

  const vecs_type_t *t = vecs_component_get_type(c);
  vecs_type_iter_t c_typ_iter = vecs_type_iter(t);

  VecsTypeIter(t, type_component) {
    vecs_string_t typ = vecs_type_iter_get(&type_component);
    printf("%.*s ", VecsPrintfStringArgs(typ));
  }

  printf("\n");

  vecs_context_destroy(&ctx);
  return 0;
}
