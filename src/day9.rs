#![allow(unused_variables, dead_code)]
use itertools::Itertools;
use std::cmp::Ordering;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn preamble_valid(v: &[i64], i: usize, window: usize) -> bool {
    if i < window {
        return true;
    }
    let check_val = v[i];
    let considered_prev = &v[(i - window)..i];

    for (a, b) in considered_prev.iter().tuple_combinations() {
        if a + b == check_val {
            return true;
        }
    }

    false
}

pub fn find_first_invalid(v: &[i64], window: usize) -> i64 {
    let first_inv = (0..v.len()).find(|i| !preamble_valid(v, *i, window));

    v[first_inv.unwrap()]
}

pub fn test_contiguous(v: &[i64], start_idx: usize, needle: i64) -> Option<(usize, usize)> {
    let mut size = 2;
    while v.len() > start_idx + size {
        let range_sum: i64 = v[start_idx..(start_idx + size)].iter().sum();
        match range_sum.cmp(&needle) {
            Ordering::Equal => {
                return Some((start_idx, start_idx + size));
            }
            Ordering::Greater => {
                return None;
            }
            _ => {}
        }
        size += 1;
    }
    None
}

pub fn find_contiguous_sum(v: &[i64], needle: i64) -> i64 {
    let range = (0..v.len()).find_map(|i| test_contiguous(v, i, needle));

    let range = &v[range.unwrap().0..range.unwrap().1];

    range.iter().max().unwrap() + range.iter().min().unwrap()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    find_first_invalid(input, 25) as usize
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> usize {
    let needle = find_first_invalid(input, 25);
    find_contiguous_sum(input, needle) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    const INPUT: &str = include_str!("../input/2020/day9.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(find_first_invalid(&content, 5), 127);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(find_contiguous_sum(&content, 127), 62);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 15690279);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 2174232);
    }
}
