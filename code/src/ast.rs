use std::ops::Range;

use rand::{Rng, RngExt};

pub type NodePtr = Box<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    Var(usize),
    Const(f64),
    Unary(UnaryOp, NodePtr),
    Binary(BinaryOp, NodePtr, NodePtr),
}

pub const GROW_OPERATOR_PROB: f64 = 0.7;
pub const BINARY_OP_PROB: f64 = 0.7;
pub const VAR_PROB: f64 = 0.5;

#[derive(Clone, Copy, Debug)]
pub enum TreeGenMethod {
    Grow,
    Full,
}

pub struct TreeGenerator<'a, R: Rng> {
    rng: &'a mut R,
    ops: &'a OperatorSet,
    const_range: Range<f64>,
    max_depth: usize,
    vars_len: usize,
}

pub struct OperatorSet {
    unary: Vec<UnaryOp>,
    binary: Vec<BinaryOp>,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Neg,
    Abs,
    Sin,
    Cos,
    Tan,
    Cot,
    Exp,
    Ln,
}

impl UnaryOp {
    pub const ALL: [UnaryOp; 8] = [
        UnaryOp::Neg,
        UnaryOp::Abs,
        UnaryOp::Sin,
        UnaryOp::Cos,
        UnaryOp::Tan,
        UnaryOp::Cot,
        UnaryOp::Exp,
        UnaryOp::Ln,
    ];

