use itertools::Itertools;

fn part1(input: &str) {
    let input: Vec<u64> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n') // split the lines
                .map(|num| num.parse::<u64>().unwrap_or(0)) // for each num, parse it
                .sum() // sum the numbers
        })
        .sorted()
        .rev()
        .collect();

    println!("Part 1: {:?}", &input[0]);
    println!("Part 2: {:?}", &input[0] + &input[1] + &input[2]);
}

fn main() {
    part1(include_str!("../input/day01.txt"));
}
