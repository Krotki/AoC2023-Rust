use regex::Regex;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> i32 {
    return input.lines().map(|line| {
        let first = line.matches(|c: char| c.is_ascii_digit()).next().unwrap();
        let last = line.rmatches(|c: char| c.is_ascii_digit()).next().unwrap();
        format!("{}{}", first, last).parse::<i32>().unwrap()
    }).sum();
}

fn to_nums(str_num: &str) -> &str {
    return match str_num {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => panic!()
    };
}
pub fn part_two(input: &str) -> i32 {
    let first_re: Regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
    let last_re: Regex = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
    return input.lines().map(|line| {
        let (_, [first]) = first_re.captures(line).unwrap().extract();
        let (_, [last]) = last_re.captures(line).unwrap().extract();

        let first = to_nums(first);
        let last = to_nums(last);

        format!("{}{}", first, last).parse::<i32>().unwrap()
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, 281);
    }
}