    pub fn evaluate(self, val: f64) -> f64 {
        match self {
            Self::Neg => -val,
            Self::Abs => val.abs(),
            Self::Sin => val.sin(),
            Self::Cos => val.cos(),
            Self::Tan => val.tan().clamp(-1e12, 1e12),
            Self::Cot => {
                let t = val.tan();
                if t.abs() > 1e-6 {
                    1.0 / t
                } else {
                    1.0 / (t.signum() * 1e-6)
                }
            }
            Self::Ln => {
                if val.abs() > 1e-9 {
                    val.abs().ln()
                } else {
                    0.0
                }
            }
            Self::Exp => val.clamp(-50.0, 50.0).exp(),
        }
    }
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Neg => "-",
            Self::Abs => "",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Cot => "cot",
            Self::Exp => "exp",
            Self::Ln => "ln",
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self::ALL[rng.random_range(0..Self::ALL.len())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
impl BinaryOp {
    pub const ALL: [BinaryOp; 5] = [
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Div,
        BinaryOp::Pow,
    ];

    pub fn evaluate(self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => {
                if right.abs() > 1e-6 {
                    left / right
                } else {
                    left / (right.signum() * 1e-6)
                }
            }
            Self::Pow => {
                let r = left.powf(right);
                let r = if r.is_nan() {
                    left.abs().powf(right)
                } else {
                    r
                };
                r.clamp(-1e12, 1e12)
            }
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Pow => "^",
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self::ALL[rng.random_range(0..Self::ALL.len())]
    }
}

impl Node {
    pub fn evaluate(&self, vars: &[f64]) -> f64 {
        match self {
            Node::Var(idx) => vars[*idx],
            Node::Const(val) => *val,
            Node::Unary(op, node) => op.evaluate(node.evaluate(vars)),
            Node::Binary(op, node, node1) => op.evaluate(node.evaluate(vars), node1.evaluate(vars)),
        }
    }

    pub fn generate_random_tree(
        generator: &mut TreeGenerator<impl Rng>,
        method: &TreeGenMethod,
        current_depth: usize,
    ) -> Node {
        if current_depth >= generator.max_depth {
            Self::generate_terminal(generator)
        } else {
            match method {
                TreeGenMethod::Full => Self::generate_operator(generator, method, current_depth),
                TreeGenMethod::Grow => {
                    //70% operator, 30% terminal
                    if current_depth == 1 || generator.rng.random_bool(GROW_OPERATOR_PROB) {
                        Self::generate_operator(generator, method, current_depth)
                    } else {
                        Self::generate_terminal(generator)
                    }
                }
            }
        }
    }

    fn generate_operator(
        generator: &mut TreeGenerator<impl Rng>,
        method: &TreeGenMethod,
        current_depth: usize,
    ) -> Node {
        // 70% binary, 30% unary
        if generator.rng.random_bool(BINARY_OP_PROB) {
            let ops = &generator.ops.binary;
            let op = ops[generator.rng.random_range(0..ops.len())];
            Node::binary(
                op,
                Self::generate_random_tree(generator, method, current_depth + 1),
                Self::generate_random_tree(generator, method, current_depth + 1),
            )
        } else {
            let ops = &generator.ops.unary;
            let op = ops[generator.rng.random_range(0..ops.len())];
            Node::unary(
                op,
                Self::generate_random_tree(generator, method, current_depth + 1),
            )
        }
    }
    fn generate_terminal(generator: &mut TreeGenerator<impl Rng>) -> Node {
        match generator.rng.random_bool(VAR_PROB) {
            true => Node::Var(generator.rng.random_range(0..generator.vars_len)),
            false => Node::Const(generator.rng.random_range(generator.const_range.clone())),
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Var(_) | Self::Const(_) => 1,
            Node::Unary(_, node) => node.count() + 1,
            Node::Binary(_, left, right) => left.count() + right.count() + 1,
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::Var(_) | Self::Const(_) => 1,
            Node::Unary(_, node) => node.depth() + 1,
            Node::Binary(_, left, right) => left.depth().max(right.depth()) + 1,
        }
    }

    pub fn replace_subtree(&mut self, target_index: usize, new_subtree: Node) {
        let found = self.replace_subtree_rec(&mut 0, target_index, &mut Some(new_subtree));
        assert!(found, "index out of bounds");
    }

    fn replace_subtree_rec(
        &mut self,
        current_index: &mut usize,
        target_index: usize,
        new_subtree: &mut Option<Node>,
    ) -> bool {
        if *current_index == target_index {
            *self = new_subtree.take().expect("already taken");
            return true;
        } else {
            *current_index += 1;
            match self {
                Node::Var(_) | Node::Const(_) => false,
                Node::Unary(_, node) => {
                    node.replace_subtree_rec(current_index, target_index, new_subtree)
                }
                Node::Binary(_, left, right) => {
                    left.replace_subtree_rec(current_index, target_index, new_subtree)
                        || right.replace_subtree_rec(current_index, target_index, new_subtree)
                }
            }
        }
    }

    pub fn get_subtree(&self, target_index: usize) -> &Node {
        self.get_subtree_rec(&mut 0, target_index)
            .expect("index out of bounds")
    }

    fn get_subtree_rec(&self, current_index: &mut usize, target_index: usize) -> Option<&Node> {
        if *current_index == target_index {
            return Some(self);
        } else {
            *current_index += 1;
            match self {
                Node::Var(_) | Node::Const(_) => None,
                Node::Unary(_, node) => node.get_subtree_rec(current_index, target_index),
                Node::Binary(_, left, right) => left
                    .get_subtree_rec(current_index, target_index)
                    .or_else(|| right.get_subtree_rec(current_index, target_index)),
            }
        }
    }

    pub fn get_subtree_mut(&mut self, target_index: usize) -> &mut Node {
        self.get_subtree_mut_rec(&mut 0, target_index)
            .expect("index out of bounds")
    }

    fn get_subtree_mut_rec(
        &mut self,
        current_index: &mut usize,
        target_index: usize,
    ) -> Option<&mut Node> {
        if *current_index == target_index {
            return Some(self);
        } else {
            *current_index += 1;
            match self {
                Node::Var(_) | Node::Const(_) => None,
                Node::Unary(_, node) => node.get_subtree_mut_rec(current_index, target_index),
                Node::Binary(_, left, right) => left
                    .get_subtree_mut_rec(current_index, target_index)
                    .or_else(|| right.get_subtree_mut_rec(current_index, target_index)),
            }
        }
    }

    pub fn unary(op: UnaryOp, node: Node) -> Node {
        Node::Unary(op, Box::new(node))
    }
    pub fn binary(op: BinaryOp, node: Node, node1: Node) -> Node {
        Node::Binary(op, Box::new(node), Box::new(node1))
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Var(idx) => write!(f, "v{}", idx),
            Node::Const(val) => write!(f, "{:.4}", val),
            Node::Unary(op, node) => match op {
                UnaryOp::Abs => write!(f, "|{}|", node),
                _ => write!(f, "{}({})", op.symbol(), node),
            },
            Node::Binary(op, node, node1) => write!(f, "({} {} {})", node, op.symbol(), node1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node::{Const, Var};

    #[test]
    fn test_node_display() {
        let equation = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Const(2.0), Var(0)),
            Var(0),
        );

        let expected = String::from("((2.0000 ^ v0) + v0)");
        let result = format!("{}", equation);

        assert_eq!(
            result, expected,
            "node display failed for ((2.0000 ^ v0) + v0)"
        )
    }

    #[test]
    fn test_evaluate() {
        // 2^x + x
        let equation = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Const(2.0), Var(0)),
            Var(0),
        );

        let x = 3.0;
        let expected = 11.0;
        let result = equation.evaluate(&[x]);

        assert_eq!(result, expected, "evaluation of 2^v0 + x failed for v0=3.0");
    }

