#![allow(unused_variables, dead_code)]

#[derive(Debug, Clone)]
pub struct Content {
    pub counts: [u64; 10],
}

impl Content {
    pub fn count_zeros(&self) -> usize {
        self.counts[0] as usize
    }

    pub fn age(&mut self) {
        let mut counts: [u64; 10] = [0; 10];
        counts[6] += self.counts[0];
        for n in 1..10 {
            counts[n - 1] += self.counts[n];
        }
        self.counts = counts;
    }

    pub fn spawn_new(&mut self, count: usize) {
        self.counts[8] += count as u64;
    }

    pub fn sim_fish(&mut self, days: usize) {
        for day in 0..days {
            let to_add = self.count_zeros();
            self.age();
            self.spawn_new(to_add);
        }
    }

    pub fn count(&self) -> usize {
        self.counts.iter().sum::<u64>() as usize
    }

    pub fn from_ages(ages: &[u64]) -> Self {
        let mut counts: [u64; 10] = [0; 10];
        for n in 0..10 {
            counts[n] = ages.iter().filter(|v| **v == n as u64).count() as u64;
        }
        Content { counts }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Content {
    let ages: Vec<u64> = input
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect();
    Content::from_ages(&ages)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut input = input.clone();
    input.sim_fish(80);
    input.count()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut input = input.clone();
    input.sim_fish(256);
    input.count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
3,4,3,1,2
";
    const INPUT: &str = include_str!("../input/2021/day6.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 5934);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 26984457539);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 380243);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 1708791884591);
    }
}
