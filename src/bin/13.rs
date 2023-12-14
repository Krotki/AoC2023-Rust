use color_print::cprintln;
use itertools::Itertools;
advent_of_code::solution!(13);

fn transpose(v: &Vec<String>) -> Vec<String> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner.as_bytes()[i].clone() as char).join(""))
        .collect()
}

fn find_mirror(pattern: &Vec<String>) -> Option<usize> {
    assert!(!pattern.is_empty());
    for idx in 0..pattern.len() - 1 {
        if pattern[idx] == pattern[idx + 1] {
            let mut is_mirror = true;
            let mut min = idx.clone();
            let mut max = idx.clone() + 1;
            while min > 0 && max < pattern.len() - 1 {
                min -= 1;
                max += 1;
                if pattern[min] != pattern[max] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                return Some(idx + 1);
            }
        }
    }
    None
}

fn find_mirror2(pattern: &Vec<String>) -> Option<usize> {
    assert!(!pattern.is_empty());
    for idx in 0..pattern.len() - 1 {
        let mut min = idx.clone() as i32;
        let mut max = idx.clone() as i32 + 1;
        let mut distance = 0;
        let mut changed_line: (usize, usize) = (0, 0);
        while min >= 0 && max < pattern.len() as i32 {
            let chars_min = pattern[min as usize].as_bytes();
            let chars_max = pattern[max as usize].as_bytes();

            let mut dst = 0;
            for i in 0..pattern[min as usize].len() {
                if chars_min[i] != chars_max[i] { dst += 1 }
            }

            distance += dst;
            if dst == 1 {
                changed_line = (min as usize, max as usize);
            }
            min -= 1;
            max += 1;
        }
        if distance == 1 {
            let mir_size = idx.min(pattern.len() - 1 - idx);
            let mir = idx + 1 - mir_size..=idx + mir_size + 1;

            println!("Diff: {distance}");
            for (i, l) in pattern.iter().enumerate() {
                if mir.contains(&i) {
                    cprintln!("<blue>{l}</blue>");
                } else {
                    cprintln!("<white>{l}</white>");
                }

                if i == idx {
                    cprintln!("<red>────────");
                }
            }
            println!("\n");

            return Some(idx + 1);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = input.split("\n\n").collect_vec();
    let result = patterns.iter().map(|&pattern| {
        let pattern = pattern.lines().map(|l| l.to_string()).collect_vec();
        if let Some(value) = find_mirror(&pattern) {
            return 100 * value as u32;
        }
        if let Some(value) = find_mirror(&transpose(&pattern)) {
            return value as u32;
        }
        panic!("No mirror in pattern found!");
    }).sum();

    Some(result)
}


pub fn part_two(input: &str) -> Option<u32> {
    let patterns = input.split("\n\n").collect_vec();
    let result = patterns.iter().enumerate().map(|(idx, &pattern)| {
        print!("Pattern: {idx} ");
        let pattern = pattern.lines().map(|l| l.to_string()).collect_vec();
        if let Some(value) = find_mirror2(&pattern) {
            return 100 * value as u32;
        }
        print!("Transposed ");
        if let Some(value) = find_mirror2(&transpose(&pattern)) {
            return value as u32;
        }
        panic!("No mirror in pattern found!");
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let output = r"#.##..#
..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##...";

        let input = input.lines().map(|l| l.to_string()).collect_vec();
        let output = output.lines().map(|l| l.to_string()).collect_vec();

        let result = transpose(&input);

        // for line in &result {
        //     println!("{line}")
        // }

        assert_eq!(result, output);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_part_two2() {
        let input = r"#...#..##..
.########.#
#.#...##...
#.#...##...
.########.#
#...#..##..
..#.###..#.
#.####.#..#
#.####.#..#
..#.###....
#...#..##..
.########.#
#.#...##...";

        let result = part_two(input);
        // assert_eq!(result, Some(400));
    }
}
