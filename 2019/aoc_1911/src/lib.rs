use arbgrid::{ArbGrid, Coord2D};

use intcode::{Atom, IntMachine, RunMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn atom(self) -> Atom {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn from_atom(other: Atom) -> Option<Self> {
        match other {
            0 => Some(Color::Black),
            1 => Some(Color::White),
            _ => None,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

pub fn run_gridpaint(grid: &mut ArbGrid<Color>, input: &[Atom]) {
    let mut cpu = IntMachine::new(input.to_vec());
    let mut location: Coord2D = Coord2D(0, 0);
    let mut direction = Direction::Up;

    loop {
        let cur_tile = grid.get_mut(location);
        cpu.feed_one(cur_tile.atom());

        let halt = cpu.run();
        assert!(cpu.output.len() == 2);

        *cur_tile = Color::from_atom(cpu.output.pop_front().unwrap()).unwrap();
        direction = match cpu.output.pop_front().unwrap() {
            0 => direction.turn_ccw(),
            1 => direction.turn_cw(),
            _ => panic!(),
        };

        location = match direction {
            Direction::Up => location.up(1),
            Direction::Down => location.down(1),
            Direction::Left => location.left(1),
            Direction::Right => location.right(1),
        };

        match halt {
            RunMode::EndPgm => break,
            RunMode::Running => panic!(),
            RunMode::InputStalled => continue,
        };
    }
}

pub fn do_aoc1911(input: &[Atom]) -> usize {
    let mut grid = ArbGrid::new(Color::Black);
    run_gridpaint(&mut grid, input);
    println!("{:?}", grid.bounds());
    println!(
        "{}",
        grid.to_string(&|color, _coord| match color {
            Color::Black => ' ',
            Color::White => '#',
        })
    );
    grid.iter().count()
}
pub fn do_aoc1911_b(input: &[Atom]) -> ArbGrid<Color> {
    let mut grid = ArbGrid::new(Color::Black);
    *grid.get_mut(Coord2D(0, 0)) = Color::White;
    run_gridpaint(&mut grid, input);

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem() {
        let in_nums: Result<Vec<Atom>, _> = include_str!("test_input.txt")
            .lines()
            .collect::<String>()
            .split(',')
            .map(|s| s.parse::<Atom>())
            .collect();
        assert_eq!(do_aoc1911(&in_nums.unwrap()), 2418);
    }
    #[test]
    fn problem_b() {
        let in_nums: Result<Vec<Atom>, _> = include_str!("test_input.txt")
            .lines()
            .collect::<String>()
            .split(',')
            .map(|s| s.parse::<Atom>())
            .collect();

        let expected_str = "
  ##  ###  ####   ##  ##  #    ###  ###
 #  # #  # #       # #  # #    #  # #  #
 #    #  # ###     # #  # #    #  # #  #
 # ## ###  #       # #### #    ###  ###
 #  # # #  #    #  # #  # #    #    # #
  ### #  # ####  ##  #  # #### #    #  #";

        let expected_grid = ArbGrid::from_str(Coord2D(0, -5), expected_str, &|c, _xy| match c {
            ' ' => None,
            '#' => Some(Color::White),
            _ => panic!(),
        });
        println!("EXPECTED:");
        println!("{:?}", expected_grid.bounds());
        println!(
            "{}",
            expected_grid.to_string(&|color, _coord| match color {
                Color::Black => ' ',
                Color::White => '#',
            })
        );

        let mut calc_grid = do_aoc1911_b(&in_nums.unwrap());
        calc_grid.trim();
        println!("ACTUAL:");
        println!("{:?}", calc_grid.bounds());
        println!(
            "{}",
            calc_grid.to_string(&|color, _coord| match color {
                Color::Black => ' ',
                Color::White => '#',
            })
        );
        assert_eq!(calc_grid, expected_grid);
    }
}
