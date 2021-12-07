#![allow(unused_variables, dead_code)]

pub struct Content {
    posns: Vec<u32>,
}

impl Content {
    fn fuel_for(&self, dest: u32) -> u32 {
        let fuel = self
            .posns
            .iter()
            .map(|p| (*p as i32 - dest as i32).abs() as u32)
            .sum();
        // println!("fuel for {}: {}", dest, fuel);
        fuel
    }
    fn fuel_for_exp(&self, dest: u32) -> u32 {
        let fuel = self
            .posns
            .iter()
            .map(|p| (*p as i32 - dest as i32).abs() as u32)
            .map(|n| (n * (n + 1)) / 2)
            .sum();
        // println!("fuel for {}: {}", dest, fuel);
        fuel
    }

    fn min(&self) -> u32 {
        *self.posns.iter().min().unwrap()
    }
    fn max(&self) -> u32 {
        *self.posns.iter().max().unwrap()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Content {
    Content {
        posns: input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect(),
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let best_fuel = (input.min()..=input.max())
        .map(|i| input.fuel_for(i))
        .min()
        .unwrap();
    best_fuel as usize
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let best_fuel = (input.min()..=input.max())
        .map(|i| input.fuel_for_exp(i))
        .min()
        .unwrap();
    best_fuel as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
16,1,2,0,4,2,7,1,2,14";
    const INPUT: &str = include_str!("../input/2021/day7.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 37);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 168);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 344605);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 93699985);
    }
}
