#![allow(dead_code)]

type Deg = i64;
type Unit = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cmd {
    N(Unit),
    S(Unit),
    E(Unit),
    W(Unit),
    Left(Deg),
    Right(Deg),
    Fwd(Unit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub loc: (Unit, Unit),
    angle: Deg, // From right, +=ccw
}

impl Pos {
    pub fn new() -> Self {
        Pos {
            loc: (0, 0),
            angle: 0,
        }
    }

    pub fn move_rel(mut self, x: Unit, y: Unit) -> Self {
        self.loc.0 += x;
        self.loc.1 += y;

        self
    }

    pub fn rotate(mut self, deg: Deg) -> Self {
        assert!(deg > -360);
        assert!(deg < 360);
        self.angle += deg + 360;
        self.angle %= 360;

        self
    }

    pub fn fwd(self, dist: Unit) -> Self {
        assert!(self.angle < 360);
        match self.angle {
            0 => self.move_rel(dist, 0),
            90 => self.move_rel(0, dist),
            180 => self.move_rel(-dist, 0),
            270 => self.move_rel(0, -dist),
            _ => panic!(),
        }
    }

    pub fn angle(self) -> Deg {
        self.angle
    }
    pub fn set_angle(mut self, deg: Deg) -> Self {
        assert!(deg > -360);
        assert!(deg < 360);
        self.angle = (deg + 360) % 360;
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WayPos {
    pub loc: (Unit, Unit),
    pub way_loc: (Unit, Unit),
}

impl WayPos {
    pub fn new() -> Self {
        WayPos {
            loc: (0, 0),
            way_loc: (0, 0),
        }
    }

    pub fn move_way(mut self, x: Unit, y: Unit) -> Self {
        self.way_loc.0 += x;
        self.way_loc.1 += y;

        self
    }

    pub fn rotate(mut self, deg: Deg) -> Self {
        assert!(deg > -360);
        assert!(deg < 360);
        let deg = (deg + 360) % 360;

        match deg {
            0 => {}
            90 => {
                self.way_loc = (-self.way_loc.1, self.way_loc.0);
            }
            180 => {
                self.way_loc = (-self.way_loc.0, -self.way_loc.1);
            }
            270 => {
                self.way_loc = (self.way_loc.1, -self.way_loc.0);
            }
            _ => panic!(),
        }

        self
    }

    pub fn fwd(mut self, dist: Unit) -> Self {
        self.loc.0 += self.way_loc.0 * dist;
        self.loc.1 += self.way_loc.1 * dist;

        self
    }
}

impl Cmd {
    pub fn from_str(s: &str) -> Option<Self> {
        let (op, arg) = s.split_at(1);
        let arg: i64 = arg.parse().unwrap();

        match op.chars().next().unwrap() {
            'N' => Some(Cmd::N(arg)),
            'S' => Some(Cmd::S(arg)),
            'E' => Some(Cmd::E(arg)),
            'W' => Some(Cmd::W(arg)),
            'L' => Some(Cmd::Left(arg)),
            'R' => Some(Cmd::Right(arg)),
            'F' => Some(Cmd::Fwd(arg)),
            _ => None,
        }
    }

    pub fn step(&self, pos: Pos) -> Pos {
        match self {
            Cmd::N(i) => pos.move_rel(0, *i),
            Cmd::S(i) => pos.move_rel(0, -*i),
            Cmd::E(i) => pos.move_rel(*i, 0),
            Cmd::W(i) => pos.move_rel(-*i, 0),
            Cmd::Left(i) => pos.rotate(*i),
            Cmd::Right(i) => pos.rotate(-*i),
            Cmd::Fwd(i) => pos.fwd(*i),
        }
    }
    pub fn step_way(&self, pos: WayPos) -> WayPos {
        match self {
            Cmd::N(i) => pos.move_way(0, *i),
            Cmd::S(i) => pos.move_way(0, -*i),
            Cmd::E(i) => pos.move_way(*i, 0),
            Cmd::W(i) => pos.move_way(-*i, 0),
            Cmd::Left(i) => pos.rotate(*i),
            Cmd::Right(i) => pos.rotate(-*i),
            Cmd::Fwd(i) => pos.fwd(*i),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Cmd> {
    input.lines().map(|l| Cmd::from_str(l).unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Cmd]) -> usize {
    let mut pos = Pos::new();
    for cmd in input {
        pos = cmd.step(pos);
    }

    (pos.loc.0.abs() + pos.loc.1.abs()) as usize
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Cmd]) -> usize {
    let mut pos = WayPos::new();
    pos.way_loc = (10, 1);
    for cmd in input {
        pos = cmd.step_way(pos);
    }

    (pos.loc.0.abs() + pos.loc.1.abs()) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
F10
N3
F7
R90
F11";
    const INPUT: &str = include_str!("../input/2020/day12.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 25);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 286);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 882);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 28885);
    }
}
