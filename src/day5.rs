// use regex::Regex;
// use std::collections::HashMap;
// use std::iter::repeat;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut input: Vec<_> = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let null = '\0';
    let mut change_tries = 1;

    loop {
        // eprintln!("{:?}", input);
        change_tries = change_tries - 1;
        // let mut delete_next = false;

        // let mut iter = input.iter_mut().filter(|c| *c != &null).peekable();

        for i in 0..input.len() {
            let c = input[i];

            let (j, d) = match input
                .iter()
                .enumerate()
                .skip(i + 1)
                .find(|(_, c)| *c != &null)
            {
                Some((j, d)) => (j, *d),
                None => break,
            };

            if c == null || d == null {
                continue;
            }

            let swap = if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            };

            // eprintln!("{}{} swap {}", c, d, swap);

            if swap == d {
                change_tries += 1;
                input[i] = null;
                input[j] = null;
            }
        }

        if change_tries < 1 {
            break;
        }
    }
    let s: String = input.iter().filter(|c| *c != &null).collect();
    // eprintln!("{:?}", s);

    // input.iter().filter(|c| *c != &null).count()
    s.len()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|alpha| {
            let filtered: String = input
                .chars()
                .filter(|c| c.to_ascii_lowercase() != alpha)
                .collect();
            // eprintln!("{}", filtered);
            part1(&filtered)
        }).min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = "aA";
        assert_eq!(0, part1(input));
    }
    #[test]
    fn part1_example2() {
        let input = "abBA";
        assert_eq!(0, part1(input));
    }
    #[test]
    fn part1_example3() {
        let input = "abAB";
        assert_eq!(4, part1(input));
    }
    #[test]
    fn part1_example4() {
        let input = "aabAAB";
        assert_eq!(6, part1(input));
    }
    #[test]
    fn part1_example5() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(10, part1(input));
    }
}
