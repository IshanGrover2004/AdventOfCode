use std::collections::HashSet;

use itertools::Itertools;

pub fn find_marker(input: &str, sequence_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(sequence_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == sequence_size)
        .map(|pos| pos + sequence_size)

    // Alternating solution using array of booleans
    /*input
        .as_bytes()
        .windows(4)
        .position(|window| {
            let mut arr = [false; 256];
            for &item in window {
                if arr[item as usize] {
                    return false;
                }
                arr[item as usize] = true;
            }
            return true;
        })
        .map(|pos| pos + 4)
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_marker_4() {
        use super::find_marker;
        assert_eq!(Some(7), find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(Some(5), find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(Some(6), find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(
            Some(10),
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
        );
        assert_eq!(Some(11), find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test_find_marker_14() {
        use super::find_marker;

        assert_eq!(Some(19), find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(Some(23), find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(Some(23), find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(
            Some(29),
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
        );
        assert_eq!(
            Some(26),
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)
        );
    }

    // #[test]
    // fn part1_test() {}
    //
    // #[test]
    // fn part2_test() {}
}
