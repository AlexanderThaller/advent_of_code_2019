pub mod angles;
pub mod canvas;
pub mod ray;

use angles::Angles;
use ray::Ray;
use std::{
    collections::BTreeMap,
    f64::consts::PI,
};

impl Field {
    pub fn find_best_monitoring_location(&self) -> (Position, usize) {
        let asteroids = self
            .entries
            .iter()
            .filter(|(_, object)| **object != Object::Space)
            .collect::<BTreeMap<_, _>>();

        let mut canvas = canvas::Canvas::default();

        let (field_w, field_h) = self.get_field_size();
        canvas.add_field(field_w as f32, field_h as f32);

        let mut collision_count: BTreeMap<&Position, usize> = BTreeMap::default();

        for (check_position, _) in &asteroids {
            canvas.add_object(check_position, &Object::Asteroid);

            for (position, _) in &asteroids {
                if check_position == position {
                    continue;
                }

                let angle = check_position.get_angle(position);

                dbg!(angle);

                let mut ray: Ray = (*position).into();

                dbg!(&ray);

                loop {
                    ray += Ray::get_step(angle, ray::RAY_MIN_STEP);

                    match self.check_collision(&ray) {
                        Collision::Object => {
                            println!("collision object {:?}", ray);

                            *collision_count.entry(position).or_insert(0) += 1;
                            break;
                        }
                        Collision::Border => {
                            break;
                        }
                        Collision::None => {}
                    }

                    canvas.add_ray(&ray);
                }
            }
        }

        canvas.run();

        let (position, count) = collision_count.iter().max().unwrap();

        ((*position).clone(), *count)
    }

    fn check_collision(&self, ray: &Ray) -> Collision {
        if self.check_collision_border(ray) {
            return Collision::Border;
        }

        if self.check_collision_objects(ray) {
            return Collision::Object;
        }

        Collision::None
    }

    fn get_field_size(&self) -> (usize, usize) {
        let max = self.entries.keys().max().unwrap();

        (max.x, max.y)
    }

    fn check_collision_border(&self, ray: &Ray) -> bool {
        if ray.x.abs() < std::f64::EPSILON || ray.x < 0.0 as f64 {
            return true;
        }

        if ray.y.abs() < std::f64::EPSILON || ray.y < 0.0 as f64 {
            return true;
        }

        let max = self.entries.keys().max().unwrap();

        if (ray.x - max.x as f64).abs() < std::f64::EPSILON || ray.x > max.x as f64 {
            return true;
        }

        if (ray.y - max.y as f64).abs() < std::f64::EPSILON || ray.y > max.y as f64 {
            return true;
        }

        false
    }

    fn check_collision_objects(&self, ray: &Ray) -> bool {
        self.entries
            .keys()
            .filter(|position| position == ray)
            .count()
            != 0
    }
}

#[derive(Debug)]
pub enum Collision {
    Object,
    Border,
    None,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Field {
    entries: BTreeMap<Position, Object>,
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let mut entries = BTreeMap::default();

        for (y, line) in s.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                entries.insert(Position { x, y }, symbol.into());
            }
        }

        Self { entries }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_line = 0;

        for (position, object) in &self.entries {
            if position.y != last_line {
                writeln!(f)?;
                last_line = position.y;
            }

            write!(f, "{}", object)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

impl PartialEq<Ray> for &Position {
    fn eq(&self, other: &Ray) -> bool {
        (self.x as f64 - other.x as f64).abs() < std::f64::EPSILON
            && (self.y as f64 - other.y as f64).abs() < std::f64::EPSILON
    }
}

impl Position {
    fn get_angle(&self, other: &Position) -> f64 {
        ((other.x as f64 - self.x as f64)
            .atan2(self.y as f64 - other.y as f64)
            .rem_euclid(2.0 * PI))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Object {
    Asteroid,
    Space,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Asteroid => '#',
            Self::Space => '.',
        };

        write!(f, "{}", out)
    }
}

impl From<char> for Object {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Asteroid,
            '.' => Self::Space,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Field,
        Position,
    };
    use pretty_assertions::assert_eq;

    const EXAMPLE_01: &str = ".#..#
.....
#####
....#
...##";

    #[test]
    fn field_from_str_day_10_example_01() {
        let input = EXAMPLE_01;
        let expected = input;

        let field: Field = input.into();
        let got = format!("{}", field);

        assert_eq!(expected, got);
    }

    #[test]
    fn find_best_monitoring_location_day_10_example_01() {
        let input: Field = EXAMPLE_01.into();

        let expected = (Position { x: 3, y: 4 }, 8);
        let got = input.find_best_monitoring_location();

        assert_eq!(expected, got);
    }
}
