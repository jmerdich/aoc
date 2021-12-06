#![allow(unused_variables, dead_code)]

pub struct Content {
    pub ages: Vec<u32>
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Content {
    let ages: Vec<u32> = input.split(",").map(|n| n.trim().parse().unwrap()).collect();
    Content { ages }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Content) -> usize {
    0
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
3,4,3,1,2
";
    const INPUT: &str = include_str!("../input/2021/day6.txt");

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
