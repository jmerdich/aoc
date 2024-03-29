#![allow(dead_code)]
extern crate num;
#[macro_use]
extern crate num_derive;

use std::collections::VecDeque;

pub type Atom = i64;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Op(Atom);

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, Debug)]
enum OpCode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equals = 8,
    IncStack = 9,
    EndPgm = 99,
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, Debug)]
enum OpMode {
    Pos = 0,
    Imm = 1,
    Stack = 2,
}

impl Op {
    fn from_atom(value: Atom) -> Op {
        assert!(value >= 0); // Negative opcodes are likely an error?
        Op { 0: value }
    }

    fn opcode(self) -> Option<OpCode> {
        num::FromPrimitive::from_i64(self.0 % 100i64)
    }

    fn param_mode(self, param_idx: u8) -> Option<OpMode> {
        if param_idx > (((std::i64::MAX as f32).log10().ceil() as u8) - 2) {
            // Index too big!
            return None;
        }
        num::FromPrimitive::from_i64((self.0 / 10i64.pow((param_idx + 2) as u32)) % 10i64)
    }
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, Debug)]
pub enum RunMode {
    Running,
    EndPgm,
    InputStalled,
}

pub struct IntMachine {
    tape: Vec<Atom>,
    pc: usize,
    sp: usize,
    cur_op: Op,
    run_mode: RunMode,
    pub debug_mode: bool,
    pub input: VecDeque<Atom>,
    pub output: VecDeque<Atom>,
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, Debug)]
enum AluKind {
    Add,
    Mult,
    LessThan,
    Equals,
}

