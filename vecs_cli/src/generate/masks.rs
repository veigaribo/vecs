use std::fmt::Display;

use derive_display_hash::DisplayHash;

use crate::resolve::cst::{Component, Node};

#[derive(Debug, Clone, DisplayHash)]
pub struct ComponentMaskName<'a> {
  pub name: &'a str,
}

impl<'a> ComponentMaskName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { name }
  }
}

impl<'a> Display for ComponentMaskName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_component_{}_mask", self.name)
  }
}

#[derive(Debug, Clone, DisplayHash)]
pub struct NodeMaskName<'a> {
  pub name: &'a str,
}

impl<'a> NodeMaskName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { name }
  }
}

impl<'a> Display for NodeMaskName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_node_{}_mask", self.name)
  }
}

// Formats masks of components.
pub struct ComponentMask {
  pub mask_size: u16,
  pub mask_i: u16,
  pub mask_j: u8,
}

impl ComponentMask {
  pub fn from_component(c: &Component, mask_size: u16) -> Self {
    Self {
      mask_size,
      mask_i: c.mask_i,
      mask_j: c.mask_j,
    }
  }
}

impl Display for ComponentMask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.mask_size == 0 {
      return "{}".fmt(f);
    }

    write!(f, "{{")?;

    if self.mask_i == 0 {
      write!(f, "{:#x}", 1 << self.mask_j)?;
    } else {
      write!(f, "0")?;
    }

    for i in 1..self.mask_size {
      if i == self.mask_i.into() {
        write!(f, ", {:#x}", 1 << self.mask_j)?;
      } else {
        write!(f, ", 0")?;
      }
    }

    write!(f, "}}")?;
    Ok(())
  }
}

// Formats masks of nodes.
pub struct NodeMask {
  pub mask_size: u16,
  pub components: Vec<u64>,
}

impl NodeMask {
  pub fn from_node(n: &Node, mask_size: u16) -> Self {
    Self {
      mask_size,
      components: n.mask.clone(),
    }
  }
}

impl Display for NodeMask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.mask_size == 0 {
      return write!(f, "{{}}");
    }

    write!(f, "{{")?;
    write!(f, "{:#x}", self.components.get(0).unwrap_or(&0))?;

    for i in 1..self.mask_size {
      write!(f, ", {:#x}", self.components.get(i as usize).unwrap_or(&0))?;
    }

    write!(f, "}}")?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::ComponentMask;

  #[test]
  fn test_fmt_component_mask() {
    let fmter = ComponentMask {
      mask_size: 1,
      mask_i: 0,
      mask_j: 6,
    };
    let fmted = format!("{}", fmter);
    assert_eq!(fmted, "{0x40}");

    let fmter = ComponentMask {
      mask_size: 6,
      mask_i: 4,
      mask_j: 20,
    };
    let fmted = format!("{}", fmter);
    assert_eq!(fmted, "{0, 0, 0, 0, 0x100000, 0}");
  }
}
