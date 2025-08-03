use std::collections::HashSet;

/// Conversion defines the relationship of two units so that source_unit = rate * target_unit.
/// E.g. kg=1000g
#[derive(Debug)]
pub struct Conversion {
    pub source_unit: String,
    pub rate: f64,
    pub target_unit: String,
}

impl Conversion {
    /// Creates a new conversion with the given values.
    pub fn new(source_unit: &str, rate: f64, target_unit: &str) -> Conversion {
        Conversion {
            source_unit: String::from(source_unit),
            rate,
            target_unit: String::from(target_unit),
        }
    }

    /// Inverts the conversion by transposing the units and using the rate's reciprocal.
    /// E.g. kg=1000g becomes g=0.001kg
    pub fn invert(&self) -> Conversion {
        Conversion {
            source_unit: self.target_unit.clone(),
            rate: 1.0 / self.rate,
            target_unit: self.source_unit.clone(),
        }
    }
}

pub fn find_path(
    source_unit: &str,
    target_unit: &str,
    conversions: &Vec<Conversion>,
) -> Vec<Vec<Conversion>> {
    do_find_path(source_unit, target_unit, conversions, Vec::new(), HashSet::new())
}

fn do_find_path(
    unit: &str,
    target_unit: &str,
    conversions: &Vec<Conversion>,
    path: Vec<Conversion>,
    visited: HashSet<String>,
) -> Vec<Vec<Conversion>> {
    vec![]
}
