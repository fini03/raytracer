use crate::math::Point3;

#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self::new(
            Point3::from_values(
                f32::INFINITY, 
                f32::INFINITY,
                f32::INFINITY
            ),
            Point3::from_values(
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY
            )
        )
    }

    pub fn merge(&mut self, other: &Self) {
        self.min = Point3::from_values(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z)
        );
        self.max = Point3::from_values(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z)
        );
    }

    pub fn surface(&self) -> f32 {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        2. * (dx * dy + dx * dz + dy * dz)
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self::empty()
    }
}
