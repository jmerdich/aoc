#![allow(unused_variables, dead_code)]

use regex::Regex;

lazy_static! {
    static ref WRITE_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct Mask {
    value: u64,
    mask: u64, // derives from 'X's
}

impl Mask {
    pub fn new() -> Self {
        Mask { value: 0, mask: !0 }
    }
    pub fn apply(self, int: u64) -> u64 {
        (int & self.mask) | self.value
    }

    pub fn from_str(s: &str) -> Self {
        let mut mask: Mask = Mask { value: 0, mask: 0 };
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => {}
                '1' => {
                    mask.value |= 1 << i;
                }
                'X' => {
                    mask.mask |= 1 << i;
                }
                _ => {
                    panic!();
                }
            }
        }
        mask
    }
}

pub enum Op {
    Mask(Mask),
    Write { val: u64, dst: u64 },
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|l| {
            if let Some(stripped) = l.strip_prefix("mask = ") {
                Op::Mask(Mask::from_str(stripped))
            } else {
                let caps = WRITE_RE.captures(l).unwrap();
                Op::Write {
                    val: caps.get(2).unwrap().as_str().parse().unwrap(),
                    dst: caps.get(1).unwrap().as_str().parse().unwrap(),
                }
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Op]) -> u64 {
    let arr_size: usize = input
        .iter()
        .filter_map(|o| {
            if let Op::Write { val: _, dst } = o {
                Some(*dst as usize)
            } else {
                None
            }
        })
        .max()
        .unwrap()
        + 1;

    let mut mem: Vec<u64> = vec![0; arr_size];
    let mut cur_mask: Mask = Mask::new();

    for op in input {
        match op {
            Op::Mask(m) => {
                cur_mask = *m;
            }
            Op::Write { val, dst } => {
                mem[*dst as usize] = cur_mask.apply(*val);
            }
        }
    }

    mem.iter().copied().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Op]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    const INPUT: &str = include_str!("../input/2020/day14.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 165);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 11179633149677);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
