use std::collections::HashMap;

type Jolts = u64;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Jolts> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Jolts]) -> usize {
    let mut sorted: Vec<Jolts> = input.to_vec();
    sorted.sort_unstable();

    let mut cur_j = 0;
    let (mut j1, mut _j2, mut j3) = (0, 0, 1);

    for val in sorted {
        match val - cur_j {
            1 => {
                j1 += 1;
            }
            2 => {
                _j2 += 1;
            }
            3 => {
                j3 += 1;
            }
            _ => {
                panic!();
            }
        }
        cur_j = val;
    }

    j1 * j3
}

pub fn count_perms_down_from(
    sorted_v: &[Jolts],
    from: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(res) = cache.get(&from) {
        *res
    } else {
        let mut res = 0;
        for i in 1..=3 {
            if from < i {
                continue;
            }
            let val = sorted_v[from - i];
            if val + 3 < sorted_v[from] {
                continue;
            }
            res += count_perms_down_from(sorted_v, from - i, cache);
        }

        if sorted_v[from] <= 3 {
            res += 1;
        }

        cache.insert(from, res);
        res
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Jolts]) -> usize {
    let mut sorted: Vec<Jolts> = input.to_vec();
    sorted.sort_unstable();

    let mut cache: HashMap<usize, usize> = HashMap::new();

    count_perms_down_from(&sorted, sorted.len() - 1, &mut cache)
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT1: &str = "\
16
10
15
5
1
11
7
19
6
12
4";
    const EG_INPUT2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    const INPUT: &str = include_str!("../input/2020/day10.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT1);
        assert_eq!(solve_part1(&content), 35);
        let content = input_generator(EG_INPUT2);
        assert_eq!(solve_part1(&content), 220);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT1);
        assert_eq!(solve_part2(&content), 8);
        let content = input_generator(EG_INPUT2);
        assert_eq!(solve_part2(&content), 19208);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 1917);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 113387824750592);
    }
}
