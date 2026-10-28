use std::{error::Error, fmt::Display, ops::Range};

use crate::ast::{BinaryOp, UnaryOp};

#[derive(Debug, Clone, Copy)]
pub enum Fitness {
    Mse,
    Mae,
    LogErr,
    RSquared,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub population_size: usize,
    pub max_depth: usize,
    pub const_range: Range<f64>,
    pub unary_ops: Vec<UnaryOp>,
    pub binary_ops: Vec<BinaryOp>,
    pub tournament_size: f64,
    pub elitism: f64,
    pub p_cross: f64,
    pub p_mut: f64,
    pub max_generations: usize,
    pub target_fitness: f64,
    pub fitness: Fitness,
    pub seed: Option<u64>,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let (start, end) = (self.const_range.start, self.const_range.end);
        if !start.is_finite() || !end.is_finite() || start >= end {
            return Err(ConfigError::ConstRangeError { start, end });
        }
        if self.unary_ops.is_empty() && self.binary_ops.is_empty() {
            return Err(ConfigError::NoOperators);
        }
        let pop = self.population_size;
        if pop < 2 {
            return Err(ConfigError::LowPopulation { pop });
        }
        let depth = self.max_depth;
        if depth < 2 {
            return Err(ConfigError::LowDepth { depth });
        }
        if self.max_generations == 0 {
            return Err(ConfigError::NoGenerations);
        }
        let target = self.target_fitness;
        if target.is_nan() || target < 0.0 {
            return Err(ConfigError::InvalidTarget { target });
        }
        let probs = [
            ("p_cross", self.p_cross),
            ("p_mut", self.p_mut),
            ("tournament_size", self.tournament_size),
            ("elitism", self.elitism),
        ];
        if let Some(&(what, found)) = probs.iter().find(|(_, v)| !(0.0..=1.0).contains(v)) {
            return Err(ConfigError::IncorrectProb { what, found });
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            population_size: 500,
            max_depth: 6,
            const_range: -10.0..10.0,
            unary_ops: UnaryOp::ALL.to_vec(),
            binary_ops: BinaryOp::ALL.to_vec(),
            tournament_size: 0.01,
            elitism: 0.01,
            p_cross: 0.8,
            p_mut: 0.15,
            max_generations: 500,
            target_fitness: 1e-6,
            fitness: Fitness::Mse,
            seed: None,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NoOperators,
    ConstRangeError { start: f64, end: f64 },
    LowPopulation { pop: usize },
    LowDepth { depth: usize },
    NoGenerations,
    InvalidTarget { target: f64 },
    IncorrectProb { what: &'static str, found: f64 },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NoOperators => {
                write!(f, "at least one unary or binary operator is required")
            }
            ConfigError::ConstRangeError { start, end } => write!(
                f,
                "invalid const range {start}..{end}: bounds must be finite and start less than end"
            ),
            ConfigError::LowPopulation { pop } => {
                write!(f, "population size must be at least 2, got {pop}")
            }
            ConfigError::LowDepth { depth } => {
                write!(f, "max depth must be at least 2, got {depth}")
            }
            ConfigError::NoGenerations => write!(f, "max generations must be at least 1"),
            ConfigError::InvalidTarget { target } => {
                write!(f, "target fitness must be non-negative, got {target}")
            }
            ConfigError::IncorrectProb { what, found } => {
                write!(f, "{what} must be in range 0..=1, got {found}")
            }
        }
    }
}

impl Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(Config::default().validate().is_ok());
        let bad = |c: Config| c.validate().unwrap_err().to_string();
        println!(
            "{}",
            bad(Config {
                population_size: 1,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                max_depth: 1,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                max_generations: 0,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                target_fitness: f64::NAN,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                const_range: f64::NEG_INFINITY..1.0,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                p_mut: 1.5,
                ..Default::default()
            })
        );
        println!(
            "{}",
            bad(Config {
                unary_ops: vec![],
                binary_ops: vec![],
                ..Default::default()
            })
        );
    }
}
