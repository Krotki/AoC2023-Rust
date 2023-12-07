use itertools::{Itertools};
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {

    let lines = input.lines().collect_vec();

    let times = lines[0].split_whitespace().skip(1)
        .map(|n| n.parse::<u32>().expect("A number"))
        .collect_vec();
    let distances = lines[1].split_whitespace().skip(1)
        .map(|n| n.parse::<u32>().expect("A number"))
        .collect_vec();

    return Some(times.iter().interleave(distances.iter()).tuples().map(|(&time, distance)| {
        (1..time).map(|t| t * (time - t)).filter(|d| d > distance).count() as u32
    }).fold(1, |a, v| a * v));
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();

    let time: u64 = lines[0].split_whitespace().skip(1).join("").parse().unwrap();
    let distance: u64 = lines[1].split_whitespace().skip(1).join("").parse().unwrap();

    let result = (1..time).map(|t| t * (time - t)).filter(|&d| d > distance).count() as u64;
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
