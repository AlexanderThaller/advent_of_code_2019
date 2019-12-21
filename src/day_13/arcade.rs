use super::computer::{
    Computer,
    Reader,
    Writer,
};
use crossbeam_channel::unbounded;
use std::{
    collections::BTreeMap,
    thread,
};

#[derive(Debug, Default)]
pub struct Arcade {
    pub tiles: BTreeMap<Position, Tile>,
}

pub enum RunError {
    Receive(crossbeam_channel::RecvError),
}

impl From<crossbeam_channel::RecvError> for RunError {
    fn from(err: crossbeam_channel::RecvError) -> Self {
        Self::Receive(err)
    }
}

impl Arcade {
    pub fn run(&mut self) -> Result<(), RunError> {
        let (sender_output, receiver_output) = unbounded();
        let (sender_input, receiver_input) = unbounded();

        thread::spawn(move || {
            let input = Reader::Channel(receiver_input);
            let output = Writer::Channel(sender_output);

            let software = include_str!("arcade_software.txt")
                .split(',')
                .map(|split| split.trim().parse())
                .collect::<Result<Vec<isize>, std::num::ParseIntError>>()
                .unwrap();

            let mut computer = Computer::default()
                .with_software(software)
                .with_input(input)
                .with_output(output)
                .debug_low();

            computer.run().unwrap();
        });

        loop {
            let x = receiver_output.recv()?;
            let y = receiver_output.recv()?;
            let tile = receiver_output.recv()?.into();

            self.tiles.insert(Position { x, y }, tile);
        }
    }
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<isize> for Tile {
    fn from(i: isize) -> Self {
        match i {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => unreachable!(),
        }
    }
}
