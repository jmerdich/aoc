#![allow(unused_variables, dead_code, non_camel_case_types)]

use std::cmp::{min, max};

use coord::prelude::*;
use coord::vec2;

use scan_fmt::scan_fmt;
use grid::Grid;

type gridsize = u32;
type griddata = u8;


type Coord = coord::vec2::Vec2<gridsize>;

#[derive(Clone, Debug)]
pub struct Line {
    start: Coord,
    end: Coord
}

impl Line {
    fn from_str(s: &str) -> Result<Self, scan_fmt::parse::ScanError> {
        let pts = scan_fmt!(s,
            "{d}, {d} -> {d}, {d}",
             gridsize, gridsize, gridsize, gridsize)?;

        Ok(Self{
            start: vec2!(pts.0, pts.1),
            end: vec2!(pts.2, pts.3),
        })
    }

    fn is_hor_vert(&self) -> bool {
        self.is_hor() || self.is_vert()
    }
    fn is_hor(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vert(&self) -> bool {
        self.start.x == self.end.x
    }

    fn bounds(lines: &[Self]) -> Self {
        let min_x = lines.iter().map(|l| min(l.start.x, l.end.x)).min().unwrap();
        let max_x = lines.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap();
        let min_y = lines.iter().map(|l| min(l.start.y, l.end.y)).min().unwrap();
        let max_y = lines.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap();

        Line {
            start: vec2!(min_x, min_y),
            end: vec2!(max_x, max_y),
        }
    }

    fn sub(self, c: Coord) -> Self {
        Self {
            start: self.start - c,
            end: self.end - c
        }
    }
}

#[derive(Clone)]
pub struct Content {
    lines: Vec<Line>,
    grid: Grid<griddata>,
    grid_start: Coord
}

impl Content {
    fn draw_line(&mut self, l: &Line) {
        let mut l = l.clone().sub(self.grid_start);
        let step: Coord;
        if l.is_hor() {
            if l.start.x > l.end.x {
                std::mem::swap(&mut l.start.x, &mut l.end.x);
            }
            assert!(l.start.x < l.end.x);
            step = vec2!(1,0);
        } else if l.is_vert() {
            if l.start.y > l.end.y {
                std::mem::swap(&mut l.start.y, &mut l.end.y);
            }
            step = vec2!(0,1);
        } else {
            return;
        }
        let mut cur_pt = l.start;
        loop {
            *self.grid.get_mut(cur_pt.y as usize, cur_pt.x as usize).unwrap() += 1;
            cur_pt += step;
            if cur_pt.x > l.end.x || cur_pt.y > l.end.y {
                break;
            }
        };
    }
}


#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Content {
    // Meaning the drawn lines, not each line of the file :P
    let lines: Vec<Line> = input.lines().map(|l| Line::from_str(l).unwrap()).collect();
    let bounds = Line::bounds(&lines);
    Content {
        lines,
        grid: Grid::new((bounds.end.y + 1 - bounds.start.y) as usize, 
                        (bounds.end.x + 1 - bounds.start.x) as usize),
        grid_start: Coord{ x: bounds.start.x, y: bounds.start.y}
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut input = input.clone();
    let lines = input.lines.clone();
    for line in lines.iter() {
        if line.is_hor_vert() {
            input.draw_line(&line);
        }
    }
    input.grid.iter().filter(|v| **v >= 2).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    const INPUT: &str = include_str!("../input/2021/day5.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 5);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 6841);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
