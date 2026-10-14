use std::{f64::consts::PI};

use rand::rng;

use crate::ast::{BinaryOp, Node};

mod ast;
mod engine;


fn main() {
    let mut rng = rng();
    // let tree = Node::generate_random_tree(&mut rng, 0, &5, &(-5.0..5.0));
    // println!("{:?}", tree);
    // println!("x = 2.0 -> {}", tree.evaluate(2.0));
    let equation = Node::binary(BinaryOp::Add, Node::binary(BinaryOp::Pow, Node::Const(2.0), Node::unary(ast::UnaryOp::Abs, Node::Var)), Node::Var);

    println!("{}", equation)
}
