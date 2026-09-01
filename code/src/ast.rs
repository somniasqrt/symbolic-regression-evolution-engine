pub type NodePtr = Box<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    // Leafs
    Var,
    Const(f64),
    
    // Unary
    Neg(NodePtr),
    Abs(NodePtr),
    Sin(NodePtr),
    Cos(NodePtr),
    Tan(NodePtr),
    Cot(NodePtr),
    
    // Binary
    Add(NodePtr, NodePtr),
    Sub(NodePtr, NodePtr),
    Mul(NodePtr, NodePtr),
    Div(NodePtr, NodePtr),
    Pow(NodePtr, NodePtr),
    Log(NodePtr, NodePtr),
}

impl Node {

    pub fn evaluate(&self, val: f64) -> f64 {
        match self {
            // Leafs
            Node::Var => val,
            Node::Const(node) => *node,

            // Unary
            Node::Neg(node) => -node.evaluate(val),
            Node::Abs(node) => node.evaluate(val).abs(),
            Node::Sin(node) => node.evaluate(val).sin(),
            Node::Cos(node) => node.evaluate(val).cos(),
            Node::Tan(node) => node.evaluate(val).tan(),
            Node::Cot(node) => 1.0 / node.evaluate(val).tan(),

            // Binary
            Node::Add(left, right) => left.evaluate(val) + right.evaluate(val),
            Node::Sub(left, right) => left.evaluate(val) - right.evaluate(val),
            Node::Mul(left, right) => left.evaluate(val) * right.evaluate(val),
            Node::Div(left, right) => left.evaluate(val) / right.evaluate(val),
            Node::Pow(left, right) => left.evaluate(val).powf(right.evaluate(val)),
            Node::Log(left, right) => right.evaluate(val).log(left.evaluate(val)),

        }
    }

    // Unary ops
    pub fn neg(node: Node) -> Node {
        Node::Neg(Box::new(node))
    }
    pub fn abs(node: Node) -> Node {
        Node::Abs(Box::new(node))
    }
    pub fn sin(node: Node) -> Node {
        Node::Sin(Box::new(node))
    }
    pub fn cos(node: Node) -> Node {
        Node::Cos(Box::new(node))
    }
    pub fn tan(node: Node) -> Node {
        Node::Tan(Box::new(node))
    }
    pub fn cot(node: Node) -> Node {
        Node::Cot(Box::new(node))
    }
    
    // Binary ops
    pub fn add(left: Node, right: Node) -> Node {
        Node::Add(Box::new(left), Box::new(right))
    }
    pub fn sub(left: Node, right: Node) -> Node {
        Node::Sub(Box::new(left), Box::new(right))
    }
    pub fn mul(left: Node, right: Node) -> Node {
        Node::Mul(Box::new(left), Box::new(right))
    }
    pub fn div(left: Node, right: Node) -> Node {
        Node::Div(Box::new(left), Box::new(right))
    }
    pub fn pow(left: Node, right: Node) -> Node {
        Node::Pow(Box::new(left), Box::new(right))
    }
    pub fn log(left: Node, right: Node) -> Node {
        Node::Log(Box::new(left), Box::new(right))
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_add_pow() {
        // 2^x + x
        let equation = Node::add(Node::pow(Node::Const(2.0), Node::Var), Node::Var);
        
        let x = 3.0;
        let expected = 11.0;
        let result = equation.evaluate(x);

        assert_eq!(result, expected, "evaluation of 2^x + x failed for x=3.0");
    }
}