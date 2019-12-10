extern crate intcode;

#[cfg(test)]
mod tests {
    use intcode::{IntMachine, RunMode};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn quine() {
        let in_str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut cpu = IntMachine::from_str(in_str).unwrap();
        let input = cpu.get_tape().to_owned();
        cpu.run();
        assert_eq!(cpu.get_status(), RunMode::EndPgm);
        assert_eq!(cpu.output, input);
    }

    #[test]
    fn test_64a() {
        let mut cpu = IntMachine::from_str("1102,34915192,34915192,7,4,7,99,0").unwrap();
        cpu.run();
        assert_eq!(cpu.get_status(), RunMode::EndPgm);
        assert_eq!(cpu.output.pop_front().unwrap(), 1219070632396864);
    }
    #[test]
    fn test_64b() {
        let mut cpu = IntMachine::from_str("104,1125899906842624,99").unwrap();
        cpu.run();
        assert_eq!(cpu.get_status(), RunMode::EndPgm);
        assert_eq!(cpu.output.pop_front().unwrap(), 1125899906842624);
    }

    #[test]
    fn prob_9a() {
        let mut cpu = IntMachine::from_str(include_str!("test_input.txt")).unwrap();
        cpu.feed_one(1);
        cpu.run();
        assert_eq!(cpu.get_status(), RunMode::EndPgm);
        assert_eq!(cpu.output.pop_front().unwrap(), 3460311188);
    }
    #[test]
    fn prob_9b() {
        let mut cpu = IntMachine::from_str(include_str!("test_input.txt")).unwrap();
        cpu.feed_one(2);
        cpu.run();
        assert_eq!(cpu.get_status(), RunMode::EndPgm);
        assert_eq!(cpu.output.pop_front().unwrap(), 42202);
    }
}
