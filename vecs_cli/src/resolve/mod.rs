pub mod cst;
pub mod result;
pub mod state;
pub mod strukt;
pub mod system;
pub mod values;

use crate::{
  parse::{ast::Ast, data::str::Span},
  resolve::{
    cst::Cst,
    result::{ResolveError, ResolveResult},
    state::resolve_state,
    strukt::resolve_struct,
    system::resolve_system,
    values::{ValueKind, VarTable},
  },
};

// This bundles information that is communicated between functions but is only used
// in error messages.
#[derive(Debug, Copy, Clone)]
pub struct ResolveMeta<'src, 'a, 'b> {
  pub ast: &'a Ast<'src>,
  pub cst: &'b Cst<'src>,
  pub span: Span<'src>,
}

pub fn resolve<'src>(ast: Ast<'src>) -> ResolveResult<'src, Cst<'src>> {
  let table = VarTable::<'src>::new();
  let mut cst = Cst::default();

  let exprs = &ast.0;

  for expr in exprs {
    let span = expr.span;

    let info = ResolveMeta {
      span,
      ast: &ast,
      cst: &cst,
    };

    let application = table.resolve(expr)?;

    if let ValueKind::Application(els) = application.kind {
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
      } else if car.kind == ValueKind::Symbol("state") {
        let state = resolve_state(info, cdr)?;
        cst.add_state(state);
      } else if let ValueKind::Symbol(_) = car.kind {
        return Err(ResolveError::new(car.span, format!("unknown tag {}", car)));
      } else {
        return Err(ResolveError::new(
          car.span,
          format!(
            "expected a tag: `component`, `event` or `system`. instead found {}",
            car,
          ),
        ));
      }
    } else {
      panic!(
        "malformed ast: root expression is not an application. this is a bug.\n{}",
        ast,
      );
    }
  }

  Ok(cst)
}
