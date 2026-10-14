use std::fmt::Display;

use crate::ast::*;

#[derive(Clone, Debug)]
pub struct Individual {
    pub node: Node,
    pub fitness: f64,
    pub complexity: usize
}

impl Individual {
    pub fn new(node: Node) -> Self {
        let complexity = node.count();
        Self {
            node,
            fitness: f64::INFINITY,
            complexity,
        }
    }
    pub fn evaluate_fitness(&mut self, dataset: &Dataset) {
        let mse = 1.0 / dataset.outputs.len() as f64 * 
            dataset.inputs
                .iter()
                .zip(dataset.outputs
                    .iter())
                .map(|(i, o)| {
                    let pred = self.node.evaluate(i); // TODO: MULTIVARIABLE
                    let err = pred - o;
                    err * err
                }).sum::<f64>();
        self.fitness = if mse.is_finite() {mse} else {1e12};
    }
}

pub struct Dataset {
    pub names: Vec<String>, // just names, x y z w pressure etc
    pub inputs: Vec<Vec<f64>>, // for example x y
    pub outputs: Vec<f64> // for example f(x,y) = z
    // |  x  |  y  |  z  |
    // | f64 | f64 | f64 | -> | out | f64 |
    // | f64 | f64 | f64 | 
    // inputs[0][1] is first Y 
    // inputs [1][0] is second X
}



impl Dataset {
    pub fn new(names: Vec<String>, inputs: Vec<Vec<f64>>, outputs: Vec<f64>) -> Result<Self, &'static str> {
        if names.is_empty() || inputs.is_empty() || outputs.is_empty() {
            return Err("dataset collections cannot be empty");
        }
        if inputs.len() != outputs.len() {
            return Err("inputs and outputs muts be the same len");
        }
        let names_len = names.len();
        if inputs.iter().any(|row| row.len() != names_len) {
            return Err("names size not same as inputs rows size");
        }
        if inputs.iter().flatten().chain(outputs.iter()).any(|v| !v.is_finite()) {
            return Err("numbers cannot be infinite");
        }
        Ok(Self {
            names,
            inputs,
            outputs
        })
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, Node};

    #[test]
    fn test_fitness_evaluate_exact() {
        let dataset = Dataset::new(
            vec!["x".to_string()],
            vec![vec![1.0], vec![2.0], vec![3.0]],
            vec![2.0, 4.0, 6.0],
        ).expect("Valid dataset");

        // 2.0 * x
        let tree = Node::binary(BinaryOp::Mul, Node::Const(2.0), Node::Var(0));
        let mut ind = Individual::new(tree);
        ind.evaluate_fitness(&dataset);

        assert_eq!(ind.fitness, 0.0);
    }

    #[test]
    fn test_fitness_evaluate_imperfect() {
        // [2.0, 4.0, 6.0]
        let dataset = Dataset::new(
            vec!["x".to_string()],
            vec![vec![1.0], vec![2.0], vec![3.0]],
            vec![2.0, 4.0, 6.0],
        ).expect("Valid dataset");

        // Model x -> predictions: [1.0, 2.0, 3.0]
        // Errors (2-1)^2 = 1, (4-2)^2 = 4, (6-3)^2 = 9
        // MSE = (1 + 4 + 9) / 3 = 14 / 3 ~ 4.6666667
        let tree = Node::Var(0);
        let mut ind = Individual::new(tree);
        ind.evaluate_fitness(&dataset);

        let expected_mse = 14.0 / 3.0;
        assert!(
            (ind.fitness - expected_mse).abs() < 1e-9,
            "Expected {}, got {}",
            expected_mse,
            ind.fitness
        );
    }

    #[test]
    fn test_dataset_validation() {
        assert!(Dataset::new(
            vec!["x".to_string()],
            vec![vec![1.0], vec![2.0]],
            vec![1.0]
        ).is_err());

        assert!(Dataset::new(vec![], vec![], vec![]).is_err());

        assert!(Dataset::new(
            vec!["x".to_string()],
            vec![vec![f64::NAN]],
            vec![1.0]
        ).is_err());
    }
}