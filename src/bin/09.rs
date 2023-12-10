use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let result = input.lines().map(|line| {
        let mut agg: Vec<Vec<i32>> = Vec::with_capacity(50);
        let numbers = line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect_vec();
        agg.push(numbers.clone());
        let mut c = numbers;
        loop {
            let b = c.iter().copied().tuple_windows().map(|(a, b)| b - a).collect_vec();
            agg.push(b.clone());
            if b.iter().all(|v| *v == 0) {
                break
            }
            c = b;
        }

        return agg.iter().rfold(0_i32, |acc, vec| {
            acc + vec.last().unwrap()
        });
    }).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let result = input.lines().map(|line| {
        let mut agg: Vec<Vec<i32>> = Vec::with_capacity(50);
        let numbers = line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect_vec();
        agg.push(numbers.clone());
        let mut c = numbers;
        loop {
            let b = c.iter().copied().tuple_windows().map(|(a, b)| b - a).collect_vec();
            agg.push(b.clone());
            if b.iter().all(|v| *v == 0) {
                break
            }
            c = b;
        }

        return agg.iter().rfold(0_i32, |acc, vec| {
            vec.first().unwrap() - acc
        });
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_single() {
        let result = part_two("10 13 16 21 30 45");
        assert_eq!(result, Some(5));
    }
}
