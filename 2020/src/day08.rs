#![allow(unused_variables, dead_code, clippy::upper_case_acronyms)]
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Inst {
    ACC(i64),
    JMP(i64),
    NOP(i64),
    END,
}

impl Inst {
    fn from_str(s: &str) -> Option<Inst> {
        let (op, arg) = s.split(' ').collect_tuple().unwrap();
        let arg: i64 = arg.parse().ok()?;

        match op {
            "acc" => Some(Inst::ACC(arg)),
            "jmp" => Some(Inst::JMP(arg)),
            "nop" => Some(Inst::NOP(arg)),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Mach {
    pub acc: i64,
    pc: u64,
    done: bool,
    pub insts: Vec<Inst>,
    hitmap: Vec<u32>,
}

impl Mach {
    fn from_str(s: &str) -> Option<Mach> {
        let mut insts: Vec<Inst> = s
            .lines()
            .map(Inst::from_str)
            .collect::<Option<Vec<Inst>>>()?;
        insts.push(Inst::END);
        let mut hitmap: Vec<u32> = Vec::new();
        hitmap.resize(insts.len(), 0);

        Some(Mach {
            acc: 0,
            pc: 0,
            done: false,
            insts,
            hitmap,
        })
    }

    fn step(&mut self) {
        if self.done {
            return;
        }
        self.hitmap[self.pc as usize] += 1;
        dbg!(self.pc);
        dbg!(&self.insts[self.pc as usize]);

        match self.insts[self.pc as usize] {
            Inst::ACC(arg) => {
                self.acc += arg;
                self.pc += 1;
            }
            Inst::JMP(dest) => {
                self.pc = (self.pc as i64 + dest) as u64;
            }
            Inst::NOP(_) => {
                self.pc += 1;
            }
            Inst::END => {
                self.done = true;
            }
        }
    }

    fn max_hit(&self) -> u32 {
        *self.hitmap.iter().max().unwrap()
    }

    fn clear(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.done = false;
        let mut hitmap: Vec<u32> = Vec::new();
        hitmap.resize(self.insts.len(), 0);
        self.hitmap = hitmap;
    }

    fn is_done(&self) -> bool {
        self.done
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Mach {
    Mach::from_str(input).unwrap()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Mach) -> usize {
    let mut mach = input.clone();

    let mut last_accum = 0;
    while mach.max_hit() < 2 {
        last_accum = mach.acc;
        mach.step();
    }
    last_accum as usize
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Mach) -> usize {
    let mut mach = input.clone();
    let inst_stream = mach.insts.clone();
    for i in 0..inst_stream.len() {
        let inst = inst_stream[i].clone();
        let new_inst = match inst {
            Inst::NOP(arg) => Inst::JMP(arg),
            Inst::JMP(arg) => Inst::NOP(arg),
            _ => {
                continue;
            }
        };
        mach.clear();
        let mut insts = inst_stream.clone();
        insts[i] = new_inst;
        mach.insts = insts;

        while mach.max_hit() < 2 && !mach.is_done() {
            mach.step();
        }

        if mach.is_done() {
            return mach.acc as usize;
        }
    }
    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    const INPUT: &str = include_str!("../input/2020/day8.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 5);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 8);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 1684);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 2188);
    }
}
