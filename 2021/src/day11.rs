#![allow(unused_variables, dead_code)]

use grid::Grid;

#[derive(Clone)]
pub struct Content {
    grid: Grid<u8>,
}

impl Content {
    fn adjacent_vals(&self, x: usize, y: usize) -> Vec<u8> {
        self.adjacent(x, y)
            .iter()
            .map(|(x, y)| self.grid.get(*y, *x).unwrap())
            .copied()
            .collect()
    }
    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();

        let mut x_vals = vec![x];
        if x > 0 {
            x_vals.push(x - 1);
        }
        if x + 1 < self.grid.cols() {
            x_vals.push(x + 1);
        }

        let mut y_vals = vec![y];
        if y > 0 {
            y_vals.push(y - 1);
        }
        if y + 1 < self.grid.cols() {
            y_vals.push(y + 1);
        }

        for xi in &x_vals {
            for yi in &y_vals {
                if *xi != x || *yi != y {
                    adjacent.push((*xi, *yi));
                }
            }
        }

        adjacent
    }

    fn full_flash(&mut self) -> usize {
        let mut flashed = Grid::init(self.grid.rows(), self.grid.cols(), false);

        for x in 0..self.grid.cols() {
            for y in 0..self.grid.rows() {
                *self.grid.get_mut(y, x).unwrap() += 1;
            }
        }
        for x in 0..self.grid.cols() {
            for y in 0..self.grid.rows() {
                self.flash_for_point(x, y, &mut flashed);
            }
        }

        let mut num_flashes = 0;
        for x in 0..self.grid.cols() {
            for y in 0..self.grid.rows() {
                let tile = self.grid.get_mut(y, x).unwrap();
                if *tile > 9 {
                    num_flashes += 1;
                    *tile = 0;
                }
            }
        }
        num_flashes
    }

    fn flash_for_point(&mut self, x: usize, y: usize, flashed: &mut Grid<bool>) {
        if *flashed.get(y, x).unwrap() || *self.grid.get(y, x).unwrap() <= 9 {
            return;
        }

        *flashed.get_mut(y, x).unwrap() = true;

        for (px, py) in self.adjacent(x, y) {
            *self.grid.get_mut(py, px).unwrap() += 1;
            self.flash_for_point(px, py, flashed);
        }
    }
}

#[aoc_generator(day11)]
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

#[aoc(day11, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut tot_flashes = 0;
    let mut input = input.clone();
    for _ in 0..100 {
        tot_flashes += (&mut input).full_flash();
    }
    tot_flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut input = input.clone();
    let mut i = 0;
    loop {
        i += 1;
        if input.full_flash() == (input.grid.rows() * input.grid.cols()) {
            return i;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    const INPUT: &str = include_str!("../input/2021/day11.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 1656);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 195);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 1755);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 212);
    }
}
