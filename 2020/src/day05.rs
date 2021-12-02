pub struct Seat {
    pub row: u8,
    pub col: u8,
}

type Range = (u8, u8);

fn bisect_range(range: Range, upper: bool) -> Range {
    assert!(range.1 > range.0);
    let half = (range.1 - range.0) / 2;
    if upper {
        (range.0 + half, range.1)
    } else {
        (range.0, range.1 - half)
    }
}

impl Seat {
    /// Make a Seat from a string encoded in the janky BSP form
    pub fn from_bsp(input: &str) -> Option<Seat> {
        let row_chars = input.get(0..7)?;
        let col_chars = input.get(7..10)?;

        Some(Seat {
            row: row_chars
                .chars()
                .map(|c| c == 'B')
                .fold((0, 128), bisect_range)
                .0,
            col: col_chars
                .chars()
                .map(|c| c == 'R')
                .fold((0, 8), bisect_range)
                .0,
        })
    }

    pub fn id(&self) -> usize {
        self.row as usize * 8usize + self.col as usize
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    let seats: Option<Vec<Seat>> = input.lines().map(Seat::from_bsp).collect();
    seats.unwrap()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Seat]) -> usize {
    input.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Seat]) -> usize {
    let mut seat_map: [bool; 128 * 8] = [false; 128 * 8];

    for s in input {
        seat_map[s.id()] = true;
    }

    let mut has_started = false;
    for id in 0..128 * 8 {
        has_started |= seat_map[id];

        if has_started && !seat_map[id] {
            assert!(seat_map[id - 1] && seat_map[id + 1]);
            return id;
        }
    }

    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input/2020/day5.txt");

    #[test]
    fn bsp() {
        assert_eq!(Seat::from_bsp("FBFBBFFRLR").unwrap().id(), 357);
        assert_eq!(Seat::from_bsp("BFFFBBFRRR").unwrap().id(), 567);
        assert_eq!(Seat::from_bsp("FFFBBBFRRR").unwrap().id(), 119);
        assert_eq!(Seat::from_bsp("BBFFBBFRLL").unwrap().id(), 820);
    }

    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 974);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 646);
    }
}
