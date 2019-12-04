use std::convert::TryInto;
/*
--- Day 3: Crossed Wires ---

The gravity assist was successful, and you're well on your way to the Venus
refuelling station. During the rush back on Earth, the fuel management system
wasn't completely installed, so that's next on the priority list.

Opening the front panel reveals a jumble of wires. Specifically, two wires are
connected to a central port and extend outward on a grid. You trace the path
each wire takes as it leaves the central port, one wire per line of text (your
puzzle input).

The wires twist and turn, but the two wires occasionally cross paths. To fix
the circuit, you need to find the intersection point closest to the central
port. Because the wires are on a grid, use the Manhattan distance for this
measurement. While the wires do technically cross right at the central port
where they both start, this point does not count, nor does a wire count as
crossing with itself.

For example, if the first wire's path is R8,U5,L5,D3, then starting from the
central port (o), it goes right 8, up 5, left 5, and finally down 3:

...........
...........
...........
....+----+.
....|....|.
....|....|.
....|....|.
.........|.
.o-------+.
...........

Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4,
and left 4:

...........
.+-----+...
.|.....|...
.|..+--X-+.
.|..|..|.|.
.|.-X--+.|.
.|..|....|.
.|.......|.
.o-------+.
...........

These wires cross at two locations (marked X), but the lower-left one is closer
to the central port: its distance is 3 + 3 = 6.

Here are a few more examples:

    R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
    R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135

What is the Manhattan distance from the central port to the closest
intersection?

*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LineDir {
    Hor,
    Vert,
}

#[allow(non_snake_case)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn from_char(c: char) -> Result<Dir, ()> {
        match c {
            'U' => Ok(Dir::U),
            'D' => Ok(Dir::D),
            'L' => Ok(Dir::L),
            'R' => Ok(Dir::R),
            _ => Err(()),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point(i32, i32);

impl Point {
    pub fn go(self: Point, dir: Dir, dist: u32) -> Point {
        match dir {
            Dir::U => Point(self.0, self.1 + dist as i32),
            Dir::D => Point(self.0, self.1 - dist as i32),
            Dir::R => Point(self.0 + dist as i32, self.1),
            Dir::L => Point(self.0 - dist as i32, self.1),
        }
    }

    pub fn distance(self) -> u32 {
        (self.0.abs() + self.1.abs()).try_into().unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Segment {
    origin: Point,
    direction: LineDir,
    orig_dir: Dir,
    length: u32,
}

impl Segment {
    fn from_dir(p: Point, dir: Dir, dist: u32) -> Segment {
        match dir {
            Dir::U => Segment {
                origin: p,
                direction: LineDir::Vert,
                orig_dir: dir,
                length: dist,
            },
            Dir::R => Segment {
                origin: p,
                direction: LineDir::Hor,
                orig_dir: dir,
                length: dist,
            },
            Dir::D => Segment {
                origin: p.go(Dir::D, dist),
                direction: LineDir::Vert,
                orig_dir: dir,
                length: dist,
            },
            Dir::L => Segment {
                origin: p.go(Dir::L, dist),
                direction: LineDir::Hor,
                orig_dir: dir,
                length: dist,
            },
        }
    }

    fn intersection(self: &Segment, other: &Segment) -> Option<Point> {
        if self.direction == other.direction {
            return None; // Let's assume nothing for now
        }
        let res: Option<Point> = None;
        match self.direction {
            LineDir::Hor => {
                let hit = self.origin.0 <= other.origin.0
                    && (self.origin.0 + self.length as i32 >= other.origin.0)
                    && self.origin.1 >= other.origin.1
                    && (self.origin.1 <= other.origin.1 + other.length as i32);
                let loc = Point(other.origin.0, self.origin.1);
                if hit && loc != Point(0, 0) {
                    return Some(loc);
                }
            }
            LineDir::Vert => {
                let hit = self.origin.1 <= other.origin.1
                    && (self.origin.1 + self.length as i32 >= other.origin.1)
                    && self.origin.0 >= other.origin.0
                    && (self.origin.0 <= other.origin.0 + other.length as i32);
                //println!("{},{},{},{}", self.origin.1 <= other.origin.1,
                //		(self.origin.1 + self.length as i32 >= other.origin.1),
                //		self.origin.0 >= other.origin.0,
                //		(self.origin.0 <= other.origin.0 + other.length as i32));
                let loc = Point(self.origin.0, other.origin.1);
                if hit && loc != Point(0, 0) {
                    return Some(loc);
                }
            }
        }
        //println!("{:?} x {:?} => {:?}", self, other, res);
        res
    }

    fn intersect_point(&self, point: Point) -> Option<u32> {
        let hit: bool = match self.direction {
            LineDir::Hor => {
                (self.origin.0 <= point.0
                    && self.origin.0 + self.length as i32 >= point.0
                    && self.origin.1 == point.1)
            }
            LineDir::Vert => {
                (self.origin.1 <= point.1
                    && self.origin.1 + self.length as i32 >= point.1
                    && self.origin.0 == point.0)
            }
        };
        if !hit {
            return None;
        }

        let distance: u32 = match self.orig_dir {
            Dir::U => (point.1 - self.origin.1) as u32,
            Dir::D => self.length - ((point.1 - self.origin.1) as u32),
            Dir::R => (point.0 - self.origin.0) as u32,
            Dir::L => self.length - ((point.0 - self.origin.0) as u32),
        };
        Some(distance)
    }
}

#[derive(Debug)]
struct Line {
    segs: Vec<Segment>,
}

impl Line {
    fn from_str(s: &str) -> Result<Line, ()> {
        let path: Result<Vec<(Dir, u32)>, ()> = s
            .split(',')
            .map(|s| -> Result<(Dir, u32), ()> {
                let (s_dir, s_len) = s.split_at(1);
                let dir = Dir::from_char(s_dir.chars().next().unwrap()).unwrap();
                let len: u32 = s_len.parse().unwrap();
                Ok((dir, len))
            })
            .collect();
        let mut segs: Vec<Segment> = Vec::new();
        let mut cur_loc = Point(0, 0);
        for (dir, dist) in path.unwrap() {
            segs.push(Segment::from_dir(cur_loc, dir, dist));
            cur_loc = cur_loc.go(dir, dist);
        }
        Ok(Line { segs })
    }

    fn closest_manh_intersection(&self, other: &Line) -> Option<Point> {
        self.all_intersections(other)
            .and_then(|ints| ints.into_iter().min_by_key(|p| p.distance()))
    }

    fn all_intersections(&self, other: &Line) -> Option<Vec<Point>> {
        let ints: Vec<Point> = self
            .segs
            .iter()
            .map(|seg| /*-> Vec<Point>*/ {
                other
                    .segs
                    .iter()
                    .filter_map(move |oseg| seg.intersection(oseg))
            })
            .flatten()
            .collect();

        if ints.is_empty() {
            None
        } else {
            Some(ints)
        }
    }

    fn distance_ohm(&self, other: Point) -> Option<u32> {
        let mut cur_dist = 0;
        for seg in self.segs.iter() {
            if let Some(partial_dist) = seg.intersect_point(other) {
                return Some(cur_dist + partial_dist);
            }
            cur_dist += seg.length;
        }
        None
    }

    fn closest_ohm_intersection(&self, other: &Line) -> Option<(Point, u32)> {
        let distance_func =
            |p: &Point| self.distance_ohm(*p).unwrap() + other.distance_ohm(*p).unwrap();
        let min_point: Point = self
            .all_intersections(other)?
            .into_iter()
            .min_by_key(distance_func)?;
        Some((min_point, distance_func(&min_point)))
    }
}

