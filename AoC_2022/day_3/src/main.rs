fn main() {
    let input = include_str!("input.txt");

    day_3::part1::method1(&input);
    day_3::part1::method2(&input);
    day_3::solve_part2(&input);
}
