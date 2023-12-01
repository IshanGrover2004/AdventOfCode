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

pub fn solve_part2(input: &str) {}
