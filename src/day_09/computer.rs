use crossbeam_channel::{
    Receiver,
    Sender,
};
use std::{
    collections::BTreeMap,
    convert::TryInto,
};

pub struct Computer {
    pub memory: Vec<isize>,
    pub input: Reader,
    pub output: Writer,
    pub debug_flags: DebugFlags,
    pub relative_base: isize,
    pub heap: BTreeMap<usize, isize>,
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            memory: Vec::new(),
            input: Reader::Tester(Box::new(Vec::new().into_iter())),
            output: Writer::Tester { values: Vec::new() },
            debug_flags: DebugFlags::default(),
            relative_base: 0,
            heap: BTreeMap::default(),
        }
    }
}

impl Computer {
    pub fn with_memory(self, memory: Vec<isize>) -> Self {
        Self { memory, ..self }
    }

    pub fn with_input(self, input: Reader) -> Self {
        Self { input, ..self }
    }

    pub fn with_relative_base(self, relative_base: isize) -> Self {
        Self {
            relative_base,
            ..self
        }
    }

    pub fn debug_low(self) -> Self {
        Self {
            debug_flags: self
                .debug_flags
                .print_instructions()
                .print_jumps()
                .print_relative_base()
                .print_output(),
            ..self
        }
    }

    pub fn print_instructions(self) -> Self {
        Self {
            debug_flags: self.debug_flags.print_instructions(),
            ..self
        }
    }

    pub fn print_memory(self) -> Self {
        Self {
            debug_flags: self.debug_flags.print_memory(),
            ..self
        }
    }

    pub fn print_relative_base(self) -> Self {
        Self {
            debug_flags: self.debug_flags.print_relative_base(),
            ..self
        }
    }

    pub fn print_output(self) -> Self {
        Self {
            debug_flags: self.debug_flags.print_output(),
            ..self
        }
    }

    pub fn print_jumps(self) -> Self {
        Self {
            debug_flags: self.debug_flags.print_jumps(),
            ..self
        }
    }
}

#[derive(Default)]
pub struct DebugFlags {
    pub print_memory: bool,
    pub print_instructions: bool,
    pub print_relative_base: bool,
    pub print_output: bool,
    pub print_jumps: bool,
}

impl DebugFlags {
    pub fn print_instructions(self) -> Self {
        Self {
            print_instructions: true,
            ..self
        }
    }

    pub fn print_memory(self) -> Self {
        Self {
            print_memory: true,
            ..self
        }
    }

    pub fn print_relative_base(self) -> Self {
        Self {
            print_relative_base: true,
            ..self
        }
    }

    pub fn print_output(self) -> Self {
        Self {
            print_output: true,
            ..self
        }
    }

    pub fn print_jumps(self) -> Self {
        Self {
            print_jumps: true,
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
            ..Self::default()
        }
    }
}

#[derive(Debug)]
pub enum ComputerError {
    IndexNotFound(usize),
    ReadFromInput(std::io::Error),
    NegativeAddress(isize),
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
        let mut index = 0;

        loop {
            if self.debug_flags.print_memory {
                println!("memory: {:?}", self.memory);
            }

            let memory_value = self.read_value(index, &ParameterMode::Immediate)?;
            let intcode = memory_value.into();

            match intcode {
                Intcode::Add(ref first_value_mode, ref second_value_mode, ref third_value_mode)
                | Intcode::Mul(ref first_value_mode, ref second_value_mode, ref third_value_mode) =>
                {
                    let first_value = self.read_value(index + 1, first_value_mode)?;
                    let second_value = self.read_value(index + 2, second_value_mode)?;
                    let result_address = self.get_address(index + 3, third_value_mode);

                    let result_value = match intcode {
                        Intcode::Add(..) => first_value + second_value,
                        Intcode::Mul(..) => first_value * second_value,
                        _ => unreachable!(),
                    };

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({}), {}({}), {} => {}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            first_value,
                            index + 2,
                            second_value,
                            result_value,
                            index + 3,
                            result_address
                        );
                    }

                    self.write_value(result_address as usize, result_value)?;

                    index += 4;
                }

