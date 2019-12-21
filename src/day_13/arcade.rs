use super::computer::{
    Computer,
    Reader,
    Writer,
};
use crossbeam_channel::bounded;
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        Mutex,
    },
    thread,
};

#[derive(Debug, Default)]
pub struct Arcade {
    pub tiles: BTreeMap<Position, Tile>,
    quarters: isize,
    draw: bool,
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
    pub fn with_quarters(self, quarters: isize) -> Self {
        Self { quarters, ..self }
    }

    pub fn set_draw(self) -> Self {
        Self { draw: true, ..self }
    }

    pub fn run(&mut self) -> isize {
        let (sender_output, receiver_output) = bounded(0);

        let quarters = self.quarters;
        let current_ball = Arc::new(Mutex::new(Position::default()));
        let current_paddle = Arc::new(Mutex::new(Position::default()));

        {
            let current_ball = Arc::clone(&current_ball);
            let current_paddle = Arc::clone(&current_paddle);

            thread::spawn(move || {
                let bot = Bot {
                    current_ball,
                    current_paddle,
                };

                let input = Reader::Tester(Box::new(bot));
                let output = Writer::Channel(sender_output);

                let mut software = include_str!("arcade_software.txt")
                    .split(',')
                    .map(|split| split.trim().parse())
                    .collect::<Result<Vec<isize>, std::num::ParseIntError>>()
                    .unwrap();

                software[0] = quarters;

                let mut computer = Computer::default()
                    .with_software(software)
                    .with_input(input)
                    .with_output(output);

                computer.run().unwrap();
            });
        }

        let mut draw = false;
        let mut score = 0;

        loop {
            let x = receiver_output.recv();
            let y = receiver_output.recv();
            let score_tile = receiver_output.recv();

            if x.is_err() || y.is_err() || score_tile.is_err() {
                break;
            }

            let x = x.unwrap();
            let y = y.unwrap();
            let score_tile = score_tile.unwrap();

            if x == -1 && y == 0 {
                score = score_tile;
                draw = true;
            } else {
                let tile = score_tile.into();

                if tile == Tile::Ball {
                    *current_ball.lock().unwrap() = Position { x, y };
                }

                if tile == Tile::HorizontalPaddle {
                    *current_paddle.lock().unwrap() = Position { x, y };
                }

                self.tiles.insert(Position { x, y }, tile);

                if (tile == Tile::Ball || tile == Tile::HorizontalPaddle) && self.draw && draw {
                    self.draw();
                };
            }
        }

        score
    }

    pub fn draw(&self) {
        let mut canvas = super::canvas::Canvas::default();

        for (position, tile) in &self.tiles {
            canvas.add_tile(position, *tile)
        }

        canvas.run();
    }
}

struct Bot {
    pub current_ball: Arc<Mutex<Position>>,
    pub current_paddle: Arc<Mutex<Position>>,
}

impl Iterator for Bot {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let ball = self.current_ball.lock().unwrap();
        let paddle = self.current_paddle.lock().unwrap();

        let direction = match ball.x.cmp(&paddle.x) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        };

        Some(direction)
    }
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
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
