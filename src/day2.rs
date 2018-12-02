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
