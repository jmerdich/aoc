#![allow(unused_variables, dead_code)]

use itertools::Itertools;
use std::convert::TryInto;

const NUM_TILES: usize = 25;

#[derive(Clone)]
pub struct Board {
    // All in row-major order
    tiles: [u32; NUM_TILES],
    marked: [bool; NUM_TILES],
}

impl Board {
    fn from_tiles(tiles: &[u32]) -> Self {
        Self {
            tiles: tiles.try_into().unwrap(),
            marked: [false; NUM_TILES],
        }
    }

    fn mark_off(&mut self, num: u32) {
        if let Some(idx) = self.tiles.iter().position(|t| *t == num) {
            self.marked[idx] = true;
        }
    }

    fn unmarked_tiles(&self) -> impl Iterator<Item = u32> + '_ {
        self.tiles
            .iter()
            .zip(self.marked)
            .filter(|(t, m)| !m)
            .map(|(t, m)| *t)
    }

    fn winner(&self) -> bool {
        for row_i in 0..5 {
            if self.marked[row_i * 5..row_i * 5 + 5].iter().all(|p| *p) {
                return true;
            }
        }
        for col_i in 0..5 {
            if (0..5).map(|i| self.marked[i * 5 + col_i]).all(|p| p) {
                return true;
            }
        }
        false
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str("Board {\n")?;
        for group in &self.tiles.iter().zip(self.marked).chunks(5) {
            fmt.debug_list().entries(group).finish()?;
            fmt.write_str(",\n")?;
        }
        fmt.write_str("}")
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    called: Vec<u32>,
    boards: Vec<Board>,
    next_turn: usize,
}

impl Content {
    pub fn next_turn(&mut self) {
        let called_num = self.called[self.next_turn];
        for board in &mut self.boards {
            board.mark_off(called_num);
        }
        self.next_turn += 1;
    }

    pub fn find_winner(&self) -> Option<&Board> {
        self.boards.iter().find(|b| b.winner())
    }

    pub fn last_called(&self) -> u32 {
        assert!(self.next_turn != 0);
        self.called[self.next_turn - 1]
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Content {
    let mut lines = input.lines();
    let called: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut cur_board_nums: Vec<u32> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            if !cur_board_nums.is_empty() {
                boards.push(Board::from_tiles(&cur_board_nums));
                cur_board_nums.clear();
            }
            continue;
        }
        cur_board_nums.extend(
            line.split_ascii_whitespace()
                .map(|v| v.parse::<u32>().unwrap()),
        );
    }
    if !cur_board_nums.is_empty() {
        boards.push(Board::from_tiles(&cur_board_nums));
        cur_board_nums.clear();
    }
    Content {
        boards,
        called,
        next_turn: 0,
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut game = input.clone();
    loop {
        game.next_turn();
        if let Some(winner) = game.find_winner() {
            return (game.last_called() as usize) * (winner.unmarked_tiles().sum::<u32>() as usize);
        }
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut game = input.clone();
    loop {
        game.next_turn();
        if game.boards.len() == 1 {
            // One left, wait for it to win
            if let Some(winner) = game.find_winner() {
                return (game.last_called() as usize)
                    * (winner.unmarked_tiles().sum::<u32>() as usize);
            }
        } else {
            // keep only losing boards
            game.boards.retain(|b| !b.winner());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    const INPUT: &str = include_str!("../input/2021/day4.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 4512);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 1924);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 65325);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 4624);
    }
}