impl AluKind {
    fn from_opcode(op: OpCode) -> Option<AluKind> {
        match op {
            OpCode::Add => Some(AluKind::Add),
            OpCode::Mult => Some(AluKind::Mult),
            OpCode::LessThan => Some(AluKind::LessThan),
            OpCode::Equals => Some(AluKind::Equals),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, FromPrimitive, PartialEq, Eq, Debug)]
enum JumpKind {
    NonZero,
    Zero,
}
impl JumpKind {
    fn from_opcode(op: OpCode) -> Option<JumpKind> {
        match op {
            OpCode::JumpFalse => Some(JumpKind::Zero),
            OpCode::JumpTrue => Some(JumpKind::NonZero),
            _ => None,
        }
    }
}
use std::str::FromStr;
impl FromStr for IntMachine {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<IntMachine, Self::Err> {
        let in_nums: Result<Vec<Atom>, _> = input
            .lines()
            .collect::<String>()
            .split(',')
            .map(|s| s.parse::<Atom>())
            .collect();
        Ok(IntMachine::new(in_nums?))
    }
}

impl IntMachine {
    pub fn new(tape: Vec<Atom>) -> IntMachine {
        assert!(!tape.is_empty());
        let first_op = Op::from_atom(tape[0]);
        IntMachine {
            tape,
            pc: 0,
            sp: 0,
            cur_op: first_op,
            run_mode: RunMode::Running,
            debug_mode: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
    #[cfg(not(debug_assertions))]
    fn dbg<T: std::fmt::Debug>(&mut self, _num_params: u8, _op_res: Option<T>) {}

    #[cfg(debug_assertions)]
    fn dbg<T: std::fmt::Debug>(&mut self, num_params: u8, op_res: Option<T>) {
        if !self.debug_mode {
            return;
        }

        let opcode = self.cur_op.opcode().unwrap();

        print!("{} - {:?} ", self.pc, opcode);
        for i in 0..num_params {
            let param = self.tape[self.pc + (i as usize) + 1];
            match self.cur_op.param_mode(i).unwrap() {
                OpMode::Pos => print!(" {}@{}", self.get_addr(param as usize), param),
                OpMode::Imm => print!(" {}", param),
                OpMode::Stack => print!(
                    " {}@[sp:{}+{}]",
                    self.get_addr((param + self.sp as i64) as usize),
                    self.sp,
                    param
                ),
            }
        }

        if let Some(res) = op_res {
            println!(" => {:?}", res);
        } else {
            println!();
        }
    }

    // Returns true if done
    pub fn step(&mut self) -> RunMode {
        let opcode = self.cur_op.opcode();
        self.pc = match opcode {
            Some(OpCode::Add)
            | Some(OpCode::Mult)
            | Some(OpCode::LessThan)
            | Some(OpCode::Equals) => {
                self.handle_alu(AluKind::from_opcode(opcode.unwrap()).unwrap())
            }
            Some(OpCode::Input) => self.handle_input(),
            Some(OpCode::Output) => self.handle_output(),
            Some(OpCode::JumpTrue) | Some(OpCode::JumpFalse) => {
                self.handle_cond_jump(JumpKind::from_opcode(opcode.unwrap()).unwrap())
            }
            Some(OpCode::EndPgm) => self.handle_endpgm(),
            Some(OpCode::IncStack) => self.handle_incstack(),
            None => panic!("Unrecognized opcode: {}@{}", self.tape[self.pc], self.pc),
        };
        self.cur_op = Op::from_atom(self.tape[self.pc]);
        self.run_mode
    }

    pub fn run(&mut self) -> RunMode {
        while self.run_mode == RunMode::Running {
            self.step();
        }
        self.run_mode
    }

    pub fn get_tape(&self) -> &[Atom] {
        &self.tape
    }

    pub fn get_status(&self) -> RunMode {
        self.run_mode
    }

    pub fn feed_one(&mut self, value: Atom) {
        if self.run_mode == RunMode::InputStalled {
            self.run_mode = RunMode::Running;
        }
        self.input.push_back(value);
    }

    pub fn feed(&mut self, value: &[Atom]) {
        if self.run_mode == RunMode::InputStalled {
            self.run_mode = RunMode::Running;
        }
        self.input.extend(value);
    }
    fn get_addr(&mut self, addr: usize) -> Atom {
        if addr >= self.tape.len() {
            self.tape.resize(addr + 1, 0);
        }

        self.tape[addr]
    }
    fn set_addr(&mut self, addr: usize, val: Atom) {
        if addr >= self.tape.len() {
            self.tape.resize(addr + 1, 0);
        }

        self.tape[addr] = val;
    }

    fn get_param(&mut self, param_idx: u8) -> Atom {
        let param_addr = self.tape[self.pc + (param_idx as usize) + 1];
        match self.cur_op.param_mode(param_idx).unwrap() {
            OpMode::Pos => {
                assert!(param_addr >= 0); // negative absolute addresses don't make sense
                self.get_addr(param_addr as usize)
            }
            OpMode::Imm => param_addr,
            OpMode::Stack => {
                assert!(param_addr + self.sp as i64 >= 0);
                self.get_addr((param_addr + self.sp as i64) as usize)
            }
        }
    }
    fn set_param(&mut self, param_idx: u8, value: Atom) {
        let param_addr = self.tape[self.pc + (param_idx as usize) + 1];
        match self.cur_op.param_mode(param_idx).unwrap() {
            OpMode::Pos => {
                assert!(param_addr >= 0); // negative absolute addresses don't make sense
                self.set_addr(param_addr as usize, value);
            }
            OpMode::Imm => panic!(), // Writing to an immediate doesn't make sense
            OpMode::Stack => {
                assert!(param_addr + self.sp as i64 >= 0);
                self.set_addr((param_addr + self.sp as i64) as usize, value);
            }
        }
    }

    fn handle_endpgm(&mut self) -> usize {
        self.dbg::<()>(0, None);
        self.run_mode = RunMode::EndPgm;
        self.pc
    }

    fn handle_incstack(&mut self) -> usize {
        let val_1 = self.get_param(0);

        // Holy crap, Rust, really????
        assert!((self.sp as isize + (val_1 as isize)) > 0isize);
        self.sp = (self.sp as isize + (val_1 as isize)) as usize;

        self.dbg(1, Some(self.sp));

        self.pc + 2
    }

    fn handle_alu(&mut self, kind: AluKind) -> usize {
        assert!(self.tape.len() >= self.pc + 4);
        let val_1 = self.get_param(0);
        let val_2 = self.get_param(1);
        let res: Atom = match kind {
            AluKind::Add => val_1 + val_2,
            AluKind::Mult => val_1 * val_2,
            AluKind::LessThan => (val_1 < val_2) as Atom,
            AluKind::Equals => (val_1 == val_2) as Atom,
        };
        self.dbg(3, Some(res));

        self.set_param(2, res);
        self.pc + 4
    }

    fn handle_input(&mut self) -> usize {
        assert!(self.tape.len() >= self.pc + 2);
        if self.input.is_empty() {
            self.run_mode = RunMode::InputStalled;
            self.dbg(1, Some("STALLED"));
            return self.pc;
        }

        let value = self.input.pop_front().unwrap();
        self.set_param(0, value);

        self.dbg(1, Some(value));

        self.pc + 2
    }

    fn handle_output(&mut self) -> usize {
        assert!(self.tape.len() >= self.pc + 2);

        let value = self.get_param(0);
        self.output.push_back(value);

        self.dbg(1, Some(value));

        self.pc + 2
    }

    fn handle_cond_jump(&mut self, kind: JumpKind) -> usize {
        assert!(self.tape.len() >= self.pc + 3);

        let test_value = self.get_param(0);
        let target = self.get_param(1);
        assert!(target >= 0);

        let pred = match kind {
            JumpKind::NonZero => test_value != 0,
            JumpKind::Zero => test_value == 0,
        };

        self.dbg(2, Some(pred));
        if pred {
            target as usize
        } else {
            self.pc + 3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate() {
        let mut cpu = IntMachine::new(vec![1002, 4, 3, 4, 33]);
        cpu.run();
        assert_eq!(cpu.get_tape(), vec!(1002, 4, 3, 4, 99).as_slice());
    }

    #[test]
    fn basic_io() {
        let mut cpu = IntMachine::new(vec![3, 0, 4, 0, 99]);
        let echoed = 12345;
        cpu.input.push_back(echoed);
        cpu.run();
        assert_eq!(cpu.output, vec!(echoed));
    }

    #[test]
    fn test_jmps() {
        // Checks if equal to 8
        for num in vec![7, 8] {
            let mut cpu = IntMachine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
            cpu.input.push_back(num);
            cpu.run();
            assert_eq!(cpu.output, vec!((num == 8) as Atom));
            let mut cpu = IntMachine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
            cpu.input.push_back(num);
            cpu.run();
            assert_eq!(cpu.output, vec!((num == 8) as Atom));
        }
        // checks if less than 8
        for num in vec![7, 8] {
            let mut cpu = IntMachine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
            cpu.input.push_back(num);
            cpu.run();
            assert_eq!(cpu.output, vec!((num < 8) as Atom));
            let mut cpu = IntMachine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
            cpu.input.push_back(num);
            cpu.run();
            assert_eq!(cpu.output, vec!((num < 8) as Atom));
        }
    }
    #[test]
    fn test_bigger() {
        let check = |input, expected| {
            let mut cpu = IntMachine::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]);
            cpu.input.push_back(input);
            cpu.run();
            assert_eq!(cpu.output, vec!(expected))
        };

        check(-1, 999);
        check(7, 999);
        check(8, 1000);
        check(9, 1001);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
