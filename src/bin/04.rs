use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"^Card +\d+: ").unwrap();
    Some(input.lines().map(|line| {
        let replaced = re.replace(line, "");
        let (win_str, nums_str) = replaced.split_once(" | ").unwrap();
        let win = win_str.split_whitespace().collect_vec();
        let nums = nums_str.split_whitespace().collect_vec();
        let matches = win.iter().filter(|&s| nums.contains(s)).count() as u32;
        println!("{matches}");
        if matches > 1 {
            (1..matches).enumerate().map(|(idx, n)| 1_u32<<idx).sum::<u32>() + 1
        } else {
            matches
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"^Card +\d+: ").unwrap();
    let cards: HashMap<u32,u32> = HashMap::new();
    Some(input.lines().enumerate().map(|(idx, line)| {
        let replaced = re.replace(line, "");
        let (win_str, nums_str) = replaced.split_once(" | ").unwrap();
        let win = win_str.split_whitespace().collect_vec();
        let nums = nums_str.split_whitespace().collect_vec();
        let matches = win.iter().filter(|&s| nums.contains(s)).count() as u32;
        println!("{matches}");
        if matches > 1 {
            (1..matches).enumerate().map(|(idx, n)| 1_u32<<idx).sum::<u32>() + 1
        } else {
            matches
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
