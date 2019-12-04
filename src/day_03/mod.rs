//! Solutions for Advent of Code 2019 Day 03

pub mod canvas;

use std::{
    collections::BTreeSet,
    convert::TryInto,
};

pub type Distance = usize;
pub type Steps = usize;

pub const STARTING_POINT_X: isize = 0;
pub const STARTING_POINT_Y: isize = 0;

pub const INPUT1: &str =
    "R1005,U370,L335,D670,R236,D634,L914,U15,R292,D695,L345,D183,R655,U438,R203,U551,L540,U51,\
     R834,D563,L882,D605,L832,U663,R899,D775,L740,U764,L810,U442,R379,D951,L821,D703,R526,D624,\
     L100,D796,R375,U129,L957,D41,R361,D504,R358,D320,L392,D842,R509,D612,L92,U788,L361,D757,R428,\
     U257,L663,U956,L748,U938,R588,D942,R819,D732,R562,D331,L164,U801,R872,U872,L909,U260,R899,\
     D278,R822,U968,L937,D594,L786,D34,R102,D650,R920,D539,R925,U436,R347,U686,L596,D608,R730,U5,\
     R462,U831,R277,U411,R730,D828,L169,D276,L669,U167,R55,D879,L329,U258,R585,D134,R977,D609,\
     L126,U848,L601,U624,R577,D421,L880,D488,R505,U385,L103,D693,L110,D338,R809,D864,L80,U413,\
     R412,D134,L519,D988,R83,U580,R593,U435,R843,D953,R11,D655,R569,D237,R987,U894,L445,U974,L746,\
     U450,R99,U69,R84,U258,L248,D581,R215,U306,R480,U126,R275,D353,R493,D800,L386,D876,L957,D722,\
     L967,D612,L716,D901,R394,U764,R274,D686,L746,D957,R747,U517,L575,D961,R842,D753,L345,D59,\
     L215,U413,R610,D166,L646,U107,L926,D848,R445,U297,L376,U869,L345,D529,R620,D353,R682,D908,\
     R378,D221,R64,D911,L245,D364,R123,D555,L928,U412,R771,D543,L97,D477,R500,D125,R578,U150,R291,\
     D252,R948,D576,L838,D144,L289,D677,L307,U692,R802,D743,R57,U839,R896,D110,R34,D508,L595,U658,\
     L769,U47,L292,U66,R217,D8,L835,D479,L71,D24,R429,U64,R305,D406,R23,U819,R478,D7,L561,D503,\
     R349,U104,L749,D123,R548,D421,R336,D837,R464,D908,L94,U988,L137,D757,L42,U842,R260,D406,L31,\
     U965,L178,U973,L29,U276,L887,U920,L133,U243,R537,U282,R194,D152,R693,D509,L771,D365,L319,\
     D378,L61,D849,R379";

pub const INPUT2: &str =
    "L998,U242,R333,U631,L507,U313,R286,U714,R709,U585,R393,D893,R404,D448,R882,U246,L190,U238,\
     R672,D184,L275,D120,R352,D584,L626,U413,L288,D942,R770,D551,L926,D242,R568,U48,R108,D349,\
     R750,D323,L529,D703,L672,U775,L700,D465,L528,D596,R990,U366,L747,D270,L723,D469,L548,D47,\
     L873,D678,R782,D187,L397,U975,R967,D224,L295,D86,L159,U610,L767,U641,L885,D623,L160,D509,\
     R517,D981,L376,D604,R251,D140,L938,D358,L984,U63,R513,D54,L718,U90,L343,D982,L575,D692,L508,\
     D361,L297,D880,L46,D875,R40,D97,R819,U919,R319,U152,R161,U553,L388,D100,R481,U306,L201,U706,\
     L173,D657,L632,D182,R477,D332,R678,D683,L983,D584,R941,U801,R485,D376,R218,D432,R780,D617,\
     R560,D618,R466,U456,L952,D72,R339,U16,L543,U176,L423,D770,L714,U621,L850,U929,R132,D908,R993,\
     U440,R539,U374,L945,D443,L326,D651,L269,U321,R925,D777,R431,U273,R811,D63,R683,D540,L3,D617,\
     R359,U332,L736,D98,L859,D994,R131,U71,L156,D661,R879,D303,L581,U407,L166,U878,L831,D871,R953,\
     D137,L903,U200,R34,D857,R448,D412,L311,D212,R527,D707,R641,D775,L987,D814,L38,D96,R647,U868,\
     L98,U882,L838,D308,R840,U161,R83,U424,L420,U934,R353,D287,R559,D665,R695,D888,R859,U992,L283,\
     D525,L449,U255,L889,D296,R72,D899,R316,D3,L308,D404,L356,D333,R645,U274,R336,U258,R599,U746,\
     L142,U21,R301,D890,L290,D624,R565,U117,L927,U412,L687,U480,R674,U372,L382,D134,L372,D892,\
     R307,U217,L20,D535,L876,D548,L19,U590,R906,D816,R465,U768,R882,U980,L557,D788,R645,U684,L255,\
     D803,L374,U759,L693,D92,L256,U772,R591,D126,R57,U363,R347,U191,L760,U223,R591,D507,R232,U251,\
     R471,D912,R227";

