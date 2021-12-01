use regex::Regex;

pub struct PassAndPolicy {
    pub password: String,
    pub letter: char,
    pub range: (usize, usize),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PassAndPolicy> {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            PassAndPolicy {
                password: caps.get(4).unwrap().as_str().to_string(),
                letter: caps.get(3).unwrap().as_str().parse().unwrap(),
                range: (
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[PassAndPolicy]) -> usize {
    input
        .iter()
        .filter(|p| {
            let count = p.password.chars().filter(|c| *c == p.letter).count();
            count >= p.range.0 && count <= p.range.1
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[PassAndPolicy]) -> usize {
    input
        .iter()
        .filter(|p| {
            let char_a = p.password.chars().nth(p.range.0 - 1).unwrap();
            let char_b = p.password.chars().nth(p.range.1 - 1).unwrap();
            (p.letter == char_a) ^ (p.letter == char_b)
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    const INPUT: &str = include_str!("../input/2020/day2.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 2);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 1);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 536);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 558);
    }
}
