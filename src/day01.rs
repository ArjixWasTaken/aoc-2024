fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input
        .split("\n")
        .into_iter()
        .filter(|x| x.trim().len() > 0)
        .collect::<Vec<_>>();

    let n = lines.len();

    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);

    for line in lines {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<i32>().unwrap());
        right.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    (left, right)
}

fn part_01(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    assert_eq!(left.len(), right.len());

    left.sort();
    right.sort();

    let mut distance = 0;

    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }

    distance
}

fn part_02(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    assert_eq!(left.len(), right.len());

    left.sort();
    right.sort();

    let mut similarity = 0;

    for i in 0..left.len() {
        similarity += left[i] * right.iter().filter(|x| **x == left[i]).count() as i32;
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = include_str!("../inputs/day01/sample.txt");
        let (mut left, mut right) = parse_input(input.to_string());

        let dist = part_01(&mut left, &mut right);
        assert_eq!(dist, 11);
    }

    #[test]
    fn part2_sample() {
        let input = include_str!("../inputs/day01/sample.txt");
        let (mut left, mut right) = parse_input(input.to_string());

        let dist = part_02(&mut left, &mut right);
        assert_eq!(dist, 31);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day01/input.txt");
        let (mut left, mut right) = parse_input(input.to_string());

        let dist = part_01(&mut left, &mut right);
        assert_eq!(dist, 1223326);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day01/input.txt");
        let (mut left, mut right) = parse_input(input.to_string());

        let dist = part_02(&mut left, &mut right);
        assert_eq!(dist, 21070419);
    }
}
