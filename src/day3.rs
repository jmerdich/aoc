#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GridSq {
    Ground,
    Tree,
}

pub struct Map {
    width: usize,
    height: usize,
    terrain: Vec<Vec<GridSq>>,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Option<GridSq> {
        if y >= self.height {
            return None;
        }
        Some(self.terrain[y][x % self.width])
    }
    fn at_loc(&self, loc: (usize, usize)) -> Option<GridSq> {
        self.at(loc.0, loc.1)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let terrain: Vec<Vec<GridSq>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => GridSq::Ground,
                    '#' => GridSq::Tree,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    assert!(!terrain.is_empty());
    let width = terrain[0].len();
    assert!(width > 0);
    assert!(terrain.iter().all(|v| v.len() == width));

    Map {
        width,
        height: terrain.len(),
        terrain,
    }
}

fn traverse_path(map: &Map, velocity: (usize, usize)) -> usize {
    let mut loc: (usize, usize) = (0, 0);
    let mut hits = 0;
    loop {
        match map.at_loc(loc) {
            Some(GridSq::Ground) => {}
            Some(GridSq::Tree) => {
                hits += 1;
            }
            None => {
                break;
            }
        }
        loc = (loc.0 + velocity.0, loc.1 + velocity.1);
    }
    hits
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> usize {
    traverse_path(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> usize {
    let paths = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    paths.iter().map(|p| traverse_path(input, *p)).product()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    const INPUT: &str = include_str!("../input/2020/day3.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 7);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 336);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 289);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 5522401584);
    }
}
