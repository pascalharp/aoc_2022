use std::collections::HashSet;

pub fn solution(input: &str, win_size: usize) -> usize {
    input
        .as_bytes()
        .windows(win_size)
        .enumerate()
        .find_map(|(idx, w)| {
            if w.iter().collect::<HashSet<_>>().len() != w.len() {
                None
            } else {
                Some(idx + win_size)
            }
        })
        .unwrap()
}

pub fn part_one(input: &str) -> usize {
    solution(input, 4)
}

pub fn part_two(input: &str) -> usize {
    solution(input, 14)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(
            super::part_one(include_str!("input/day06_example_one.txt")), 7
        );
        assert_eq!(
            super::part_one(include_str!("input/day06_example_two.txt")), 5
        );
        assert_eq!(
            super::part_one(include_str!("input/day06_example_three.txt")), 6
        );
        assert_eq!(
            super::part_one(include_str!("input/day06_example_four.txt")), 10
        );
        assert_eq!(
            super::part_one(include_str!("input/day06_example_five.txt")), 11
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            super::part_one(include_str!("input/day06.txt")), 1920
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            super::part_two(include_str!("input/day06_example_one.txt")), 19
        );
        assert_eq!(
            super::part_two(include_str!("input/day06_example_two.txt")), 23
        );
        assert_eq!(
            super::part_two(include_str!("input/day06_example_three.txt")), 23
        );
        assert_eq!(
            super::part_two(include_str!("input/day06_example_four.txt")), 29
        );
        assert_eq!(
            super::part_two(include_str!("input/day06_example_five.txt")), 26
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            super::part_two(include_str!("input/day06.txt")), 2334
        );
    }
}
