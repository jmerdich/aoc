#![allow(unused_variables, dead_code)]

type BusId = u32;

pub struct Sched {
    depart_time: u64,
    buses: Vec<Option<BusId>>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Sched {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() == 2);
    let buses: Vec<Option<BusId>> = lines[1]
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    Sched {
        depart_time: lines[0].parse().unwrap(),
        buses,
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Sched) -> usize {
    0
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Sched) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
939
7,13,x,x,59,x,31,19";
    const INPUT: &str = include_str!("../input/2020/day13.txt");

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
