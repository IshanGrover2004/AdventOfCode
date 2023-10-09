use day_4::{part1, part2};

fn main() {
    let input = include_str!("input.txt");

    dbg!(part1::method1(&input));

    dbg!(part2::method1(&input));
}
