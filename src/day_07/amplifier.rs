use crate::day_07::computer::{
    Computer,
    DebugFlags,
    Reader,
    Writer,
};
use crossbeam_channel::unbounded;
use std::thread;

pub struct AmplifiersLoop {
    phase_settings: Vec<isize>,
    software: Vec<isize>,
}

impl AmplifiersLoop {
    pub fn new(phase_settings: Vec<isize>, software: Vec<isize>) -> Self {
        Self {
            phase_settings,
            software,
        }
    }

    pub fn run(self) -> isize {
        let (sender_a, receiver_a) = unbounded();
        let (sender_b, receiver_b) = unbounded();
        let (sender_c, receiver_c) = unbounded();
        let (sender_d, receiver_d) = unbounded();
        let (sender_e, receiver_e) = unbounded();

        sender_e.send(self.phase_settings[0]).unwrap();
        sender_e.send(0).unwrap();

        sender_a.send(self.phase_settings[1]).unwrap();
        sender_b.send(self.phase_settings[2]).unwrap();
        sender_c.send(self.phase_settings[3]).unwrap();
        sender_d.send(self.phase_settings[4]).unwrap();

        // Amp A
        {
            let software = self.software.clone();
            let receiver_e = receiver_e.clone();
            thread::spawn(move || {
                let input = Reader::Channel(receiver_e);
                let output = Writer::Channel(sender_a);
                Amplifier::new(software, input, output).run();
            });
        }

        // Amp B
        {
            let software = self.software.clone();
            thread::spawn(move || {
                let input = Reader::Channel(receiver_a);
                let output = Writer::Channel(sender_b);
                Amplifier::new(software, input, output).run();
            });
        }

        // Amp C
        {
            let software = self.software.clone();
            thread::spawn(move || {
                let input = Reader::Channel(receiver_b);
                let output = Writer::Channel(sender_c);
                Amplifier::new(software, input, output).run();
            });
        }

        // Amp D
        {
            let software = self.software.clone();
            thread::spawn(move || {
                let input = Reader::Channel(receiver_c);
                let output = Writer::Channel(sender_d);
                Amplifier::new(software, input, output).run();
            });
        }

        let input = Reader::Channel(receiver_d);
        let output = Writer::Channel(sender_e);
        Amplifier::new(self.software, input, output).run();

        receiver_e.recv().unwrap()
    }
}

pub struct Amplifiers {
    phase_settings: Vec<isize>,
    software: Vec<isize>,
}

impl Amplifiers {
    pub fn new(phase_settings: Vec<isize>, software: Vec<isize>) -> Self {
        Self {
            phase_settings,
            software,
        }
    }

    pub fn run(self) -> isize {
        let mut parameter = 0;

        for phase_setting in self.phase_settings {
            let input = Reader::Tester(Box::new(vec![phase_setting, parameter].into_iter()));
            let output = Writer::Tester { values: Vec::new() };

            let result = Amplifier::new(self.software.clone(), input, output).run();

            parameter = result[0];
        }

        parameter
    }
}

pub struct Amplifier {
    computer: Computer,
}

impl Amplifier {
    pub fn run(self) -> Vec<isize> {
        let mut computer = self.computer;
        computer.run().unwrap();

        computer.output.values()
    }

    pub fn new(software: Vec<isize>, input: Reader, output: Writer) -> Self {
        let computer = Computer {
            memory: software,
            input,
            output,
            debug_flags: DebugFlags::default(),
        };

        Self { computer }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Amplifiers,
        AmplifiersLoop,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn amplifiers_day_07_part_01_example_01() {
        let phase_settings = vec![4, 3, 2, 1, 0];

        let software = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let expected = 43210;
        let got = Amplifiers::new(phase_settings, software).run();

        assert_eq!(expected, got);
    }

    #[test]
    fn amplifiers_day_07_part_01_example_02() {
        let phase_settings = vec![0, 1, 2, 3, 4];

        let software = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let expected = 54321;
        let got = Amplifiers::new(phase_settings, software).run();

        assert_eq!(expected, got);
    }

    #[test]
    fn amplifiers_day_07_part_01_example_03() {
        let phase_settings = vec![1, 0, 4, 3, 2];

        let software = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let expected = 65210;
        let got = Amplifiers::new(phase_settings, software).run();

        assert_eq!(expected, got);
    }

    #[test]
    fn amplifiers_loop_day_07_part_02_example_01() {
        let phase_settings = vec![9, 8, 7, 6, 5];

        let software = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let expected = 139_629_729;
        let got = AmplifiersLoop::new(phase_settings, software).run();

        assert_eq!(expected, got);
    }

    #[test]
    fn amplifiers_loop_day_07_part_02_example_02() {
        let phase_settings = vec![9, 7, 8, 5, 6];

        let software = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let expected = 18216;
        let got = AmplifiersLoop::new(phase_settings, software).run();

        assert_eq!(expected, got);
    }
}