                Intcode::JumpIfTrue(ref first_value_mode, ref second_value_mode) => {
                    let first_value = self.read_value(index + 1, first_value_mode)?;
                    let second_value = self.read_value(index + 2, second_value_mode)?;

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({}), {}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            first_value,
                            index + 2,
                            second_value,
                        );
                    }

                    if first_value != 0 {
                        index = second_value.try_into().unwrap();

                        if self.debug_flags.print_jumps {
                            dbg!(index);
                        }
                    } else {
                        index += 3
                    }
                }

                Intcode::JumpIfFalse(ref first_value_mode, ref second_value_mode) => {
                    let first_value = self.read_value(index + 1, first_value_mode)?;
                    let second_value = self.read_value(index + 2, second_value_mode)?;

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({}), {}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            first_value,
                            index + 2,
                            second_value,
                        );
                    }

                    if first_value == 0 {
                        index = second_value.try_into().unwrap();

                        if self.debug_flags.print_jumps {
                            dbg!(index);
                        }
                    } else {
                        index += 3
                    }
                }

                Intcode::LessThan(
                    ref first_value_mode,
                    ref second_value_mode,
                    ref third_value_mode,
                ) => {
                    let first_value = self.read_value(index + 1, first_value_mode)?;
                    let second_value = self.read_value(index + 2, second_value_mode)?;
                    let result_address = self.get_address(index + 3, third_value_mode);
                    let result_value = if first_value < second_value { 1 } else { 0 };

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({}), {}({}), {} => {}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            first_value,
                            index + 2,
                            second_value,
                            result_value,
                            index + 3,
                            result_address
                        );
                    }

                    self.write_value(result_address as usize, result_value)?;

                    index += 4;
                }

                Intcode::Equals(
                    ref first_value_mode,
                    ref second_value_mode,
                    ref third_value_mode,
                ) => {
                    let first_value = self.read_value(index + 1, first_value_mode)?;
                    let second_value = self.read_value(index + 2, second_value_mode)?;
                    let result_address = self.get_address(index + 3, third_value_mode);
                    let result_value = if first_value == second_value { 1 } else { 0 };

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({}), {}({}), {} => {}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            first_value,
                            index + 2,
                            second_value,
                            result_value,
                            index + 3,
                            result_address
                        );
                    }

                    self.write_value(result_address as usize, result_value)?;

                    index += 4;
                }

                Intcode::Read(ref value_mode) => {
                    let address = self.get_address(index + 1, value_mode);
                    let value = self.input.read();

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{} => {}({})",
                            intcode,
                            memory_value,
                            value,
                            index + 1,
                            address,
                        );
                    }

                    self.write_value(address as usize, value)?;
                    index += 2;
                }

                Intcode::Write(ref value_mode) => {
                    let value = self.read_value(index + 1, value_mode)?;

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            value,
                        );
                    }

                    if self.debug_flags.print_output {
                        dbg!(value);
                    }

                    self.output.write(value);

                    index += 2;
                }

                Intcode::AdjustRelativeBase(ref value_mode) => {
                    let value = self.read_value(index + 1, value_mode)?;

                    if self.debug_flags.print_instructions {
                        println!(
                            "{}\t({})\t--\t{}({})",
                            intcode,
                            memory_value,
                            index + 1,
                            value,
                        );
                    }

                    self.relative_base += value;

                    if self.debug_flags.print_relative_base {
                        dbg!(self.relative_base);
                    }

                    index += 2;
                }

                Intcode::Halt => {
                    if self.debug_flags.print_instructions {
                        println!("{}\t({})", intcode, memory_value,);
                    }

                    break;
                }
                Intcode::Skip => {
                    if self.debug_flags.print_instructions {
                        println!("{}\t({})", intcode, memory_value,);
                    }

                    index += 1
                }
            }
        }

        Ok(self.memory.clone())
    }

    fn get_address(&mut self, index: usize, mode: &ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => self.memory[index],
            ParameterMode::Immediate => index as isize,
            ParameterMode::Relative => self.memory[index] + self.relative_base,
        }
    }

    fn read_value(&mut self, index: usize, mode: &ParameterMode) -> Result<isize, ComputerError> {
        let address = self.get_address(index, mode);

        if address < 0 {
            return Err(ComputerError::NegativeAddress(address));
        }

        let address = address as usize;

        let value = if self.memory.len() < address + 1 {
            self.heap.get(&(address + 1)).unwrap_or(&0)
        } else {
            self.memory
                .get(address)
                .ok_or(ComputerError::IndexNotFound(address))?
        };

        Ok(*value)
    }

    fn write_value(&mut self, address: usize, value: isize) -> Result<(), ComputerError> {
        if self.memory.len() < address + 1 {
            self.heap.insert(address + 1, value);
        } else {
            self.memory[address] = value;
        };

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Intcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Read(ParameterMode),
    Write(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),

    Halt,
    Skip,
}

