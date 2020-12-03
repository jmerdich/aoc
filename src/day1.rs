#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    panic!("No solution?");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn eg_part1() {
        let input = "\
1721
979
366
299
675
1456";
        let content = input_generator(input);
        let res = solve_part1(&content);
        assert_eq!(res, 514579);
    }
}
