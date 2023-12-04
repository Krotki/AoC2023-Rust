use std::cmp::max;
use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    return Some(input.lines().map(|line| {
        let (game_str, rest) = line.split_once(":").unwrap();
        let (_, nr) = game_str.split_once(" ").unwrap();

        let is_possible = rest.split(&[',', ';', ' '])
            .filter(|e| !e.is_empty())
            .tuples()
            .map(|(value, color)| (value.parse::<i32>().unwrap(), color) )
            .all(|t: (i32, &str)| {
                match t {
                    (_v @ 0..=12, "red") => { true }
                    (_v @ 0..=14, "blue") => { true }
                    (_v @ 0..=13, "green") => { true }
                    _ => false
                }
            });

        return if is_possible {
            nr.parse().unwrap()
        } else {
            0
        }
    }).sum());
}

pub fn part_two(input: &str) -> Option<i32> {
    return Some(input.lines().map(|line| {
        let (_, rest) = line.split_once(":").unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (value, color) in rest.split(&[',', ';', ' '])
            .filter(|e| !e.is_empty())
            .tuples()
            .map(|(value, color)| (value.parse::<i32>().unwrap(), color) )
        {
            match color {
                "red" => { red = max(red, value) }
                "green" => { green = max(green, value)}
                "blue" => { blue = max(blue, value)}
                &_ => panic!("Unknown color!")
            }
        }

        red * green * blue
    }).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
