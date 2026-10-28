use crate::ast::*;
use crate::dataset::Dataset;

#[derive(Clone, Debug)]
pub struct Individual {
    node: Node,
    fitness: f64,
    complexity: usize,
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
        let mse = 1.0 / dataset.outputs.len() as f64
            * dataset
                .inputs
                .iter()
                .zip(dataset.outputs.iter())
                .map(|(i, o)| {
                    let pred = self.node.evaluate(i);
                    let err = pred - o;
                    err * err
                })
                .sum::<f64>();
        self.fitness = if mse.is_finite() { mse } else { f64::INFINITY };
        self.complexity = self.node.count();
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
            "y".to_string(),
            vec![2.0, 4.0, 6.0],
        )
        .expect("Valid dataset");

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
            "y".to_string(),
            vec![2.0, 4.0, 6.0],
        )
        .expect("Valid dataset");

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
}
