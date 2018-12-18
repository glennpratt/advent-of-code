use aoc_runner_derive::*;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut stack = vec![(&input[0..], 0, 0)];
    let mut sum: usize = 0;

    loop {
        let (node, children_walked, pos) = stack[stack.len() - 1];
        let child_count = node[0];
        let meta_length = node[1];

        if child_count == 0 {
            sum += &node[2..(2 + meta_length)].iter().sum();
            if stack.pop().is_none() {
                break;
            }
            if let Some(parent) = stack.last_mut() {
                parent.1 += 1;
                parent.2 += 2 + meta_length;
            } else {
                break;
            }
        } else if child_count == children_walked {
            sum += &node[2 + pos..(2 + pos + meta_length)].iter().sum();
            if stack.pop().is_none() {
                break;
            }
            if let Some(parent) = stack.last_mut() {
                parent.1 += 1;
                parent.2 += 2 + pos + meta_length;
            } else {
                break;
            }
        } else {
            let children = &node[(2 + pos)..(node.len() - meta_length)];
            stack.push((children, 0, 0));
        }
    }

    sum
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut stack = vec![(&input[0..], 0, 0, vec![0; input[0]])];
    let _sum: usize = 0;

    loop {
        let (node, children_walked, pos, ref child_values) = stack[stack.len() - 1];
        let child_count = node[0];
        let meta_length = node[1];

        if child_count == 0 {
            let addend: usize = node[2..(2 + meta_length)].iter().sum();

            stack.pop().expect("stack shouldn't empty");
            if let Some(parent) = stack.last_mut() {
                parent.3[parent.1] = addend;
                parent.1 += 1;
                parent.2 += 2 + meta_length;
            } else {
                return addend;
            }
        } else if child_count == children_walked {
            let addend: usize = node[2 + pos..(2 + pos + meta_length)]
                .iter()
                .filter_map(|meta| child_values.get(meta - 1))
                .sum();

            stack.pop().expect("stack shouldn't empty");
            if let Some(parent) = stack.last_mut() {
                parent.3[parent.1] = addend;
                parent.1 += 1;
                parent.2 += 2 + pos + meta_length;
            } else {
                return addend;
            }
        } else {
            let children = &node[(2 + pos)..(node.len() - meta_length)];
            stack.push((children, 0, 0, vec![0; child_count]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";
        assert_eq!(138, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";
        assert_eq!(66, part2(input));
    }
}
