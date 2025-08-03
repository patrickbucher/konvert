use konvert::{Conversion, find_paths};
use std::collections::HashSet;
use std::{env, process};

fn build_conversions() -> Vec<Conversion> {
    let conversions: Vec<(&str, f64, &str)> = vec![
        ("kg", 2.20462262, "lbs"),
        ("kg", 1000.0, "g"),
        ("km", 0.62137119, "mi"),
        ("km", 1000.0, "m"),
        ("m", 3.2808399, "ft"),
        ("ft", 12.0, "in"),
        ("yd", 3.0, "ft"),
        ("oz", 28.4130625, "ml"),
        ("pt", 568.26125, "ml"),
        ("l", 1000.0, "ml"),
        ("l", 10.0, "dl"),
        ("gal", 8.0, "pt"),
    ];
    let mut forward: Vec<Conversion> = conversions
        .iter()
        .map(|(s, r, t)| Conversion::new(s, *r, t))
        .collect();
    let mut backward: Vec<Conversion> = forward.iter().map(|c| c.invert()).collect();
    forward.append(&mut backward);
    forward
}

fn fail_usage() -> ! {
    eprintln!("usage: konverter VALUE SOURCE_UNIT TARGET_UNIT");
    process::exit(1);
}

fn fail_unsupported_unit(unit: &str) -> ! {
    eprintln!("unit '{unit}' not supported");
    process::exit(1);
}

fn parse_cli_args() -> (f64, String, String) {
    let mut args = env::args();
    args.next(); // "konverter"
    let value = match args.next() {
        Some(s) => match s.parse::<f64>() {
            Ok(v) => v,
            Err(_) => fail_usage(),
        },
        None => fail_usage(),
    };
    let source_unit = match args.next() {
        Some(s) => s,
        None => fail_usage(),
    };
    let target_unit = match args.next() {
        Some(s) => s,
        None => fail_usage(),
    };
    (value, source_unit, target_unit)
}

fn main() {
    let conversions = build_conversions();

    let (value, source_unit, target_unit) = parse_cli_args();

    let all_units: HashSet<&String> = conversions
        .iter()
        .flat_map(|c| [&c.source_unit, &c.target_unit])
        .collect();
    if !all_units.contains(&source_unit) {
        fail_unsupported_unit(&source_unit);
    }
    if !all_units.contains(&target_unit) {
        fail_unsupported_unit(&target_unit);
    }

    match find_paths(&source_unit, &target_unit, &conversions) {
        Some(path) => {
            let result = path.iter().fold(value, |acc, e| acc * e.rate);
            println!("{result}");
        }
        None => {
            eprintln!("conversion from '{source_unit}' to '{target_unit}' undefined");
        }
    }
}
