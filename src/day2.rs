use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    let mut doubles = 0;
    let mut triples = 0;
    let mut histogram: HashMap<char, i64> = HashMap::new();

    for line in input.lines() {
        histogram.clear();
        for c in line.chars() {
            *histogram.entry(c).or_insert(0) += 1;
        }
        if histogram.values().any(|n| *n == 3) {
            triples += 1;
        }
        if histogram.values().any(|n| *n == 2) {
            doubles += 1;
        }
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
                .filter_map(|(char_a, char_b)| {
                    if char_a == char_b {
                        Some(char_a)
                    } else {
                        None
                    }
                }).collect();
            if candidate.len() == len - 1 {
                return candidate;
            }
        }
    }
    panic!("IDs one char apart not found")
}
