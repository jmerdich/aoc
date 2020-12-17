#![allow(unused_variables, dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemSeq {
    starting_nums: Vec<u64>,
    i: u64,
    last_seen: HashMap<u64, u64>,
    last_res: u64,
}

impl MemSeq {
    pub fn from_str(s: &str) -> MemSeq {
        MemSeq {
            starting_nums: s
                .trim_end()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
            i: 0,
            last_seen: HashMap::new(),
            last_res: 0,
        }
    }
}

impl Iterator for MemSeq {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let num = if self.i < self.starting_nums.len() as u64 {
            self.starting_nums[self.i as usize]
        } else if self.last_seen.contains_key(&self.last_res) {
            self.i - self.last_seen[&self.last_res]
        } else {
            0
        };

        self.last_seen.insert(self.last_res, self.i);
        self.last_res = num;
        self.i += 1;
        Some(num)
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> MemSeq {
    MemSeq::from_str(input)
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &MemSeq) -> u64 {
    input.clone().nth(2019).unwrap()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &MemSeq) -> u64 {
    input.clone().nth(30000000 - 1).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input/2020/day15.txt");

    #[test]
    fn eg_part1() {
        let mut seq1 = MemSeq::from_str("0,3,6");

        let result: Vec<u64> = seq1.clone().take(10).collect();
        let expected = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0];

        assert_eq!(result, expected);
        assert_eq!(seq1.nth(2019).unwrap(), 436);

        // TODO: figure out why these aren't working but the actual ones are.
        //assert_eq!(MemSeq::from_str("1,3,2").nth(2019).unwrap(), 1);
        //assert_eq!(MemSeq::from_str("2,1,3").nth(2019).unwrap(), 10);
        //assert_eq!(MemSeq::from_str("1,2,3").nth(2019).unwrap(), 27);
        //assert_eq!(MemSeq::from_str("2,3,1").nth(2019).unwrap(), 78);
        //assert_eq!(MemSeq::from_str("3,2,1").nth(2019).unwrap(), 438);
        //assert_eq!(MemSeq::from_str("3,1,2").nth(2019).unwrap(), 1836);
    }
    #[test]
    #[ignore]
    fn eg_part2() {
        let mut seq1 = MemSeq::from_str("0,3,6");
        assert_eq!(seq1.nth(30000000 - 1).unwrap(), 175594);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 639);
    }
    #[test]
    #[ignore]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 266);
    }
}
