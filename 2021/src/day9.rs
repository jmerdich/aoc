#![allow(unused_variables, dead_code)]

use grid::Grid;

pub struct Content {
    grid: Grid<u8>,
}

impl Content {
    fn adjacent(&self, x: usize, y: usize) -> Vec<u8> {
        let mut adjacent = Vec::new();

        if y > 0 {
            if let Some(v) = self.grid.get(y - 1, x) {
                adjacent.push(*v);
            }
        }
        if let Some(v) = self.grid.get(y + 1, x) {
            adjacent.push(*v);
        }
        if x > 0 {
            if let Some(v) = self.grid.get(y, x - 1) {
                adjacent.push(*v);
            }
        }
        if let Some(v) = self.grid.get(y, x + 1) {
            adjacent.push(*v);
        }

        adjacent
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Content {
    let values: Vec<u8> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .flatten()
        .collect();
    let cols = input.lines().next().unwrap().len();
    Content {
        grid: Grid::from_vec(values, cols),
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut risk: usize = 0;
    for x in 0..input.grid.cols() {
        for y in 0..input.grid.rows() {
            let v = *input.grid.get(y, x).unwrap();
            if v < *input.adjacent(x, y).iter().min().unwrap() {
                risk += v as usize + 1;
            }
        }
    }
    risk
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";
    const INPUT: &str = include_str!("../input/2021/day9.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 15);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 516);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
