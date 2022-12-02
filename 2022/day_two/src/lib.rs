enum Outcome {
    Win,
    Draw,
    Loss
}

impl Outcome {
    fn new(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid outcome symbol: {input}")
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0
        }
    }
}

#[derive(Copy,Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn new(symbol: &str) -> Self {
        match symbol {
            "A" | "X" => { Self::Rock },
            "B" | "Y" => { Self::Paper },
            "C" | "Z" => { Self::Scissors },
            _ => panic!("Unknown move symbol: {symbol}")
        }
    }

    fn base_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

struct Round {
    opponent: Move,
    you: Move
}

impl Round {
    fn new_round_one(input: &str) -> Self {
        let moves: Vec<Move> = input.split(" ").map(Move::new).collect();
        if moves.len() != 2 {
            panic!("Invalid input for round: {input}")
        }

        Self { opponent: moves[0], you: moves[1] }
    }

    fn new_round_two(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        if parts.len() != 2 {
            panic!("Invalid input for round: {input}")
        }

        let opponent = Move::new(parts[0]);
        let outcome = Outcome::new(parts[1]);
        let you = match (opponent, outcome) {
            (Move::Rock, Outcome::Draw) | (Move::Paper, Outcome::Loss) | (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Paper, Outcome::Draw) | (Move::Scissors, Outcome::Loss) | (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Scissors, Outcome::Draw) | (Move::Rock, Outcome::Loss) | (Move::Paper, Outcome::Win) => Move::Scissors
        };

        Self { opponent, you }
    }

    fn score(&self) -> u32 {
        self.base_score() + self.outcome_score()
    }

    fn base_score(&self) -> u32 {
        self.you.base_score()
    }

    fn outcome_score(&self) -> u32 {
        self.outcome().score()
    }

    fn outcome(&self) -> Outcome {
        match (self.you, self.opponent) {
            (Move::Rock, Move::Scissors) | (Move::Scissors, Move::Paper) | (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors) => Outcome::Draw,
            (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) => Outcome::Loss
        }
    }
}

struct Tournament {
    rounds: Vec<Round>
}

impl Tournament {
    fn new(input: &str, round_parser: fn(&str) -> Round) -> Self {
        Self { rounds: input.lines().map(round_parser).collect() }
    }

    fn score(&self) -> u32 {
        self.rounds.iter().map(Round::score).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(
            Tournament::new(
                fs::read_to_string("example_input.txt").unwrap().as_str(),
                Round::new_round_one
            ).score(),
            15
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            Tournament::new(
                fs::read_to_string("input.txt").unwrap().as_str(),
                Round::new_round_one
            ).score(),
            13268
        );
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(
            Tournament::new(
                fs::read_to_string("example_input.txt").unwrap().as_str(),
                Round::new_round_two
            ).score(),
            12
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            Tournament::new(
                fs::read_to_string("input.txt").unwrap().as_str(),
                Round::new_round_two
            ).score(),
            15508
        );
    }
}
