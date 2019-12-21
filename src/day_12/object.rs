use super::{
    parse_values,
    position::Position,
    velocity::Velocity,
    ParseValueError,
};

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Object {
    pub position: Position,
    pub velocity: Velocity,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.position, self.velocity)
    }
}

impl std::str::FromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('<').trim_end_matches('>');
        let position = parse_values(s)?.into();

        Ok(Self {
            position,
            ..Self::default()
        })
    }
}

#[derive(Debug)]
pub enum ParseObjectError {
    ValueError(ParseValueError),
    MissingPos,
    MissingVel,
}

impl From<ParseValueError> for ParseObjectError {
    fn from(err: ParseValueError) -> Self {
        Self::ValueError(err)
    }
}

impl Object {
    pub fn pos_x(mut self, x: isize) -> Self {
        self.position.x(x);
        self
    }

    pub fn pos_y(mut self, y: isize) -> Self {
        self.position.y(y);
        self
    }

    pub fn pos_z(mut self, z: isize) -> Self {
        self.position.z(z);
        self
    }

    pub fn vel_x(mut self, x: isize) -> Self {
        self.velocity.x(x);
        self
    }

    pub fn vel_y(mut self, y: isize) -> Self {
        self.velocity.y(y);
        self
    }

    pub fn vel_z(mut self, z: isize) -> Self {
        self.velocity.z(z);
        self
    }

    pub fn parse_step_output(s: &str) -> Result<Self, ParseObjectError> {
        use regex::Regex;

        let regex = Regex::new(r"^pos=<(.*)>, vel=<(.*)>$").unwrap();
        let caps = regex.captures(s).unwrap();
        let pos = caps.get(1).ok_or(ParseObjectError::MissingPos)?.as_str();
        let vel = caps.get(2).ok_or(ParseObjectError::MissingVel)?.as_str();

        let position = parse_values(pos)?.into();
        let velocity = parse_values(vel)?.into();

        Ok(Self { position, velocity })
    }

    pub fn get_velocity(&self, other: &Object) -> Velocity {
        self.position.calculate_velocity_change(&other.position)
    }

    pub fn add_velocity(&mut self, velocity: Velocity) {
        self.velocity += velocity;
    }

    pub fn apply_velocity(&mut self) {
        self.position = Position {
            x: self.position.x + self.velocity.x,
            y: self.position.y + self.velocity.y,
            z: self.position.z + self.velocity.z,
        }
    }

    pub fn calculate_energy(self) -> isize {
        self.position.calculate_energy() * self.velocity.calculate_energy()
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_step_output() {
        let input = "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>";

        let expected = Object::default()
            .pos_x(2)
            .pos_y(-1)
            .pos_z(1)
            .vel_x(3)
            .vel_y(-1)
            .vel_z(-1);

        let got = Object::parse_step_output(input).unwrap();

        assert_eq!(expected, got);
    }
}