impl std::fmt::Display for Intcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Intcode::*;

        match self {
            Add(first_mode, second_mode, third_mode) => {
                write!(f, "ADD\t[{}, {}, {}]", first_mode, second_mode, third_mode)
            }

            Mul(first_mode, second_mode, third_mode) => {
                write!(f, "MUL\t[{}, {}, {}]", first_mode, second_mode, third_mode)
            }

            Read(mode) => write!(f, "READ\t[{}]        ", mode),

            Write(mode) => write!(f, "WRITE\t[{}]        ", mode),

            JumpIfTrue(first_mode, second_mode) => {
                write!(f, "JMPT\t[{}, {}]    ", first_mode, second_mode)
            }

            JumpIfFalse(first_mode, second_mode) => {
                write!(f, "JMPF\t[{}, {}]    ", first_mode, second_mode)
            }

            LessThan(first_mode, second_mode, third_mode) => {
                write!(f, "LESS\t[{}, {}, {}]", first_mode, second_mode, third_mode)
            }

            Equals(first_mode, second_mode, third_mode) => write!(
                f,
                "EQUAL\t[{}, {}, {}]",
                first_mode, second_mode, third_mode
            ),

            AdjustRelativeBase(mode) => write!(f, "ADJREL\t[{}]        ", mode),

            Halt => write!(f, "HALT\t          "),

            Skip => write!(f, "SKIP\t          "),
        }
    }
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

        let mut digits = digits_reverse(value);
        let optcode = digits.next().unwrap_or_default() + digits.next().unwrap_or_default() * 10;

        let first_mode = digits.next().unwrap_or_default().into();
        let second_mode = digits.next().unwrap_or_default().into();
        let third_mode: ParameterMode = digits.next().unwrap_or_default().into();

        match optcode {
            1 => Add(first_mode, second_mode, third_mode),
            2 => Mul(first_mode, second_mode, third_mode),
            3 => Read(first_mode),
            4 => Write(first_mode),
            5 => JumpIfTrue(first_mode, second_mode),
            6 => JumpIfFalse(first_mode, second_mode),
            7 => LessThan(first_mode, second_mode, third_mode),
            8 => Equals(first_mode, second_mode, third_mode),
            9 => AdjustRelativeBase(first_mode),
            99 => Halt,
            _ => Skip,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl std::fmt::Display for ParameterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParameterMode::*;

        match self {
            Position => write!(f, "p"),
            Immediate => write!(f, "i"),
            Relative => write!(f, "r"),
        }
    }
}

impl From<u8> for ParameterMode {
    fn from(val: u8) -> Self {
        use ParameterMode::*;

        match val {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => unreachable!(),
        }
    }
}

pub fn digits_reverse(mut password: usize) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || match password {
        0 => None,
        _ => {
            let digit = password % 10;
            password /= 10;
            Some(digit as _)
        }
    })
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{
        Computer,
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
            ..Computer::default()
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

        assert_eq!(Add(Position, Position, Position), 1.into());
        assert_eq!(Add(Relative, Position, Position), 201.into());
        assert_eq!(Add(Position, Relative, Position), 2001.into());
        assert_eq!(Add(Position, Relative, Position), 2001.into());
        assert_eq!(Add(Position, Position, Relative), 20001.into());
        assert_eq!(Mul(Position, Position, Position), 2.into());
        assert_eq!(Read(Position), 3.into());
        assert_eq!(Write(Position), 4.into());
        assert_eq!(Halt, 99.into());
        assert_eq!(Skip, 50.into());

        assert_eq!(Write(Immediate), 104.into());
    }

    #[test]
    fn intcodes_with_modes_day_05_example_01() {
        use super::ParameterMode::*;

        let input = 1002;
        let expected = super::Intcode::Mul(Position, Immediate, Position);

        let got: super::Intcode = (&input).into();

        assert_eq!(expected, got);
    }

    #[test]
    fn intcodes_with_modes_day_05_add() {
        use super::{
            Intcode::*,
            ParameterMode::*,
        };

        assert_eq!(Add(Position, Position, Position), 1.into());
        assert_eq!(Skip, 11.into());
        assert_eq!(Add(Immediate, Position, Position), 101.into());
        assert_eq!(Add(Position, Immediate, Position), 1001.into());
        assert_eq!(Add(Immediate, Immediate, Position), 1101.into());
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
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
            ..Computer::default()
        };

        computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_output.values(), got_output.values());
    }

    #[test]
    fn computer_run_day_09_example01() {
        let input_memory = vec![109, 19, 204, -34, 99];
        let expected = vec![42];

        let mut computer = Computer::default()
            .with_memory(input_memory)
            .with_relative_base(2000)
            .print_relative_base()
            .print_output()
            .print_instructions();

        computer.write_value(1985, 42).unwrap();
        computer.run().unwrap();

        let got = computer.output.values();

        assert_eq!(expected, got)
    }

    #[test]
    fn computer_run_day_09_example02() {
        let input_memory = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let expected_output = input_memory.clone();

        let mut computer = Computer::default()
            .with_memory(input_memory)
            .print_relative_base()
            .print_output()
            .print_instructions();

        computer.run().unwrap();
        let got_output = computer.output.values();

        assert_eq!(expected_output, got_output);
    }

    #[test]
    fn computer_run_day_09_example03() {
        let input_memory = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let expected_output = vec![1219070632396864];

        let mut computer = Computer::default()
            .with_memory(input_memory)
            .print_relative_base()
            .print_output()
            .print_instructions();

        computer.run().unwrap();
        let got_output = computer.output.values();

        assert_eq!(expected_output, got_output);
    }

    #[test]
    fn computer_run_day_09_example04() {
        let input_memory = vec![104, 1125899906842624, 99];
        let expected_output = vec![1125899906842624];

        let mut computer = Computer::default()
            .with_memory(input_memory)
            .print_relative_base()
            .print_output()
            .print_instructions();

        computer.run().unwrap();
        let got_output = computer.output.values();

        assert_eq!(expected_output, got_output);
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
