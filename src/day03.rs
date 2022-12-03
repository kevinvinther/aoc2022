use std::collections::HashSet;
use itertools::{Itertools};

pub fn input_generator(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| { 
            let (first, second) = line.split_at(line.len() / 2);
            (first, second)
        })
        .collect()
}


fn char_to_alphabet_number(c: char) -> u64 {
    let a = b'a';
    let capital_a = b'A';

    if c >= 'a' && c <= 'z' {
        (c as u8 - a + 1) as u64
    } else if c >= 'A' && c <= 'Z' {
        (c as u8 - capital_a + 27) as u64
    } else {
        panic!("Invalid character: {}", c);
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input_generator(input); // cargo aoc doesn't want this to work, for some reason.
                                        // Therefore it is done manually

    let chars: Vec<Vec<char>> = input.iter()
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


    chars.iter().fold(0,|acc, x| {
        let x: Vec<_> = x.iter().unique().collect();
        acc + x.iter().fold(0, |acc, x| acc + char_to_alphabet_number(**x))
    })
}

// #[aoc(day3, part2)]
// pub fn part2(input: &Vec<(&str, &str)>) -> u64 {
//     1
// }

#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn test1() {
        let input = include_str!("../input/2022/test/day3.txt");
        assert_eq!(part1(input), 157);
    }

    // #[test]
    // fn test2() {
    //     let input = include_str!("../input/2022/test/day3.txt");
    //     assert_eq!(part2(input), 157);
    // }
}
