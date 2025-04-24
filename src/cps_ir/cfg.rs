
use super::IR;

#[derive(Clone)]
pub enum NodeInfo<'a> {
    ProgramEntry,
    ProgramExit,
    FunEntry(usize), //
    FunExit(usize),  // usize here refers to Atom::Lam(args, IR::node(label,...))
    //                                               -----
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
    constraint_fun: Box<dyn FnMut(NodeInfo<'a>, L) -> L>,
}

impl<'a, L: Lattice> NodePool<'a, L> {
    pub fn new(direction: bool, constraint_fun: Box<dyn FnMut(NodeInfo<'a>, L) -> L>) -> Self {
        NodePool {
            nodes: vec![],
            worklist: vec![],
            direction,
            constraint_fun,
        }
    }
    pub fn new_node(&mut self, node: Node<'a,L>) -> usize{
        let count = self.nodes.len();
        self.nodes.push(node);
        count
    }

    pub fn push_worklist(&mut self, n: usize) {
        self.worklist.push(n);
    }

    pub fn add_edge(&mut self, from : usize, to:usize){
        self.nodes[from].successors.push(to);
        self.nodes[to].predecessors.push(from);
    }
    pub fn run_worklist(&mut self) {
        while !self.worklist.is_empty() {
            let Some(node) = self.worklist.pop() else {
                unreachable!()
            };
            let input = self.nodes[node].result_in.clone();
            let result = (self.constraint_fun)(self.nodes[node].info.clone(), input);
            if &result != &self.nodes[node].result_out {
                self.nodes[node].result_out = result;
                let successors =if self.direction {
                    self.nodes[node].successors.clone()
                } else {
                    self.nodes[node].predecessors.clone()
                };
                for succ in successors{
                    let new =L::join(&self.nodes[succ].result_in, &self.nodes[node].result_out);
                    if &new != &self.nodes[succ].result_in {
                        self.nodes[succ].result_in = new;
                        self.push_worklist(succ);
                    }
                }
            }
        }
    }

}
