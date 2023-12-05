use std::collections::HashMap;

fn parse_numbers(string: &str) -> Vec<u8> {
    string
        .split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
}

fn eval_points(winning_numbers: Vec<u8>, my_numbers: Vec<u8>) -> u32 {
    let mut total = 0;
    let mut score: i32 = -1;
    for num in my_numbers {
        if winning_numbers.contains(&num) {
            score += 1;
        }
    }

    if score > -1 {
        total += 2_u32.pow(score as u32);
    }

    total as u32
}

pub fn solve_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let cards = line
                .split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .collect::<Vec<_>>();
            let winning_numbers = parse_numbers(cards[0]);
            let my_numbers = parse_numbers(cards[1]);

            eval_points(winning_numbers, my_numbers)
        })
        .collect::<Vec<_>>();

    result.into_iter().sum()
}

pub fn matching_numbers(winning_numbers: Vec<u8>, numbers: Vec<u8>) -> usize {
    numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut copies = HashMap::new();

    input.lines().enumerate().for_each(|(idx, line)| {
        let cards = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .collect::<Vec<_>>();
        let winning_numbers = parse_numbers(cards[0]);
        let my_numbers = parse_numbers(cards[1]);

        let count = matching_numbers(winning_numbers, my_numbers);

        let copy = copies.entry(idx + 1).or_default();
        *copy += 1;
        let copy = *copy;

        for id in idx + 2..=(idx + 1 + count) {
            *copies.entry(id).or_default() += copy;
        }
    });

    copies.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = solve_part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = solve_part2(input);
        assert_eq!(result, 30);
    }
}
