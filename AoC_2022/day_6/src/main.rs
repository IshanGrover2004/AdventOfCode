fn main() {
    let input = include_str!("input.txt");

    dbg!(day_6::find_marker(&input, 4));
    dbg!(day_6::find_marker(&input, 14));
}
