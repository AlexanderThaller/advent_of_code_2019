pub struct Computer {
    memory: Vec<usize>,
}

impl From<Vec<usize>> for Computer {
    fn from(memory: Vec<usize>) -> Self {
        Self { memory }
    }
}

#[derive(Debug)]
pub enum ComputerError {
    IndexNotFound(usize),
}

impl Computer {
    pub fn run(self) -> Result<Vec<usize>, ComputerError> {
        use ComputerError::*;

        let mut memory = self.memory;

        let mut index = 0;
        loop {
            let memory_value = memory.get(index).ok_or(IndexNotFound(index))?;
            let intcode = memory_value.into();

            match intcode {
                Intcode::Add | Intcode::Mul => {
                    let first_address = memory[index + 1];
                    let second_address = memory[index + 2];
                    let result_address = memory[index + 3];

                    let first_value = memory.get(first_address).ok_or(IndexNotFound(index))?;
                    let second_value = memory.get(second_address).ok_or(IndexNotFound(index))?;

                    let result_value = match intcode {
                        Intcode::Add => first_value + second_value,
                        Intcode::Mul => first_value * second_value,
                        _ => unreachable!(),
                    };

                    index += 4;

                    memory[result_address] = result_value;
                }
                Intcode::Halt => break,
                Intcode::Unkown => index += 1,
            }
        }

        Ok(memory)
    }
}

enum Intcode {
    Add,
    Mul,
    Halt,
    Unkown,
}

impl From<&usize> for Intcode {
    fn from(val: &usize) -> Self {
        use Intcode::*;
        match val {
            1 => Add,
            2 => Mul,
            99 => Halt,
            _ => Unkown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Computer;
    use pretty_assertions::assert_eq;

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
}
