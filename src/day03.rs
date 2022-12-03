use itertools::Itertools;
use std::collections::HashSet;

fn input_generator_part1(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            (first, second)
        })
        .collect()
}

fn common_chars(s1: &str, s2: &str, s3: &str) -> Vec<char> {
    let mut common = Vec::new();

    // create a vector of characters for each string
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let s3_chars: Vec<char> = s3.chars().collect();

    // iterate over the characters in the first string
    for c in s1_chars {
        // check if the character is also in the second and third strings
        if s2_chars.contains(&c) && s3_chars.contains(&c) {
            common.push(c);
        }
    }

    common.iter().unique().cloned().collect()
}

fn char_to_alphabet_number(c: char) -> u64 {
    let a = b'a';
    let capital_a = b'A';

    if ('a'..='z').contains(&c) {
        (c as u8 - a + 1) as u64
    } else if ('A'..='Z').contains(&c) {
        (c as u8 - capital_a + 27) as u64
    } else {
        panic!("Invalid character: {}", c);
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input_generator_part1(input); // cargo aoc doesn't want this to work, for some reason.
                                              // Therefore it is done manually

    let chars: Vec<Vec<char>> = input
        .iter()
        .map(|(a, b)| {
            let set: HashSet<char> = a.chars().collect();
            let mut common_chars: Vec<char> = Vec::new();

            b.chars().for_each(|c| {
                if set.contains(&c) {
                    common_chars.push(c);
                }
            });

            common_chars
        })
        .collect();

    chars.iter().fold(0, |acc, x| {
        let x: Vec<_> = x.iter().unique().collect();
        acc + x
            .iter()
            .fold(0, |acc, x| acc + char_to_alphabet_number(**x))
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let mut commons: Vec<Vec<char>> = Vec::new();
    let lines = &input.lines().count();
    let input: Vec<&str> = input.lines().collect();

    for i in 0..*lines {
        if i % 3 == 0 {
            commons.push(common_chars(input[i], input[i + 1], input[i + 2]));
        }
    }

    commons.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |acc, y| acc + char_to_alphabet_number(*y))
    })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1() {
        let input = include_str!("../input/2022/test/day3.txt");
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn test2() {
        let input = include_str!("../input/2022/test/day3.txt");
        assert_eq!(part2(input), 70);
    }
}
