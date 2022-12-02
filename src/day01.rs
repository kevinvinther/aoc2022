use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n') // split the lines
                .map(|num| num.parse::<u64>().unwrap_or(0)) // for each num, parse it
                .sum() // sum the numbers
        })
        .sorted()
        .rev()
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    input[0]
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    input[0] + input[1] + input[2]
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator(include_str!("../input/2022/test/day1.txt"))), 24000);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator(include_str!("../input/2022/test/day1.txt"))), 45000)
    }
}
