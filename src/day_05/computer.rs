pub struct Computer {
    pub memory: Vec<isize>,
    pub input: Reader,
    pub output: Writer,
}

impl From<Vec<isize>> for Computer {
    fn from(memory: Vec<isize>) -> Self {
        Self {
            memory,
            input: Reader::Disabled,
            output: Writer::Disabled,
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
}

impl Reader {
    fn read(&mut self) -> isize {
        use Reader::*;

        match self {
            Disabled => panic!("trying to use disabled reader"),
            Tester(iter) => iter.next().expect("reader tester ran out of values"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Writer {
    Disabled,
    Tester { values: Vec<isize> },
}

impl Writer {
    fn write(&mut self, value: isize) {
        use Writer::*;

        match self {
            Disabled => panic!("trying to use disabled writer"),

            Tester { values } => {
                values.push(value);
            }
        }
    }
}

impl Computer {
    pub fn run(&mut self) -> Result<Vec<isize>, ComputerError> {
        use ComputerError::*;
        use ParameterMode::*;

        let mut memory = self.memory.clone();

        let mut index = 0;
        loop {
            let memory_value = memory.get(index).ok_or(IndexNotFound(index))?;
            let intcode = memory_value.into();

            match intcode {
                Intcode::Add(ref first_value_mode, ref second_value_mode)
                | Intcode::Mul(ref first_value_mode, ref second_value_mode) => {
                    let first_value_address = match first_value_mode {
                        PositionMode => memory[index + 1] as usize,
                        ImmediateMode => index + 1,
                    };

                    let first_value = memory
                        .get(first_value_address)
                        .ok_or(IndexNotFound(index))?;

                    let second_value_address = match second_value_mode {
                        PositionMode => memory[index + 2] as usize,
                        ImmediateMode => index + 2,
                    };

                    let second_value = memory
                        .get(second_value_address)
                        .ok_or(IndexNotFound(index))?;

                    let result_value = match intcode {
                        Intcode::Add(..) => first_value + second_value,
                        Intcode::Mul(..) => first_value * second_value,
                        _ => unreachable!(),
                    };

                    let result_address = memory[index + 3] as usize;
                    memory[result_address] = result_value;

                    index += 4;
                }
                Intcode::Read | Intcode::Write => {
                    let address = memory[index + 1] as usize;

                    match intcode {
                        Intcode::Read => {
                            let value = self.input.read();
                            memory[address] = value;
                        }
                        Intcode::Write => {
                            let value = memory[address];
                            self.output.write(value);
                        }
                        _ => unreachable!(),
                    }

                    index += 2;
                }
                Intcode::Halt => break,
                Intcode::Skip => index += 1,
            }
        }

        Ok(memory)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Intcode {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Read,
    Write,

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
            4 => Write,
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
        };

        let got_memory = computer.run().unwrap();
        let got_output = computer.output;

        assert_eq!(expected_memory, got_memory);
        assert_eq!(expected_output, got_output);
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
        assert_eq!(Write, 4.into());
        assert_eq!(Halt, 99.into());
        assert_eq!(Skip, 50.into());
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
