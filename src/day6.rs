use aoc_runner_derive::*;

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    pub area: u64,
    pub is_infinite: bool,
}

impl Coordinate {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(", ");
        Coordinate::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }

    fn new(x: i64, y: i64) -> Self {
        Coordinate {
            x,
            y,
            area: 0,
            is_infinite: false,
        }
    }

    fn distance(&self, x: i64, y: i64) -> u64 {
        ((self.x - x).abs() + (self.y - y).abs()) as u64
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let mut coordinates: Vec<Coordinate> = input.lines().map(Coordinate::parse).collect();

    let size = 400;

    for x in 0..size {
        for y in 0..size {
            let mut closest = 2000;
            let mut owner_opt = None;
            for coordinate in coordinates.iter_mut() {
                let distance = coordinate.distance(x, y);
                if distance == closest {
                    owner_opt = None
                } else if distance < closest {
                    closest = distance;
                    owner_opt = Some(coordinate);
                }
            }

            if let Some(owner) = owner_opt {
                owner.area += 1;
                if x < 2 || y < 2 || x > size - 2 || y > size - 2 {
                    owner.is_infinite = true;
                }
            }
        }
    }

    coordinates
        .iter()
        .filter_map(|c| if c.is_infinite { None } else { Some(c.area) })
        .max()
        .unwrap()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let coordinates: Vec<Coordinate> = input.lines().map(Coordinate::parse).collect();

    let limit = if cfg!(test) { 32 } else { 10_000 };
    let mut count = 0;

    for x in -100..600 {
        for y in -100..600 {
            let total_distance: u64 = coordinates.iter().map(|c| c.distance(x, y)).sum();

            if total_distance < limit {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = "1, 1\n\
                     1, 6\n\
                     8, 3\n\
                     3, 4\n\
                     5, 5\n\
                     8, 9\n";
        assert_eq!(17, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = "1, 1\n\
                     1, 6\n\
                     8, 3\n\
                     3, 4\n\
                     5, 5\n\
                     8, 9\n";
        assert_eq!(16, part2(input));
    }
}
