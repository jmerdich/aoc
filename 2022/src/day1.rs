#![allow(unused_variables, dead_code)]

type Content = Vec<Vec<u64>>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Content {
    let mut data: Content = Vec::new();

    let mut cur_elf: Vec<u64> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            data.push(std::mem::take(&mut cur_elf));
        } else {
            cur_elf.push(line.parse().unwrap());
        }
    }
    if !cur_elf.is_empty() {
        data.push(cur_elf);
    }
    data
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Content) -> usize {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0) as usize
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Content) -> usize {
    let mut tots: Vec<u64> = input.iter().map(|elf| elf.iter().sum()).collect();

    tots.sort();
    tots.reverse();

    tots[0..3].iter().sum::<u64>() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    const INPUT: &str = include_str!("../input/2022/day1.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 24000);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 45000);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 67658);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 200158);
    }
}
