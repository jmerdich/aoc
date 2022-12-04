#![allow(unused_variables, dead_code)]

use itertools::Itertools;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Unknown char '{}'", c),
    }
}

type Sack = (String, String);
type Content = Vec<Sack>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Content {
    let mut out: Content = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let sz = line.len();
        assert!(sz != 0);

        out.push((line[0..sz / 2].to_string(), line[sz / 2..].to_string()))
    }
    out
}

fn common_items(sack: &Sack) -> String {
    let mut out = String::new();
    for c in sack.0.chars() {
        if sack.1.contains(c) && !out.contains(c) {
            out.push(c);
        }
    }

    out
}
fn common_items2(a: &str, b: &str) -> String {
    let mut out = String::new();
    for c in a.chars() {
        if b.contains(c) && !out.contains(c) {
            out.push(c);
        }
    }
    out
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Content) -> usize {
    input
        .iter()
        .map(common_items)
        .map(|com| com.chars().map(priority).sum::<u32>() as usize)
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut out_sum = 0;
    for group in input.clone().chunks(3) {
        assert!(group.len() == 3);
        let cur_item = &group[0];
        let mut common_items = cur_item.0.clone() + cur_item.1.as_str();

        for i in 1..3 {
            let cur_item = &group[i];
            let cur_item_s = cur_item.0.clone() + cur_item.1.as_str();

            common_items = common_items2(&common_items, &cur_item_s);
        }
        assert!(common_items.len() == 1);
        out_sum += priority(common_items.chars().collect_vec()[0]);
    }
    out_sum as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    const INPUT: &str = include_str!("../input/2022/day3.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 157);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 70);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 8109);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 2738);
    }
}
