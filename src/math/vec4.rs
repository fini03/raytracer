use super::Vec3;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,
};

#[derive(Debug, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }

    pub fn from_vec3(vec: &Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: 0.,
        }
    }

    pub fn from_values(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(&self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
            + self.w * other.w
    }

    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2. * self.dot(normal) * normal
    }

    pub fn assign_min(&mut self, other: &Self) {
        self.x = self.x.min(other.x);
        self.y = self.y.min(other.y);
        self.z = self.z.min(other.z);
        self.w = self.w.min(other.w);
    }

    pub fn assign_max(&mut self, other: &Self) {
        self.x = self.x.max(other.x);
        self.y = self.y.max(other.y);
        self.z = self.z.max(other.z);
        self.w = self.w.max(other.w);
    }

    pub fn quat_mult(&self, other: &Self) -> Self {
        let q1_yzw = Vec3::from_values(self.y, self.z, self.w);
        let q2_yzw = Vec3::from_values(other.y, other.z, other.w);
        let x = self.x * other.x - q1_yzw.dot(&q2_yzw);
        let yzw = self.x * &q2_yzw
            + other.x * &q1_yzw
            + q1_yzw.cross(&q2_yzw);

        Self {
            x,
            y: yzw.x,
            z: yzw.y,
            w: yzw.z,
        }
    }

    pub fn quat_sq(&self) -> Self {
        let q_yzw = Vec3::from_values(self.y, self.z, self.w);
        let x = self.x * self.x - q_yzw.dot(&q_yzw);
        let yzw = 2. * self.x * q_yzw;

        Self {
            x,
            y: yzw.x,
            z: yzw.y,
            w: yzw.z,
        }
    }
}

impl Display for Vec4 {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: 0. - self.x,
            y: 0. - self.y,
            z: 0. - self.z,
            w: 0. - self.w,
        }
    }
}

impl Neg for &Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: 0. - self.x,
            y: 0. - self.y,
            z: 0. - self.z,
            w: 0. - self.w,
        }
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl AddAssign<&Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl MulAssign<&f32> for Vec4 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl DivAssign<&f32> for Vec4 {
    fn div_assign(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<&Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn mul(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: &Vec4) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<Vec4> for &Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<&Vec4> for &f32 {
    type Output = Vec4;

    fn mul(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
    }
}

impl Mul<&Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
    }
}

impl Mul<Vec4> for &f32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
    }
}

impl Mul<&f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<&f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<&Vec4> for &f32 {
    type Output = Vec4;

    fn div(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
            w: self / other.w,
        }
    }
}

impl Div<&Vec4> for f32 {
    type Output = Vec4;

    fn div(self, other: &Vec4) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
            w: self / other.w,
        }
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
            w: self / other.w,
        }
    }
}

impl Div<Vec4> for &f32 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Self::Output {
        Self::Output {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
            w: self / other.w,
        }
    }
}

impl Div<&f32> for &Vec4 {
    type Output = Vec4;

    fn div(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Div<&f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: &f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Div<f32> for &Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}
