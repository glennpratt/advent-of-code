use regex::Regex;
use std::collections::HashMap;
use std::iter::repeat;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let guards = load_guards(input);
    let sleepy_guard = guards
        .values()
        .max_by_key(|g| g.minutes_asleep)
        .expect("couldn't find sleepiest guard");

    sleepy_guard.id * sleepy_guard.sleepiest_minute_and_count().0
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let guards = load_guards(input);
    let sleepy_guard = guards
        .values()
        .max_by_key(|g| g.sleepiest_minute_and_count().1)
        .expect("couldn't find guard with the sleepiest minute");

    sleepy_guard.id * sleepy_guard.sleepiest_minute_and_count().0
}

fn load_guards(input: &str) -> HashMap<usize, Guard> {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();

    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (Guard #(\d+))?(.+)").unwrap();
    let mut current_id_opt: Option<usize> = None;
    let mut minute_asleep_opt: Option<usize> = None;
    let mut guards = HashMap::new();

    for line in lines {
        let caps = re.captures(line).unwrap();
        let minute: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        if let Some(guard_id) = caps.get(3) {
            let id: usize = guard_id.as_str().parse().unwrap();
            current_id_opt = Some(id);
            guards.entry(id).or_insert_with(|| Guard::new(id));
        } else {
            let current_id = current_id_opt.expect("guard state found without guard on duty");
            match caps.get(4).unwrap().as_str() {
                "falls asleep" => minute_asleep_opt = Some(minute),
                "wakes up" => {
                    let minute_asleep = minute_asleep_opt.expect("guard woke when not sleeping");
                    minute_asleep_opt = None;
                    guards
                        .get_mut(&current_id)
                        .unwrap()
                        .add_sleep(minute_asleep, minute);
                }
                _ => panic!("unknown guard state"),
            }
        }
    }

    guards
}

#[derive(Debug)]
struct Guard {
    pub id: usize,
    pub minutes_asleep: usize,
    sleep_histogram: HashMap<usize, usize>,
}

impl Guard {
    fn new(id: usize) -> Self {
        Guard {
            id,
            minutes_asleep: 0,
            sleep_histogram: repeat(0).enumerate().take(60).collect(),
        }
    }

    fn add_sleep(&mut self, start: usize, stop: usize) {
        let addend = stop - start;
        self.minutes_asleep += addend;
        for minute in start..stop {
            *self.sleep_histogram.get_mut(&minute).unwrap() += 1;
        }
    }

    fn sleepiest_minute_and_count(&self) -> (usize, usize) {
        let minute_and_count = self
            .sleep_histogram
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap();
        (*minute_and_count.0, *minute_and_count.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift\n\
                     [1518-11-01 00:05] falls asleep\n\
                     [1518-11-01 00:25] wakes up\n\
                     [1518-11-01 00:30] falls asleep\n\
                     [1518-11-01 00:55] wakes up\n\
                     [1518-11-01 23:58] Guard #99 begins shift\n\
                     [1518-11-02 00:40] falls asleep\n\
                     [1518-11-02 00:50] wakes up\n\
                     [1518-11-03 00:05] Guard #10 begins shift\n\
                     [1518-11-03 00:24] falls asleep\n\
                     [1518-11-03 00:29] wakes up\n\
                     [1518-11-04 00:02] Guard #99 begins shift\n\
                     [1518-11-04 00:36] falls asleep\n\
                     [1518-11-04 00:46] wakes up\n\
                     [1518-11-05 00:03] Guard #99 begins shift\n\
                     [1518-11-05 00:45] falls asleep\n\
                     [1518-11-05 00:55] wakes up";
        assert_eq!(240, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift\n\
                     [1518-11-01 00:05] falls asleep\n\
                     [1518-11-01 00:25] wakes up\n\
                     [1518-11-01 00:30] falls asleep\n\
                     [1518-11-01 00:55] wakes up\n\
                     [1518-11-01 23:58] Guard #99 begins shift\n\
                     [1518-11-02 00:40] falls asleep\n\
                     [1518-11-02 00:50] wakes up\n\
                     [1518-11-03 00:05] Guard #10 begins shift\n\
                     [1518-11-03 00:24] falls asleep\n\
                     [1518-11-03 00:29] wakes up\n\
                     [1518-11-04 00:02] Guard #99 begins shift\n\
                     [1518-11-04 00:36] falls asleep\n\
                     [1518-11-04 00:46] wakes up\n\
                     [1518-11-05 00:03] Guard #99 begins shift\n\
                     [1518-11-05 00:45] falls asleep\n\
                     [1518-11-05 00:55] wakes up";
        assert_eq!(4455, part2(input));
    }
}
