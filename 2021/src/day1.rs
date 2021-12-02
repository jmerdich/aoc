#![allow(unused_variables, dead_code)]

type Content = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Content> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Content]) -> usize {
    input.windows(2).filter(|w| (w[0] < w[1])).count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Content]) -> usize {
    let sums: Vec<i32> = input.windows(3).map(|w| w.iter().sum()).collect();
    sums.windows(2).filter(|w| (w[0] < w[1])).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 7);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 5);
    }
    const INPUT: &str = include_str!("../input/2021/day1.txt");
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 1722);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 1748);
    }
}
