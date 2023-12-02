static MAX_RED: usize = 12;
static MAX_GREEN: usize = 13;
static MAX_BLUE: usize = 14;

#[derive(Debug)]
struct Colour {
    red: usize,
    green: usize,
    blue: usize,
}

impl Colour {
    pub fn check_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }

    fn get_power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

fn parse_id(string: &str) -> u32 {
    string
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn parse_game(game_str: &str) -> Vec<Colour> {
    let mut subsets = Vec::new();

    let sets: Vec<&str> = game_str.split("; ").collect();
    for set in sets.iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes: Vec<&str> = set.split(',').collect();
        for cube in cubes {
            let parts: Vec<&str> = cube.trim().split(' ').collect();
            let count = parts[0].parse::<usize>().unwrap();
            match parts[1] {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => (),
            }
        }

        subsets.push(Colour { red, green, blue });
    }

    subsets
}

fn check_game_validity(game: Vec<Colour>) -> bool {
    game.into_iter()
        .all(|colour_subset| colour_subset.check_valid())
}

pub fn solve_part1(input: &str) -> u32 {
    let mut result_vec: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let splited_contents = line.split(":").collect::<Vec<_>>();
        let id = parse_id(splited_contents[0]);
        let game = parse_game(splited_contents[1]);

        if check_game_validity(game) {
            result_vec.push(id);
        }
    });

    result_vec.into_iter().sum()
}

fn get_minium_cubes(game: Vec<Colour>) -> Colour {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    game.iter().for_each(|subset_colour| {
        max_red = max_red.max(subset_colour.red);
        max_green = max_green.max(subset_colour.green);
        max_blue = max_blue.max(subset_colour.blue);
    });

    Colour {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let splited_contents = line.split(":").collect::<Vec<_>>();
            let game = parse_game(splited_contents[1]);

            get_minium_cubes(game).get_power()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = solve_part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, 2286);
    }
}
