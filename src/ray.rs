use crate::types::{Vec3, Point};


#[derive(Debug)]
/// Vec3 representation
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3
}

impl Ray{


    pub fn at(&self, t: f32) -> Point{
        &self.orig + &(&self.dir * t)
    }
}



#[cfg(test)]
pub(crate) mod test {
    use crate::ray::Ray;
    use crate::types::Vec3;
    use crate::types::Point;

    #[test]
    fn test_projection() {
        let r = Ray {
            orig: Point{x: 0.0, y: 0.0, z: 0.0},
            dir: Vec3{x: 1.0, y: 1.0, z: 1.0},
        };

        let projected = r.at(5.0);
        let expected = Vec3 { x: 5.0, y: 5.0, z: 5.0 };
        assert_eq!(expected, projected);
    }
}