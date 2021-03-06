use aoc_runner_derive::*;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut input: Vec<_> = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut change_tries = 1;

    loop {
        change_tries -= 1;

        for i in 0..input.len() {
            let j = i + 1;
            if j >= input.len() {
                break;
            }
            let c = input[i];
            let d = input[j];
            let swap = if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            };

            if swap == d {
                change_tries += 1;
                input.remove(i);
                input.remove(j - 1);
            }
        }

        if change_tries < 1 {
            break;
        }
    }

    input.len()
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
            part1(&filtered)
        })
        .min()
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
