use std::default::Default;
use std::ops::{Mul, MulAssign};
use super::Vec3;

#[derive(Debug, Clone)]
pub struct Mat4 {
    pub e: [f32; 16]
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            e: [
                1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Self {
        let forward = (eye - center).unit_vector();
        let right = up.cross(&forward).unit_vector();
        let nup = forward.cross(&right).unit_vector();

        Self {
            e: [
                right.x, nup.x, forward.x, eye.x,
                right.y, nup.y, forward.y, eye.y,
                right.z, nup.z, forward.z, eye.z,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn translate(v: &Vec3) -> Self {
        Self {
            e: [
                1., 0., 0., v.x,
                0., 1., 0., v.y,
                0., 0., 1., v.z,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn scale(v: &Vec3) -> Self {
        Self {
            e: [
                v.x, 0., 0., 0.,
                0., v.y, 0., 0.,
                0., 0., v.z, 0.,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn rotate_x(theta: f32) -> Self {
        let rads = theta.to_radians();
        let s = rads.sin();
        let c = rads.cos();
        Self {
            e: [
                1., 0., 0., 0.,
                0., c, -s, 0.,
                0., s, c, 0.,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn rotate_y(theta: f32) -> Self {
        let rads = theta.to_radians();
        let s = rads.sin();
        let c = rads.cos();
        Self {
            e: [
                c, 0., s, 0.,
                0., 1., 0., 0.,
                -s, 0., c, 0.,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn rotate_z(theta: f32) -> Self {
        let rads = theta.to_radians();
        let s = rads.sin();
        let c = rads.cos();
        Self {
            e: [
                c, -s, 0., 0.,
                s, c, 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            ]
        }
    }

    pub fn transpose(&self) -> Self {
        let e = &self.e;
        Self {
            e: [
                e[0], e[4], e[8], e[12],
                e[1], e[5], e[9], e[13],
                e[2], e[6], e[10], e[14],
                e[3], e[7], e[11], e[15],
            ]
        }
    }

    pub fn mul_dir(&self, v: &Vec3) -> Vec3 {
        let e = &self.e;
        Vec3::from_values(
            v.x * e[0] + v.y * e[1] + v.z * e[2],
            v.x * e[4] + v.y * e[5] + v.z * e[6],
            v.x * e[8] + v.y * e[9] + v.z * e[10]
        )
    }

    pub fn tbn(t: &Vec3, b: &Vec3, n: &Vec3) -> Self {
        Self {
            #[rustfmt::skip]
            e: [
                t.x, b.x, n.x, 0.,
                t.y, b.y, n.y, 0.,
                t.z, b.z, n.z, 0.,
                0., 0., 0., 1.
            ],
        }
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul<&Vec3> for &Mat4 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Self::Output {
        let e = &self.e;
        Vec3::from_values(
            v.x * e[0] + v.y * e[1] + v.z * e[2] + e[3],
            v.x * e[4] + v.y * e[5] + v.z * e[6] + e[7],
            v.x * e[8] + v.y * e[9] + v.z * e[10] + e[11]
        )
    }
}

impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        let l = &self.e;
        let r = &rhs.e;
        Self::Output {
            e: [
                l[0]*r[0] + l[1]*r[4] + l[2]*r[8] + l[3]*r[12],
                l[0]*r[1] + l[1]*r[5] + l[2]*r[9] + l[3]*r[13],
                l[0]*r[2] + l[1]*r[6] + l[2]*r[10] + l[3]*r[14],
                l[0]*r[3] + l[1]*r[7] + l[2]*r[11] + l[3]*r[15],

                l[4]*r[0] + l[5]*r[4] + l[6]*r[8] + l[7]*r[12],
                l[4]*r[1] + l[5]*r[5] + l[6]*r[9] + l[7]*r[13],
                l[4]*r[2] + l[5]*r[6] + l[6]*r[10] + l[7]*r[14],
                l[4]*r[3] + l[5]*r[7] + l[6]*r[11] + l[7]*r[15],

                l[8]*r[0] + l[9]*r[4] + l[10]*r[8] + l[11]*r[12],
                l[8]*r[1] + l[9]*r[5] + l[10]*r[9] + l[11]*r[13],
                l[8]*r[2] + l[9]*r[6] + l[10]*r[10] + l[11]*r[14],
                l[8]*r[3] + l[9]*r[7] + l[10]*r[11] + l[11]*r[15],

                l[12]*r[0] + l[13]*r[4] + l[14]*r[8] + l[15]*r[12],
                l[12]*r[1] + l[13]*r[5] + l[14]*r[9] + l[15]*r[13],
                l[12]*r[2] + l[13]*r[6] + l[14]*r[10] + l[15]*r[14],
                l[12]*r[3] + l[13]*r[7] + l[14]*r[11] + l[15]*r[15]
            ]
        }
    }
}

impl MulAssign<&Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: &Mat4) {
        let l = self.e.clone();
        let r = &rhs.e;

        self.e[0] = l[0]*r[0] + l[1]*r[4] + l[2]*r[8] + l[3]*r[12];
        self.e[1] = l[0]*r[1] + l[1]*r[5] + l[2]*r[9] + l[3]*r[13];
        self.e[2] = l[0]*r[2] + l[1]*r[6] + l[2]*r[10] + l[3]*r[14];
        self.e[3] = l[0]*r[3] + l[1]*r[7] + l[2]*r[11] + l[3]*r[15];

        self.e[4] = l[4]*r[0] + l[5]*r[4] + l[6]*r[8] + l[7]*r[12];
        self.e[5] = l[4]*r[1] + l[5]*r[5] + l[6]*r[9] + l[7]*r[13];
        self.e[6] = l[4]*r[2] + l[5]*r[6] + l[6]*r[10] + l[7]*r[14];
        self.e[7] = l[4]*r[3] + l[5]*r[7] + l[6]*r[11] + l[7]*r[15];

        self.e[8] = l[8]*r[0] + l[9]*r[4] + l[10]*r[8] + l[11]*r[12];
        self.e[9] = l[8]*r[1] + l[9]*r[5] + l[10]*r[9] + l[11]*r[13];
        self.e[10] = l[8]*r[2] + l[9]*r[6] + l[10]*r[10] + l[11]*r[14];
        self.e[11] = l[8]*r[3] + l[9]*r[7] + l[10]*r[11] + l[11]*r[15];

        self.e[12] = l[12]*r[0] + l[13]*r[4] + l[14]*r[8]
            + l[15]*r[12];
        self.e[13] = l[12]*r[1] + l[13]*r[5] + l[14]*r[9]
            + l[15]*r[13];
        self.e[14] = l[12]*r[2] + l[13]*r[6] + l[14]*r[10]
            + l[15]*r[14];
        self.e[15] = l[12]*r[3] + l[13]*r[7] + l[14]*r[11]
            + l[15]*r[15];
    }
}
