use std::ops::{
    Neg, AddAssign, MulAssign,
    DivAssign, Add, Sub, Mul, Div
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }

    pub fn from_values(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(&self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x +
            self.y * other.y +
            self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        } 
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2. * self.dot(normal) * normal
    }

    pub fn assign_min(&mut self, other: &Self) {
        self.x = self.x.min(other.x);
        self.y = self.y.min(other.y);
        self.z = self.z.min(other.z);
    }

    pub fn assign_max(&mut self, other: &Self) {
        self.x = self.x.max(other.x);
        self.y = self.y.max(other.y);
        self.z = self.z.max(other.z);
    }
}

impl Display for Vec3 {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // TODO: Subtracting from 0, as negative zero currently
        // breaks our AABB intersection code
        Self {
            x: 0. - self.x,
            y: 0. - self.y,
            z: 0. - self.z
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        // TODO: Subtracting from 0, as negative zero currently
        // breaks our AABB intersection code
        Self::Output {
            x: 0. - self.x,
            y: 0. - self.y,
            z: 0. - self.z
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&f32> for Vec3 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl DivAssign<&f32> for Vec3 {
    fn div_assign(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<&Vec3> for &f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self 
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl Mul<Vec3> for &f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl Mul<&f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<&f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div<&Vec3> for &f32 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z
        }
    }
}

impl Div<&Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z 
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z
        }
    }
}

impl Div<Vec3> for &f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z
        }
    }
}

impl Div<&f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Div<&f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}
