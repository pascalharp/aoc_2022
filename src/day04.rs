pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|r| {
                let mut it = r.split('-');
                (
                    it.next().unwrap().parse::<u32>().unwrap(),
                    it.next().unwrap().parse::<u32>().unwrap(),
                )
            });

            let (a_from, a_to) = it.next().unwrap();
            let (b_from, b_to) = it.next().unwrap();

            if (a_from <= b_from && a_to >= b_to) || (b_from <= a_from && b_to >= a_to) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|r| {
                let mut it = r.split('-');
                (
                    it.next().unwrap().parse::<u32>().unwrap(),
                    it.next().unwrap().parse::<u32>().unwrap(),
                )
            });

            let (a_from, a_to) = it.next().unwrap();
            let (b_from, b_to) = it.next().unwrap();

            if a_from <= b_to && b_from <= a_to {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(super::part_one(include_str!("input/day04_example.txt")), 2);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input/day04.txt")), 651);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part_two(include_str!("input/day04_example.txt")), 4);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input/day04.txt")), 956);
    }
}
