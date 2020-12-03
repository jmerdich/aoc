use regex::Regex;

pub struct PassAndPolicy {
    pub password: String,
    pub letter: char,
    pub min: usize,
    pub max: usize
}


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PassAndPolicy> {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();
        PassAndPolicy{
            password: caps.get(4).unwrap().as_str().to_string(),
            letter: caps.get(3).unwrap().as_str().parse().unwrap(),
            min: caps.get(1).unwrap().as_str().parse().unwrap(),
            max: caps.get(2).unwrap().as_str().parse().unwrap()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[PassAndPolicy]) -> usize {
    input.iter().filter(|p| {
        let count = p.password.chars().filter(|c| *c == p.letter).count();
        count >= p.min && count <= p.max
    }).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[PassAndPolicy]) -> i64 {
    panic!("No solution?");
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str= "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 2);
    }

}
