#![allow(unused_variables, dead_code)]

use itertools::Itertools;
use itertools::enumerate;
use std::convert::TryInto;


// These use the standard "around then middle" scheme, not AoC's odd system
const DIGIT_SEGS: [u8; 10] = [
    0x3F,
    0x06,
    0x5B,
    0x4F,
    0x66,
    0x6D,
    0x7D,
    0x07,
    0x7F,
    0x6F,
];

struct Readout {
    digits: Vec<String>,
    values: Vec<String>
}

pub struct Content {
    readouts : Vec<Readout>
}

fn generate_mapping(digits: &[&str]) -> [char; 7] {
    for seq in ('a'..='g').combinations(7) {
        let mut found = true;
        for testval in digits {
            let mut exact_match = false;
            for possible_seg in DIGIT_SEGS {
                exact_match = enumerate(&seq).all(|(i,c)| testval.contains(*c) == (possible_seg & (1<<i) != 0));
            }
            if !exact_match {
                found = false;
                break;
            }
        }
        if found {
            return seq.try_into().unwrap();
        }
    }
    todo!();
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Content {
    let readouts = input.lines().map(|l| {
        let (digits, values) = l.split("|").collect_tuple().unwrap();
        let digits = digits.split_ascii_whitespace().map(|s| s.to_string());
        let values = values.split_ascii_whitespace().map(|s| s.to_string());

        Readout {
            digits: digits.collect(),
            values: values.collect()
        }
    }).collect();
    Content { readouts }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let unique_nums = vec![2, 3, 4, 7];
    input.readouts.iter().map(|r| r.values.iter().filter(|d| unique_nums.contains(&d.len())).count()).sum()

}
#[aoc(day8, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    const INPUT: &str = include_str!("../input/2021/day8.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 26);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 470);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
