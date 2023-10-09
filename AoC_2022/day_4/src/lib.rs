pub mod part1 {
    use std::ops::{RangeBounds, RangeInclusive};

    trait InclusiveRangeExt {
        // Check one range contains fully in another range
        fn contain_range(&self, other: &Self) -> bool;
        fn is_contained(&self, other: &Self) -> bool {
            self.contain_range(other) || other.contain_range(self)
        }
    }

    impl<T> InclusiveRangeExt for RangeInclusive<T>
    where
        T: PartialOrd,
    {
        fn contain_range(&self, other: &Self) -> bool {
            self.contains(other.start()) && self.contains(other.end())
        }
    }

    pub fn method1(input: &str) -> usize {
        let result = input
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|range| {
                        let mut iter = range
                            .split("-")
                            .map(|num| num.parse::<u32>().expect("Failed to parse in u32"));
                        let start = iter.next().unwrap();
                        let end = iter.next().unwrap();
                        start..=end
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|ranges| {
                ranges.iter().enumerate().any(|(i, a)| {
                    ranges
                        .iter()
                        .enumerate()
                        .any(|(j, b)| i != j && a.is_contained(b))
                })
            })
            .count();

        result
    }
}

pub mod part2 {
    use std::ops::{RangeBounds, RangeInclusive};

    trait InclusiveRangeExt {
        // Check one range contains fully in another range
        fn overlap(&self, other: &Self) -> bool;
        fn is_overlaped(&self, other: &Self) -> bool {
            self.overlap(other) || other.overlap(self)
        }
    }

    impl<T> InclusiveRangeExt for RangeInclusive<T>
    where
        T: PartialOrd,
    {
        fn overlap(&self, other: &Self) -> bool {
            self.contains(other.start()) || self.contains(other.end())
        }
    }
    pub fn method1(input: &str) -> usize {
        let result = input
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|range| {
                        let mut iter = range
                            .split("-")
                            .map(|num| num.parse::<u32>().expect("Failed to parse in u32"));
                        let start = iter.next().unwrap();
                        let end = iter.next().unwrap();
                        start..=end
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|ranges| {
                ranges.iter().enumerate().any(|(i, a)| {
                    ranges
                        .iter()
                        .enumerate()
                        .any(|(j, b)| i != j && a.is_overlaped(b))
                })
            })
            .count();

        result
    }
}

#[cfg(test)]
mod test {
    use crate::part1;

    fn test_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part1::method1(input), 2);
    }
}
