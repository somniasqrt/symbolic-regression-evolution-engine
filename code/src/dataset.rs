use std::{error::Error, fmt::Display, iter::once};

/// ```text
/// |  x  |  y  |  z  |
/// | f64 | f64 | f64 | -> | out | f64 |
/// | f64 | f64 | f64 |
/// inputs[0][1] is first Y
/// inputs[1][0] is second X
/// ```
pub struct Dataset {
    /// just names, x y z w pressure etc
    pub(crate) names: Vec<String>,
    /// for example x y
    pub(crate) inputs: Vec<Vec<f64>>,
    /// output column name
    pub(crate) out: String,
    /// for example f(x,y) = z
    pub(crate) outputs: Vec<f64>,
}

impl Dataset {
    pub fn new(
        names: Vec<String>,
        inputs: Vec<Vec<f64>>,
        out: String,
        outputs: Vec<f64>,
    ) -> Result<Self, DatasetError> {
        if names.is_empty() || inputs.is_empty() || outputs.is_empty() {
            return Err(DatasetError::Empty);
        }
        if inputs.len() != outputs.len() {
            return Err(DatasetError::LenMismatch {
                inputs: inputs.len(),
                outputs: outputs.len(),
            });
        }
        if let Some(row) = inputs.iter().position(|r| r.len() != names.len()) {
            return Err(DatasetError::RowLenMismatch {
                row,
                expected: names.len(),
                found: inputs[row].len(),
            });
        }
        if let Some((row, col)) = inputs
            .iter()
            .enumerate()
            .find_map(|(row, r)| r.iter().position(|c| !c.is_finite()).map(|col| (row, col)))
        {
            return Err(DatasetError::NonFiniteInput { row, col });
        }
        if let Some(row) = outputs.iter().position(|v| !v.is_finite()) {
            return Err(DatasetError::NonFiniteOutput { row });
        }
        Ok(Self {
            names,
            inputs,
            out,
            outputs,
        })
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let widths: Vec<usize> = self
            .names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                self.inputs
                    .iter()
                    .map(|row| row[i].to_string().len())
                    .chain(once(name.len()))
                    .max()
                    .unwrap_or(0)
            })
            .collect();
        let out_w = self
            .outputs
            .iter()
            .map(|v| v.to_string().len())
            .chain(once(self.out.len()))
            .max()
            .unwrap_or(0);

        for (name, &w) in self.names.iter().zip(&widths) {
            write!(f, "{name:>w$} | ")?;
        }
        writeln!(f, "{:>out_w$}", self.out)?;
        for (row, out) in self.inputs.iter().zip(&self.outputs) {
            for (v, &w) in row.iter().zip(&widths) {
                write!(f, "{v:>w$} | ")?;
            }
            writeln!(f, "{out:>out_w$}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DatasetError {
    Empty,
    LenMismatch {
        inputs: usize,
        outputs: usize,
    },
    RowLenMismatch {
        row: usize,
        expected: usize,
        found: usize,
    },
    NonFiniteInput {
        row: usize,
        col: usize,
    },
    NonFiniteOutput {
        row: usize,
    },
}

impl Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "dataset must not be empty"),
            Self::LenMismatch { inputs, outputs } => write!(
                f,
                "number of input rows ({inputs}) does not match number of outputs ({outputs})"
            ),
            Self::RowLenMismatch {
                row,
                expected,
                found,
            } => write!(f, "row {row}: expected {expected} values, found {found}"),
            Self::NonFiniteInput { row, col } => {
                write!(f, "row {row}, column {col}: input value is not finite")
            }
            Self::NonFiniteOutput { row } => write!(f, "row {row}: output value is not finite"),
        }
    }
}
impl Error for DatasetError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_validation() {
        assert!(
            Dataset::new(
                vec!["x".to_string()],
                vec![vec![1.0], vec![2.0]],
                "y".to_string(),
                vec![1.0]
            )
            .is_err()
        );

        assert!(Dataset::new(vec![], vec![], "".to_string(), vec![]).is_err());

        assert!(
            Dataset::new(
                vec!["x".to_string()],
                vec![vec![f64::NAN]],
                "y".to_string(),
                vec![1.0]
            )
            .is_err()
        );
    }
}