    fn with_generator(max_depth: usize, f: impl FnOnce(&mut TreeGenerator<rand::rngs::ThreadRng>)) {
        let mut rng = rand::rng();
        let ops = OperatorSet {
            unary: UnaryOp::ALL.to_vec(),
            binary: BinaryOp::ALL.to_vec(),
        };
        let mut generator = TreeGenerator {
            rng: &mut rng,
            ops: &ops,
            const_range: -10.0..10.0,
            max_depth,
            vars_len: 2,
        };
        f(&mut generator);
    }

    #[test]
    fn test_generate_full_tree() {
        for max_depth in 1..=6 {
            with_generator(max_depth, |generator| {
                for _ in 0..50 {
                    let tree = Node::generate_random_tree(generator, &TreeGenMethod::Full, 1);
                    assert_eq!(tree.depth(), max_depth, "max_depth={max_depth}: {tree}");
                }
            });
        }
    }

    #[test]
    fn test_generate_grow_tree() {
        for max_depth in 1..=6 {
            with_generator(max_depth, |generator| {
                for _ in 0..50 {
                    let tree = Node::generate_random_tree(generator, &TreeGenMethod::Grow, 1);
                    let min_depth = max_depth.min(2);
                    assert!(
                        (min_depth..=max_depth).contains(&tree.depth()),
                        "max_depth={max_depth}: {tree}"
                    );
                }
            });
        }
    }

    #[test]
    fn test_protected_ops() {
        assert!(BinaryOp::Div.evaluate(1.0, 0.0).is_finite());
        assert_eq!(BinaryOp::Pow.evaluate(0.0, 0.0), 1.0);
        assert_eq!(BinaryOp::Pow.evaluate(-2.0, 2.0), 4.0);
        assert_eq!(BinaryOp::Pow.evaluate(-4.0, 0.5), 2.0);
        assert_eq!(BinaryOp::Pow.evaluate(0.0, -1.0), 1e12);
        assert_eq!(BinaryOp::Pow.evaluate(10.0, 400.0), 1e12);
        assert!(UnaryOp::Cot.evaluate(0.0).is_finite());
        assert_eq!(UnaryOp::Ln.evaluate(0.0), 0.0);
        assert!(UnaryOp::Exp.evaluate(1e6).is_finite());
    }

    #[test]
    fn test_count() {
        let tree = Node::unary(
            UnaryOp::Sin,
            Node::binary(BinaryOp::Add, Node::Var(0), Node::Const(2.0)),
        );
        assert_eq!(tree.count(), 4);
    }

    #[test]
    fn test_depth() {
        let tree = Node::unary(
            UnaryOp::Sin,
            Node::binary(BinaryOp::Add, Node::Var(0), Node::Const(2.0)),
        );
        assert_eq!(tree.depth(), 3);
    }

    #[test]
    fn test_get_subtree() {
        // 2^x + x
        let tree = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Node::Const(2.0), Node::Var(0)),
            Node::Var(0),
        );
        assert_eq!(format!("{}", tree.get_subtree(0)), "((2.0000 ^ v0) + v0)");
        assert_eq!(format!("{}", tree.get_subtree(1)), "(2.0000 ^ v0)");
        assert_eq!(format!("{}", tree.get_subtree(2)), "2.0000");
    }

    #[test]
    fn test_get_subtree_mut() {
        // 2^x + x
        let mut tree = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Node::Const(2.0), Node::Var(0)),
            Node::Var(0),
        );
        if let Node::Binary(op, _, _) = tree.get_subtree_mut(0) {
            *op = BinaryOp::Mul;
        }
        assert_eq!(format!("{}", tree), "((2.0000 ^ v0) * v0)");
        if let Node::Const(val) = tree.get_subtree_mut(2) {
            *val = 7.5;
        }
        assert_eq!(format!("{}", tree), "((7.5000 ^ v0) * v0)");
    }

    #[test]
    fn test_replace_subtree() {
        // 2^x + x
        let mut tree = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Node::Const(2.0), Node::Var(0)),
            Node::Var(0),
        );
        tree.replace_subtree(2, Node::Const(5.0));
        assert_eq!(format!("{}", tree), "((5.0000 ^ v0) + v0)");
        tree.replace_subtree(1, Node::unary(UnaryOp::Sin, Node::Var(0)));
        assert_eq!(format!("{}", tree), "(sin(v0) + v0)");
        tree.replace_subtree(0, Node::Var(0));
        assert_eq!(format!("{}", tree), "v0");
    }
}
