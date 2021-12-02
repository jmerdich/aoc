#![allow(unused_variables, dead_code)]

pub enum Dir {
    Forward,
    Up,
    Down
}

impl Dir {
    fn from_str(s: &str) -> Option<Dir> {
        match s {
            "forward" => Some(Dir::Forward),
            "up" => Some(Dir::Up),
            "down" => Some(Dir::Down),
            _ => None
        }
    }
}

pub struct Content {
    dir: Dir,
    count: u32
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Content> {
    input.lines().map(|l| {
        let strs: Vec<&str> = l.split(" ").collect();
        Content {
            dir: Dir::from_str(strs[0]).unwrap(),
            count: strs[1].parse().unwrap()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Content]) -> u32 {
    let mut hor = 0;
    let mut vert = 0;

    for mov in input {
        match mov.dir {
            Dir::Forward => {
                hor += mov.count;
            },
            Dir::Up => {
                vert -= mov.count;
            },
            Dir::Down => {
                vert += mov.count;
            },
        }
    }
    hor * vert
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Content]) -> u32 {
    let mut aim: i32 = 0;
    let mut hor: i32 = 0;
    let mut vert: i32 = 0;

    for mov in input {
        let count = mov.count as i32;
        match mov.dir {
            Dir::Forward => {
                hor += count;
                vert += count * aim;
            },
            Dir::Up => {
                aim -= count;
            },
            Dir::Down => {
                aim += count;
            },
        }
    }
    (hor * vert) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
    const INPUT: &str = include_str!("../input/2021/day2.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 150);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 900);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 1698735);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 1594785890);
    }
}
