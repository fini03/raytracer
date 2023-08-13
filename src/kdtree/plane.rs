use super::AABB;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dimension {
    X,
    Y,
    Z
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub dimension: Dimension,
    pub pos: f32
}

impl Plane {
    pub fn new(dimension: Dimension, pos: f32) -> Self {
        Plane { dimension, pos }
    }

    pub fn new_x(pos: f32) -> Self {
        Plane::new(Dimension::X, pos)
    }

    pub fn new_y(pos: f32) -> Self {
        Plane::new(Dimension::Y, pos)
    }

    pub fn new_z(pos: f32) -> Self {
        Plane::new(Dimension::Z, pos)
    }

    pub fn is_cutting(&self, space: &AABB) -> bool {
        match self.dimension {
            Dimension::X =>
                self.pos > space.min.x && self.pos < space.max.x,
            Dimension::Y =>
                self.pos > space.min.y && self.pos < space.max.y,
            Dimension::Z =>
                self.pos > space.min.z && self.pos < space.max.z
        }
    }
}
