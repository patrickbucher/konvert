use konvert::Conversion;

fn build_conversions() -> Vec<Conversion> {
    let conversions: Vec<(&str, f64, &str)> = vec![
        ("kg", 2.20462262, "lbs"),
        ("kg", 1000.0, "g"),
        ("g", 1000.0, "mg"),
    ];
    let mut forward: Vec<Conversion> = conversions
        .iter()
        .map(|(s, r, t)| Conversion::new(&s, *r, &t))
        .collect();
    let mut backward: Vec<Conversion> = forward.iter().map(|c| c.invert()).collect();
    forward.append(&mut backward);
    forward
}

fn main() {
    let conversions = build_conversions();
    println!("{:?}", conversions);
}
