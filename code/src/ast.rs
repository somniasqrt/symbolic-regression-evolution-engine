use std::ops::{Deref, Range};

use rand::{Rng, RngExt};

pub type NodePtr = Box<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    Var,
    Const(f64),
    Unary(UnaryOp, NodePtr),
    Binary(BinaryOp, NodePtr, NodePtr),
}

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
}

impl UnaryOp {
    pub const ALL: [UnaryOp; 6] = [
        UnaryOp::Neg,
        UnaryOp::Abs,
        UnaryOp::Sin,
        UnaryOp::Cos,
        UnaryOp::Tan,
        UnaryOp::Cot,
    ];

    pub fn evaluate(self, val: f64) -> f64 {
        match self {
            Self::Neg => -val,
            Self::Abs => val.abs(),
            Self::Sin => val.sin(),
            Self::Cos => val.cos(),
            Self::Tan => val.tan(),
            Self::Cot => 1.0 / val.tan(),
        }
    }
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Neg => "-",
            // Self::Abs => "abs",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Cot => "cot",
            _ => unreachable!(),
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
    Log,
}
impl BinaryOp {
    pub const ALL: [BinaryOp; 6] = [
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Div,
        BinaryOp::Pow,
        BinaryOp::Log,
    ];

    pub fn evaluate(self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
            Self::Pow => left.powf(right),
            Self::Log => right.log(left),
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Pow => "^",
            Self::Log => "log",
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self::ALL[rng.random_range(0..Self::ALL.len())]
    }
}

impl Node {
    pub const LENGTH: u32 = 14;

    pub fn evaluate(&self, var: f64) -> f64 {
        match self {
            Node::Var => var,
            Node::Const(val) => *val,
            Node::Unary(op, node) => op.evaluate(node.evaluate(var)),
            Node::Binary(op, node, node1) => op.evaluate(node.evaluate(var), node1.evaluate(var)),
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
                    if generator.rng.random_bool(0.7) {
                        Self::generate_operator(generator, method, current_depth)
                    }
                    else {
                        Self::generate_terminal(generator)
                    }
                },
            }
        }
    }

    fn generate_operator(generator: &mut TreeGenerator<impl Rng>, method: &TreeGenMethod, current_depth: usize) -> Node {
        // 70% binary, 30% unary
        if generator.rng.random_bool(0.7) {
            let ops = &generator.ops.binary;
            let op = ops[generator.rng.random_range(0..ops.len())];
            Node::binary(
                op, 
                Self::generate_random_tree(generator, method, current_depth + 1), 
                Self::generate_random_tree(generator, method, current_depth + 1)
            )
        } 
        else {
            let ops = &generator.ops.unary;
            let op = ops[generator.rng.random_range(0..ops.len())];
            Node::unary(
                op, 
                Self::generate_random_tree(generator, method, current_depth + 1)
            )
        }
    }
    fn generate_terminal(generator: &mut TreeGenerator<impl Rng>) -> Node {
        match generator.rng.random_bool(0.5) {
            true => Node::Var,
            false => Node::Const(generator.rng.random_range(generator.const_range.clone())),
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
            Node::Var => write!(f, "x"),
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
    use std::thread;

use crate::ast::Node::{Const, Var};

    use super::*;

    #[test]
    fn test_node_display() {
        let equation = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Const(2.0), Var),
            Var,
        );

        let expected = String::from("((2.0000 ^ x) + x)");
        let result = format!("{}", equation);

        assert_eq!(
            result, expected,
            "node display failed for ((2.0000 ^ x) + x)"
        )
    }

    #[test]
    fn test_evaluate() {
        // 2^x + x
        let equation = Node::binary(
            BinaryOp::Add,
            Node::binary(BinaryOp::Pow, Const(2.0), Var),
            Var,
        );

        let x = 3.0;
        let expected = 11.0;
        let result = equation.evaluate(x);

        assert_eq!(result, expected, "evaluation of 2^x + x failed for x=3.0");
    }

    #[test]
    fn test_generate_random_tree() {
        let mut rng = rand::rng();
        let ops = OperatorSet {
            unary: UnaryOp::ALL.to_vec(),
            binary: BinaryOp::ALL.to_vec(),
        };
        let mut generator = TreeGenerator {
            rng: &mut rng,
            ops: &ops,
            const_range: -10.0..10.0,
            max_depth: 3,
        };
        let tree_full= Node::generate_random_tree(&mut generator, &TreeGenMethod::Full, 0);
        println!("tree_full -> {}", tree_full);
        let tree_grow = Node::generate_random_tree(&mut generator, &TreeGenMethod::Grow, 0);
        println!("tree_grow -> {}", tree_grow);
    }
}