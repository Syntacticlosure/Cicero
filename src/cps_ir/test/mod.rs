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
            E::papp("i32_leq", vec![E::v("x"), E::i32(1)]),
            E::i32(1),
            E::papp(
                "i32_mul",
                vec![
                    E::app(
                        E::v("fact"),
                        vec![E::papp("i32_sub", vec![E::v("x"), E::i32(1)])],
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
