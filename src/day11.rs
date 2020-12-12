#![allow(unused_variables, dead_code)]

use array2d::Array2D;
use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Floor,
    Empty,
    Occ,
}

impl Space {
    fn from_char(c: char) -> Option<Space> {
        match c {
            '.' => Some(Space::Floor),
            'L' => Some(Space::Empty),
            '#' => Some(Space::Occ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    arr: Array2D<Space>,
    changed: bool,
}

impl Content {
    fn surrounding_occ(&self, x: usize, y: usize) -> u8 {
        let mut sum = 0;
        for x_i in max(x as isize - 1, 0)..(min(self.arr.row_len(), x + 2) as isize) {
            for y_i in max(y as isize - 1, 0)..(min(self.arr.column_len(), y + 2) as isize) {
                let x_i = x_i as usize;
                let y_i = y_i as usize;
                if ((x_i, y_i) != (x, y)) && self.arr[(y_i, x_i)] == Space::Occ {
                    sum += 1;
                }
            }
        }
        sum
    }
    fn surrounding_occ_pt2(&self, x: usize, y: usize) -> u8 {
        let it = (-1isize..=1).cartesian_product(-1isize..=1);
        let mut occ = 0;
        for (dir_x, dir_y) in it {
            if (dir_x, dir_y) == (0, 0) {
                continue;
            }

            let (mut x_i, mut y_i): (isize, isize) = (x as isize + dir_x, y as isize + dir_y);

            while (0isize..(self.arr.row_len() as isize)).contains(&x_i)
                && (0isize..(self.arr.column_len() as isize)).contains(&y_i)
            {
                match self.arr[(y_i as usize, x_i as usize)] {
                    Space::Occ => {
                        occ += 1;
                        break;
                    }
                    Space::Empty => {
                        break;
                    }
                    _ => {}
                }
                x_i += dir_x;
                y_i += dir_y;
            }
        }
        occ
    }

    fn calc_tile_step(&self, x: usize, y: usize) -> Space {
        match self.arr[(y, x)] {
            Space::Empty => {
                if self.surrounding_occ(x, y) == 0 {
                    Space::Occ
                } else {
                    Space::Empty
                }
            }
            Space::Occ => {
                if self.surrounding_occ(x, y) >= 4 {
                    Space::Empty
                } else {
                    Space::Occ
                }
            }
            Space::Floor => Space::Floor,
        }
    }
    fn calc_tile_step_pt2(&self, x: usize, y: usize) -> Space {
        match self.arr[(y, x)] {
            Space::Empty => {
                if self.surrounding_occ_pt2(x, y) == 0 {
                    Space::Occ
                } else {
                    Space::Empty
                }
            }
            Space::Occ => {
                if self.surrounding_occ_pt2(x, y) >= 5 {
                    Space::Empty
                } else {
                    Space::Occ
                }
            }
            Space::Floor => Space::Floor,
        }
    }
    fn changed(&self) -> bool {
        self.changed
    }

    fn step(&mut self) {
        let mut new_arr = self.arr.clone();

        for x in 0..self.arr.row_len() {
            for y in 0..self.arr.column_len() {
                new_arr[(y, x)] = self.calc_tile_step(x, y);
            }
        }

        self.changed = self.arr != new_arr;
        self.arr = new_arr;
    }
    fn step_pt2(&mut self) {
        let mut new_arr = self.arr.clone();

        for x in 0..self.arr.row_len() {
            for y in 0..self.arr.column_len() {
                new_arr[(y, x)] = self.calc_tile_step_pt2(x, y);
            }
        }

        self.changed = self.arr != new_arr;
        self.arr = new_arr;
    }

    fn seats_occ(&self) -> usize {
        self.arr
            .elements_row_major_iter()
            .filter(|s| **s == Space::Occ)
            .count()
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Content {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let gen = input
        .lines()
        .flat_map(|l| l.chars().map(|c| Space::from_char(c).unwrap()));

    Content {
        arr: Array2D::from_iter_row_major(gen, h, w),
        changed: true,
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut grid = (*input).clone();

    while grid.changed() {
        grid.step();
    }
    grid.seats_occ()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut grid = (*input).clone();

    while grid.changed() {
        grid.step_pt2();
    }
    grid.seats_occ()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    const INPUT: &str = include_str!("../input/2020/day11.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 37);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 26);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 2412);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 2176);
    }
}
