use euclid::{Box2D, point2};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic: Vec<&str> = input.lines().collect();

    for (y, &line) in schematic.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            Box2D::new(point2(x,y), point2(x,y)).
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
