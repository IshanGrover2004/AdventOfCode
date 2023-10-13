#[derive(Debug)]
pub enum Error {
    ParseError(&'static str),
    FindError(&'static str),
}

pub mod part1 {
    use std::str::FromStr;

    type Stack = Vec<char>;

    #[derive(Debug)]
    struct Move {
        count: usize,
        from: usize,
        to: usize,
    }

    //                        count   from  to
    //                          👇     👇   👇
    // Parse this line -> "move 5 from 4 to 5"
    impl FromStr for Move {
        type Err = super::Error;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let mut split_line = input.split(" ").into_iter();

            split_line.next();

            // Find out count from line
            let count = split_line
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

    // fn parse_stacks(input: &str)

    fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        // Check if the input matrix is empty
        if matrix.is_empty() {
            return vec![];
        }

        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        // Initialize the result matrix with the same number of columns as rows
        let mut transposed_matrix: Vec<Vec<T>> = vec![vec![Default::default(); num_rows]; num_cols];

        for i in 0..num_rows {
            for j in 0..num_cols {
                transposed_matrix[j][i] = matrix[i][j].clone();
            }
        }

        transposed_matrix
    }

    pub fn solve(input: &str) {
        // let x = "move 5 from 4 to 5";
        // let s = Move::from_str(x).map_err(|_e| eprintln!("error")).unwrap();
        // dbg!(s);
    }
}

pub mod part2 {
    pub fn solve(input: &str) {}
}
