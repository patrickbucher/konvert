/// Conversion defines the relationship of two units so that source_unit = rate * target_unit.
/// E.g. kg=1000g
#[derive(Debug, PartialEq)]
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

pub fn find_conversion_path<'a>(
    source_unit: &'a str,
    target_unit: &'a str,
    conversions: &'a Vec<Conversion>,
) -> Option<Vec<&'a Conversion>> {
    let paths = find_paths(source_unit, target_unit, conversions, Vec::new());
    let mut weighted_paths: Vec<_> = paths.iter().map(|p| (p, p.len())).collect();
    weighted_paths.sort_by(|(_, m), (_, n)| m.cmp(n));
    weighted_paths.into_iter().map(|(p, _)| p).next().cloned()
}

fn find_paths<'a>(
    unit: &'a str,
    target_unit: &'a str,
    conversions: &'a Vec<Conversion>,
    path: Vec<&'a Conversion>,
) -> Vec<Vec<&'a Conversion>> {
    let candidates: Vec<&'a Conversion> = conversions
        .iter()
        .filter(|c| c.source_unit == unit && !path.contains(c))
        .collect();
    let mut paths: Vec<Vec<&Conversion>> = Vec::new();
    for candidate in candidates {
        let mut new_path = path.clone();
        new_path.push(candidate);
        if candidate.target_unit == target_unit {
            paths.push(new_path);
        } else {
            for found in find_paths(&candidate.target_unit, target_unit, conversions, new_path) {
                paths.push(found);
            }
        }
    }
    paths
}
