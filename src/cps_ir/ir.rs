use super::Atom;
use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
pub enum IR {
    LetCont(usize, String, Vec<String>, Box<IR>, Box<IR>),
    Let(usize, String, String, Vec<Atom>, Box<IR>),
    LetVal(usize, String, Atom, Box<IR>),
    If(usize, Atom, Box<IR>, Box<IR>),
    App(usize, Atom, Vec<Atom>, Cont),
    Fix(usize, Vec<String>, Vec<Atom>, Box<IR>),
    AppCont(usize, Cont, Vec<Atom>),
}

#[derive(Debug)]
pub enum Cont {
    Named(String),
    Return,
}

pub enum BuilderExpr {
    Var(String),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    Bool(bool),
    Char(char),
    StringLiteral(String),
    Lam(Vec<String>, Box<BuilderExpr>),
    App(Box<BuilderExpr>, Vec<BuilderExpr>),
    PrimApp(String, Vec<BuilderExpr>),
    If(Box<BuilderExpr>, Box<BuilderExpr>, Box<BuilderExpr>),
    Fix(Vec<String>, Vec<BuilderExpr>, Box<BuilderExpr>),
    Let(String, Box<BuilderExpr>, Box<BuilderExpr>),
}

impl BuilderExpr {
    pub fn v(v : &str) -> Self{
        BuilderExpr::Var(v.to_string())
    }
    pub fn i32(v: i32) -> Self {
        BuilderExpr::I32(v)
    }
    pub fn i64(v: i64) -> Self {
        BuilderExpr::I64(v)
    }
    pub fn u32(v: u32) -> Self {
        BuilderExpr::U32(v)
    }
    pub fn u64(v: u64) -> Self {
        BuilderExpr::U64(v)
    }
    pub fn bool(v: bool) -> Self {
        BuilderExpr::Bool(v)
    }
    pub fn char(v: char) -> Self {
        BuilderExpr::Char(v)
    }
    pub fn str(v: &str) -> Self {
        BuilderExpr::StringLiteral(v.to_string())
    }
    pub fn lam(vars: &[&str], body: BuilderExpr) -> Self {
        BuilderExpr::Lam(vars.iter().map(|s| s.to_string()).collect(), Box::new(body))
    }
    pub fn app(f: BuilderExpr, args: Vec<BuilderExpr>) -> Self {
        BuilderExpr::App(Box::new(f), args)
    }
    pub fn papp(op: &str, args: Vec<BuilderExpr>) -> Self {
        BuilderExpr::PrimApp(op.to_string(), args)
    }
    pub fn if_(test: BuilderExpr, then_: BuilderExpr, else_: BuilderExpr) -> Self {
        BuilderExpr::If(Box::new(test), Box::new(then_), Box::new(else_))
    }

    pub fn fix(vars: &[&str], vals: Vec<BuilderExpr>, body: BuilderExpr) -> Self {
        let vars_string = vars.iter().map(|s| s.to_string()).collect();
        BuilderExpr::Fix(vars_string, vals, Box::new(body))
    }

    pub fn let_(var: &str, val: BuilderExpr, body: BuilderExpr) -> Self {
        BuilderExpr::Let(var.to_string(), Box::new(val), Box::new(body))
    }
}

pub struct CPSContext {
    label_count: usize,
    cont_count: usize,
    var_count: usize,
}

impl CPSContext {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CPSContext {
            label_count: 0,
            cont_count: 0,
            var_count: 0,
        }))
    }
    fn alloc_label(&mut self) -> usize {
        let label = self.label_count;
        self.label_count += 1;
        label
    }
    fn alloc_cont(&mut self) -> String {
        let cont = format!("g_cont_{}", self.cont_count);
        self.cont_count += 1;
        cont
    }
    fn alloc_var(&mut self) -> String {
        let var = format!("g_var_{}", self.var_count);
        self.var_count += 1;
        var
    }
}

pub fn cps_vec(
    ctx: Rc<RefCell<CPSContext>>,
    mut v: Vec<BuilderExpr>,
    k: Box<dyn FnOnce(Vec<Atom>) -> IR>,
) -> IR {
    if v.is_empty() {
        k(vec![])
    } else {
        let ctx1 = ctx.clone();
        let arg: BuilderExpr = v.remove(0);
        cps(
            ctx,
            arg,
            Box::new(|r| {
                cps_vec(
                    ctx1,
                    v,
                    Box::new(|mut rv| {
                        rv.push(r);
                        k(rv)
                    }),
                )
            }),
        )
    }
}

