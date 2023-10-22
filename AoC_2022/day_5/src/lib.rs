use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    ParseError(&'static str),
    FindError(&'static str),
}

#[derive(Debug)]
pub(crate) struct Move {
    count: usize,
    from: usize,
    to: usize,
}

//                        count   from  to
//                          👇     👇   👇
// Parse this line -> "move 5 from 4 to 5"
impl FromStr for Move {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split_line = input.split(" ").into_iter();

        split_line.next();

        // Find out "count" from line
        let count: &str = split_line
            .next()
            .ok_or(Self::Err::FindError("Unable to find count"))?;
        // Parsing count in usize
        let count = count
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError("failed to parse \"count\": {count}"))?;

        split_line.next();

        // Find out "from"
        let from = split_line
            .next()
            .ok_or(Self::Err::FindError("Unable to find \"to\""))?;
        // Parse "from" in usize
        let from = from
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError("failed to parse \"from\": {from}"))?;

        split_line.next();

        // Find out "from"
        let to = split_line
            .next()
            .ok_or(Self::Err::FindError("Unable to find \"to\": {to}"))?;
        // Parse "from" in usize
        let to = to
            .parse::<usize>()
            .map_err(|_| Self::Err::ParseError("failed to parse \"to\": {to}"))?;

        Ok(Move { count, from, to })
    }
}

impl Move {
    pub(crate) fn move_crates(&self, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let from = &mut stacks[self.from - 1];
        let mut moved = from.drain(0..self.count).collect::<Vec<_>>();

        moved.reverse();

        let to = &mut stacks[self.to - 1];
        to.splice(0..0, moved);

        stacks
    }

    pub(crate) fn move_multiple_crates(&self, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let from = &mut stacks[self.from - 1];
        let moved = from.drain(0..self.count).collect::<Vec<_>>();

        let to = &mut stacks[self.to - 1];
        to.splice(0..0, moved);

        stacks
    }
}

// Parse the stacks(in string) to format of Stack ( Vec<Vec<char>> )
pub(crate) fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let lines: Vec<Vec<char>> = transpose(lines);

    (1..lines.len())
        .step_by(4)
        .map(|i| {
            let line = &lines[i];
            let line = &line[..line.len() - 1];
            line.to_vec()
                .into_iter()
                .filter(|c| c.is_alphabetic())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

pub(crate) fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if matrix[0].len() == 0 {
        return vec![];
    }

    (0..matrix[0].len())
        .map(|colm| matrix.iter().map(|row| row[colm].clone()).collect())
        .collect()
}

pub(crate) fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    (
        parse_stacks(stacks),
        moves
            .lines()
            .map(|line| Move::from_str(line).expect("Parsing problem"))
            .collect::<Vec<Move>>(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let (stacks, moves) = parse_input(input);
    let mut stacks = stacks;

    for m in moves {
        stacks = m.move_crates(stacks);
    }

    // Return stacks upper character
    stacks
        .into_iter()
        .flat_map(|c| c.first().copied())
        .collect::<String>()
}

pub fn solve_part2(input: &str) -> String {
    let (stacks, moves) = parse_input(input);
    let mut stacks = stacks;

    for m in moves {
        stacks = m.move_multiple_crates(stacks);
    }

    // Return stacks upper character
    stacks
        .into_iter()
        .flat_map(|c| c.first().copied())
        .collect::<String>()
}
