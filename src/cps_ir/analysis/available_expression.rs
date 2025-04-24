// Available Expression Analysis,a must, forward analysis
// Set(Expression) , /\

use crate::cps_ir::{IR,Atom,cfg};
use std::collections::HashSet;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Expression(String, Vec<Atom>); // atom here shouldn't contain Atom::Lam

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ExpressionLattice {
    Bot,
    Exprs(HashSet<Expression>)
}

impl cfg::Lattice for ExpressionLattice {
    fn join(x:&Self,y:&Self) -> Self{
        match (x,y) {
            (a, ExpressionLattice::Bot) => a.clone(),
            (ExpressionLattice::Bot, b) => b.clone(),
            (ExpressionLattice::Exprs(a),ExpressionLattice::Exprs(b)) => 
                ExpressionLattice::Exprs(a.intersection(b).cloned().collect::<HashSet<_>>()),
        }
        
    }
    fn bottom() -> Self {
        ExpressionLattice::Bot
    }
}




