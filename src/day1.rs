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

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    for i in 0..input.len() {
        let v_i = input[i];
        for j in (i + 1)..input.len() {
            let v_j = input[j];
            for v_k in input.iter().skip(j + 1) {
                if v_i + v_j + v_k == 2020 {
                    return v_i * v_j * v_k;
                }
            }
        }
    }
    panic!("No solution?");
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
1721
979
366
299
675
1456";

    const INPUT: &str = include_str!("../input/2020/day1.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        let res = solve_part1(&content);
        assert_eq!(res, 514579);
    }

    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        let res = solve_part2(&content);
        assert_eq!(res, 241861950);
    }

    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        let res = solve_part1(&content);
        assert_eq!(res, 793524);
    }

    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        let res = solve_part2(&content);
        assert_eq!(res, 61515678);
    }
}
