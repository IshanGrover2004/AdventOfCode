#[derive(Debug)]
struct Input {
    races: Vec<Race>,
}

// Sample Input Part1
// Time:      7  15   30
// Distance:  9  40  200
impl Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let time_vec = lines
            .next()
            .unwrap()
            .split_whitespace()
            .into_iter()
            .filter_map(|str| str.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let distance_vec = lines
            .next()
            .unwrap()
            .split_whitespace()
            .into_iter()
            .filter_map(|str| str.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let races = time_vec
            .into_iter()
            .zip(distance_vec)
            .map(|(time, distance)| Race::new(time, distance))
            .collect::<Vec<_>>();

        Input { races }
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Race { time, distance }
    }

    fn get_winning(&self) -> Vec<u64> {
        let mut winning: Vec<u64> = Vec::new();

        for i in 1..=self.time {
            let button_hold = i;
            let time_left = self.time - button_hold;

            let mul = button_hold * time_left;
            if mul > self.distance {
                winning.push(button_hold);
            }
        }

        winning
    }
}

fn get_max_min_diff(vec: Vec<u64>) -> u64 {
    let max = vec.iter().max().unwrap();
    let min = vec.iter().min().unwrap();

    max - min + 1
}

pub fn solve_part1(input: &str) -> u64 {
    let race = Input::parse(input).races;
    race.iter()
        .map(|race| {
            let winning = race.get_winning();
            get_max_min_diff(winning)
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, str_num| acc + str_num)
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, str_num| acc + str_num)
        .parse::<u64>()
        .unwrap();

    Race { time, distance }
}

pub fn solve_part2(input: &str) -> u64 {
    let race = parse_input2(input);

    let winning = race.get_winning();

    get_max_min_diff(winning)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = solve_part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = solve_part2(input);
        assert_eq!(result, 71503);
    }
}
