#![allow(unused_variables, dead_code)]

trait PairMatch {
    fn is_begin(self) -> bool;
    fn is_end(self) -> bool;
    fn matching(self) -> Self;
    fn score(self) -> usize;
}
impl PairMatch for char {
    fn is_begin(self) -> bool {
        match self {
            '(' | '[' | '{' | '<' => true,
            _ => false
        }
    }
    fn is_end(self) -> bool {
        match self {
            ')' | ']' | '}' | '>' => true,
            _ => false
        }
    }

    fn matching(self) -> char {
        match self {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => '?'
        }
    }
    fn score(self) -> usize {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        }
    }
}

#[derive(Default)]
pub struct Stack {
    back: String
}

impl Stack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, c: char) {
        assert!(c.is_begin());
        self.back.push(c);
    }

    fn pop(&mut self, c: char) -> Result<(), usize> {
        assert!(c.is_end());
        if self.back.is_empty()  {
            return Err(c.score());
        }

        let endc = self.back.pop().unwrap();

        if c != endc.matching() {
            self.back.push(endc);
            return Err(c.score());
        }

        Ok(())
    }


    fn parse_line(&mut self, s: &str) -> Result<(), usize> {
        for c in s.chars() {
            if c.is_begin() {
                self.push(c);
            } else {
                self.pop(c)?;
            }
        }
        Ok(())
    }

}

pub struct Content {
    lines: Vec<String>
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Content {
    Content {
        lines: input.lines().map(|s| s.to_string()).collect()
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Content) -> usize {
    let mut score = 0;
    for l in &input.lines {
        let mut stack =  Stack::new();
        match stack.parse_line(l.as_str()) {
            Ok(_) => {},
            Err(line_score) => {
                score += line_score;
            }
        }
    }
    score
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Content) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    const INPUT: &str = include_str!("../input/2021/day10.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 26397);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 390993);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
