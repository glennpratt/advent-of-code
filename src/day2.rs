use aoc_runner_derive::*;
use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    let mut doubles = 0;
    let mut triples = 0;
    let mut histogram: HashMap<char, i64> = HashMap::new();

    for line in input.lines() {
        for c in line.chars() {
            *histogram.entry(c).or_insert(0) += 1;
        }

        let (mut has_double, mut has_triple) = (false, false);
        histogram.drain().any(|(_, n)| {
            match n {
                2 => has_double = true,
                3 => has_triple = true,
                _ => {}
            }
            has_double && has_triple
        });

        doubles += has_double as i64;
        triples += has_triple as i64;
    }

    doubles * triples
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    for line_a in input.lines() {
        let len = line_a.len();
        for line_b in input.lines() {
            let candidate: String = line_a
                .chars()
                .zip(line_b.chars())
                .filter_map(
                    |(char_a, char_b)| {
                        if char_a == char_b {
                            Some(char_a)
                        } else {
                            None
                        }
                    },
                )
                .collect();
            if candidate.len() == len - 1 {
                return candidate;
            }
        }
    }
    panic!("IDs one char apart not found")
}
