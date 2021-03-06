use advent2020::error::NoSolution;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

// O(N) time, O(N) space
fn load_entries<P: AsRef<Path>>(input: P) -> Result<HashSet<i32>, Box<dyn Error>> {
    let mut entries = HashSet::new();
    for line in BufReader::new(File::open(input)?).lines() {
        entries.insert(line?.parse()?);
    }
    Ok(entries)
}

// O(N) time, O(1) space
fn solve_part1(entries: &HashSet<i32>) -> Result<i32, NoSolution> {
    for entry in entries {
        let delta = 2020 - entry;
        if entries.contains(&delta) {
            return Ok(delta * entry);
        }
    }
    Err(NoSolution)
}

// O(N²) time, O(N²) space
fn solve_part2(entries: &HashSet<i32>) -> Result<i32, NoSolution> {
    let mut pairs = HashMap::new();
    for first in entries {
        for second in entries {
            if first != second {
                pairs.insert(first + second, (first, second));
            }
        }
    }
    for third in entries {
        let delta = 2020 - third;
        if let Some((&first, &second)) = pairs.get(&delta) {
            return Ok(first * second * third);
        }
    }
    Err(NoSolution)
}

fn main() {
    let input = "tests/day1/input";
    let entries = load_entries(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    let answer1 = solve_part1(&entries).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });
    println!("{}", answer1);
    let answer2 = solve_part2(&entries).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(2);
    });
    println!("{}", answer2);
}
