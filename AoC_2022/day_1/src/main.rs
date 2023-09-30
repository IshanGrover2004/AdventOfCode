fn main() {
    let result1 = method_1().unwrap();
    let result2 = method_2().unwrap();

    assert_eq!(result1, result2);
}

// Method 1:
fn method_1() -> Result<u64, std::io::Error> {
    // Reading contents from input file
    let input_str = std::fs::read_to_string("src/input.txt")?;

    let mut max = 0;

    // looping and getting each elf (group) by splitting("\n")
    for group in input_str.split("\n\n") {
        let mut sum = 0;

        // Geting each lines/values of 1 elf/group
        for each_value in group.lines() {
            // Converting string input to u64
            let each_value = each_value.parse::<u64>().unwrap();
            sum += each_value;
        }

        // Checking for max value
        if sum > max {
            max = sum;
        }
    }

    println!("Result by method 1 = {max}");
    Ok(max)
}

// Mathod 2:
fn method_2() -> Result<u64, std::io::Error> {
    // Reading lines from input.txt in Vector
    let lines = include_str!("input.txt")
        .lines()
        .map(|val| val.parse::<u64>().ok())
        .collect::<Vec<_>>();

    // Extract 1 elf/group & collect the sum of elf calorie in vector
    let group_of_elf_sum = lines
        .split(|lines| lines.is_none())
        .map(|group| group.into_iter().map(|value| value.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();

    // Find max element from group_of_elf_sum by iterating
    let result = group_of_elf_sum.into_iter().max();
    println!("Result by method 2 = {}", result.unwrap());

    Ok(result.unwrap())
}
