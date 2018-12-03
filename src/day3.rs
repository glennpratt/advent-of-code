use std::collections::HashMap;
use std::iter::repeat;
use std::mem::replace;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

fn load(input: &str) -> (Grid, HashMap<usize, Rc<AtomicBool>>) {
    let mut grid = Grid::new();
    let mut claims: HashMap<usize, Rc<AtomicBool>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(|c| c == '@' || c == ':');
        let mut id: usize = parts.next().unwrap()[1..].trim().parse().unwrap();
        let mut start = parts.next().unwrap().split(',');
        let start_x: usize = start.next().unwrap().trim().parse().unwrap();
        let start_y: usize = start.next().unwrap().parse().unwrap();
        let mut size = parts.next().unwrap().split('x');
        let width: usize = size.next().unwrap().trim().parse().unwrap();
        let height: usize = size.next().unwrap().parse().unwrap();

        let mut is_unique = Rc::new(AtomicBool::new(true));

        for x in start_x..(start_x + width) {
            for y in start_y..(start_y + height) {
                grid.mark(x, y, &mut is_unique);
            }
        }

        claims.insert(id, is_unique);
    }
    (grid, claims)
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i64 {
    let (grid, _) = load(input);
    grid.overlaps()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let (_, claims) = load(input);

    *claims
        .iter()
        .find(|(_, is_unique)| (*is_unique).load(Ordering::Relaxed))
        .expect("at least one clear claim")
        .0
}

#[derive(Debug)]
struct Grid(Vec<Vec<(u8, Option<Rc<AtomicBool>>)>>);

impl Grid {
    fn new() -> Self {
        let row = repeat((0, None)).take(1000).collect();
        let grid = repeat(row).take(1000).collect();
        Grid(grid)
    }

    fn mark(&mut self, x: usize, y: usize, new_is_unique: &mut Rc<AtomicBool>) {
        self.0[y][x].0 = self.0[y][x].0.saturating_add(1);
        let mut is_none = true;
        if let Some(ref mut first_is_unique) = self.0[y][x].1 {
            (*first_is_unique).store(false, Ordering::Relaxed);
            (*new_is_unique).store(false, Ordering::Relaxed);
            is_none = false;
        }
        if is_none {
            replace(&mut self.0[y][x].1, Some(Rc::clone(new_is_unique)));
        }
    }

    fn overlaps(&self) -> i64 {
        let mut overlaps = 0;
        for row in &self.0 {
            for item in row {
                if item.0 > 1 {
                    overlaps += 1;
                }
            }
        }
        overlaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            4,
            part1(
                "#1 @ 1,3: 4x4\n\
                 #2 @ 3,1: 4x4\n\
                 #3 @ 5,5: 2x2\n"
            )
        );
    }
}
