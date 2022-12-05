use std::collections::LinkedList;

fn parse_crates(input: &str) -> Vec<LinkedList<char>> {
    let mut input = input.lines();
    // Consume last line first
    let crate_count = input
        .next_back()
        .unwrap()
        .split(' ')
        .filter(|e| !e.is_empty())
        .count();

    let mut crates: Vec<LinkedList<char>> = Vec::with_capacity(crate_count);
    for _ in 0..crate_count {
        crates.push(LinkedList::new());
    }

    for l in input {
        for (i, c) in l.as_bytes().chunks(4).enumerate() {
            if c[1] != b' ' {
                crates.get_mut(i).unwrap().push_front(c[1] as char);
            }
        }
    }

    crates
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut digits = l
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|d| d.parse::<usize>().unwrap());
            (
                digits.next().unwrap(),
                digits.next().unwrap() - 1,
                digits.next().unwrap() - 1,
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> String {
    let mut input = input.split("\n\n");
    let mut crates = parse_crates(input.next().unwrap());
    let moves = parse_moves(input.next().unwrap());

    for (count, from, to) in moves {
        let from = &mut crates[from];
        let take = from.split_off(from.len() - count);
        crates[to].extend(take.iter().rev())
    }

    crates
        .iter()
        .filter_map(|ll| ll.back())
        .collect()
}

pub fn part_two(input: &str) -> String {
    let mut input = input.split("\n\n");
    let mut crates = parse_crates(input.next().unwrap());
    let moves = parse_moves(input.next().unwrap());

    for (count, from, to) in moves {
        let from = &mut crates[from];
        let mut take = from.split_off(from.len() - count);
        crates[to].append(&mut take);
    }

    crates
        .iter()
        .filter_map(|ll| ll.back())
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(super::part_one(include_str!("input/day05_example.txt")), "CMZ");
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input/day05.txt")), "JCMHLVGMG");
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part_two(include_str!("input/day05_example.txt")), "MCD");
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input/day05.txt")), "LVMRWSSPZ");
    }
}
