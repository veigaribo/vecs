use std::{collections::HashSet, fmt::Display};

use crate::resolve::cst::Cst;

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName},
  constants::{ComponentMaskName, NodeMaskName},
  generics::{
    common::{function_name, method_name},
    dyn_arrays::DynArray,
    dyn_queue::DynQueue,
    hash_dyn_arrays::HashDynArray,
    sparse_dyn_arrays::SparseDynArray,
  },
};

pub struct Impl<'a> {
  pub header_name: &'a str,
  pub data: &'a Cst<'a>,
}

impl<'a> Display for Impl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "#include <stdlib.h>\n")?;
    write!(f, "#include <string.h>\n")?;
    write!(f, "#include \"{}\"\n\n", self.header_name)?;

    DynArray::new("uint32_t").imple().fmt(f)?;

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      DynQueue::new(event_t).imple().fmt(f)?;
    }

    let id_hash_fn_name = function_name!("hash"; "vecs_id_t");

    write!(
      f,
      concat!(
        "// http://www.cse.yorku.ca/~oz/hash.html\n",
        "static inline uint32_t {hash_fn_name}(vecs_id_t key) {{\n",
        "  uint32_t hash = 5381;\n",
        "  /* hash * 33 + c */\n",
        "  for (uint32_t i = 0; i < sizeof(uint32_t) * 8; i += 8) {{\n",
        "    uint32_t c = (key.index >> i) & 0xff;\n",
        "    hash = ((hash << 5) + hash) + c;\n",
        "  }}\n",
        "  for (uint32_t i = 0; i < sizeof(uint32_t) * 8; i += 8) {{\n",
        "    uint32_t c = (key.gen >> i) & 0xff;\n",
        "    hash = ((hash << 5) + hash) + c;\n",
        "  }}\n",
        "  return hash;\n",
        "}}\n",
      ),
      hash_fn_name = id_hash_fn_name,
    )?;

    let id_eq_fn_name = function_name!("eq"; "vecs_id_t");

    write!(
      f,
      concat!(
        "// http://www.cse.yorku.ca/~oz/hash.html\n",
        "static inline bool {eq_fn_name}(vecs_id_t a, vecs_id_t b) {{\n",
        "  return a.index == b.index && a.gen == b.gen;\n",
        "}}\n",
      ),
      eq_fn_name = id_eq_fn_name,
    )?;

    let index_hash_array = HashDynArray::new("vecs_id_t", "uint32_t");

    let index_hash_array_t = index_hash_array.get_type();
    index_hash_array.imple().fmt(f)?;

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);

      DynArray::new(component_t.clone()).imple().fmt(f)?;
      SparseDynArray::new(component_t.clone()).imple().fmt(f)?;
    }

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      DynArray::new(node_t).imple().fmt(f)?;
    }

    DynArray::new("vecs_entity_t").imple().fmt(f)?;
    let entity_array = SparseDynArray::new("vecs_entity_t");
    entity_array.imple().fmt(f)?;

    // Engine methods:

    let entity_array_t = entity_array.get_type();

    write!(
      f,
      concat!(
        "vecs_id_t vecs_add_entity(vecs_engine_t *e) {{\n",
        "  vecs_entity_t ent = {{0}};\n",
        "  vecs_id_t id;\n",
        "  {entity_array_method_push}(&e->entities, ent, &id.index, &id.gen);\n",
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
      for state in self.data.states.values() {
        let component_name = component.name();
        let component_t = ComponentStructName::new(component_name);
        let component_mask_name = ComponentMaskName::new(component_name);

        let entity_array = SparseDynArray::new("vecs_entity_t");
        let entity_array_name = entity_array.get_type();

        let component_array = SparseDynArray::new(component_t.clone());
        let component_array_name = component_array.get_type();

        // Add components:
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
          component_array_method_push = method_name!(&component_array_name, "push"),
          entity_to_component_array_method_add =
            method_name!(&index_hash_array_t, "add"),
        )?;

        // Remove components:
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
            "  {component_array_method_remove}(&e->components_{component_name}, component_id.index);\n",
            "  return true;\n",
            "}}\n",
          ),
          state_name = state.name,
          component_name = component_name,
          component_array_method_remove =
            method_name!(&component_array_name, "remove"),
          entity_to_component_array_method_remove =
            method_name!(&index_hash_array_t, "remove"),
        )?;

        // Disable components:
        write!(
          f,
          concat!(
            "void vecs_{state_name}_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity) {{\n",
            "  vecs_entity_t *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
          ),
          state_name = state.name,
          entity_array_method_get = method_name!(&entity_array_name, "get"),
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
              entity_to_node_method_remove =
                method_name!(&index_hash_array_t, "remove"),
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
          entity_array_method_get = method_name!(&entity_array_name, "get"),
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
                "    uint32_t component_index;\n",
                "    {node_t} node;\n",
              ),
              node_mask_name = node_mask_name,
              node_t = node_t,
            )?;

            for node_component in node.components.iter() {
              write!(
                f,
                concat!(
                  "    {entity_to_component_array_method_get}(&e->entity_to_component_{component_name}, entity, &component_index);\n",
                  "    node.{component_name}_index = component_index;\n",
                ),
                component_name = node_component,
                entity_to_component_array_method_get =
                  method_name!(&index_hash_array_t, "get"),
              )?;
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
              entity_to_node_method_add = method_name!(&index_hash_array_t, "add"),
            )?;
          }
        }

        write!(f, concat!("}}\n",),)?;
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
      // State transitions:
      for other_state in self.data.states.values() {
        if state.name == other_state.name {
          continue;
        }

        write!(
          f,
          "void vecs_state_{}_to_{}(vecs_engine_t *e) {{\n",
          state.name, other_state.name,
        )?;

        let old_relevant_nodes = state
          .nodes
          .iter()
          .map(|n| self.data.nodes.get(n).unwrap())
          .collect::<HashSet<_>>();

        let new_relevant_nodes = other_state
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
              "    for (uint32_t j = 0; j < e->entities.holes.len; ++j) {{\n",
              "      uint32_t hole = e->entities.holes.items[j];\n",
              "      if (i == hole)\n",
              "        goto continue_outer;\n",
              "    }}\n",
              "\n",
              "    vecs_entity_t *ent = &e->entities.items.items[i];\n",
              "    uint32_t gen = e->entities.gens.items[i];\n",
              "    vecs_id_t entity = {{.index = i, .gen = gen}};\n",
            ),
          )?;

          for new_relevant_node in new_nodes {
            let node_t = NodeStructName::new(new_relevant_node.name);
            let node_mask_name = NodeMaskName::new(new_relevant_node.name);

            write!(
              f,
              concat!(
                "    if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "      uint32_t component_index;\n",
                "      {node_t} node;\n",
              ),
              node_t = node_t,
              node_mask_name = node_mask_name,
            )?;

            for component in new_relevant_node.components.iter() {
              write!(
                f,
                concat!(
                  "      {entity_to_component_array_method_get}(&e->entity_to_component_{component_name}, entity, &component_index);\n",
                  "      node.{component_name}_index = component_index;\n",
                ),
                component_name = component,
                entity_to_component_array_method_get =
                  method_name!(&index_hash_array_t, "get"),
              )?;
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
              entity_to_node_method_add = method_name!(&index_hash_array_t, "add"),
            )?;
          }

          write!(f, concat!("continue_outer:\n", "    ;\n", "  }}\n",),)?;
        }

        write!(f, "}}\n",)?;
      }

      // State loops:
      write!(
        f,
        "void vecs_run_state_{}(vecs_engine_t *e) {{\n",
        state.name
      )?;

      for event in self.data.events.values() {
        let event_t = EventStructName::new(event.name);
        let event_queue = DynQueue::new(event_t.clone());
        let event_queue_name = event_queue.get_type();

        write!(f, "  while (e->events_{}.len > 0) {{\n", event.name)?;
        write!(
          f,
          "    {} ev = {}(&e->events_{});\n",
          event_t,
          method_name!(&event_queue_name, "dequeue"),
          event.name,
        )?;

        for system_layer in state.systems.iter() {
          for system_name in system_layer {
            let system = self
              .data
              .systems
              .get(system_name)
              .expect("failed to find system in state");

            if system.event == event.name {
              let node_t = NodeStructName::new(system.node);

              write!(
                f,
                concat!(
                  "    for (size_t i = 0; i < e->nodes_{node_name}.len; ++i) {{\n",
                  "      {node_t} *node = &e->nodes_{node_name}.items[i];\n",
                  "      {system_name}(e, *node, ev);\n",
                  "    }}\n",
                ),
                node_name = system.node,
                node_t = node_t,
                system_name = system.name,
              )?;
            }
          }
        }
        write!(f, "  }}\n")?;
      }

      write!(f, "}}\n")?;
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
