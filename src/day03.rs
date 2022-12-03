use std::collections::HashSet;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a') as u32 + 1,
        'A'..='Z' => (c as u8 - b'A') as u32 + 27,
        _ => panic!("Unexpected common letter"),
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let map: HashSet<char> = left.chars().collect();
            let common = right.chars().find(|c| map.contains(c)).unwrap();
            priority(common)
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .array_chunks()
        .map(|[a, b, c]| {
            let a: HashSet<char> = a.chars().collect();
            let b: HashSet<char> = b.chars().collect();
            let c: HashSet<char> = c.chars().collect();
            let ab: HashSet<char> = a.intersection(&b).map(|c| c.to_owned()).collect();
            let common = ab.intersection(&c).next().unwrap().to_owned();
            priority(common)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(
            super::part_one(include_str!("input/day03_example.txt")),
            157
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input/day03.txt")), 7581);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part_two(include_str!("input/day03_example.txt")), 70);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input/day03.txt")), 2525);
    }
}
