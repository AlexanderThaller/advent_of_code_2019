use super::object::{
    Object,
    ParseObjectError,
};

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Field {
    objects: Vec<Object>,
}

impl std::str::FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let objects = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Object>, ParseObjectError>>()?;

        Ok(Self { objects })
    }
}

#[derive(Debug)]
pub enum ParseFieldError {
    ObjectError(ParseObjectError),
}

impl From<ParseObjectError> for ParseFieldError {
    fn from(err: ParseObjectError) -> Self {
        Self::ObjectError(err)
    }
}

impl Field {
    pub fn step(&self) -> Self {
        let mut objects: Vec<Object> = Vec::new();

        for object_check in &self.objects {
            let mut out = *object_check;

            for object in &self.objects {
                let velocity = object_check.get_velocity(object);
                out.add_velocity(velocity);
            }

            out.apply_velocity();

            objects.push(out)
        }

        Self { objects }
    }

    pub fn step_n(mut self, count: usize) -> Self {
        for _ in 0..count {
            self = self.step();
        }

        self
    }

    pub fn calculate_energy(self) -> isize {
        self.objects
            .into_iter()
            .map(|object| object.calculate_energy())
            .sum()
    }

    pub fn objects_x(&self) -> Vec<(isize, isize)> {
        self.objects
            .iter()
            .map(|object| (object.position.x, object.velocity.x))
            .collect()
    }

    pub fn objects_y(&self) -> Vec<(isize, isize)> {
        self.objects
            .iter()
            .map(|object| (object.position.y, object.velocity.y))
            .collect()
    }

    pub fn objects_z(&self) -> Vec<(isize, isize)> {
        self.objects
            .iter()
            .map(|object| (object.position.z, object.velocity.z))
            .collect()
    }

    #[allow(dead_code)]
    fn parse_step_output(s: &str) -> Result<Self, ParseFieldError> {
        let objects = s
            .lines()
            .map(|line| Object::parse_step_output(line.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { objects })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::object::Object,
        Field,
    };
    use pretty_assertions::assert_eq;
    use test::Bencher;

    const INPUT_DAY_12_EXAMPLE_01: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const INPUT_DAY_12_EXAMPLE_02: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn parse_field_day_12_example_01() {
        let input = INPUT_DAY_12_EXAMPLE_01;
        let expected = Field {
            objects: vec![
                Object::default().pos_x(-1).pos_y(0).pos_z(2),
                Object::default().pos_x(2).pos_y(-10).pos_z(-7),
                Object::default().pos_x(4).pos_y(-8).pos_z(8),
                Object::default().pos_x(3).pos_y(5).pos_z(-1),
            ],
        };

        let got: Field = input.parse().unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn field_step_day_12_example_01_step_1() {
        let input: Field = INPUT_DAY_12_EXAMPLE_01.parse().unwrap();

        let expected_raw = "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
        pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
        pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
        pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>";

        let expected: Field = Field::parse_step_output(expected_raw).unwrap();

        let got = input.step();

        assert_eq!(expected, got);
    }

    #[test]
    fn field_step_day_12_example_01_step_2() {
        let input: Field = INPUT_DAY_12_EXAMPLE_01.parse().unwrap();

        let expected_raw = "pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>";

        let expected: Field = Field::parse_step_output(expected_raw).unwrap();

        let got = input.step_n(2);

        assert_eq!(expected, got);
    }

    #[test]
    fn field_step_day_12_example_01_step_10() {
        let input: Field = INPUT_DAY_12_EXAMPLE_01.parse().unwrap();

        let expected_raw = "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>";

        let expected: Field = Field::parse_step_output(expected_raw).unwrap();

        let got = input.step_n(10);

        assert_eq!(expected, got);
    }

    #[test]
    fn field_calculate_energy_day_12_example_01_step_10() {
        let input: Field = INPUT_DAY_12_EXAMPLE_01.parse().unwrap();
        let expected = 179;
        let got = input.step_n(10).calculate_energy();

        assert_eq!(expected, got);
    }

    #[test]
    fn field_step_day_12_example_02_step_100() {
        let input: Field = INPUT_DAY_12_EXAMPLE_02.parse().unwrap();

        let expected_raw = "pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>";

        let expected: Field = Field::parse_step_output(expected_raw).unwrap();

        let got = input.step_n(100);

        assert_eq!(expected, got);
    }

    #[test]
    fn field_calculate_energy_day_12_example_02_step_100() {
        let input: Field = INPUT_DAY_12_EXAMPLE_02.parse().unwrap();
        let expected = 1940;
        let got = input.step_n(100).calculate_energy();

        assert_eq!(expected, got);
    }

    #[bench]
    fn field_calculate_energy_day_12_example_01_step_1(b: &mut Bencher) {
        b.iter(|| {
            let input: Field = INPUT_DAY_12_EXAMPLE_01.parse().unwrap();
            input.step()
        });
    }
}
