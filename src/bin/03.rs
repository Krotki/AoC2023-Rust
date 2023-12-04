use euclid::{Box2D, point2, Point2D, UnknownUnit};
use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(3);


#[inline]
fn contains_i(b: Box2D<i32, UnknownUnit>, p: Point2D<i32, UnknownUnit>) -> bool {
    b.min.x <= p.x && p.x <= b.max.x && b.min.y <= p.y && p.y <= b.max.y
}

fn offset_to_x_y(offset: i32, line_size: i32, newline_size: i32) -> (i32, i32) {
    let y = offset / (line_size + newline_size); // + 2 chars for \r\n
    let x = (offset - (y * newline_size)) % line_size;
    (x, y)
}


pub fn part_one(input: &str) -> Option<i64> {
    let (xsize, nlsize) = if let Some(idx) = input.find("\r\n") {
        (idx as i32, 2)
    } else if let Some(idx) = input.find("\n") {
        (idx as i32, 1)
    } else {
        panic!()
    };

    println!("xsize: {xsize} new_line: {nlsize}");

    let mut numbers: Vec<(Box2D<i32, UnknownUnit>, i32)> = vec![];
    let mut numbers_to_add: Vec<i64> = vec![];

    let re = Regex::new(r#"\d+"#).unwrap();

    re.captures_iter(input).for_each(|cap| {
        let m = cap.get(0).unwrap();
        let v = cap[0].parse::<i32>().unwrap();

        let (x1, _) = offset_to_x_y(m.start() as i32, xsize, nlsize);
        let (x2, y) = offset_to_x_y(m.end() as i32 - 1, xsize, nlsize);
        // println!("y: {y} x1: {x1} x2: {x2}");

        numbers.push((Box2D::new(point2(x1 - 1, y - 1), point2(x2 + 1, y + 1)), v))
    });

    println!("{numbers:?}");

    let re = Regex::new(r#"[^.0-9\r\n]"#).unwrap();
    re.captures_iter(input).for_each(|cap| {
        let m = cap.get(0).unwrap();
        let (x, y) = offset_to_x_y(m.start() as i32, xsize, nlsize);
        let point = point2(x, y);

        println!("{point:?} ==> {}", &cap[0]);

        numbers.retain(|a| {
            if contains_i(a.0, point) {
                numbers_to_add.push(a.1 as i64);
                println!("Found intersect with num {}", a.1);
                false
            } else {
                true
            }
        });
    });

    // println!("{numbers_to_add:#?}");

    Some(numbers_to_add.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (xsize, nlsize) = if let Some(idx) = input.find("\r\n") {
        (idx as i32, 2)
    } else if let Some(idx) = input.find("\n") {
        (idx as i32, 1)
    } else {
        panic!()
    };

    println!("xsize: {xsize} new_line: {nlsize}");

    let mut numbers: Vec<(Box2D<i32, UnknownUnit>, i32)> = vec![];
    let mut numbers_to_add: Vec<i32> = vec![];

    let re = Regex::new(r#"\d+"#).unwrap();

    re.captures_iter(input).for_each(|cap| {
        let m = cap.get(0).unwrap();
        let v = cap[0].parse::<i32>().unwrap();

        let (x1, _) = offset_to_x_y(m.start() as i32, xsize, nlsize);
        let (x2, y) = offset_to_x_y(m.end() as i32 - 1, xsize, nlsize);

        numbers.push((Box2D::new(point2(x1 - 1, y - 1), point2(x2 + 1, y + 1)), v))
    });

    println!("{numbers:?}");

    let re = Regex::new(r#"[*]"#).unwrap();
    re.captures_iter(input).for_each(|cap| {
        let m = cap.get(0).unwrap();
        let (x, y) = offset_to_x_y(m.start() as i32, xsize, nlsize);
        let point = point2(x, y);

        println!("{point:?} ==> {}", &cap[0]);

        match numbers.iter().filter(|a| contains_i(a.0, point)).collect_tuple() {
            Some(((_, first), (_, second))) => numbers_to_add.push(first * second),
            None => {}
        }
    });

    // println!("{numbers_to_add:#?}");

    Some(numbers_to_add.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset() {
        let result = offset_to_x_y(9, 10, 2);
        assert_eq!(result, (9, 0));

        let result = offset_to_x_y(12, 10, 2);
        assert_eq!(result, (0, 1));

        let result = offset_to_x_y(117, 10, 2);
        assert_eq!(result, (9, 9));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