pub fn cps_lam(ctx: Rc<RefCell<CPSContext>>, term: BuilderExpr) -> Atom {
    match term {
        BuilderExpr::Lam(args, body) => Atom::Lam(
            args,
            Box::new(cps(
                ctx.clone(),
                *body,
                Box::new(move |r| {
                    let label = ctx.borrow_mut().alloc_label();
                    IR::AppCont(label, Cont::Return, vec![r])
                }),
            )),
        ),
        _ => panic!("not a lambda"),
    }
}

pub fn quick_cps(t : BuilderExpr) -> IR{ 
    let ctx = CPSContext::new();
    let label = ctx.borrow_mut().alloc_label();
    cps(ctx,t, Box::new(move |r| IR::AppCont(label, Cont::Return ,vec![r])))
}
pub fn cps(ctx: Rc<RefCell<CPSContext>>, term: BuilderExpr, k: Box<dyn FnOnce(Atom) -> IR>) -> IR {
    match term {
        BuilderExpr::Var(v) => k(Atom::Var(v)),
        BuilderExpr::I32(v) => k(Atom::I32(v)),
        BuilderExpr::I64(v) => k(Atom::I64(v)),
        BuilderExpr::U32(v) => k(Atom::U32(v)),
        BuilderExpr::U64(v) => k(Atom::U64(v)),
        BuilderExpr::Bool(v) => k(Atom::Bool(v)),
        BuilderExpr::Char(c) => k(Atom::Char(c)),
        BuilderExpr::StringLiteral(s) => k(Atom::StringLiteral(s)),
        BuilderExpr::Lam(args, body) => k(cps_lam(ctx, BuilderExpr::Lam(args, body))),
        BuilderExpr::App(f, args) => cps(
            ctx.clone(),
            *f,
            Box::new(|f_| {
                cps_vec(
                    ctx.clone(),
                    args,
                    Box::new(move |mut args_atom| {
                        let label1 = ctx.borrow_mut().alloc_label();
                        let label2 = ctx.borrow_mut().alloc_label();
                        let cont_name = ctx.borrow_mut().alloc_cont();
                        let var_name = ctx.borrow_mut().alloc_var();
                        args_atom.reverse();
                        IR::LetCont(
                            label1,
                            cont_name.clone(),
                            vec![var_name.clone()],
                            Box::new(k(Atom::Var(var_name))),
                            Box::new(IR::App(label2, f_, args_atom, Cont::Named(cont_name))),
                        )
                    }),
                )
            }),
        ),
        BuilderExpr::PrimApp(op, args) => cps_vec(
            ctx.clone(),
            args,
            Box::new(move |mut args_atom| {
                let label = ctx.borrow_mut().alloc_label();
                let var_name = ctx.borrow_mut().alloc_var();
                args_atom.reverse();
                IR::Let(
                    label,
                    var_name.clone(),
                    op,
                    args_atom,
                    Box::new(k(Atom::Var(var_name))),
                )
            }),
        ),
        BuilderExpr::If(test, then_, else_) => cps(
            ctx.clone(),
            *test,
            Box::new(move |test_r| {
                let label = ctx.borrow_mut().alloc_label();
                let label_cont = ctx.borrow_mut().alloc_label();
                let label_test = ctx.borrow_mut().alloc_label();
                let label_else = ctx.borrow_mut().alloc_label();
                let cont = ctx.borrow_mut().alloc_cont();
                let var = ctx.borrow_mut().alloc_var();
                let cont1 = cont.clone();
                let cont2 = cont.clone();
                IR::LetCont(
                    label_cont,
                    cont.clone(),
                    vec![var.clone()],
                    Box::new(k(Atom::Var(var))),
                    Box::new(IR::If(
                        label,
                        test_r,
                        Box::new(cps(
                            ctx.clone(),
                            *then_,
                            Box::new(move |x| IR::AppCont(label_test, Cont::Named(cont1), vec![x])),
                        )),
                        Box::new(cps(
                            ctx.clone(),
                            *else_,
                            Box::new(move |x| IR::AppCont(label_else, Cont::Named(cont2), vec![x])),
                        )),
                    )),
                )
            }),
        ),
        BuilderExpr::Let(var, expr, body) => cps(
            ctx.clone(),
            *expr,
            Box::new(move |r| {
                let label = ctx.borrow_mut().alloc_label();
                IR::LetVal(
                    label,
                    var,
                    r,
                    Box::new(cps(ctx.clone(), *body, Box::new(|bodyr| k(bodyr)))),
                )
            }),
        ),
        BuilderExpr::Fix(vars, mut vals, body) => {
            let label = ctx.borrow_mut().alloc_label();
            IR::Fix(
                label,
                vars,
                vals.drain(..)
                    .map(|val| cps_lam(ctx.clone(), val))
                    .collect(),
                Box::new(cps(ctx.clone(), *body, Box::new(|body_r| k(body_r)))),
            )
        }
    }
}
