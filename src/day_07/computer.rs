use crossbeam_channel::{
    Receiver,
    Sender,
};
use std::convert::TryInto;

pub struct Computer {
    pub memory: Vec<isize>,
    pub input: Reader,
    pub output: Writer,
    pub debug_flags: DebugFlags,
}

#[derive(Default)]
pub struct DebugFlags {
    pub print_memory: bool,
    pub print_instructions: bool,
}

impl DebugFlags {
    pub fn print_instructions(self) -> Self {
        Self {
            print_instructions: true,
            ..self
        }
    }
}

impl From<Vec<isize>> for Computer {
    fn from(memory: Vec<isize>) -> Self {
        Self {
            memory,
            input: Reader::Disabled,
            output: Writer::Disabled,
            debug_flags: DebugFlags::default(),
        }
    }
}

#[derive(Debug)]
pub enum ComputerError {
    IndexNotFound(usize),
    ReadFromInput(std::io::Error),
}

pub enum Reader {
    Disabled,
    Tester(Box<dyn Iterator<Item = isize>>),
    Channel(Receiver<isize>),
}

impl Reader {
    fn read(&mut self) -> isize {
        use Reader::*;

        match self {
            Disabled => panic!("trying to use disabled reader"),
            Tester(iter) => iter.next().expect("reader tester ran out of values"),
            Channel(receiver) => receiver.recv().unwrap(),
        }
    }
}

pub enum Writer {
    Disabled,
    Tester { values: Vec<isize> },
    Channel(Sender<isize>),
}

