#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Velocity {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Velocity {
    pub fn x(&mut self, x: isize) {
        self.x = x
    }

    pub fn y(&mut self, y: isize) {
        self.y = y
    }

    pub fn z(&mut self, z: isize) {
        self.z = z
    }
}

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vel=<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl From<(isize, isize, isize)> for Velocity {
    fn from(tuple: (isize, isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl std::ops::AddAssign for Velocity {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Velocity {
    pub fn calculate_energy(self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
