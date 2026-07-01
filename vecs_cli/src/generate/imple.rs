use std::{collections::HashSet, fmt::Display};

use crate::{
  generate::{
    common::ComponentTmpOps,
    constants::StateIdName,
    generics::skip_lists::{SkipList, SkipListImplInit},
  },
  resolve::cst::Cst,
};

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName},
  constants::{ComponentMaskName, NodeMaskName},
  generics::{
    common::{function_name, method_name},
    dyn_arrays::DynArray,
    dyn_queue::DynQueue,
    sparse_dyn_arrays::SparseDynArray,
  },
};

pub struct Impl<'a> {
  pub header_name: &'a str,
  pub data: &'a Cst<'a>,
}

impl<'a> Display for Impl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "#include <assert.h>\n")?;
    write!(f, "#include <stdlib.h>\n")?;
    write!(f, "#include <string.h>\n")?;
    write!(f, "#include \"{}\"\n\n", self.header_name)?;

    DynArray::new("uint32_t").imple().fmt(f)?;
    DynArray::new("uint64_t").imple().fmt(f)?;
    DynQueue::new("uint32_t").imple().fmt(f)?;

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      DynQueue::new(event_t).imple().fmt(f)?;
    }

    // Skip list of vecs_id_t:

    let max_key_fn_name = function_name!("max"; "vecs_id_t");

    write!(
      f,
      concat!(
        "static inline vecs_id_t {max_key_fn_name}() {{\n",
        "  return vecs_id_max();\n",
        "}}\n",
      ),
      max_key_fn_name = max_key_fn_name,
    )?;

    let id_eq_fn_name = function_name!("eq"; "vecs_id_t");

    write!(
      f,
      concat!(
        "static inline bool {eq_fn_name}(vecs_id_t a, vecs_id_t b) {{\n",
        "  return vecs_id_eq(a, b);\n",
        "}}\n",
      ),
      eq_fn_name = id_eq_fn_name,
    )?;

    let id_cmp_fn_name = function_name!("cmp"; "vecs_id_t");

    write!(
      f,
      concat!(
        "static inline int8_t {cmp_fn_name}(vecs_id_t a, vecs_id_t b) {{\n",
        "  return vecs_id_cmp(a, b);\n",
        "}}\n",
      ),
      cmp_fn_name = id_cmp_fn_name,
    )?;

    let index_index = SkipList::new("vecs_id_t", "uint32_t");
    let index_index_t = index_index.get_type();

    SkipListImplInit {}.fmt(f)?;
    index_index.imple().fmt(f)?;

    // Arrays:

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);

      if !component.is_empty() {
        DynArray::new(component_t.clone()).imple().fmt(f)?;
        SparseDynArray::new(component_t.clone()).imple().fmt(f)?;
      }
    }

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      DynArray::new(node_t).imple().fmt(f)?;
    }

    DynArray::new("vecs_entity_t").imple().fmt(f)?;
    let entity_array = SparseDynArray::new("vecs_entity_t");
    entity_array.imple().fmt(f)?;

    let op_add_component_queue = DynQueue::new("vecs_op_union_add_component_t");
    let op_add_component_queue_t = op_add_component_queue.get_type();
    op_add_component_queue.imple().fmt(f)?;

    let op_other_queue = DynQueue::new("vecs_op_union_other_t");
    let op_other_queue_t = op_other_queue.get_type();
    op_other_queue.imple().fmt(f)?;

    let op_remove_component_queue = DynQueue::new("vecs_op_union_remove_component_t");
    let op_remove_component_queue_t = op_remove_component_queue.get_type();
    op_remove_component_queue.imple().fmt(f)?;

    // Engine methods:

    let entity_array_t = entity_array.get_type();

    write!(
      f,
      concat!(
        "void vecs_init(vecs_engine_t *e) {{\n",
        "  e->state = VECS_STATE_NONE;\n",
        "  e->entities_to_add = 0;\n",
        "  e->next_state = VECS_STATE_NONE;\n",
        "  {entity_array_method_init}(&e->entities, 0);\n",
        "  {op_add_component_queue_method_init}(&e->ops_add_component, 0);\n",
        "  {op_other_queue_method_init}(&e->ops_other, 0);\n",
        "  {op_remove_component_queue_method_init}(&e->ops_remove_component, 0);\n",
      ),
      entity_array_method_init = method_name!(&entity_array_t, "init"),
      op_add_component_queue_method_init =
        method_name!(&op_add_component_queue_t, "init"),
      op_other_queue_method_init = method_name!(&op_other_queue_t, "init"),
      op_remove_component_queue_method_init =
        method_name!(&op_remove_component_queue_t, "init"),
    )?;

    for component in self.data.components.values() {
      if !component.is_empty() {
        let component_name = component.name();
        let component_t = ComponentStructName::new(component_name);

        let dyn_array = SparseDynArray::new(component_t.clone());
        let dyn_array_t = dyn_array.get_type();

        write!(
          f,
          concat!(
            "  {component_array_method_init}(&e->components_{component_name}, 0);\n",
            "  {index_index_method_init}(&e->entity_to_component_{component_name});\n",
          ),
          component_name = component_name,
          component_array_method_init = method_name!(&dyn_array_t, "init"),
          index_index_method_init = method_name!(&index_index_t, "init"),
        )?;
      }
    }

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      let dyn_array = DynArray::new(node_t);
      let dyn_array_t = dyn_array.get_type();

      write!(
        f,
        concat!(
          "  {node_array_method_init}(&e->nodes_{node_name}, 0);\n",
          "  {index_index_method_init}(&e->entity_to_node_{node_name});\n",
        ),
        node_name = node.name,
        node_array_method_init = method_name!(&dyn_array_t, "init"),
        index_index_method_init = method_name!(&index_index_t, "init"),
      )?;
    }

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      let dyn_queue = DynQueue::new(event_t);
      let dyn_queue_t = dyn_queue.get_type();

      write!(
        f,
        concat!("{event_queue_method_init}(&e->events_{event_name}, 0);\n"),
        event_name = event.name,
        event_queue_method_init = method_name!(&dyn_queue_t, "init"),
      )?;
    }

    write!(
      f,
      concat!(
        "}}\n",
        "void vecs_destroy(vecs_engine_t *e) {{\n",
        "  e->state = VECS_STATE_NONE;\n",
        "  {entity_array_method_destroy}(&e->entities);\n",
      ),
      entity_array_method_destroy = method_name!(&entity_array_t, "destroy"),
    )?;

    for component in self.data.components.values() {
      if !component.is_empty() {
        let component_name = component.name();
        let component_t = ComponentStructName::new(component_name);

        let dyn_array = SparseDynArray::new(component_t.clone());
        let dyn_array_t = dyn_array.get_type();

        write!(
          f,
          concat!(
            "  {component_array_method_destroy}(&e->components_{component_name});\n",
            "  {index_index_method_destroy}(&e->entity_to_component_{component_name});\n",
          ),
          component_name = component_name,
          component_array_method_destroy = method_name!(&dyn_array_t, "destroy"),
          index_index_method_destroy = method_name!(&index_index_t, "destroy"),
        )?;
      }
    }

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      let dyn_array = DynArray::new(node_t);
      let dyn_array_t = dyn_array.get_type();

      write!(
        f,
        concat!(
          "  {node_array_method_destroy}(&e->nodes_{node_name});\n",
          "  {index_index_method_destroy}(&e->entity_to_node_{node_name});\n",
        ),
        node_name = node.name,
        node_array_method_destroy = method_name!(&dyn_array_t, "destroy"),
        index_index_method_destroy = method_name!(&index_index_t, "destroy"),
      )?;
    }

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      let dyn_queue = DynQueue::new(event_t);
      let dyn_queue_t = dyn_queue.get_type();

      write!(
        f,
        concat!("{event_queue_method_destroy}(&e->events_{event_name});\n"),
        event_name = event.name,
        event_queue_method_destroy = method_name!(&dyn_queue_t, "destroy"),
      )?;
    }

    write!(
      f,
      concat!(
        "}}\n",
        "vecs_id_t vecs_add_entity(vecs_engine_t *e) {{\n",
        "  vecs_entity_t ent = {{0}};\n",
        "  vecs_id_t id;\n",
        "  {entity_array_method_push}(&e->entities, ent, &id.index, &id.gen);\n",
        "  return id;\n",
        "}}\n",
        "vecs_tmp_id_t vecs_schedule_add_entity(vecs_engine_t *e) {{\n",
        "  vecs_tmp_id_t id = {{.index = e->entities_to_add}};\n",
        "  ++e->entities_to_add;\n",
        "  return id;\n",
        "}}\n",
      ),
      entity_array_method_push = method_name!(&entity_array_t, "push"),
    )?;

    // Mask utilities:
    write!(
      f,
      concat!(
        "static inline void mix_mask(const uint64_t *mask1, uint64_t *mask2) {{\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    mask2[i] |= mask1[i];\n",
        "  }}\n",
        "}}\n",
        "static inline void unmix_mask(const uint64_t *mask1, uint64_t *mask2) {{\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    mask2[i] &= ~mask1[i];\n",
        "  }}\n",
        "}}\n",
        "static inline bool match_mask(uint64_t *outer, const uint64_t *inner) {{\n",
        "  bool result = true;\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    result = result && ((inner[i] & outer[i]) == inner[i]);\n",
        "  }}\n",
        "  return result;\n",
        "}}\n",
      ),
      mask_size = self.data.node_mask_arr_size,
    )?;

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);
      let component_mask_name = ComponentMaskName::new(component_name);

      let entity_array = SparseDynArray::new("vecs_entity_t");
      let entity_array_t = entity_array.get_type();

      let component_array = SparseDynArray::new(component_t.clone());
      let component_array_t = component_array.get_type();

      // Has component:
      write!(
        f,
        concat!(
          "// Currently, this method will not detect disabled components.\n",
          "bool vecs_has_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
          "  vecs_entity_t *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
          "  return match_mask(ent->mask, {component_mask_name});\n",
          "}}\n",
        ),
        component_name = component_name,
        entity_array_method_get = method_name!(&entity_array_t, "get"),
        component_mask_name = component_mask_name,
      )?;

      if !component.is_empty() {
        write!(
          f,
          concat!(
            "{component_t} *vecs_get_component_{component_name}(vecs_engine_t *e, vecs_id_t component_id) {{\n",
            "  return {component_array_method_get}(&e->components_{component_name}, component_id.index, component_id.gen);\n",
            "}}\n",
          ),
          component_name = component_name,
          component_t = component_t,
          component_array_method_get = method_name!(&component_array_t, "get"),
        )?;
      }

      for state in self.data.states.values() {
        // Add components:
        if !component.is_empty() {
          write!(
            f,
            concat!(
              "vecs_id_t vecs_{state_name}_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
              "  vecs_id_t component_id;\n",
              "  {component_array_method_push}(&e->components_{component_name}, component, &component_id.index, &component_id.gen);\n",
              "  {entity_to_component_array_method_add}(&e->entity_to_component_{component_name}, entity, component_id.index);\n",
              "\n",
              "  vecs_{state_name}_enable_component_{component_name}(e, entity);\n",
              "  return component_id;\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
            component_t = component_t,
            component_array_method_push = method_name!(&component_array_t, "push"),
            entity_to_component_array_method_add =
              method_name!(&index_index_t, "add"),
          )?;
        } else {
          write!(
            f,
            concat!(
              "void vecs_{state_name}_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
              "  vecs_{state_name}_enable_component_{component_name}(e, entity);\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
          )?;
        }

        // Upsert components:
        if !component.is_empty() {
          write!(
            f,
            concat!(
              "vecs_id_t vecs_{state_name}_upsert_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
              "  if (vecs_has_component_{component_name}(e, entity)) {{\n",
              "    return vecs_{state_name}_update_component_{component_name}(e, entity, component);\n",
              "  }} else {{\n",
              "    return vecs_{state_name}_add_component_{component_name}(e, entity, component);\n",
              "  }}\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
            component_t = component_t,
          )?;
        }

        // Update components:
        if !component.is_empty() {
          write!(
            f,
            concat!(
              "vecs_id_t vecs_{state_name}_update_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
              "  uint32_t component_index;\n",
              "  {entity_to_component_array_method_get}(&e->entity_to_component_{component_name}, entity, &component_index);\n",
              "  {component_t} *found = {component_array_method_get_unchecked}(&e->components_{component_name}, component_index);\n",
              "  *found = component;\n",
              "  return (vecs_id_t){{.index = component_index, .gen = e->components_{component_name}.gens.items[component_index]}};\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
            component_t = component_t,
            entity_to_component_array_method_get =
              method_name!(&index_index_t, "get"),
            component_array_method_get_unchecked =
              method_name!(&component_array_t, "get_unchecked"),
          )?;
        }

        // Remove components:
        if !component.is_empty() {
          write!(
            f,
            concat!(
              "bool vecs_{state_name}_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
              "  vecs_{state_name}_disable_component_{component_name}(e, entity);\n",
              "\n",
              "  vecs_id_t component_id;\n",
              "  bool found = {entity_to_component_array_method_remove}(&e->entity_to_component_{component_name}, entity, &component_id.index);\n",
              "  if (!found)\n",
              "    return false;\n",
              "  {component_t} component;\n",
              "  {component_array_method_remove_unchecked}(&e->components_{component_name}, component_id.index, &component);\n",
              "  return true;\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
            component_t = component_t,
            component_array_method_remove_unchecked =
              method_name!(&component_array_t, "remove_unchecked"),
            entity_to_component_array_method_remove =
              method_name!(&index_index_t, "remove"),
          )?;
        } else {
          write!(
            f,
            concat!(
              "void vecs_{state_name}_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
              "  vecs_{state_name}_disable_component_{component_name}(e, entity);\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
          )?;
        }

        // Disable components:
        write!(
          f,
          concat!(
            "// A disabled component will not produce nodes and will not be a part of the entity mask.\n",
            "// It will however be found in the entity_to_component index.\n",
            "void vecs_{state_name}_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_entity_t *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
          ),
          state_name = state.name,
          entity_array_method_get = method_name!(&entity_array_t, "get"),
          component_name = component_name,
        )?;

        for node in state.nodes.iter().map(|n| self.data.nodes.get(n).unwrap()) {
          if node.components.contains(component_name) {
            let node_t = NodeStructName::new(node.name);
            let node_mask_name = NodeMaskName::new(node.name);

            let node_array = DynArray::new(node_t);
            let node_array_t = node_array.get_type();

            write!(
              f,
              concat!(
                "  if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "    uint32_t node_index;\n",
                "    {entity_to_node_method_remove}(&e->entity_to_node_{node_name}, entity, &node_index);\n",
                "    {node_array_method_remove}(&e->nodes_{node_name}, node_index);\n",
                "  }}\n",
              ),
              node_mask_name = node_mask_name,
              node_name = node.name,
              node_array_method_remove = method_name!(&node_array_t, "swap_remove"),
              entity_to_node_method_remove = method_name!(&index_index_t, "remove"),
            )?;
          }
        }

        write!(
          f,
          concat!("  unmix_mask({component_mask_name}, ent->mask);\n", "}}\n",),
          component_mask_name = component_mask_name,
        )?;

        // Enable components:
        write!(
          f,
          concat!(
            "void vecs_{state_name}_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_entity_t *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
            "  mix_mask({component_mask_name}, ent->mask);\n",
          ),
          state_name = state.name,
          entity_array_method_get = method_name!(&entity_array_t, "get"),
          component_name = component_name,
          component_mask_name = component_mask_name,
        )?;

        for node in state.nodes.iter().map(|n| self.data.nodes.get(n).unwrap()) {
          if node.components.contains(component_name) {
            let node_t = NodeStructName::new(node.name);
            let node_mask_name = NodeMaskName::new(node.name);

            write!(
              f,
              concat!(
                "  if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "    {node_t} node;\n",
              ),
              node_mask_name = node_mask_name,
              node_t = node_t,
            )?;

            if !node.is_empty() {
              write!(f, "    uint32_t component_index;\n")?;
            }

            for node_component_name in node.components.iter() {
              let node_component = self
                .data
                .components
                .get(node_component_name)
                .expect("component not found");

              if !node_component.is_empty() {
                write!(
                  f,
                  concat!(
                    "    {entity_to_component_array_method_get}(&e->entity_to_component_{component_name}, entity, &component_index);\n",
                    "    node.{component_name}_index = component_index;\n",
                  ),
                  component_name = node_component_name,
                  entity_to_component_array_method_get =
                    method_name!(&index_index_t, "get"),
                )?;
              }
            }

            let node_array = DynArray::new(node_t);
            let node_array_t = node_array.get_type();

            write!(
              f,
              concat!(
                "    uint32_t node_index = {node_array_method_push}(&e->nodes_{node_name}, node);\n",
                "    {entity_to_node_method_add}(&e->entity_to_node_{node_name}, entity, node_index);\n",
                "  }}\n",
              ),
              node_name = node.name,
              node_array_method_push = method_name!(&node_array_t, "push"),
              entity_to_node_method_add = method_name!(&index_index_t, "add"),
            )?;
          }
        }

        write!(f, concat!("}}\n",),)?;
      }

      // Callbacks that are stored in the deferred operations and apply them
      for state in self.data.states.values() {
        if !component.is_empty() {
          write!(
            f,
            concat!(
              "static void vecs_{state_name}_apply_store_entity_in_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_store_entity_t store = op.store_entity;\n",
              "  vecs_id_t entity = new_entities[store.tmp_entity.index];\n",
              "  vecs_id_t *location = (vecs_id_t*)((uint8_t*)e->components_{component_name}.items.items + store.location_offset);\n",
              "  *location = entity;\n",
              "}}\n",
              "static void vecs_{state_name}_apply_store_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_store_component_t store = op.store_component;\n",
              "  vecs_id_t component = new_components[store.tmp_component.index];\n",
              "  vecs_id_t *location = (vecs_id_t*)((uint8_t*)e->components_{component_name}.items.items + store.location_offset);\n",
              "  *location = component;\n",
              "}}\n",
              "static vecs_id_t vecs_{state_name}_apply_add_component_{component_name}(vecs_engine_t *e, vecs_id_t *new_entities, vecs_op_union_add_component_t op) {{\n",
              "  vecs_op_add_component_{component_name}_t add = op.add_{component_name};\n",
              "  return vecs_{state_name}_add_component_{component_name}(e, add.entity, add.component);\n",
              "}}\n",
              "static vecs_id_t vecs_{state_name}_apply_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_id_t *new_entities, vecs_op_union_add_component_t op) {{\n",
              "  vecs_op_tmp_add_component_{component_name}_t add_tmp = op.add_tmp_{component_name};\n",
              "  vecs_id_t entity = new_entities[add_tmp.tmp_entity.index];\n",
              "  return vecs_{state_name}_add_component_{component_name}(e, entity, add_tmp.component);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_upsert_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_update_component_{component_name}_t upsert = op.update_{component_name};\n",
              "  vecs_{state_name}_upsert_component_{component_name}(e, upsert.entity, upsert.component);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_update_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_update_component_{component_name}_t update = op.update_{component_name};\n",
              "  vecs_{state_name}_update_component_{component_name}(e, update.entity, update.component);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_remove_component_{component_name}(vecs_engine_t *e, vecs_op_union_remove_component_t op) {{\n",
              "  vecs_op_remove_component_t remove = op.remove;\n",
              "  vecs_{state_name}_remove_component_{component_name}(e, remove.entity);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_enable_component_t enable = op.enable;\n",
              "  vecs_{state_name}_enable_component_{component_name}(e, enable.entity);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_disable_component_t disable = op.disable;\n",
              "  vecs_{state_name}_disable_component_{component_name}(e, disable.entity);\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
          )?;
        } else {
          write!(
            f,
            concat!(
              "static vecs_id_t vecs_{state_name}_apply_add_component_{component_name}(vecs_engine_t *e, vecs_id_t *new_entities, vecs_op_union_add_component_t op) {{\n",
              "  vecs_op_add_component_{component_name}_t add = op.add_{component_name};\n",
              "  vecs_{state_name}_add_component_{component_name}(e, add.entity);\n",
              "  return vecs_id_invalid;\n",
              "}}\n",
              "static vecs_id_t vecs_{state_name}_apply_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_id_t *new_entities, vecs_op_union_add_component_t op) {{\n",
              "  vecs_op_tmp_add_component_{component_name}_t add_tmp = op.add_tmp_{component_name};\n",
              "  vecs_id_t entity = new_entities[add_tmp.tmp_entity.index];\n",
              "  vecs_{state_name}_add_component_{component_name}(e, entity);\n",
              "  return vecs_id_invalid;\n",
              "}}\n",
              "static void vecs_{state_name}_apply_remove_component_{component_name}(vecs_engine_t *e, vecs_op_union_remove_component_t op) {{\n",
              "  vecs_op_remove_component_t remove = op.remove;\n",
              "  vecs_{state_name}_remove_component_{component_name}(e, remove.entity);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_enable_component_t enable = op.enable;\n",
              "  vecs_{state_name}_enable_component_{component_name}(e, enable.entity);\n",
              "}}\n",
              "static void vecs_{state_name}_apply_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) {{\n",
              "  vecs_op_disable_component_t disable = op.disable;\n",
              "  vecs_{state_name}_disable_component_{component_name}(e, disable.entity);\n",
              "}}\n",
            ),
            state_name = state.name,
            component_name = component_name,
          )?;
        }
      }
    }

    let op_add_component_names = ["add_component", "tmp_add_component"];

    let op_nonempty_other_names = [
      "store_entity_in",
      "store_component",
      "upsert_component",
      "update_component",
      "enable_component",
      "disable_component",
    ];
    let op_empty_other_names = ["enable_component", "disable_component"];

    let op_remove_component_names = ["remove_component"];

    // Build the operation maps, where there is an array for each component type and
    // an operation callback for each state
    for component in self.data.components.values() {
      for op_name in op_add_component_names {
        let component_name = component.name();
        write!(
          f,
          concat!(
            "vecs_id_t (* const vecs_op_map_{op_name}_{component_name}[{states_len}])(vecs_engine_t *, vecs_id_t *, vecs_op_union_add_component_t) = {{\n",
            "  NULL,\n",
          ),
          op_name = op_name,
          component_name = component_name,
          states_len = self.data.states.len() + 1,
        )?;

        // `states` being a BTreeMap guarantees this will be in the same order as the
        // states enum, meaning it will be able to index this properly
        for state in self.data.states.values() {
          write!(
            f,
            "  &vecs_{state_name}_apply_{op_name}_{component_name},\n",
            op_name = op_name,
            component_name = component_name,
            state_name = state.name,
          )?;
        }

        write!(f, "}};\n",)?;
      }
    }

    for component in self.data.components.values() {
      let op_other_names = if !component.is_empty() {
        op_nonempty_other_names.as_slice()
      } else {
        op_empty_other_names.as_slice()
      };

      for op_name in op_other_names {
        let component_name = component.name();
        write!(
          f,
          concat!(
            "void (* const vecs_op_map_{op_name}_{component_name}[{states_len}])(vecs_engine_t *e, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, vecs_op_union_other_t op) = {{\n",
            "  NULL,\n",
          ),
          op_name = op_name,
          component_name = component_name,
          states_len = self.data.states.len() + 1,
        )?;

        // `states` being a BTreeMap guarantees this will be in the same order as the
        // states enum, meaning it will be able to index this properly
        for state in self.data.states.values() {
          write!(
            f,
            "  &vecs_{state_name}_apply_{op_name}_{component_name},\n",
            op_name = op_name,
            component_name = component_name,
            state_name = state.name,
          )?;
        }

        write!(f, "}};\n",)?;
      }
    }

    for op_name in op_remove_component_names {
      for component in self.data.components.values() {
        let component_name = component.name();
        write!(
          f,
          concat!(
            "void (* const vecs_op_map_{op_name}_{component_name}[{states_len}])(vecs_engine_t *, vecs_op_union_remove_component_t) = {{\n",
            "  NULL,\n",
          ),
          op_name = op_name,
          component_name = component_name,
          states_len = self.data.states.len() + 1,
        )?;

        // `states` being a BTreeMap guarantees this will be in the same order as the
        // states enum, meaning it will be able to index this properly
        for state in self.data.states.values() {
          write!(
            f,
            "  &vecs_{state_name}_apply_{op_name}_{component_name},\n",
            op_name = op_name,
            component_name = component_name,
            state_name = state.name,
          )?;
        }

        write!(f, "}};\n",)?;
      }
    }

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);
      let ops = ComponentTmpOps::new(component_name);

      if !component.is_empty() {
        write!(
          f,
          concat!(
            "void vecs_schedule_store_entity_in_{component_name}(vecs_engine_t *e, vecs_tmp_id_t tmp_entity, vecs_id_t *location) {{\n",
            "  assert(sizeof(uint8_t*) == sizeof(vecs_id_t*) && sizeof(uint8_t*) == sizeof({component_t}*));\n",
            "\n",
            "  // Make sure the location is inside the component array\n",
            "  assert((uint8_t*)location >= (uint8_t*)e->components_{component_name}.items.items);\n",
            "  assert((uint8_t*)location < (uint8_t*)e->components_{component_name}.items.items + e->components_{component_name}.items.len * sizeof({component_t}));\n",
            "\n",
            "  ptrdiff_t offset = (uint8_t*)e->components_{component_name}.items.items - (uint8_t*)location;\n",
            "  vecs_op_store_entity_t store = {{.tmp_entity = tmp_entity, .location_offset = offset}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_store_entity_in_{component_name}[e->state], .store_entity = store}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "void vecs_schedule_store_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t tmp_component, vecs_id_t *location) {{\n",
            "  assert(sizeof(uint8_t*) == sizeof(vecs_id_t*) && sizeof(uint8_t*) == sizeof({component_t}*));\n",
            "\n",
            "  // Make sure the location is inside the component array\n",
            "  assert((uint8_t*)location >= (uint8_t*)e->components_{component_name}.items.items);\n",
            "  assert((uint8_t*)location < (uint8_t*)e->components_{component_name}.items.items + e->components_{component_name}.items.len * sizeof({component_t}));\n",
            "\n",
            "  ptrdiff_t offset = (uint8_t*)e->components_{component_name}.items.items - (uint8_t*)location;\n",
            "  vecs_op_store_component_t store = {{.tmp_component = tmp_component, .location_offset = offset}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_store_component_{component_name}[e->state], .store_component = store}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "vecs_tmp_id_t vecs_schedule_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
            "  vecs_tmp_id_t id = {{.index = e->ops_add_component.len}};\n",
            "  {component_add_t} add = {{.entity = entity, .component = component}};\n",
            "  vecs_op_union_add_component_t op = {{.apply = vecs_op_map_add_component_{component_name}[e->state], .add_{component_name} = add}};\n",
            "  {op_add_component_queue_method_enqueue}(&e->ops_add_component, op);\n",
            "  return id;\n",
            "}}\n",
            "vecs_tmp_id_t vecs_schedule_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t entity, {component_t} component) {{\n",
            "  vecs_tmp_id_t id = {{.index = e->ops_add_component.len}};\n",
            "  {component_add_tmp_t} add_tmp = {{.tmp_entity = entity, .component = component}};\n",
            "  vecs_op_union_add_component_t op = {{.apply = vecs_op_map_tmp_add_component_{component_name}[e->state], .add_tmp_{component_name} = add_tmp}};\n",
            "  {op_add_component_queue_method_enqueue}(&e->ops_add_component, op);\n",
            "  return id;\n",
            "}}\n",
            "void vecs_schedule_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_enable_component_t enable = {{.entity = entity}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_enable_component_{component_name}[e->state], .enable = enable}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "void vecs_schedule_upsert_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
            "  {component_update_t} upsert = {{.entity = entity, .component = component}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_upsert_component_{component_name}[e->state], .update_{component_name} = upsert}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "void vecs_schedule_update_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component) {{\n",
            "  {component_update_t} update = {{.entity = entity, .component = component}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_update_component_{component_name}[e->state], .update_{component_name} = update}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "void vecs_schedule_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_remove_component_t remove = {{.entity = entity}};\n",
            "  vecs_op_union_remove_component_t op = {{.apply = vecs_op_map_remove_component_{component_name}[e->state], .remove = remove}};\n",
            "  {op_remove_component_queue_method_enqueue}(&e->ops_remove_component, op);\n",
            "}}\n",
            "void vecs_schedule_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_disable_component_t disable = {{.entity = entity}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_disable_component_{component_name}[e->state], .disable = disable}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
          ),
          component_name = component_name,
          component_t = component_t,
          component_add_t = ops.add_t,
          component_add_tmp_t = ops.add_tmp_t,
          component_update_t = ops.update_t,
          op_add_component_queue_method_enqueue =
            method_name!(&op_add_component_queue_t, "enqueue"),
          op_other_queue_method_enqueue = method_name!(&op_other_queue_t, "enqueue"),
          op_remove_component_queue_method_enqueue =
            method_name!(&op_remove_component_queue_t, "enqueue"),
        )?;
      } else {
        write!(
          f,
          concat!(
            "void vecs_schedule_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_tmp_id_t id = {{.index = e->ops_add_component.len}};\n",
            "  {component_add_t} add = {{.entity = entity}};\n",
            "  vecs_op_union_add_component_t op = {{.apply = vecs_op_map_add_component_{component_name}[e->state], .add_{component_name} = add}};\n",
            "  {op_add_component_queue_method_enqueue}(&e->ops_add_component, op);\n",
            "}}\n",
            "void vecs_schedule_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t entity) {{\n",
            "  vecs_tmp_id_t id = {{.index = e->ops_add_component.len}};\n",
            "  {component_add_tmp_t} add_tmp = {{.tmp_entity = entity}};\n",
            "  vecs_op_union_add_component_t op = {{.apply = vecs_op_map_tmp_add_component_{component_name}[e->state], .add_tmp_{component_name} = add_tmp}};\n",
            "  {op_add_component_queue_method_enqueue}(&e->ops_add_component, op);\n",
            "}}\n",
            "void vecs_schedule_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_enable_component_t enable = {{.entity = entity}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_enable_component_{component_name}[e->state], .enable = enable}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
            "void vecs_schedule_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_remove_component_t remove = {{.entity = entity}};\n",
            "  vecs_op_union_remove_component_t op = {{.apply = vecs_op_map_remove_component_{component_name}[e->state], .remove = remove}};\n",
            "  {op_remove_component_queue_method_enqueue}(&e->ops_remove_component, op);\n",
            "}}\n",
            "void vecs_schedule_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_op_disable_component_t disable = {{.entity = entity}};\n",
            "  vecs_op_union_other_t op = {{.apply = vecs_op_map_disable_component_{component_name}[e->state], .disable = disable}};\n",
            "  {op_other_queue_method_enqueue}(&e->ops_other, op);\n",
            "}}\n",
          ),
          component_name = component_name,
          component_add_t = ops.add_t,
          component_add_tmp_t = ops.add_tmp_t,
          op_add_component_queue_method_enqueue =
            method_name!(&op_add_component_queue_t, "enqueue"),
          op_other_queue_method_enqueue = method_name!(&op_other_queue_t, "enqueue"),
          op_remove_component_queue_method_enqueue =
            method_name!(&op_remove_component_queue_t, "enqueue"),
        )?;
      }
    }

    // Node getters:
    for node in self.data.nodes.values() {
      write!(
        f,
        concat!(
          "vecs_node_{node_name}_array_t vecs_nodes_{node_name}(vecs_engine_t *e) {{\n",
          "  return e->nodes_{node_name};\n",
          "}}\n",
        ),
        node_name = node.name,
      )?;
    }

    for state in self.data.states.values() {
      let state_id = StateIdName::new(state.name);

      write!(
        f,
        concat!(
          "void vecs_schedule_state_to_{state_name}(vecs_engine_t *e) {{\n",
          "  e->next_state = {state_id};\n",
          "}}\n\n",
        ),
        state_name = state.name,
        state_id = state_id,
      )?;

      // State transitions:
      for other_state in self.data.states.values() {
        if state.name == other_state.name {
          continue;
        }

        write!(
          f,
          "void vecs_state_{}_to_{}(vecs_engine_t *e) {{\n",
          other_state.name, state.name,
        )?;

        let old_relevant_nodes = other_state
          .nodes
          .iter()
          .map(|n| self.data.nodes.get(n).unwrap())
          .collect::<HashSet<_>>();

        let new_relevant_nodes = state
          .nodes
          .iter()
          .map(|n| self.data.nodes.get(n).unwrap())
          .collect::<HashSet<_>>();

        // Remove unnecessary nodes:
        for old_relevant_node in old_relevant_nodes.difference(&new_relevant_nodes) {
          let node_t = NodeStructName::new(old_relevant_node.name);
          let node_array = DynArray::new(node_t);
          let node_array_t = node_array.get_type();

          write!(
            f,
            concat!("  {node_array_method_destroy}(&e->nodes_{node_name});\n",),
            node_name = old_relevant_node.name,
            node_array_method_destroy = method_name!(&node_array_t, "destroy"),
          )?;
        }

        // Add new necessary nodes:
        let mut new_nodes = new_relevant_nodes
          .difference(&old_relevant_nodes)
          .peekable();

        // If there is some new node to track
        if new_nodes.peek().is_some() {
          write!(
            f,
            concat!(
              "  for (uint32_t i = 0; i < e->entities.len; ++i) {{\n",
              "    if ({entity_method_is_hole}(&e->entities, i)) {{\n",
              "      goto continue_outer;\n",
              "    }}\n",
              "\n",
              "    vecs_entity_t *ent = &e->entities.items.items[i];\n",
              "    uint32_t gen = e->entities.gens.items[i];\n",
              "    vecs_id_t entity = {{.index = i, .gen = gen}};\n",
            ),
            entity_method_is_hole = method_name!(&entity_array_t, "is_hole"),
          )?;

          for new_relevant_node in new_nodes {
            let node_t = NodeStructName::new(new_relevant_node.name);
            let node_mask_name = NodeMaskName::new(new_relevant_node.name);

            write!(
              f,
              concat!(
                "    if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "      {node_t} node;\n",
              ),
              node_t = node_t,
              node_mask_name = node_mask_name,
            )?;

            if !new_relevant_node.is_empty() {
              write!(f, "    uint32_t component_index;\n")?;
            }

            for component_name in new_relevant_node.components.iter() {
              let component = self
                .data
                .components
                .get(component_name)
                .expect("component not found");

              if !component.is_empty() {
                write!(
                  f,
                  concat!(
                    "      {entity_to_component_array_method_get}(&e->entity_to_component_{component_name}, entity, &component_index);\n",
                    "      node.{component_name}_index = component_index;\n",
                  ),
                  component_name = component_name,
                  entity_to_component_array_method_get =
                    method_name!(&index_index_t, "get"),
                )?;
              }
            }

            let node_array = DynArray::new(node_t);
            let node_array_t = node_array.get_type();

            write!(
              f,
              concat!(
                "      uint32_t node_index = {node_array_method_push}(&e->nodes_{node_name}, node);\n",
                "      {entity_to_node_method_add}(&e->entity_to_node_{node_name}, entity, node_index);\n",
                "    }}\n",
              ),
              node_name = new_relevant_node.name,
              node_array_method_push = method_name!(&node_array_t, "push"),
              entity_to_node_method_add = method_name!(&index_index_t, "add"),
            )?;
          }

          write!(f, concat!("continue_outer:\n", "    ;\n", "  }}\n",),)?;
        }

        write!(
          f,
          concat!("  e->state = {state_id};\n", "}}\n",),
          state_id = state_id,
        )?;
      }

      // State loops:
      write!(
        f,
        concat!(
          "void vecs_run_state_{state_name}(vecs_engine_t *e) {{\n",
          "  e->state = {state_id};\n",
        ),
        state_name = state.name,
        state_id = state_id,
      )?;

      for event in self.data.events.values() {
        let event_t = EventStructName::new(event.name);
        let event_queue = DynQueue::new(event_t.clone());
        let event_queue_name = event_queue.get_type();

        write!(
          f,
          concat!(
            "  while (e->events_{event_name}.len > 0) {{\n",
            "    {event_t} ev = {events_method_dequeue}(&e->events_{event_name});\n",
            "    size_t nodes_len;\n",
          ),
          event_name = event.name,
          event_t = event_t,
          events_method_dequeue = method_name!(&event_queue_name, "dequeue"),
        )?;

        for system_layer in state.systems.iter() {
          for system_name in system_layer {
            let system = self
              .data
              .systems
              .get(system_name)
              .expect("failed to find system in state");

            if system.event == event.name {
              if let Some(node) = system.node {
                let node_t = NodeStructName::new(node);

                write!(
                  f,
                  concat!(
                    "    nodes_len = e->nodes_{node_name}.len;\n",
                    "    for (size_t i = 0; i < nodes_len; ++i) {{\n",
                    "      {node_t} *node = &e->nodes_{node_name}.items[i];\n",
                    "      {system_name}(e, *node, ev);\n",
                    "    }}\n",
                  ),
                  node_name = node,
                  node_t = node_t,
                  system_name = system.name,
                )?;
              } else {
                write!(f, "    {system_name}(e, ev);\n", system_name = system.name)?;
              }
            }
          }
        }
        write!(f, "  }}\n")?;
      }

      write!(
        f,
        concat!(
          "  vecs_id_t *new_things = NULL;\n",
          "  size_t new_component_count = e->ops_add_component.len;\n",
          "  size_t new_thing_count = e->entities_to_add + new_component_count;\n",
          "\n",
          "  if (new_thing_count > 0) {{\n",
          "    // TODO: Preallocate this.\n",
          "    new_things = malloc(new_thing_count * sizeof(vecs_id_t));\n",
          "  }}\n",
          "\n",
          "  vecs_id_t *new_entities = new_things + 0;\n",
          "  vecs_id_t *new_components = new_things + e->entities_to_add;\n",
          "\n",
          "  for (size_t i = 0; i < e->entities_to_add; ++i) {{\n",
          "    new_entities[i] = vecs_add_entity(e);\n",
          "  }}\n",
          "\n",
          "  for (size_t i = 0; i < new_component_count; ++i) {{\n",
          "    vecs_op_union_add_component_t op = {op_add_component_queue_method_dequeue}(&e->ops_add_component);\n",
          "    new_components[i] = op.apply(e, new_entities, op);\n",
          "  }}\n",
          "\n",
          "  size_t ops_other_count = e->ops_other.len;\n",
          "  for (size_t i = 0; i < ops_other_count; ++i) {{\n",
          "    vecs_op_union_other_t op = {op_other_queue_method_dequeue}(&e->ops_other);\n",
          "    op.apply(e, new_entities, new_components, op);\n",
          "  }}\n",
          "\n",
          "  size_t remove_component_count = e->ops_remove_component.len;\n",
          "  for (size_t i = 0; i < remove_component_count; ++i) {{\n",
          "    vecs_op_union_remove_component_t op = {op_remove_component_queue_method_dequeue}(&e->ops_remove_component);\n",
          "    op.apply(e, op);\n",
          "  }}\n",
          "\n",
          "  free(new_things);\n",
          "}}\n",
        ),
        op_add_component_queue_method_dequeue =
          method_name!(&op_add_component_queue_t, "dequeue"),
        op_other_queue_method_dequeue = method_name!(&op_other_queue_t, "dequeue"),
        op_remove_component_queue_method_dequeue =
          method_name!(&op_remove_component_queue_t, "dequeue"),
      )?;
    }

    // Event emition:
    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      let event_queue = DynQueue::new(event_t.clone());
      let event_queue_t = event_queue.get_type();

      write!(
        f,
        concat!(
          "void vecs_emit_{event_name}(vecs_engine_t *e, {event_t} ev) {{\n",
          "  {event_queue_method_enqueue}(&e->events_{event_name}, ev);\n",
          "}}\n",
        ),
        event_name = event.name,
        event_t = event_t,
        event_queue_method_enqueue = method_name!(&event_queue_t, "enqueue"),
      )?;
    }

    Ok(())
  }
}
