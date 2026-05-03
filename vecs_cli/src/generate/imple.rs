use std::fmt::Display;

use crate::resolve::cst::Cst;

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName},
  generics::{
    common::FunctionName, dyn_arrays::DynArray, dyn_queue::DynQueue,
    hash_dyn_arrays::HashDynArray, sparse_dyn_arrays::SparseDynArray,
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

    DynArray::new("size_t".to_string()).imple().fmt(f)?;

    for event in self.data.events.values() {
      let event_struct_name = EventStructName { name: event.name };
      let event_t = format!("struct {}", event_struct_name);
      DynQueue::new(event_t).imple().fmt(f)?;
    }

    let hash_fn_name = FunctionName::new("hash", vec!["size_t"]);

    write!(
      f,
      concat!(
        "// http://www.cse.yorku.ca/~oz/hash.html\n",
        "size_t {hash_fn_name}(size_t key) {{\n",
        "  uint64_t hash = 5381;\n",
        "  /* hash * 33 + c */\n",
        "  for (size_t i = 0; i < sizeof(size_t) * 8; i += 8) {{\n",
        "    uint64_t c = (key >> i) & 0xff;\n",
        "    hash = ((hash << 5) + hash) + c;\n",
        "  }}\n",
        "  return hash;\n",
        "}}\n",
      ),
      hash_fn_name = hash_fn_name,
    )?;

    HashDynArray::new(
      "size_t".to_string(),
      "struct vecs_sparse_array_id".to_string(),
    )
    .imple()
    .fmt(f)?;

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_struct_name = ComponentStructName {
        name: component_name,
      };

      let component_t = format!("struct {}", component_struct_name);
      DynArray::new(component_t.clone()).imple().fmt(f)?;
      SparseDynArray::new(component_t.clone()).imple().fmt(f)?;
    }

    for node in self.data.nodes.values() {
      let node_struct_name = NodeStructName { name: node.name };
      let node_t = format!("struct {}", node_struct_name);
      DynArray::new(node_t).imple().fmt(f)?;
    }

    let entity_t = "struct vecs_entity".to_string();
    DynArray::new(entity_t.clone()).imple().fmt(f)?;
    let entity_array = SparseDynArray::new(entity_t);
    entity_array.imple().fmt(f)?;

    Ok(())
  }
}
