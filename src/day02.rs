enum OpponentMove {
    Rock,
    Paper,
    Scissors,
}

enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let input: Vec<(OpponentMove, PlayerMove)> = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.trim().split(' ').collect();
            let opponent = match moves[0] {
                "A" => OpponentMove::Rock,
                "B" => OpponentMove::Paper,
                "C" => OpponentMove::Scissors,
                _ => unreachable!("Invalid opponent move"),
            };
            let player = match moves[1] {
                "X" => PlayerMove::Rock,
                "Y" => PlayerMove::Paper,
                "Z" => PlayerMove::Scissors,
                _ => unreachable!("Invalid player move"),
            };
            (opponent, player)
        })
        .collect();

    let scores: Vec<u64> = input
        .iter()
        .map(|(opp, play)| {
            let result: GameResult = match (opp, play) {
                (OpponentMove::Rock, PlayerMove::Rock) => GameResult::Draw,
                (OpponentMove::Rock, PlayerMove::Paper) => GameResult::Win,
                (OpponentMove::Rock, PlayerMove::Scissors) => GameResult::Lose,
                (OpponentMove::Paper, PlayerMove::Rock) => GameResult::Lose,
                (OpponentMove::Paper, PlayerMove::Paper) => GameResult::Draw,
                (OpponentMove::Paper, PlayerMove::Scissors) => GameResult::Win,
                (OpponentMove::Scissors, PlayerMove::Rock) => GameResult::Win,
                (OpponentMove::Scissors, PlayerMove::Paper) => GameResult::Lose,
                (OpponentMove::Scissors, PlayerMove::Scissors) => GameResult::Draw,
            };
            let match_point = match result {
                GameResult::Win => 6,
                GameResult::Draw => 3,
                GameResult::Lose => 0,
            };
            let game_point: u64 = match play {
                PlayerMove::Rock => 1,
                PlayerMove::Paper => 2,
                PlayerMove::Scissors => 3,
            };
            match_point + game_point
        })
        .collect();

    let sum = scores.iter().sum::<u64>();
    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let input: Vec<(OpponentMove, GameResult)> = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.trim().split(' ').collect();
            let opponent = match moves[0] {
                "A" => OpponentMove::Rock,
                "B" => OpponentMove::Paper,
                "C" => OpponentMove::Scissors,
                _ => unreachable!("Invalid opponent move"),
            };
            let result = match moves[1] {
                "X" => GameResult::Lose,
                "Y" => GameResult::Draw,
                "Z" => GameResult::Win,
                _ => unreachable!("Invalid player move"),
            };
            (opponent, result)
        })
        .collect();

    let game: Vec<u64> = input
        .iter()
        .map(|(opp, result)| {
            let newmove = match (opp, result) {
                (OpponentMove::Rock, GameResult::Win) => PlayerMove::Paper,
                (OpponentMove::Rock, GameResult::Draw) => PlayerMove::Rock,
                (OpponentMove::Rock, GameResult::Lose) => PlayerMove::Scissors,
                (OpponentMove::Paper, GameResult::Win) => PlayerMove::Scissors,
                (OpponentMove::Paper, GameResult::Draw) => PlayerMove::Paper,
                (OpponentMove::Paper, GameResult::Lose) => PlayerMove::Rock,
                (OpponentMove::Scissors, GameResult::Win) => PlayerMove::Rock,
                (OpponentMove::Scissors, GameResult::Draw) => PlayerMove::Scissors,
                (OpponentMove::Scissors, GameResult::Lose) => PlayerMove::Paper,
            };

            let match_point = match result {
                GameResult::Win => 6,
                GameResult::Draw => 3,
                GameResult::Lose => 0,
            };
            let game_point: u64 = match newmove {
                PlayerMove::Rock => 1,
                PlayerMove::Paper => 2,
                PlayerMove::Scissors => 3,
            };

            match_point + game_point
        })
        .collect();

    game.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1() {
        assert_eq!(part1(include_str!("../input/2022/test/day2.txt")), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(include_str!("../input/2022/test/day2.txt")), 12);
    }
}
