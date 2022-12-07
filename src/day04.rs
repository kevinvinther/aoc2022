#[aoc_generator(day4)]
pub fn parser(input: &str) -> Vec<[u64; 4]> {
    let new_input = input.lines().map(|x| {
        let range: Vec<&str> = x.split(',').collect();
        let range: Vec<Vec<&str>> = range.iter().map(|x| x.split('-').collect()).collect();

        [range[0][0].parse().unwrap(), range[0][1].parse().unwrap(), range[1][0].parse().unwrap(), range[1][1].parse().unwrap()]
    }).collect();
    new_input
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<[u64; 4]>) -> usize {
    input.iter().filter(|[l1, l2, r1, r2]| (l1 >= r1 && l2 <= r2) || (r1 >= l1 && r2 <= l2)).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<[u64; 4]>) -> usize {
    input.iter().filter(|[l1, l2, r1, r2]| (l1 <= r1 && l2 >= r1) || (r1 <= l1 && r2 >= l1)).count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, parser};

    #[test]
    fn parser_test() {
        let input = "1-3,5-7\n1-2,5-8";
        assert_eq!(parser(input), [[1, 3, 5, 7], [1, 2, 5, 8]]);
    }

    #[test]
    fn test1() {
        let input = include_str!("../input/2022/test/day4.txt");
        assert_eq!(part1(&parser(input)), 2);
    }

    #[test]
    fn test2() {
        let input = include_str!("../input/2022/test/day4.txt");
        assert_eq!(part2(&parser(input)), 4);
    }
}
