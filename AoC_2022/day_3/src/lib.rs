#![feature(iter_array_chunks)]

pub mod part1 {
    use std::collections::HashMap;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Item(u8);

    // Return Item struct having value of char
    impl TryFrom<u8> for Item {
        type Error = String;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err("Invalid character".to_string()),
            }
        }
    }

    impl Item {
        pub fn score(&self) -> usize {
            match self.0 {
                b'a'..=b'z' => 1 + (self.0 - b'a') as usize,
                b'A'..=b'Z' => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
    pub fn method1(input: &str) {
        let score: u32 = input
            .lines()
            .into_iter()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);

                let item1 = first
                    .bytes()
                    .map(Item::try_from)
                    .collect::<Result<Vec<_>, String>>()
                    .unwrap();

                second
                    .bytes()
                    .map(Item::try_from)
                    .find_map(|item| {
                        item.ok()
                            .and_then(|item| item1.iter().find(|&&first_item| first_item == item))
                    })
                    .expect("any reason")
                    .score() as u32
            })
            .sum();

        println!("Part 1 (Method 1): {}", score);
    }

    pub fn method2(input: &str) {
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

        println!("Part 1 (Method 2): {}", result);
    }
}

use std::collections::HashMap;

pub fn solve_part2(input: &str) {
    let char_with_val = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, char)| (char, index + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            char_with_val.get(&common_char).unwrap()
        })
        .sum::<usize>();

    println!("Part 2: {}", result);
}
