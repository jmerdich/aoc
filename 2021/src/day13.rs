#![allow(unused_variables, dead_code)]
use vek::Vec2;
use scan_fmt::scan_fmt;
use std::cmp::max;

type BGrid = grid::Grid<bool>;
type UVec2 = Vec2<usize>;


#[derive(Clone, Copy)]
enum Fold {
    Hor(usize),
    Vert(usize),
}

pub struct Content {
    points: Vec<Vec2<usize>>,
    folds: Vec<Fold>,
}
fn fill_pts(grid: &mut BGrid, pts: &[UVec2]) {
    for Vec2{x, y} in pts {
        *grid.get_mut(*y, *x).unwrap() = true;
    }
}

fn fold(grid: &mut BGrid, dir: Fold) {
    match dir {
        Fold::Hor(tgt) => {
            while grid.cols() > tgt {
                let col = grid.pop_col().unwrap();
                if grid.cols() != tgt {
                    for row in 0..grid.rows() {
                        *grid.get_mut(row, 2*tgt - grid.cols()).unwrap() |= col[row];
                    }
                }
            }
        },
        Fold::Vert(tgt) => {
            while grid.rows() > tgt {
                let row = grid.pop_row().unwrap();
                if grid.rows() != tgt {
                    for col in 0..grid.cols() {
                        *grid.get_mut(2*tgt - grid.rows(), col).unwrap() |= row[col];
                    }
                }
            }
        },
    }
}

impl Content {

    fn range(&self) -> UVec2 {
        let mut range = Vec2::new(0, 0);

        for p in &self.points {
            range.x = max(p.x+1, range.x);
            range.y = max(p.y+1, range.y);
        }
        range
    }



}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Content {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for l in input.lines() {
        if let Ok((x, y)) = scan_fmt!(l, "{d},{d}", usize, usize) {
            points.push(Vec2::new(x, y));

        } else if let Ok((dir, loc)) = scan_fmt!(l, "fold along {[xy]}={d}", char, usize) {
            folds.push(match dir {
                'y' => Fold::Vert(loc),
                'x' => Fold::Hor(loc),
                _ => { panic!(); }
            });
        }
    }
    Content {
        points, folds
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let range = input.range();
    let mut grid = BGrid::init(range.y, range.x, false);

    fill_pts(&mut grid, &input.points);
    fold(&mut grid, input.folds[0]);

    grid.iter().filter(|v| **v).count()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let range = input.range();
    let mut grid = BGrid::init(range.y, range.x, false);

    fill_pts(&mut grid, &input.points);
    for f in &input.folds {
        fold(&mut grid, *f);
    }

    // TODO: real unit test here.
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if *grid.get(y, x).unwrap() {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    const INPUT: &str = include_str!("../input/2021/day13.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 17);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 755);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
