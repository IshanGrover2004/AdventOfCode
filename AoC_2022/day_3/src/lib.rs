use std::{collections::HashMap, usize};

pub fn solve_part1(input: &str) {
    let char_with_val = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, char)| (char, index + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|line| {
            let str_size: usize = line.len();

            let compartment_1 = &line[0..=str_size / 2];
            let compartment_2 = &line[(str_size / 2)..str_size];

            let common_char = compartment_1
                .chars()
                .find(|c| compartment_2.contains(*c))
                .unwrap();

            char_with_val.get(&common_char).unwrap()
        })
        .sum::<usize>();

    println!("Part 1: {}", result);
}
pub fn solve_part2(input: &str) {}
