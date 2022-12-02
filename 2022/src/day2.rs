#![allow(unused_variables, dead_code)]

use scan_fmt::scan_fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            _ => panic!("Unknown RPS type"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameRes {
    Loss,
    Draw,
    Win,
}
impl GameRes {
    fn score(&self) -> u32 {
        match self {
            GameRes::Loss => 0,
            GameRes::Draw => 3,
            GameRes::Win => 6,
        }
    }

    fn from_throws(us: RPS, them: RPS) -> GameRes {
        if us == them {
            return GameRes::Draw;
        }
        if (us == RPS::Rock && them == RPS::Scissors)
            || (us == RPS::Scissors && them == RPS::Paper)
            || (us == RPS::Paper && them == RPS::Rock)
        {
            return GameRes::Win;
        } else {
            return GameRes::Loss;
        }
    }

    fn from_bad_match(m: &Match) -> GameRes {
        GameRes::from_throws(m.des_res.make_bad_assumption(), m.them)
    }
    fn from_char(c: char) -> Self {
        match c {
            'X' => GameRes::Loss,
            'Y' => GameRes::Draw,
            'Z' => GameRes::Win,
            _ => panic!("Unknown GameRes type"),
        }
    }

    fn make_bad_assumption(self) -> RPS {
        match self {
            GameRes::Loss => RPS::Rock,
            GameRes::Draw => RPS::Paper,
            GameRes::Win => RPS::Scissors,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Match {
    des_res: GameRes,
    them: RPS,
}

impl Match {
    fn bad_score(&self) -> u32 {
        self.des_res.make_bad_assumption().score() + GameRes::from_bad_match(self).score()
    }
    fn score(&self) -> u32 {
        self.des_res.score() + self.pick_throw().score()
    }

    fn from_pair(pair: (char, char)) -> Match {
        Match {
            des_res: GameRes::from_char(pair.1),
            them: RPS::from_char(pair.0),
        }
    }

    fn pick_throw(self) -> RPS {
        match (self.des_res, self.them) {
            (GameRes::Loss, RPS::Rock) => RPS::Scissors,
            (GameRes::Loss, RPS::Paper) => RPS::Rock,
            (GameRes::Loss, RPS::Scissors) => RPS::Paper,
            (GameRes::Win, RPS::Rock) => RPS::Paper,
            (GameRes::Win, RPS::Paper) => RPS::Scissors,
            (GameRes::Win, RPS::Scissors) => RPS::Rock,
            (GameRes::Draw, _) => self.them,
        }
    }
}

type Content = Vec<Match>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Content {
    input
        .lines()
        .map(|l| Match::from_pair(scan_fmt!(l, "{} {}", char, char).unwrap()))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Content) -> usize {
    input.iter().map(|m| m.bad_score() as u64).sum::<u64>() as usize
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Content) -> usize {
    input.iter().map(|m| m.score() as u64).sum::<u64>() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
A Y
B X
C Z
";
    const INPUT: &str = include_str!("../input/2022/day2.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 15);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 12);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 12276);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 9975);
    }
}
