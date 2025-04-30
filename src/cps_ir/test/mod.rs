use crate::cps_ir::builtin_call::BuiltinOp;

use super::{BuilderExpr as E, quick_cps};
use super::{IR, Store, Value, interp};
use std::collections::HashMap;

fn simple_interp<'a>(ir: &'a IR) -> Value<'a> {
    let env = HashMap::new();
    let mut store = Store::new();
    let res = interp(&ir, env, &mut store);
    res
}
#[test]
pub fn test1() {
    let fact = E::lam(
        &["x"],
        E::if_(
            E::papp(BuiltinOp::I32Leq, vec![E::v("x"), E::i32(1)]),
            E::i32(1),
            E::papp(
                BuiltinOp::I32Mul,
                vec![
                    E::app(
                        E::v("fact"),
                        vec![E::papp(BuiltinOp::I32Sub, vec![E::v("x"), E::i32(1)])],
                    ),
                    E::v("x"),
                ],
            ),
        ),
    );
    let prog = E::fix(&["fact"], vec![fact], E::app(E::v("fact"), vec![E::i32(5)]));
    let ir = quick_cps(prog);
    assert_eq!(simple_interp(&ir), Value::I32(120))
}
