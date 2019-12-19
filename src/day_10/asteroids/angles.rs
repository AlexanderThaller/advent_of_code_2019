const MIN_STEP_ANGLE: f64 = 0.5;
const MAX_ANGLE: f64 = 360.0;

#[derive(Default)]
pub struct Angles {
    current: f64,
}

impl Iterator for Angles {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current - MAX_ANGLE).abs() < std::f64::EPSILON {
            return None;
        }

        self.current += MIN_STEP_ANGLE;

        Some(self.current)
    }
}
