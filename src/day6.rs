#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let mut coordinates: Vec<Coordinate> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(", ");
            Coordinate::new(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        }).collect();

    let size = 400;

    for x in 0..size {
        for y in 0..size {
            let mut closest = 2000;
            let mut owner = None;
            for mut coordinate in coordinates.iter_mut() {
                let distance = coordinate.distance(x, y);
                if distance == closest {
                    owner = None
                } else if distance < closest {
                    closest = distance;
                    owner = Some(coordinate);
                }
            }

            if let Some(owner) = owner {
                owner.area += 1;
                if x < 2 || y < 2 || x > size - 2 || y > size - 2 {
                    owner.is_infinite = true;
                }
            }
        }
    }

    eprintln!("{:?}", coordinates);

    coordinates
        .iter()
        .filter_map(|c| if c.is_infinite { None } else { Some(c.area) })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    pub area: u64,
    pub is_infinite: bool,
}

impl Coordinate {
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
}
