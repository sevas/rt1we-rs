const PI: f32 = std::f32::consts::PI;
const INF: f32 = f32::INFINITY;

pub fn rad2deg(rad: f32) -> f32 {
    (rad * 180.0) / PI
}

pub fn deg2rad(deg: f32) -> f32 {
    (deg * PI) / 180.0
}

#[cfg(test)]
pub(crate) mod test {
    use crate::trig::{deg2rad, rad2deg, PI};

    #[test]
    fn test_deg_rad_conversions() {
        let angle = 30.0;

        let angle_r = deg2rad(angle);
        assert_f32_near!(angle_r, PI / 6.0);

        let angle_d = rad2deg(angle_r);
        assert_f32_near!(angle_d, angle);
    }
}
