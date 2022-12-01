pub fn part_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(str::parse::<u32>)
                .map(std::result::Result::unwrap)
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut vals = input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(str::parse::<u32>)
                .map(std::result::Result::unwrap)
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    vals.sort();
    vals.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(
            super::part_one(include_str!("input/day01_example.txt")),
            24000
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input/day01.txt")), 70764);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            super::part_two(include_str!("input/day01_example.txt")),
            45000
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input/day01.txt")), 203905);
    }
}
