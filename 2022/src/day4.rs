#![allow(unused_variables, dead_code)]

use scan_fmt::scan_fmt;

type Range = std::ops::Range<u32>;
type Content = Vec<(Range, Range)>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Content {
    let mut out = Vec::new();
    for line in input.lines() {
        let (r1a, r1b, r2a, r2b) = scan_fmt!(line, "{}-{},{}-{}", u32, u32, u32, u32).unwrap();
        out.push((Range{start: r1a, end: r1b+1}, Range{start: r2a, end: r2b+1}))
    }
    out
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Content) -> usize {
    input.iter().filter(|(a, b)| {
        (a.contains(&b.start) && a.contains(&(b.end - 1))) ||
        (b.contains(&a.start) && b.contains(&(a.end - 1)))
    }).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Content) -> usize {
    input.iter().filter(|(a, b)| {
        (a.contains(&b.start) || a.contains(&(b.end - 1))) ||
        (b.contains(&a.start) || b.contains(&(a.end - 1)))
    }).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    const INPUT: &str = include_str!("../input/2022/day4.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 2);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 4);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 513);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 878);
    }
}
