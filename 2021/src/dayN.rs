#![allow(unused_variables, dead_code)]

pub struct Content {
}

#[aoc_generator(dayN)]
pub fn input_generator(input: &str) -> Content {
    Vec::new()
}

#[aoc(dayN, part1)]
pub fn solve_part1(input: &Content) -> usize {
    0
}

#[aoc(dayN, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
deadbeef
deadbeef";
    const INPUT: &str = include_str!("../input/2021/dayN.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 0);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 0);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
