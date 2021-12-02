use arbgrid::{ArbGrid, Coord2D};
use std::cmp::{max, min, PartialEq};
use std::ops::Rem;

/// Euclid's two-thousand-year-old algorithm for finding the greatest common
/// divisor.
/// Taken from the rust standard library docs :)
/// ```
/// assert_eq!(aoc_1910::gcd(1, 10), 1);
/// assert_eq!(aoc_1910::gcd(2, 10), 2);
/// ```
pub fn gcd<T: PartialEq + Rem<Output = T> + Default + Copy>(x: T, y: T) -> T {
    let mut x = x;
    let mut y = y;
    while y != T::default() {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridSquare {
    Known(usize),
    Unknown,
    Empty,
}
pub type AstGrid = ArbGrid<GridSquare>;

impl Default for GridSquare {
    fn default() -> Self {
        Self::Empty
    }
}

impl GridSquare {
    pub fn invalidate(&mut self) {
        *self = match self {
            Self::Known(_) | Self::Unknown => Self::Unknown,
            Self::Empty => Self::Empty,
        };
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Unknown,
            '0'..='9' => Self::Known(c.to_digit(10).unwrap() as usize),
            _ => panic!(),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Unknown => '#',
            Self::Known(count) => {
                if *count < 10 {
                    (*count as u8).into()
                } else {
                    '@'
                }
            }
        }
    }
}

/// Find the whole-numbered points directly between two points
/// ```
/// use aoc_1910::visible_points;
/// use arbgrid::Coord2D;
/// assert_eq!(visible_points(Coord2D(0,0), Coord2D(0,0)), vec!());
/// assert_eq!(visible_points(Coord2D(1,5), Coord2D(1,1)).len(), 3);
/// assert_eq!(visible_points(Coord2D(4,0), Coord2D(10,0)).len(), 5);
/// assert_eq!(visible_points(Coord2D(0,0), Coord2D(1,5)).len(), 0);
/// ```
pub fn visible_points(this: Coord2D, other: Coord2D) -> Vec<Coord2D> {
    let min_x = min(other.0, this.0);
    let max_x = max(other.0, this.0);
    let min_y = min(other.1, this.1);
    let max_y = max(other.1, this.1);
    let delta = Coord2D(other.0 - this.0, other.1 - this.1);
    let delta_gcd = gcd(delta.0.abs(), delta.1.abs());
    if delta.0 == 0 {
        // Case 1: Points are in a vertical line
        return (min_y + 1..max_y).map(|y| Coord2D(this.0, y)).collect();
    } else if delta.1 == 0 {
        // Case 2: Points are in a horizontal line
        return (min_x + 1..max_x).map(|x| Coord2D(x, this.1)).collect();
    } else if delta_gcd == 1 {
        // Case 3: Points are diagonal and no point exists between
        return vec![];
    } else {
        // Case 4: Diagonal and some points between
        let delta_unit = Coord2D(delta.0 / delta_gcd, delta.1 / delta_gcd);
        return (1..delta_gcd)
            .map(|m| Coord2D(this.0 + delta_unit.0 * m, this.1 + delta_unit.1 * m))
            .collect();
    }
    // Rust: "warning: unreachable statement"
    // Me: LEARN TO READ!!!! THIS IS YOUR MACRO!!! WTF!!!
    #[allow(unreachable_code)]
    {
        unreachable!();
    }
}

pub fn do_10a(input: &str) -> (Coord2D, usize) {
    let mut grid = AstGrid::from_str_ogl(Coord2D(0, 0), input, &|c, _xy| {
        Some(GridSquare::from_char(c))
    });
    grid.trim();

    let filled_grid = clone_and_fill(&grid);

    filled_grid
        .iter()
        .map(|(xy, sq)| {
            (
                *xy,
                if let GridSquare::Known(count) = sq {
                    *count
                } else {
                    0
                },
            )
        })
        .max_by_key(|(_xy, sq)| *sq)
        .unwrap()
}

pub fn clone_and_clear(other: &AstGrid) -> AstGrid {
    let mut new_grid = other.clone();
    for (_k, v) in new_grid.iter_mut() {
        v.invalidate();
    }
    new_grid
}

pub fn clone_and_fill(grid: &AstGrid) -> AstGrid {
    let mut filled_grid = grid.clone();

    for (square_xy, square) in filled_grid.iter_mut() {
        let count = grid
            .iter()
            .map(|(other_xy, _)| {
                visible_points(*square_xy, *other_xy)
                    .iter()
                    .all(|xy| *grid.get(*xy) == GridSquare::Empty)
                    && other_xy != square_xy
            })
            .filter(|b| *b)
            .count();
        *square = GridSquare::Known(count);
    }
    filled_grid
}

pub fn show_visible(grid: &AstGrid, xy: Coord2D) -> AstGrid {
    let mut filled_grid = AstGrid::new(GridSquare::Empty);

    let visible_pts: Vec<Coord2D> = grid
        .iter()
        .filter(|(_, sq)| **sq != GridSquare::Empty)
        .map(|(other_xy, _)| {
            (
                other_xy,
                visible_points(xy, *other_xy)
                    .iter()
                    .all(|int_xy| *grid.get(*int_xy) == GridSquare::Empty)
                    && *other_xy != xy,
            )
        })
        .filter(|t| t.1)
        .map(|t| *t.0)
        .collect();
    let pt_count = visible_pts.len();
    for visible_pt in visible_pts {
        filled_grid.insert(visible_pt, GridSquare::Unknown);
    }
    filled_grid.insert(xy, GridSquare::Known(pt_count));
    filled_grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_GRID_1_FILLED: &str = "\
.7..7
.....
67775
....7
...87";
    const SAMPLE_GRID_1: &str = "\
.#..#
.....
#####
....#
...##";

    const SAMPLE_GRID_2: &str = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    const SAMPLE_GRID_3: &str = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

    const SAMPLE_GRID_4: &str = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

    const SAMPLE_GRID_5: &str = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn test_clear() {
        let filled_grid = AstGrid::from_str_ogl(Coord2D(0, 0), SAMPLE_GRID_1_FILLED, &|c, _xy| {
            Some(GridSquare::from_char(c))
        });
        let blank_grid = AstGrid::from_str_ogl(Coord2D(0, 0), SAMPLE_GRID_1, &|c, _xy| {
            Some(GridSquare::from_char(c))
        });
        let new_blank_grid = clone_and_clear(&filled_grid);
        assert_eq!(blank_grid, new_blank_grid);
    }
    #[test]
    fn test_sg1() {
        assert_eq!(do_10a(SAMPLE_GRID_1).1, 8);
    }
    #[test]
    fn test_sg2() {
        let grid = AstGrid::from_str_ogl(Coord2D(0, 0), SAMPLE_GRID_2, &|c, _xy| {
            Some(GridSquare::from_char(c))
        });
        println!("{}", grid.to_string_ogl(&|sq, _xy| GridSquare::to_char(sq)));
        println!(
            "{}",
            show_visible(&grid, Coord2D(5, 8)).to_string_ogl(&|sq, _xy| GridSquare::to_char(sq))
        );

        assert_eq!(do_10a(SAMPLE_GRID_2), (Coord2D(5, 8), 33));
    }
    #[test]
    fn test_sg3() {
        assert_eq!(do_10a(SAMPLE_GRID_3), (Coord2D(1, 2), 35));
    }
    #[test]
    fn test_sg4() {
        assert_eq!(do_10a(SAMPLE_GRID_4), (Coord2D(6, 3), 41));
    }
    #[test]
    fn test_sg5() {
        assert_eq!(do_10a(SAMPLE_GRID_5), (Coord2D(11, 13), 210));
    }
    #[test]
    fn test_1a() {
        assert_eq!(
            do_10a(include_str!("test_input.txt")),
            (Coord2D(37, 25), 309)
        );
    }
}
