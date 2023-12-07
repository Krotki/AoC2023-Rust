use std::cmp::Ordering;
use itertools::Itertools;
advent_of_code::solution!(7);


pub fn part_one(input: &str) -> Option<u64> {

    const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

    fn card_score(card: &char) -> u64 {
        return CARDS.iter().position(|&c| c == *card).unwrap() as u64 + 1;
    }

    let result = input.lines().map(|line| {
        let (code, bid) = line.split_whitespace().next_tuple().expect("Entry");

        let hand = code.chars().sorted().dedup_with_count().sorted_by(|(count1, c1), (count2, c2)| {
            let top = Ord::cmp(&count2, &count1);
            if top == Ordering::Equal {
                let card1 = card_score(c1);
                let card2 = card_score(c2);
                return Ord::cmp(&card2, &card1);
            }
            top
        }).collect_vec();

        println!("{hand:?}");

        let strength = match hand[..] {
            // Five of a kind
            [(_v @ 5, _)] => 7,
            // Four of a kind
            [(_v @ 4, _), (_v1 @ 1, _)] => 6,
            // Full house
            [(_v @ 3, _), (_v1 @ 2, _)] => 5,
            //Three of a kind
            [(_v @ 3, _), (_v1 @ 1, _), (_v2 @ 1, _)] => 4,
            // Two pair
            [(_v @ 2, _), (_v1 @ 2, _), (_v2 @ 1, _)] => 3,
            // Pair
            [(_v @ 2, _), (_v1 @ 1, _), (_v2 @ 1, _), (_v3 @ 1, _)] => 2,
            // High card
            [(_v @ 1, _), (_v1 @ 1, _), (_v2 @ 1, _), (_v3 @ 1, _), (_v4 @ 1, _)] => 1,
            _ => { panic!() }
        };

        let bid = bid.parse::<u64>().unwrap();
        let code2 = code.chars().map(|a| card_score(&a)).collect_vec();
        (strength, code2, code, bid)
    }).sorted().enumerate().map(|(idx, t)| {
        println!("{t:?}");
        (idx as u64 + 1) * t.3
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {

    const CARDS: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    fn card_score(card: &char) -> u64 {
        return CARDS.iter().position(|&c| c == *card).unwrap() as u64 + 1;
    }

    let result = input.lines().map(|line| {
        let (code, bid) = line.split_whitespace().next_tuple().expect("Entry");

        let mut hand = code.chars().sorted().dedup_with_count().sorted_by(|(count1, c1), (count2, c2)| {
            let top = Ord::cmp(&count2, &count1);
            if top == Ordering::Equal {
                let card1 = card_score(c1);
                let card2 = card_score(c2);
                return Ord::cmp(&card2, &card1);
            }
            top
        }).collect_vec();

        let jokers = hand.iter().enumerate().find(|(_, (count, card))| *card == 'J' && *count != 5);
        if let Some((idx, j)) = jokers {
            println!("{idx}, {j:?}");
            if hand[0].1 == 'J' {
                hand[1].0 += j.0;
            } else {
                hand[0].0 += j.0;
            }
            hand.remove(idx);
        }

        println!("{hand:?}");

        let strength = match hand[..] {
            // Five of a kind
            [(_v @ 5, _)] => 7,
            // Four of a kind
            [(_v @ 4, _), (_v1 @ 1, _)] => 6,
            // Full house
            [(_v @ 3, _), (_v1 @ 2, _)] => 5,
            //Three of a kind
            [(_v @ 3, _), (_v1 @ 1, _), (_v2 @ 1, _)] => 4,
            // Two pair
            [(_v @ 2, _), (_v1 @ 2, _), (_v2 @ 1, _)] => 3,
            // Pair
            [(_v @ 2, _), (_v1 @ 1, _), (_v2 @ 1, _), (_v3 @ 1, _)] => 2,
            // High card
            [(_v @ 1, _), (_v1 @ 1, _), (_v2 @ 1, _), (_v3 @ 1, _), (_v4 @ 1, _)] => 1,
            _ => { panic!() }
        };

        let bid = bid.parse::<u64>().unwrap();
        let code2 = code.chars().map(|a| card_score(&a)).collect_vec();
        (strength, code2, code, bid)
    }).sorted().enumerate().map(|(idx, t)| {
        println!("{t:?}");
        (idx as u64 + 1) * t.3
    }).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
