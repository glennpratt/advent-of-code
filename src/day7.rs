use aoc_runner_derive::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::repeat;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let mut dag: HashMap<char, HashSet<char>> = HashMap::new();
    let re =
        Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begi").expect("regex");

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let parent: char = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let item: char = caps.get(2).unwrap().as_str().chars().next().unwrap();

        dag.entry(parent).or_insert_with(HashSet::new);
        dag.entry(item).or_insert_with(HashSet::new).insert(parent);
    }

    let mut out: String = "".into();
    loop {
        let mut ready: Vec<char> = dag
            .iter()
            .filter_map(
                |(c, parents)| {
                    if parents.is_empty() {
                        Some(*c)
                    } else {
                        None
                    }
                },
            )
            .collect();

        if ready.is_empty() {
            break;
        }
        ready.sort();

        let c = ready[0];

        for parents in dag.values_mut() {
            parents.remove(&c);
        }

        dag.remove(&c);
        out.push(c);
    }

    out
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let worker_count = if cfg!(test) { 2 } else { 5 };
    let base_effort = if cfg!(test) { 0 } else { 60 };

    let mut dag: HashMap<char, HashSet<char>> = HashMap::new();
    let re =
        Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begi").expect("regex");

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let parent: char = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let item: char = caps.get(2).unwrap().as_str().chars().next().unwrap();

        dag.entry(parent).or_insert_with(HashSet::new);
        dag.entry(item).or_insert_with(HashSet::new).insert(parent);
    }

    let mut workers: Vec<Option<(char, i64)>> = repeat(None).take(worker_count).collect();

    let mut time = 0;
    loop {
        let mut any_active_worker = false;
        for worker in workers.iter_mut() {
            if let Some((c, time_left)) = worker {
                *time_left -= 1;
                if *time_left <= 0 {
                    for parents in dag.values_mut() {
                        parents.remove(&c);
                    }

                    dag.remove(&c);
                    *worker = None;
                } else {
                    any_active_worker = true;
                }
            }
        }

        if !any_active_worker && dag.is_empty() {
            break;
        }

        let mut ready: Vec<char> = dag
            .iter()
            .filter_map(
                |(c, parents)| {
                    if parents.is_empty() {
                        Some(*c)
                    } else {
                        None
                    }
                },
            )
            .collect();

        if !ready.is_empty() {
            ready.sort();

            for c in ready {
                let time_needed = i64::from(c.to_digit(36).unwrap() - 9) + base_effort;

                for worker in workers.iter_mut() {
                    if worker.is_none() {
                        dag.remove(&c);
                        *worker = Some((c, time_needed));
                        break;
                    }
                }
            }
        }

        time += 1;
    }

    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = "Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.\n\
                     Step A must be finished before step D can begin.\n\
                     Step B must be finished before step E can begin.\n\
                     Step D must be finished before step E can begin.\n\
                     Step F must be finished before step E can begin.\n";
        assert_eq!("CABDFE", part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = "Step C must be finished before step A can begin.\n\
                     Step C must be finished before step F can begin.\n\
                     Step A must be finished before step B can begin.\n\
                     Step A must be finished before step D can begin.\n\
                     Step B must be finished before step E can begin.\n\
                     Step D must be finished before step E can begin.\n\
                     Step F must be finished before step E can begin.\n";
        assert_eq!(15, part2(input));
    }
}