pub fn aoc_3a(s1: &str, s2: &str) -> Option<Point> {
    Line::from_str(s1)
        .unwrap()
        .closest_manh_intersection(&Line::from_str(s2).unwrap())
}
pub fn aoc_3b(s1: &str, s2: &str) -> Option<(Point, u32)> {
    Line::from_str(s1)
        .unwrap()
        .closest_ohm_intersection(&Line::from_str(s2).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc3a_eg() {
        assert_eq!(aoc_3a("R8,U5,L5,D3", "U7,R6,D4,L4").unwrap().distance(), 6);
        assert_eq!(
            aoc_3a(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
            .unwrap()
            .distance(),
            135
        );
        assert_eq!(
            aoc_3a(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
            .unwrap()
            .distance(),
            159
        );
    }
    #[test]
    fn aoc3a_prob() {
        let prob: Vec<&str> = include_str!("test_input.txt")
            .split("\n")
            .filter(|s| s.len() > 0)
            .collect();
        assert_eq!(prob.len(), 2);
        assert_eq!(aoc_3a(prob[0], prob[1]).unwrap().distance(), 870);
    }
    #[test]
    fn aoc3b_eg() {
        assert_eq!(aoc_3b("R8,U5,L5,D3", "U7,R6,D4,L4").unwrap().1, 30);
        assert_eq!(
            aoc_3b(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
            .unwrap()
            .1,
            410
        );
        assert_eq!(
            aoc_3b(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
            .unwrap()
            .1,
            610
        );
    }
    #[test]
    fn aoc3b_prob() {
        let prob: Vec<&str> = include_str!("test_input.txt")
            .split("\n")
            .filter(|s| s.len() > 0)
            .collect();
        assert_eq!(prob.len(), 2);
        assert_eq!(aoc_3b(prob[0], prob[1]).unwrap().1, 13698);
    }
}
