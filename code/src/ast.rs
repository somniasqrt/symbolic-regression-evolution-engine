use std::{fmt::{Display}, ops::Range};

use rand::{Rng, RngExt, random_range, rng};

pub type NodePtr = Box<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    Var,
    Const(f64),
    Unary(UnaryOp, NodePtr),
    Binary(BinaryOp, NodePtr, NodePtr),
}

#[derive(Clone, Copy, Debug)]
pub enum TreeGrowMethon {
    Grow,
    Full
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

    pub const ALL: [UnaryOp; 6] = [UnaryOp::Neg, UnaryOp::Abs, UnaryOp::Sin, UnaryOp::Cos, UnaryOp::Tan, UnaryOp::Cot,];
        
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
            Self::Abs => "abs",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Cot => "cot",
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

    pub const ALL: [BinaryOp; 6] = [BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul, BinaryOp::Div, BinaryOp::Pow, BinaryOp::Log,];
        
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

    pub fn generate_random_tree(rng: &mut impl Rng, current_depth: usize, max_depth: usize, const_range: (f64, f64)) -> Node {
        if current_depth >= max_depth {
            match rng.random_bool(0.5) {
                true => Node::Var,
                false => Node::Const(rng.random_range(const_range.0..const_range.1)),
            }
        }
        else {
            match rng.random_bool(0.3){
                true => {
                    let op = UnaryOp::random(rng);
                    let node = Node::generate_random_tree(rng, current_depth + 1, max_depth, const_range);
                    Node::unary(op, node)
                },
                false => {
                    let op = BinaryOp::random(rng);
                    let node = Node::generate_random_tree(rng, current_depth + 1, max_depth, const_range);
                    let node1 = Node::generate_random_tree(rng, current_depth + 1, max_depth, const_range);
                    Node::binary(op, node, node1)
                },
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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Var => write!(f, "x"),
            Node::Const(val) => write!(f, "{:.4}", val),
            Node::Unary(op, node) => write!(f, "{}({})", op.symbol(), node),
            Node::Binary(op, node, node1) => write!(f, "({} {} {})", node, op.symbol(), node1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Node::{Const, Var};

use super::*;

    #[test]
    fn test_node_display() {
        let equation = Node::binary(BinaryOp::Add, Node::binary(BinaryOp::Pow, Const(2.0), Var), Var);

        let expected = String::from("((2.0000 ^ x) + x)");
        let result = format!("{}", equation);

        assert_eq!(result, expected, "node display failed for ((2.0000 ^ x) + x)")
        
    }

    #[test]
    fn test_evaluate() {
        // 2^x + x
        let equation = Node::binary(BinaryOp::Add, Node::binary(BinaryOp::Pow, Const(2.0), Var), Var);
        
        let x = 3.0;
        let expected = 11.0;
        let result = equation.evaluate(x);

        assert_eq!(result, expected, "evaluation of 2^x + x failed for x=3.0");
    }
    
    #[test]
    fn test_generate_random_tree() {
        let tree = Node::generate_random_tree(&mut rng(), 0, 3, (-10.0, 10.0));
        println!("{}", tree)
    }
}