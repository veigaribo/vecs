pub mod component;
pub mod cst;
pub mod event;
pub mod include;
pub mod node;
pub mod result;
pub mod setting;
pub mod state;
pub mod strukt;
pub mod system;
pub mod values;

use cst::StructField;
use node::resolve_node;
use setting::resolve_setting;

use crate::{
  parse::{ast::Ast, data::str::Span},
  resolve::{
    component::resolve_component,
    cst::Cst,
    event::resolve_event,
    include::resolve_include,
    result::{ResolveError, ResolveResult},
    state::resolve_state,
    system::resolve_system,
    values::{ValueKind, VarTable},
  },
};

// This bundles information that is communicated between functions but is only used
// in error messages.
#[derive(Debug, Copy, Clone)]
pub struct ResolveMeta<'src, 'a> {
  pub cst: &'a Cst<'src>,
  pub span: Span<'src>,
}

pub fn resolve<'src>(ast: Ast<'src>) -> ResolveResult<'src, Cst<'src>> {
  let table = VarTable::<'src>::new();
  let mut cst = Cst::default();

  // The default `frame` event.
  cst.add_event(cst::Struct {
    span: Span::default(),
    name: "frame",
    fields: vec![
      StructField {
        typ: vec!["float"],
        name: "delta",
      },
      StructField {
        typ: vec!["double"],
        name: "runtime",
      },
      StructField {
        typ: vec!["uint64_t"],
        name: "frame",
      },
    ],
  });

  let exprs = ast.0;

  for expr in exprs {
    let span = expr.span.clone();
    let info = ResolveMeta { span, cst: &cst };

    let application = table.resolve(expr)?;

    if let ValueKind::Application(mut els) = application.kind {
      if els.is_empty() {
        // TODO: Can this happen?
        continue;
      }

      let car = els.pop_front().unwrap();

      if car.kind == ValueKind::Symbol("component") {
        let component = resolve_component(info, els)?;
        cst.add_component(component);
      } else if car.kind == ValueKind::Symbol("event") {
        let event = resolve_event(info, els)?;
        cst.add_event(event);
      } else if car.kind == ValueKind::Symbol("node") {
        let node = resolve_node(info, els)?;
        cst.add_node(node);
      } else if car.kind == ValueKind::Symbol("system") {
        let (system, node) = resolve_system(info, els)?;
        cst.add_system(system);
        cst.add_node(node);
      } else if car.kind == ValueKind::Symbol("state") {
        let state = resolve_state(info, els)?;
        cst.add_state(state);
      } else if car.kind == ValueKind::Symbol("include") {
        let include = resolve_include(info, els)?;
        cst.add_include(include);
      } else if car.kind == ValueKind::Symbol("set") {
        resolve_setting(info.span, els, &mut cst)?;
      } else if let ValueKind::Symbol(_) = car.kind {
        return Err(ResolveError::new(car.span, format!("unknown tag {}", car)));
      } else {
        return Err(ResolveError::new(
          car.span,
          format!(
            "expected a tag: `component`, `event`, `node`, `system`, `state`, `include` or `set`. instead found {}",
            car,
          ),
        ));
      }
    } else {
      panic!(
        "malformed ast: root expression is not an application. this is a bug. run with VECS_DEBUG_AST set to dump the AST",
      );
    }
  }

  Ok(cst)
}
