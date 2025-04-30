// Available Expression Analysis,a must, forward analysis
// Set(Expression) , /\

use crate::cps_ir::{
    Atom, IR,
    builtin_call::BuiltinOp,
    cfg::{self, Lattice, NodePool},
};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression(BuiltinOp, Vec<Atom>); // shouldn't contain Atom::Lam

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionLattice {
    Bot,
    Exprs(HashSet<Expression>),
}

impl ExpressionLattice {
    pub fn from(op: BuiltinOp, args: Vec<Atom>) -> Self {
        let mut a = HashSet::new();
        a.insert(Expression(op, args));
        ExpressionLattice::Exprs(a)
    }
}

impl cfg::Lattice for ExpressionLattice {
    fn join(x: &Self, y: &Self) -> Self {
        match (x, y) {
            (a, ExpressionLattice::Bot) => a.clone(),
            (ExpressionLattice::Bot, b) => b.clone(),
            (ExpressionLattice::Exprs(a), ExpressionLattice::Exprs(b)) => {
                ExpressionLattice::Exprs(a.intersection(b).cloned().collect::<HashSet<_>>())
            }
        }
    }
    fn bottom() -> Self {
        ExpressionLattice::Bot
    }
}

pub fn make_analysis<'a>() -> NodePool<'a, ExpressionLattice> {
    NodePool::new(
        true,
        Box::new(|_label, ir, lattice| match ir {
            IR::Let(_, _var, op, args, _body) => ExpressionLattice::join(
                &lattice,
                &ExpressionLattice::from(op.clone(), args.clone()),
            ),
            _ => lattice,
        }),
    )
}
