use konvert::{Calculation, Conversion, find_conversion_path};
use std::collections::HashSet;
use std::{env, process};

fn build_conversions() -> Vec<Conversion> {
    let conversions: Vec<(&str, Calculation, &str)> = vec![
        ("kg", Calculation::Rate(2.20462262), "lbs"),
        ("kg", Calculation::Rate(1000.0), "g"),
        ("km", Calculation::Rate(0.62137119), "mi"),
        ("km", Calculation::Rate(1000.0), "m"),
        ("m", Calculation::Rate(3.2808399), "ft"),
        ("ft", Calculation::Rate(12.0), "in"),
        ("yd", Calculation::Rate(3.0), "ft"),
        ("oz", Calculation::Rate(28.4130625), "ml"),
        ("pt", Calculation::Rate(568.26125), "ml"),
        ("l", Calculation::Rate(1000.0), "ml"),
        ("l", Calculation::Rate(10.0), "dl"),
        ("gal", Calculation::Rate(8.0), "pt"),
        (
            "C",
            Calculation::Func {
                forward: |c| c * 9.0 / 5.0 + 32.0,
                reverse: |f| (f - 32.0) * 5.0 / 9.0,
            },
            "F",
        ),
    ];
    conversions
        .iter()
        .flat_map(|(s, c, t)| {
            let c = Conversion::new(s, c.clone(), t);
            let r = c.invert();
            [c, r]
        })
        .collect()
}

fn fail_usage() -> ! {
    eprintln!("usage: konverter VALUE SOURCE_UNIT TARGET_UNIT");
    process::exit(1);
}

fn fail_unsupported_unit(unit: &str) -> ! {
    eprintln!("unit '{unit}' not supported");
    process::exit(1);
}

enum Command {
    List,
    Conversion(f64, String, String),
}

fn parse_cli_args() -> Command {
    let mut args = env::args();
    args.next(); // "konverter"
    let value = match args.next() {
        Some(s) => match s.parse::<f64>() {
            Ok(v) => v,
            Err(_) => fail_usage(),
        },
        None => {
            return Command::List;
        }
    };
    let source_unit = match args.next() {
        Some(s) => s,
        None => fail_usage(),
    };
    let target_unit = match args.next() {
        Some(s) => s,
        None => fail_usage(),
    };
    Command::Conversion(value, source_unit, target_unit)
}

fn main() {
    let conversions = build_conversions();
    let all_units: HashSet<&String> = conversions
        .iter()
        .flat_map(|c| [&c.source_unit, &c.target_unit])
        .collect();

    match parse_cli_args() {
        Command::List => {
            let mut units: Vec<&String> = all_units.into_iter().collect();
            units.sort();
            for unit in units {
                println!("{unit}");
            }
        }
        Command::Conversion(value, source_unit, target_unit) => {
            if !all_units.contains(&source_unit) {
                fail_unsupported_unit(&source_unit);
            }
            if !all_units.contains(&target_unit) {
                fail_unsupported_unit(&target_unit);
            }

            match find_conversion_path(&source_unit, &target_unit, &conversions) {
                Some(path) => {
                    let result = path.iter().fold(value, |acc, e| match &e.calculation {
                        Calculation::Rate(r) => acc * r,
                        Calculation::Func { forward, .. } => forward(acc),
                    });
                    println!("{result}");
                }
                None => {
                    eprintln!("conversion from '{source_unit}' to '{target_unit}' undefined");
                }
            }
        }
    }
}