pub fn closest_intersection() -> (Distance, Steps) {
    let first = parse_directions(INPUT1).unwrap().walk();
    let second = parse_directions(INPUT2).unwrap().walk();

    let start_point = Position {
        x: STARTING_POINT_X,
        y: STARTING_POINT_Y,
    };

    let intersections = first.intersection(&second);
    let closest = intersections.closest(start_point).unwrap().1;
    let distance = closest.distance(start_point);

    let (steps, _) = intersections
        .0
        .iter()
        .map(|intersection| {
            (
                // + 2 for both missing the final step
                first.steps_to(intersection).len() + second.steps_to(intersection).len() + 2,
                intersection,
            )
        })
        .min()
        .unwrap();

    (distance, steps)
}

#[allow(dead_code)]
pub fn closest_intersection_draw() -> (Distance, Steps) {
    let first = parse_directions(INPUT1).unwrap().walk();
    let second = parse_directions(INPUT2).unwrap().walk();

    let mut canvas = canvas::Canvas::default();

    let start_point = Position {
        x: STARTING_POINT_X,
        y: STARTING_POINT_Y,
    };

    canvas.add_start_point(&start_point, canvas::WHITE);
    canvas.add_positions(&first, canvas::GREEN);
    canvas.add_positions(&second, canvas::BLUE);

    let intersections = first.intersection(&second);
    canvas.add_intersections(&intersections, canvas::YELLOW);

    let closest = intersections.closest(start_point).unwrap().1;
    canvas.add_closest_intersection(&closest, canvas::RED);

    let distance = closest.distance(start_point);

    let (steps, intersection) = intersections
        .0
        .iter()
        .map(|intersection| {
            (
                // + 2 for both missing the final step
                first.steps_to(intersection).len() + second.steps_to(intersection).len() + 2,
                intersection,
            )
        })
        .min()
        .unwrap();

    canvas.add_closest_intersection(&intersection, canvas::MAGENTA);

    canvas.run();

    (distance, steps)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let direction = chars.next().unwrap();

        let numbers_raw = chars.fold(String::new(), |mut acc, x| {
            acc.push(x);
            acc
        });

        let numbers = numbers_raw.parse().unwrap();

        use Direction::*;
        Ok(match direction {
            'R' => Right(numbers),
            'L' => Left(numbers),
            'U' => Up(numbers),
            'D' => Down(numbers),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn distance(&self, other: Self) -> Distance {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}

fn parse_directions(input: &str) -> Result<Directions, String> {
    Ok(Directions(
        input
            .split(',')
            .map(|split| split.parse())
            .collect::<Result<Vec<Direction>, _>>()?,
    ))
}

impl std::ops::Add<Direction> for Position {
    type Output = Vec<Self>;

    fn add(self, other: Direction) -> Vec<Self> {
        use Direction::*;

        let (length, dir_x, dir_y) = match other {
            Up(length) => (length, 0, 1),
            Down(length) => (length, 0, -1),
            Left(length) => (length, -1, 0),
            Right(length) => (length, 1, 0),
        };

        let mut out = Vec::new();
        let mut current_position = self;

        for _ in 0..length {
            let new_position = Position {
                x: current_position.x + dir_x,
                y: current_position.y + dir_y,
            };

            out.push(new_position);
            current_position = new_position;
        }

        out
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Directions(Vec<Direction>);

#[derive(Debug)]
pub struct Positions(Vec<Position>);

impl Into<Vec<(f32, f32)>> for &Positions {
    fn into(self) -> Vec<(f32, f32)> {
        self.0
            .iter()
            .map(|position| (position.x as f32, position.y as f32))
            .collect()
    }
}

impl Directions {
    fn walk(self) -> Positions {
        let start = Position {
            x: STARTING_POINT_X,
            y: STARTING_POINT_Y,
        };

        let mut directions_iter = self.0.into_iter();

        let out = std::iter::successors(Some((Vec::new(), start)), |(_, current_position)| {
            directions_iter.next().and_then(|direction| {
                let steps = *current_position + direction;
                let new_position = *steps.iter().last().unwrap();

                Some((steps, new_position))
            })
        })
        .map(|(steps, _)| steps)
        .flatten()
        .collect::<Vec<_>>();

        Positions(out)
    }
}

impl Positions {
    #[allow(dead_code)]
    fn last(self) -> Option<Position> {
        self.0.into_iter().last()
    }

    fn intersection(&self, other: &Self) -> Positions {
        let first_set = self.0.iter().collect::<BTreeSet<_>>();

        Positions(
            other
                .0
                .clone()
                .into_iter()
                .filter(|position| *position != Position { x: 0, y: 0 })
                .filter(|postion| first_set.contains(postion))
                .collect(),
        )
    }

    fn closest(&self, other: Position) -> Option<(Distance, &Position)> {
        let distances: Vec<_> = self
            .0
            .iter()
            .map(|position| (position.distance(other), position))
            .collect();

        distances.into_iter().min()
    }

    fn steps_to(&self, other: &Position) -> Self {
        Self(
            self.0
                .clone()
                .into_iter()
                .take_while(|position| position != other)
                .collect(),
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        parse_directions,
        Direction::*,
        Directions,
        Position,
        STARTING_POINT_X,
        STARTING_POINT_Y,
    };

    #[test]
    fn manhatten_distance_example_text() {
        let start = Position {
            x: STARTING_POINT_X,
            y: STARTING_POINT_Y,
        };

        let input = Position { x: 3, y: 3 };
        let expected = 6;
        let got = input.distance(start);

        assert_eq!(expected, got);
    }

    #[test]
    fn walk_example_text() {
        let input = Directions(vec![Right(8), Up(5), Left(5), Down(3)]);
        let expected = Some(Position { x: 3, y: 2 });
        let got = input.walk().last();

        assert_eq!(expected, got);
    }

    #[test]
    fn walk_example_text_2() {
        let input = Directions(vec![Up(7), Right(6), Down(4), Left(4)]);
        let expected = Some(Position { x: 2, y: 3 });
        let got = input.walk().last();

        assert_eq!(expected, got);
    }

    #[test]
    fn example01() {
        let start = Position {
            x: STARTING_POINT_X,
            y: STARTING_POINT_Y,
        };

        let first = Directions(vec![
            Right(75),
            Down(30),
            Right(83),
            Up(83),
            Left(12),
            Down(49),
            Right(71),
            Up(7),
            Left(72),
        ])
        .walk();

        let second = Directions(vec![
            Up(62),
            Right(66),
            Up(55),
            Right(34),
            Down(71),
            Right(55),
            Down(58),
            Right(83),
        ])
        .walk();

        let expected = 159;
        let got = first.intersection(&second).closest(start).unwrap().0;

        assert_eq!(expected, got);
    }

    #[test]
    fn example02() {
        let start = Position {
            x: STARTING_POINT_X,
            y: STARTING_POINT_Y,
        };

        let first = Directions(vec![
            Right(98),
            Up(47),
            Right(26),
            Down(63),
            Right(33),
            Up(87),
            Left(62),
            Down(20),
            Right(33),
            Up(53),
            Right(51),
        ])
        .walk();

        let second = Directions(vec![
            Up(98),
            Right(91),
            Down(20),
            Right(16),
            Down(67),
            Right(40),
            Up(7),
            Right(15),
            Up(6),
            Right(7),
        ])
        .walk();

        let expected = 135;
        let got = first.intersection(&second).closest(start).unwrap().0;

        assert_eq!(expected, got);
    }

    #[test]
    fn parse_directions_text() {
        let input = "R8,U5,L5,D3";
        let expected = Directions(vec![Right(8), Up(5), Left(5), Down(3)]);
        let got = parse_directions(input).unwrap();

        assert_eq!(expected, got);
    }
}
