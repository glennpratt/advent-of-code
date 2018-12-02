use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    input.lines().map(|s| s.parse::<i64>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut seen: HashSet<i64> = HashSet::new();
    let mut sum = 0i64;
    seen.insert(sum);

    input
        .lines()
        .cycle()
        .find_map(|s| {
            let delta = s.parse::<i64>().unwrap();
            sum += delta;
            if seen.insert(sum) {
                None
            } else {
                Some(sum)
            }
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example1() {
        assert_eq!(2, part2("+1\n-2\n+3\n+1\n+1\n-2"));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(0, part2("+1\n-1"));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(10, part2("+3\n+3\n+4\n-2\n-4"));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(5, part2("-6\n+3\n+8\n+5\n-6"));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(14, part2("+7\n+7\n-2\n-7\n-4"));
    }
}
