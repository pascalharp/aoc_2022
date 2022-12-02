use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        return match self {
            Choice::Rock => {
                if *other == Choice::Paper {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            Choice::Paper => {
                if *other == Choice::Scissor {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            Choice::Scissor => {
                if *other == Choice::Rock {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        };
    }
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(format!("Invalid choice {}", s).into()),
        }
    }
}

impl Choice {
    fn play(&self, other: &Self) -> u32 {
        return match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        } + match self.partial_cmp(other).unwrap() {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
    }
}

#[derive(Debug)]
enum Strategy {
    Win,
    Lose,
    Draw,
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Invalid strategy {}", s).into()),
        }
    }
}

impl Strategy {
    fn choose(&self, other: &Choice) -> Choice {
        match self {
            Strategy::Win => match other {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissor,
                Choice::Scissor => Choice::Rock,
            },
            Strategy::Lose => match other {
                Choice::Rock => Choice::Scissor,
                Choice::Paper => Choice::Rock,
                Choice::Scissor => Choice::Paper,
            },
            Strategy::Draw => *other,
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let elf = split.next().unwrap().parse::<Choice>().unwrap();
            let me = split.next().unwrap().parse::<Choice>().unwrap();
            me.play(&elf)
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" ");
            let elf = split.next().unwrap().parse::<Choice>().unwrap();
            let me = split
                .next()
                .unwrap()
                .parse::<Strategy>()
                .unwrap()
                .choose(&elf);
            me.play(&elf)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example() {
        assert_eq!(super::part_one(include_str!("input/day02_example.txt")), 15);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input/day02.txt")), 14827);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part_two(include_str!("input/day02_example.txt")), 12);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input/day02.txt")), 13889);
    }
}
