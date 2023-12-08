use std::collections::HashMap;
use itertools::Itertools;
use num::integer::lcm;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let sym = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(sym, (left, right));
    }

    let mut cnode = "AAA";
    let mut count = 0_u64;
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => cnode = map[cnode].0,
            'R' => cnode = map[cnode].1,
            _ => panic!("Unknown instruction!")
        }
        count += 1;
        if cnode == "ZZZ" {
            break;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let sym = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(sym, (left, right));
    }

    let mut nodes = map.keys().filter(|&&k| k.ends_with('A')).copied().collect_vec();
    let mut counts = Vec::with_capacity(nodes.len());

    for node in &mut nodes {
        let mut count = 0_u64;
        for instruction in instructions.chars().cycle() {
            match instruction {
                'L' => *node = map[node].0,
                'R' => *node = map[node].1,
                _ => panic!("Unknown instruction!")
            }
            count += 1;
            if node.ends_with('Z') {
                break;
            }
        }
        counts.push(count);
    }

    let result = counts.iter().fold(1_u64, |a, count| {
        lcm(a, *count)
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
