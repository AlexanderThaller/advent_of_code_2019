use super::Position;

pub const RAY_MIN_STEP: f64 = 0.1;

#[derive(Debug)]
pub struct Ray {
    pub x: f64,
    pub y: f64,
}

impl From<&Position> for Ray {
    fn from(p: &Position) -> Self {
        Self {
            x: p.x as f64,
            y: p.y as f64,
        }
    }
}

impl std::ops::AddAssign for Ray {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ray {
    pub fn get_step(angle: f64, step: f64) -> Self {
        if angle == 0.0 || angle == 360.0 {
            return Self { x: 1.0, y: 0.0 };
        }

        if angle == 90.0 {
            return Self { x: 0.0, y: -1.0 };
        }

        if angle == 180.0 {
            return Self { x: -1.0, y: 0.0 };
        }

        if angle == 270.0 {
            return Self { x: 0.0, y: 1.0 };
        }

        let angle_a = Ray::get_inner_angle(angle);

        let angle_b = 90.0;
        let angle_c = 180.0 - angle_a - angle_b;

        let length_c = step;
        let length_a = length_c / angle_c.sin() * angle_a.sin();
        let length_b = length_c / angle_c.sin() * angle_b.sin();

        let (direction_x, direction_y) = Ray::get_direction(angle);
        let x = length_b * direction_x;
        let y = length_a * direction_y;

        Self { x, y }
    }

    fn get_inner_angle(angle: f64) -> f64 {
        if (angle - 90.0).abs() < std::f64::EPSILON || angle < 90.0 {
            return angle;
        }

        if (angle - 180.0).abs() < std::f64::EPSILON || angle < 180.0 {
            return angle - 90.0;
        }

        if (angle - 270.0).abs() < std::f64::EPSILON || angle < 270.0 {
            return angle - 180.0;
        }

        if (angle - 360.0).abs() < std::f64::EPSILON || angle < 360.0 {
            return angle - 270.0;
        }

        0.0
    }

    fn get_direction(angle: f64) -> (f64, f64) {
        if (angle - 90.0).abs() < std::f64::EPSILON || angle < 90.0 {
            return (1.0, -1.0);
        }

        if (angle - 180.0).abs() < std::f64::EPSILON || angle < 180.0 {
            return (-1.0, -1.0);
        }

        if (angle - 270.0).abs() < std::f64::EPSILON || angle < 270.0 {
            return (-1.0, 1.0);
        }

        if (angle - 360.0).abs() < std::f64::EPSILON || angle < 360.0 {
            return (1.0, 1.0);
        }

        (1.0, -1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn get_step_right() {
        let input = 0.0;

        let expected = Ray { x: 1.0, y: 0.0 };
        let got = Ray::get_step(input, 1.0);

        assert_approx_eq!(expected.x, got.x);
        assert_approx_eq!(expected.y, got.y);
    }

    #[test]
    fn get_step_up() {
        let input = 90.0;

        let expected = Ray { x: 0.0, y: -1.0 };
        let got = Ray::get_step(input, 1.0);

        assert_approx_eq!(expected.x, got.x);
        assert_approx_eq!(expected.y, got.y);
    }

    #[test]
    fn get_step_left() {
        let input = 180.0;

        let expected = Ray { x: -1.0, y: 0.0 };
        let got = Ray::get_step(input, 1.0);

        assert_approx_eq!(expected.x, got.x);
        assert_approx_eq!(expected.y, got.y);
    }

    #[test]
    fn get_step_down() {
        let input = 270.0;

        let expected = Ray { x: 0.0, y: 1.0 };
        let got = Ray::get_step(input, 1.0);

        assert_approx_eq!(expected.x, got.x);
        assert_approx_eq!(expected.y, got.y);
    }

    #[test]
    fn get_step_down_right() {
        let inputs = vec![(300.0, 1.0, 1.0), (315.0, 1.0506439776354592, 1.0)];

        for (input, expected_x, expected_y) in inputs {
            let expected = Ray {
                x: expected_x,
                y: expected_y,
            };
            let got = Ray::get_step(input, 1.0);

            dbg!(input);
            assert_approx_eq!(expected.x, got.x);
            assert_approx_eq!(expected.y, got.y);
        }
    }
}