impl Writer {
    fn write(&mut self, value: isize) {
        use Writer::*;

        match self {
            Disabled => panic!("trying to use disabled writer"),

            Tester { values } => {
                values.push(value);
            }

            Channel(sender) => sender.send(value).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn values(self) -> Vec<isize> {
        match self {
            Writer::Tester { values } => values,
            _ => Vec::new(),
        }
    }
}

impl Computer {
    pub fn run(&mut self) -> Result<Vec<isize>, ComputerError> {
        use ComputerError::*;

        let mut memory = self.memory.clone();

        let mut index = 0;
        loop {
            if self.debug_flags.print_memory {
                println!("memory: {:?}", memory);
            }

            let memory_value = memory.get(index).ok_or(IndexNotFound(index))?;
            let intcode = memory_value.into();

            match intcode {
                Intcode::Add(ref first_value_mode, ref second_value_mode)
                | Intcode::Mul(ref first_value_mode, ref second_value_mode) => {
                    let first_value = read_value(&memory, index + 1, first_value_mode)?;
                    let second_value = read_value(&memory, index + 2, second_value_mode)?;
                    let result_address = memory[index + 3] as usize;

                    let result_value = match intcode {
                        Intcode::Add(..) => {
                            if self.debug_flags.print_instructions {
                                println!(
                                    "ADD\t{}\t{}\t{}",
                                    first_value, second_value, result_address
                                );
                            }

                            first_value + second_value
                        }
                        Intcode::Mul(..) => {
                            if self.debug_flags.print_instructions {
                                println!(
                                    "MUL\t{}\t{}\t{}",
                                    first_value, second_value, result_address
                                );
                            }

                            first_value * second_value
                        }
                        _ => unreachable!(),
                    };

                    memory[result_address] = result_value;

                    index += 4;
                }

                Intcode::JumpIfTrue(ref first_value_mode, ref second_value_mode) => {
                    let first_value = read_value(&memory, index + 1, first_value_mode)?;
                    let second_value = read_value(&memory, index + 2, second_value_mode)?;

                    if first_value != 0 {
                        index = second_value.try_into().unwrap();
                        if self.debug_flags.print_instructions {
                            println!("JMPT\t{}", index);
                        }
                    } else {
                        index += 3
                    }
                }

                Intcode::JumpIfFalse(ref first_value_mode, ref second_value_mode) => {
                    let first_value = read_value(&memory, index + 1, first_value_mode)?;
                    let second_value = read_value(&memory, index + 2, second_value_mode)?;

                    if first_value == 0 {
                        index = second_value.try_into().unwrap();
                        if self.debug_flags.print_instructions {
                            println!("JMPF\t{}", index);
                        }
                    } else {
                        index += 3
                    }
                }

                Intcode::LessThan(ref first_value_mode, ref second_value_mode) => {
                    let first_value = read_value(&memory, index + 1, first_value_mode)?;
                    let second_value = read_value(&memory, index + 2, second_value_mode)?;

                    let result_value = if first_value < second_value { 1 } else { 0 };

                    let result_address = memory[index + 3] as usize;
                    memory[result_address] = result_value;

                    if self.debug_flags.print_instructions {
                        println!(
                            "LESS\t{}\t{}\t{}",
                            first_value, second_value, result_address
                        );
                    }

                    index += 4;
                }

                Intcode::Equals(ref first_value_mode, ref second_value_mode) => {
                    let first_value = read_value(&memory, index + 1, first_value_mode)?;
                    let second_value = read_value(&memory, index + 2, second_value_mode)?;

                    let result_value = if first_value == second_value { 1 } else { 0 };

                    let result_address = memory[index + 3] as usize;
                    memory[result_address] = result_value;

                    if self.debug_flags.print_instructions {
                        println!(
                            "EQUAL\t{}\t{}\t{}",
                            first_value, second_value, result_address
                        );
                    }

                    index += 4;
                }

                Intcode::Read => {
                    let address = memory[index + 1] as usize;
                    let value = self.input.read();
                    if self.debug_flags.print_instructions {
                        println!("READ\t{}\t{}", value, address);
                    }

                    memory[address] = value;
                    index += 2;
                }

                Intcode::Write(ref value_mode) => {
                    let value = read_value(&memory, index + 1, value_mode)?;

                    if self.debug_flags.print_instructions {
                        println!("WRITE\t{}", value);
                    }

                    self.output.write(value);
                    index += 2;
                }

                Intcode::Halt => {
                    if self.debug_flags.print_instructions {
                        println!("HALT");
                    }

                    break;
                }
                Intcode::Skip => index += 1,
            }
        }

        Ok(memory)
    }
}

fn read_value(
    memory: &[isize],
    index: usize,
    mode: &ParameterMode,
) -> Result<isize, ComputerError> {
    let address = match mode {
        ParameterMode::PositionMode => memory[index] as usize,
        ParameterMode::ImmediateMode => index,
    };

    let value = memory
        .get(address)
        .ok_or(ComputerError::IndexNotFound(index))?;

    Ok(*value)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Intcode {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Read,
    Write(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),

    Halt,
    Skip,
}

impl From<&isize> for Intcode {
    fn from(val: &isize) -> Self {
        (*val).into()
    }
}

impl From<isize> for Intcode {
    fn from(value: isize) -> Self {
        use Intcode::*;

        if value < 0 {
            return Skip;
        }

        let value = value as usize;

        let mut digits = crate::day_04::part_1::digits_reverse(value);
        let optcode = digits.next().unwrap_or_default() + digits.next().unwrap_or_default() * 10;

        let first_mode = digits.next().unwrap_or_default().into();
        let second_mode = digits.next().unwrap_or_default().into();
        let _third_mode: ParameterMode = digits.next().unwrap_or_default().into();

        match optcode {
            1 => Add(first_mode, second_mode),
            2 => Mul(first_mode, second_mode),
            3 => Read,
            4 => Write(first_mode),
            5 => JumpIfTrue(first_mode, second_mode),
            6 => JumpIfFalse(first_mode, second_mode),
            7 => LessThan(first_mode, second_mode),
            8 => Equals(first_mode, second_mode),
            99 => Halt,
            _ => Skip,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl From<u8> for ParameterMode {
    fn from(val: u8) -> Self {
        use ParameterMode::*;

        match val {
            0 => PositionMode,
            1 => ImmediateMode,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul<usize> for &ParameterMode {
    type Output = usize;

    fn mul(self, rhs: usize) -> usize {
        use ParameterMode::*;

        match self {
            PositionMode => 0,
            ImmediateMode => rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{
        Computer,
        DebugFlags,
        Reader,
        Writer,
    };
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn computer_run_example_text() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let got = Computer::from(input).run().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn computer_run_example01() {
        let input = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        let got = Computer::from(input).run().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn computer_run_example02() {
        let input = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        let got = Computer::from(input).run().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn computer_run_example03() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let got = Computer::from(input).run().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn computer_run_example04() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let got = Computer::from(input).run().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn computer_run_day_05_example01() {
        let input_memory = vec![3, 0, 4, 0, 99];
        let input_values = vec![42];

        let expected_memory = vec![42, 0, 4, 0, 99];
        let expected_output = Writer::Tester { values: vec![42] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        let got_memory = computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_memory, got_memory);
        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn intcodes_simple() {
        use super::{
            Intcode::*,
            ParameterMode::*,
        };

        assert_eq!(Add(PositionMode, PositionMode), 1.into());
        assert_eq!(Mul(PositionMode, PositionMode), 2.into());
        assert_eq!(Read, 3.into());
        assert_eq!(Write(PositionMode), 4.into());
        assert_eq!(Halt, 99.into());
        assert_eq!(Skip, 50.into());

        assert_eq!(Write(ImmediateMode), 104.into());
    }

    #[test]
    fn intcodes_with_modes_day_05_example_01() {
        use super::ParameterMode::*;

        let input = 1002;
        let expected = super::Intcode::Mul(PositionMode, ImmediateMode);

        let got: super::Intcode = (&input).into();

        assert_eq!(expected, got);
    }

    #[test]
    fn intcodes_with_modes_day_05_add() {
        use super::{
            Intcode::*,
            ParameterMode::*,
        };

        assert_eq!(Add(PositionMode, PositionMode,), 1.into());
        assert_eq!(Skip, 11.into());
        assert_eq!(Add(ImmediateMode, PositionMode,), 101.into());
        assert_eq!(Add(PositionMode, ImmediateMode,), 1001.into());
        assert_eq!(Add(ImmediateMode, ImmediateMode,), 1101.into());
    }

    #[test]
    fn computer_run_day_05_example02() {
        let input_memory = vec![1002, 4, 3, 4, 33];
        let expected_memory = vec![1002, 4, 3, 4, 99];

        let mut computer = Computer::from(input_memory);

        let got_memory = computer.run().unwrap();

        assert_eq!(expected_memory, got_memory);
    }

    #[test]
    fn computer_run_day_05_example03() {
        let input_memory = vec![1101, 100, -1, 4, 0];
        let expected_memory = vec![1101, 100, -1, 4, 99];

        let mut computer = Computer::from(input_memory);

        let got_memory = computer.run().unwrap();

        assert_eq!(expected_memory, got_memory);
    }

    #[test]
    fn computer_run_day_05_example04_equal() {
        let input_memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input_values = vec![8];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example04_not_equal() {
        let input_memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input_values = vec![7];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example05_less() {
        let input_memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input_values = vec![7];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example05_equal() {
        let input_memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input_values = vec![8];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example06_equal() {
        let input_memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input_values = vec![8];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example06_not_equal() {
        let input_memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input_values = vec![42];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example07_less() {
        let input_memory = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input_values = vec![-42];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example07_equal() {
        let input_memory = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input_values = vec![8];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example08_zero() {
        let input_memory = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let input_values = vec![0];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example08_not_zero() {
        let input_memory = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let input_values = vec![42];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example09_zero() {
        let input_memory = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let input_values = vec![0];

        let expected_output = Writer::Tester { values: vec![0] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example09_not_zero() {
        let input_memory = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let input_values = vec![42];

        let expected_output = Writer::Tester { values: vec![1] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example10_below_eight() {
        let input_memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input_values = vec![-42];

        let expected_output = Writer::Tester { values: vec![999] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example10_equal_eight() {
        let input_memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input_values = vec![8];

        let expected_output = Writer::Tester { values: vec![1000] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_05_example10_above_eight() {
        let input_memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input_values = vec![42];

        let expected_output = Writer::Tester { values: vec![1001] };

        let mut computer = Computer {
            memory: input_memory,
            input: Reader::Tester(Box::new(input_values.into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[bench]
    fn bench_computer_run_example_text(b: &mut Bencher) {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }

    #[bench]
    fn bench_computer_run_example01(b: &mut Bencher) {
        let input = vec![1, 0, 0, 0, 99];

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }

    #[bench]
    fn bench_computer_run_example02(b: &mut Bencher) {
        let input = vec![2, 3, 0, 3, 99];

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }

    #[bench]
    fn bench_computer_run_example03(b: &mut Bencher) {
        let input = vec![2, 4, 4, 5, 99, 0];

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }

    #[bench]
    fn bench_computer_run_example04(b: &mut Bencher) {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }

    #[bench]
    fn bench_computer_run_restore_gravity_assist_program(b: &mut Bencher) {
        let input = crate::day_02::part_1::INPUT
            .iter()
            .map(|val| *val as isize)
            .collect::<Vec<_>>();

        let mut computer = Computer::from(input);
        b.iter(|| computer.run());
    }
}
