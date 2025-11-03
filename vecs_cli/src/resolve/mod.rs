pub mod cst;
pub mod result;
pub mod strukt;
pub mod system;
pub mod values;

use crate::{
  parse::{ast::Ast, data::str::Span},
  resolve::{
    cst::Cst,
    result::{ResolveError, ResolveResult},
    strukt::resolve_struct,
    system::resolve_system,
    values::{ValueKind, VarTable},
  },
};

// This bundles information that is communicated between functions but is only used
// in error messages.
#[derive(Debug, Copy, Clone)]
pub struct ResolveMeta<'a, 'b, 'c> {
  pub ast: &'b Ast<'a>,
  pub span: Span<'a>,
  pub state: &'c Cst<'a>,
}

pub fn resolve<'a>(ast: Ast<'a>) -> ResolveResult<'a, Cst<'a>> {
  let table = VarTable::<'a>::new();
  let mut cst = Cst::default();

  let exprs = &ast.0;

  for expr in exprs {
    let span = expr.span;

    let info = ResolveMeta {
      span,
      ast: &ast,
      state: &cst,
    };

    let application = table.resolve(expr)?;

    if let ValueKind::List(els) = application.kind {
      if els.is_empty() {
        // TODO: Can this happen?
        continue;
      }

      let car = &els[0];
      let cdr = &els[1..];

      if car.kind == ValueKind::Symbol("component") {
        let component = resolve_struct(info, cdr)?;
        cst.add_component(component);
      } else if car.kind == ValueKind::Symbol("event") {
        let event = resolve_struct(info, cdr)?;
        cst.add_event(event);
      } else if car.kind == ValueKind::Symbol("system") {
        let system = resolve_system(info, cdr)?;
        cst.add_system(system);
      } else {
        return Err(ResolveError::new(car.span, "operation not found"));
      }
    } else {
      panic!(
        "malformed ast: root expression is not a list. this is a bug.\n{}",
        ast
      );
    }
  }

  Ok(cst)
}
