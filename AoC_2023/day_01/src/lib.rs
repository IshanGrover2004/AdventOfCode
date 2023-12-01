fn parse_numbers(str: &str) -> Vec<u16> {
    str.chars()
        .filter_map(|ch| ch.to_string().parse::<u16>().ok())
        .collect::<Vec<_>>()
}

fn get_calibration_value(vec: &Vec<u16>) -> u16 {
    let first = vec.first().unwrap_or(&0);
    let last = vec.last().unwrap_or(&0);

    if last == &0 {
        return first * 10 + first;
    }

    first * 10 + last
}

pub fn solve_part1(input: &str) -> u16 {
    let parsed_vec_int = input
        .lines()
        .map(|line| parse_numbers(line))
        .collect::<Vec<Vec<_>>>();

    parsed_vec_int
        .iter()
        .map(|vec| get_calibration_value(vec))
        .sum::<u16>()
}

fn parse_chars_num(str: &str) -> Vec<u16> {
    let mut numbers_vec: Vec<u16> = Vec::new();
    let mut current_number_str = String::new();

    let words: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (pos, ch) in str.chars().enumerate() {
        if ch.is_digit(10) {
            let val = ch.to_string().parse::<u16>().unwrap();

            numbers_vec.push(val);
            current_number_str.clear();
        } else {
            let s: &str = &str[pos..str.len()];
            for (idx, word_num) in words.iter().enumerate() {
                if s.starts_with(word_num) {
                    numbers_vec.push(idx as u16);

                    break;
                }
            }
        }
    }

    numbers_vec
}

pub fn solve_part2(input: &str) -> u16 {
    let parsed_vec_int = input
        .lines()
        .map(|line| parse_chars_num(line))
        .collect::<Vec<Vec<_>>>();

    parsed_vec_int
        .iter()
        .map(|vec| get_calibration_value(vec))
        .sum::<u16>()
}
