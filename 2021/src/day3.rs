#![allow(unused_variables, dead_code)]

#[derive(Clone, Debug)]
pub struct Content{
    pub vals: Vec<u32>,
    pub bits: u32
}
const MAX_BITS: u32 = 5;

impl Content {
    fn common_for_bit_le(&self, bit: u32) -> u32 {
        let count: u32 = self.vals.iter().map(|n| (n >> bit) & 1).sum();
        if count as usize * 2 >= self.vals.len() {1} else {0}
    }
    fn common_for_bit(&self, bit: u32) -> u32 {
        self.common_for_bit_le(self.inv_bit(bit))
    }
    fn uncommon_for_bit_le(&self, bit: u32) -> u32 {
        let count: u32 = self.vals.iter().map(|n| (n >> bit) & 1).sum();
        if count  as usize * 2 < self.vals.len() {1} else {0}
    }
    fn uncommon_for_bit(&self, bit: u32) -> u32 {
        self.uncommon_for_bit_le(self.inv_bit(bit))
    }

    fn discard_uncommon_for_bit(&mut self, bit: u32) {
        let bit = self.inv_bit(bit);
        let bit_mask = 1 << bit;
        let bit_val = self.common_for_bit_le(bit) << bit;

        self.vals = self.vals.iter().filter(|v| (*v &  bit_mask) == bit_val).map(|v| *v).collect();
    }
    fn discard_common_for_bit(&mut self, bit: u32) {
        let bit = self.inv_bit(bit);
        let bit_mask = 1 << bit;
        let bit_val = self.uncommon_for_bit_le(bit) << bit;

        self.vals = self.vals.iter().filter(|v| (*v &  bit_mask) == bit_val).map(|v| *v).collect();
    }

    fn inv_bit(&self, bit: u32) -> u32 {
        self.bits - 1 - bit
    }
}

fn gamma_rate(v: &Content) -> u32 {
    let mut out = 0;
    for bit in 0..v.bits {
        let val = v.common_for_bit_le(bit);
        out |= val << bit;
    }

    out
}

fn eps_rate(v: &Content) -> u32 {
    let mut out = 0;
    for bit in 0..v.bits {
        let val = v.uncommon_for_bit_le(bit);
        out |= val << bit;
    }

    out
}

fn oxy_rate(mut v: Content) -> u32 {
    for b in 0..v.bits {
        v.discard_uncommon_for_bit(b);
        if v.vals.len() == 1 {
            return v.vals[0];
        }
    }
    panic!("Not found!")
}
fn co2_rate(mut v: Content) -> u32 {
    for b in 0..v.bits {
        v.discard_common_for_bit(b);
        if v.vals.len() == 1 {
            return v.vals[0];
        }
    }
    panic!("Not found!")
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Content {
    let vals = input.lines().map(|s| (u32::from_str_radix(s, 2).unwrap())).collect();
    let bits = input.lines().next().unwrap().len();

    Content {
        vals,
        bits: bits as u32
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Content) -> usize {
    (eps_rate(input) * gamma_rate(input)) as usize
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Content) -> usize {
    (oxy_rate(input.clone()) * co2_rate(input.clone())) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    const INPUT: &str = include_str!("../input/2021/day3.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 198);
    }
    #[test]
    fn eg1_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 230);
    }
    #[test]
    fn oxy() {
        let content = input_generator(EG_INPUT);
        assert_eq!(oxy_rate(content), 23);
    }
    #[test]
    fn co2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(co2_rate(content), 10);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 2954600);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
