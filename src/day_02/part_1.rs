//! Solutions for Advent of Code 2019 Day 02 Part 1
pub const INPUT: &[usize] = &[
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 5, 19, 23, 2, 10, 23, 27, 1,
    27, 5, 31, 2, 9, 31, 35, 1, 35, 5, 39, 2, 6, 39, 43, 1, 43, 5, 47, 2, 47, 10, 51, 2, 51, 6, 55,
    1, 5, 55, 59, 2, 10, 59, 63, 1, 63, 6, 67, 2, 67, 6, 71, 1, 71, 5, 75, 1, 13, 75, 79, 1, 6, 79,
    83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 10, 91, 95, 1, 95, 9, 99, 2, 99, 13, 103, 1, 103, 6, 107,
    2, 107, 6, 111, 1, 111, 2, 115, 1, 115, 13, 0, 99, 2, 0, 14, 0,
];

pub fn restore_gravity_assist_program() -> usize {
    let mut input = INPUT.to_vec();

    input[1] = 12;
    input[2] = 2;

    let program = parse_intcodes(&input).unwrap();
    let restored = program.run();

    restored.values[0]
}

/// Represents a program made up of Intcodes.
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Program {
    instructions: Vec<IntCode>,
    pub values: Vec<Value>,
}

/// Represents an address inside a Intcode program.
pub type Address = usize;

/// Represents a value inside a Intcode program.
pub type Value = usize;

/// Flag if the program has halted or not.
pub type Halted = bool;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum IntCode {
    /// Add together first and second value and write result to address.
    Add {
        /// The address of the first value that should be added.
        first: Address,

        /// The address of the second value that should be added.
        second: Address,

        /// The address in which the result will be stored.
        result: Address,
    },

    /// Multiplicate together first and second value and write result to
    /// address.
    Mul {
        first: Address,
        second: Address,
        result: Address,
    },

    /// End program
    Halt,
}

pub fn parse_intcodes(values: &[usize]) -> Result<Program, String> {
    use crate::day_02::part_1::IntCode::*;

    let mut instructions = Vec::new();

    let mut skip = 0;
    let mut is_halt = false;

    for (index, value) in values.iter().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        let opt = match value {
            1 => {
                skip = 3;

                Add {
                    first: values[index + 1],
                    second: values[index + 2],
                    result: values[index + 3],
                }
            }

            2 => {
                skip = 3;

                Mul {
                    first: values[index + 1],
                    second: values[index + 2],
                    result: values[index + 3],
                }
            }

            99 => {
                is_halt = true;
                Halt
            }

            _ => continue,
        };

        instructions.push(opt);
        if is_halt {
            break;
        }
    }

    Ok(Program {
        instructions,
        values: values.to_vec(),
    })
}

impl Program {
    pub fn run(self) -> Self {
        std::iter::successors(Some(self.step()), |x| {
            if x.1 {
                return None;
            }

            Some(x.0.step())
        })
        .last()
        .unwrap()
        .0
    }

    fn step(&self) -> (Program, Halted) {
        use crate::day_02::part_1::IntCode::*;

        let mut values = self.values.clone();
        let mut halted = false;

        for opt in &self.instructions {
            match opt {
                Add {
                    first,
                    second,
                    result,
                } => {
                    let first_val = values
                        .get(*first)
                        .expect("can not fetch first_val for addition");

                    let second_val = values
                        .get(*second)
                        .expect("can not fetch first_val for addition");

                    let result_val = first_val + second_val;

                    values[*result] = result_val;
                }
                Mul {
                    first,
                    second,
                    result,
                } => {
                    let first_val = values
                        .get(*first)
                        .expect("can not fetch first_val for addition");

                    let second_val = values
                        .get(*second)
                        .expect("can not fetch first_val for addition");

                    let result_val = first_val * second_val;

                    values[*result] = result_val;
                }

                Halt => {
                    halted = true;
                    break;
                }
            }
        }

        (
            parse_intcodes(&values).expect("can not parse new instructions"),
            halted,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IntCode::*,
        Program,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn parse_intcodes_text_example() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Add {
                    first: 9,
                    second: 10,
                    result: 3,
                },
                Mul {
                    first: 3,
                    second: 11,
                    result: 0,
                },
                Halt,
            ],
            values: input,
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_intcodes_example01() {
        let input = vec![1, 0, 0, 0, 99];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Add {
                    first: 0,
                    second: 0,
                    result: 0,
                },
                Halt,
            ],
            values: input.clone(),
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_intcodes_example02() {
        let input = vec![2, 3, 0, 3, 99];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Mul {
                    first: 3,
                    second: 0,
                    result: 3,
                },
                Halt,
            ],
            values: input,
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_intcodes_example03() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Mul {
                    first: 4,
                    second: 4,
                    result: 5,
                },
                Halt,
            ],
            values: input,
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_intcodes_example04_in() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Add {
                    first: 1,
                    second: 1,
                    result: 4,
                },
                Halt,
            ],
            values: input,
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_intcodes_example04_out() {
        let input = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let got = super::parse_intcodes(&input).unwrap();

        let expected = Program {
            instructions: vec![
                Add {
                    first: 1,
                    second: 4,
                    result: 2,
                },
                Halt,
            ],
            values: input,
        };

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_text_example() {
        let input_raw = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected_raw = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_example01() {
        let input_raw = vec![1, 0, 0, 0, 99];
        let expected_raw = vec![2, 0, 0, 0, 99];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_example02() {
        let input_raw = vec![2, 3, 0, 3, 99];
        let expected_raw = vec![2, 3, 0, 6, 99];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_example03() {
        let input_raw = vec![2, 4, 4, 5, 99, 0];
        let expected_raw = vec![2, 4, 4, 5, 99, 9801];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_example04_01() {
        let input_raw = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected_raw = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_step_example04_02() {
        let input_raw = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];
        let expected_raw = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = (super::parse_intcodes(&expected_raw).unwrap(), true);

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn program_run_example04() {
        let input_raw = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected_raw = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let input = super::parse_intcodes(&input_raw).unwrap();
        let expected = super::parse_intcodes(&expected_raw).unwrap();

        let got = input.run();

        assert_eq!(expected, got);
    }
}
