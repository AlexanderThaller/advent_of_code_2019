use super::velocity::Velocity;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Position {
    pub fn x(&mut self, x: isize) {
        self.x = x
    }

    pub fn y(&mut self, y: isize) {
        self.y = y
    }

    pub fn z(&mut self, z: isize) {
        self.z = z
    }

    pub fn calculate_velocity_change(&self, other: &Self) -> Velocity {
        let mut out = Velocity::default();

        let change = |first: isize, second: isize| -> isize {
            let cmp = first.cmp(&second);

            match cmp {
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => 1,
            }
        };

        out.x(change(self.x, other.x));
        out.y(change(self.y, other.y));
        out.z(change(self.z, other.z));

        out
    }

    pub fn calculate_energy(self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos=<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl From<(isize, isize, isize)> for Position {
    fn from(tuple: (isize, isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}
