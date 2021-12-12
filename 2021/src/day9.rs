#![allow(unused_variables, dead_code)]

use grid::Grid;

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

        if y > 0 {
            adjacent.push((x, y - 1));
        }
        if y + 1 < self.grid.rows() {
            adjacent.push((x, y + 1));
        }
        if x > 0 {
            adjacent.push((x - 1, y));
        }
        if x + 1 < self.grid.cols() {
            adjacent.push((x + 1, y));
        }

        adjacent
    }

    fn find_mins(&self) -> Vec<(usize, usize)> {
        let mut items = Vec::new();

        for x in 0..self.grid.cols() {
            for y in 0..self.grid.rows() {
                let v = *self.grid.get(y, x).unwrap();
                if v < *self.adjacent_vals(x, y).iter().min().unwrap() {
                    items.push((x, y));
                }
            }
        }

        items
    }

    fn find_pool_size_for_point(&self, x: usize, y: usize) -> usize {
        let mut filled = Grid::init(self.grid.rows(), self.grid.cols(), false);

        self.fill_from_point(x, y, &mut filled);

        filled.iter().filter(|v| **v).count()
    }

    fn fill_from_point(&self, x: usize, y: usize, mask: &mut Grid<bool>) {
        if *mask.get(y, x).unwrap() || (*self.grid.get(y, x).unwrap() == 9) {
            return;
        }

        *mask.get_mut(y, x).unwrap() = true;

        for (px, py) in self.adjacent(x, y) {
            self.fill_from_point(px, py, mask);
        }
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
    input
        .find_mins()
        .iter()
        .map(|(x, y)| (*input.grid.get(*y, *x).unwrap() + 1) as usize)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut pool_sizes: Vec<usize> = input
        .find_mins()
        .iter()
        .map(|(x, y)| input.find_pool_size_for_point(*x, *y))
        .collect();

    pool_sizes.sort_unstable();

    let mut res = 1;

    for _ in 0..3 {
        if let Some(biggest) = pool_sizes.pop() {
            res *= biggest;
        }
    }

    res
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
        assert_eq!(solve_part2(&content), 1134);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 516);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 1023660);
    }
}
