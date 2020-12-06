#![allow(unused_variables, dead_code)]

pub struct Group {
    pub people: Vec<Person>,
}

impl Group {
    pub fn new() -> Group {
        Group { people: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.people.is_empty()
    }

    pub fn get_ans(&self, c: char) -> bool {
        self.people.iter().any(|p| p.get_ans(c))
    }
    pub fn get_ans_count(&self) -> usize {
        ('a'..='z').filter(|c| self.get_ans(*c)).count()
    }
}

pub struct Person {
    customs_ans: [bool; 26],
}

impl Person {
    pub fn from_str(s: &str) -> Option<Person> {
        if !s.chars().all(|c| c.is_ascii_lowercase()) {
            return None;
        }

        let mut p = Person {
            customs_ans: [false; 26],
        };

        for c in s.chars() {
            p.set_ans(c);
        }
        Some(p)
    }

    pub fn set_ans(&mut self, c: char) {
        assert!(c.is_ascii_lowercase());

        let idx = (c as usize) - ('a' as usize);
        self.customs_ans[idx] = true;
    }
    pub fn get_ans(&self, c: char) -> bool {
        assert!(c.is_ascii_lowercase());

        let idx = (c as usize) - ('a' as usize);
        self.customs_ans[idx]
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut cur_group = Group::new();

    for l in input.lines() {
        if l.is_empty() {
            if !cur_group.is_empty() {
                let pushed_group = std::mem::replace(&mut cur_group, Group::new());
                groups.push(pushed_group);
            }
            continue;
        }
        cur_group.people.push(Person::from_str(l).unwrap())
    }
    if !cur_group.is_empty() {
        groups.push(cur_group);
    }

    groups
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Group]) -> usize {
    input.iter().map(|g| g.get_ans_count()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Group]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
    const INPUT: &str = include_str!("../input/2020/day6.txt");
    #[test]
    fn eg_gen() {
        let content = input_generator(EG_INPUT);
        assert_eq!(content.len(), 5);
    }

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 11);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 7110);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 0);
    }
}
