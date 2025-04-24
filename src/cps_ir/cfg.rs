use super::{Atom, Cont, IR};
use std::collections::HashMap;

#[derive(Clone)]
pub enum NodeInfo<'a> {
    ProgramEntry,
    ProgramExit,
    FunEntry(usize),
    FunExit(usize),
    Common(usize, &'a IR),
}

pub struct Node<'a, L: Lattice> {
    info: NodeInfo<'a>,
    predecessors: Vec<usize>,
    successors: Vec<usize>,
    result_in: L,
    result_out: L,
}

impl<'a, L: Lattice> Node<'a, L> {
    pub fn new(info: NodeInfo<'a>) -> Self {
        Node {
            info,
            predecessors: vec![],
            successors: vec![],
            result_in: L::bottom(),
            result_out: L::bottom(),
        }
    }
    pub fn from(ir: &'a IR) -> Self {
        Self::new(NodeInfo::Common(ir.get_label(), ir))
    }
}

pub trait Lattice: Eq + Clone {
    fn join(a: &Self, b: &Self) -> Self;
    fn bottom() -> Self;
}


pub struct NodePool<'a, L: Lattice> {
    nodes: Vec<Node<'a, L>>,
    worklist: Vec<usize>,
    direction: bool, // true = foward analysis
    // false = backward analysis
    constraint_fun: Box<dyn FnMut(usize,&'a IR, L) -> L>,
    fun_entry_table: HashMap<usize, usize>,
    fun_exit_table: HashMap<usize, usize>,
    cont_table: HashMap<String, usize>,
    prog_entry: usize,
    prog_exit: usize,
}

impl<'a, L: Lattice> NodePool<'a, L> {
    pub fn new(direction: bool, constraint_fun: Box<dyn FnMut(usize,&'a IR, L) -> L>) -> Self {
        NodePool {
            nodes: vec![],
            worklist: vec![],
            direction,
            constraint_fun,
            fun_entry_table: HashMap::new(),
            fun_exit_table: HashMap::new(),
            cont_table: HashMap::new(),
            prog_entry: 0,
            prog_exit: 0,
        }
    }
    pub fn new_node(&mut self, node: Node<'a, L>) -> usize {
        let count = self.nodes.len();
        self.nodes.push(node);
        count
    }

    pub fn push_worklist(&mut self, n: usize) {
        self.worklist.push(n);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].successors.push(to);
        self.nodes[to].predecessors.push(from);
    }

    
    fn construct_atom(&mut self, a: &'a Atom) {
        match a {
            Atom::Lam(label, _args, body) => {
                let fun_entry_node = self.new_node(Node::new(NodeInfo::FunEntry(*label)));
                let fun_exit_node = self.new_node(Node::new(NodeInfo::FunExit(*label)));
                self.fun_entry_table.insert(*label, fun_entry_node);
                self.fun_exit_table.insert(*label, fun_exit_node);
                let body_node = self.new_node(Node::from(body));
                self.add_edge(fun_entry_node, body_node);
                self.construct_intra_inner(body, body_node, fun_exit_node);
            }
            _ => (),
        }
    }

    pub fn construct_intra(&mut self, ir: &'a IR) {
        let prog_entry = self.new_node(Node::new(NodeInfo::ProgramEntry));
        let prog_exit = self.new_node(Node::new(NodeInfo::ProgramExit));
        self.prog_entry = prog_entry;
        self.prog_exit = prog_exit;

        let start_node = self.new_node(Node::from(ir));
        self.add_edge(prog_entry, start_node);
        self.construct_intra_inner(ir, start_node, prog_exit);
    }

    fn construct_intra_inner(&mut self, ir: &'a IR, node: usize, exit: usize) {
        match ir {
            IR::App(_label, f, args, cont) => {
                self.construct_atom(f);
                for arg in args {
                    self.construct_atom(arg);
                }
                match cont {
                    Cont::Named(s) => {
                        let Some(cont_node) = self.cont_table.get(s) else {
                            panic!("continuation not found,named : {}", s);
                        };
                        self.add_edge(node, *cont_node);
                    }
                    Cont::Return => {
                        self.add_edge(node, exit);
                    }
                }
            }
            IR::AppCont(_label, cont, args) => {
                for arg in args {
                    self.construct_atom(arg);
                }
                match cont {
                    Cont::Named(s) => {
                        let Some(cont_node) = self.cont_table.get(s) else {
                            panic!("continuation not found,named : {}", s);
                        };
                        self.add_edge(node, *cont_node);
                    }
                    Cont::Return => {
                        self.add_edge(node, exit);
                    }
                }
            }
            IR::Fix(_label, _vars, vals, body) => {
                for val in vals {
                    self.construct_atom(val);
                }
                let body_node = self.new_node(Node::from(body));
                self.add_edge(node, body_node);
                self.construct_intra_inner(body, body_node, exit);
            }

            IR::If(_label, test, then_, else_) => {
                self.construct_atom(test);
                let then_node = self.new_node(Node::from(then_));
                let else_node = self.new_node(Node::from(else_));
                self.add_edge(node, then_node);
                self.add_edge(node, else_node);
                self.construct_intra_inner(then_, then_node, exit);
                self.construct_intra_inner(else_, else_node, exit);
            }
            IR::Let(_label, _var, _op, args, body) => {
                for arg in args {
                    self.construct_atom(arg);
                }
                let body_node = self.new_node(Node::from(body));
                self.add_edge(node, body_node);
                self.construct_intra_inner(body, body_node, exit);
            }
            IR::LetVal(_label, _var, atm, body) => {
                self.construct_atom(atm);
                let body_node = self.new_node(Node::from(body));
                self.add_edge(node, body_node);
                self.construct_intra_inner(body, body_node, exit);
            }
            IR::LetCont(_label, cont_name, _args, cont_body, body) => {
                let body_node = self.new_node(Node::from(body));
                self.add_edge(node, body_node);
                let cont_body_node = self.new_node(Node::from(cont_body));
                self.cont_table.insert(cont_name.clone(), cont_body_node);
                self.construct_intra_inner(body, body_node, exit);
                self.construct_intra_inner(cont_body, cont_body_node, exit);
            }
        }
    }

    pub fn run_worklist(&mut self) {
        self.worklist = (0..self.nodes.len()).collect();
        while !self.worklist.is_empty() {
            let Some(node) = self.worklist.pop() else {
                unreachable!()
            };
            let input = self.nodes[node].result_in.clone();
            let result = match self.nodes[node].info {
                NodeInfo::ProgramEntry => input,
                NodeInfo::ProgramExit => input,
                NodeInfo::FunEntry(_) =>input,
                NodeInfo::FunExit(_)=>input,
                NodeInfo::Common(label,ir) => (self.constraint_fun)(label,ir, input)
            };
            if &result != &self.nodes[node].result_out {
                self.nodes[node].result_out = result;
                let successors = if self.direction {
                    self.nodes[node].successors.clone()
                } else {
                    self.nodes[node].predecessors.clone()
                };
                for succ in successors {
                    let new = L::join(&self.nodes[succ].result_in, &self.nodes[node].result_out);
                    if &new != &self.nodes[succ].result_in {
                        self.nodes[succ].result_in = new;
                        self.push_worklist(succ);
                    }
                }
            }
        }
    }
}
