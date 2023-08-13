use crate::math::{Point3, Vec3};
use crate::kdtree::AABB;

#[derive(Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,

    // For faster AABB testing
    inv_dir: Vec3,
    sign: [bool; 3]
}

impl Ray {
    pub fn from_values(origin: &Point3, direction: &Vec3) -> Self {
        let inv_dir = 1. / direction;
        let sign = [
            direction.x < 0., 
            direction.y < 0.,
            direction.z < 0.
        ];

        Self {
            orig: origin.clone(),
            dir: direction.clone(),
            inv_dir,
            sign
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        &self.orig + t * &self.dir 
    }

    fn get_aabb_sign(aabb: &AABB, sign: bool) -> &Point3 {
        if sign {
            &aabb.max
        } else {
            &aabb.min
        }
    }

    pub fn intersect_aabb(&self, aabb: &AABB) -> bool {
        let mut ray_min =
            (Self::get_aabb_sign(
                aabb, 
                self.sign[0]
            ).x - self.orig.x)
            * self.inv_dir.x;
        let mut ray_max =
            (Self::get_aabb_sign(
                aabb, 
                !self.sign[0]
            ).x - self.orig.x)
            * self.inv_dir.x;
        let y_min =
            (Self::get_aabb_sign(
                aabb, 
                self.sign[1]
            ).y - self.orig.y)
            * self.inv_dir.y;
        let y_max =
            (Self::get_aabb_sign(
                aabb, 
                !self.sign[1]
            ).y - self.orig.y)
            * self.inv_dir.y;

        if ray_min > y_max || y_min > ray_max {
            return false;
        }

        if y_min > ray_min {
            ray_min = y_min;
        }
        if y_max < ray_max {
            ray_max = y_max;
        }

        let z_min =
            (Self::get_aabb_sign(
                aabb, 
                self.sign[2]
            ).z - self.orig.z)
            * self.inv_dir.z;
        let z_max =
            (Self::get_aabb_sign(
                aabb, 
                !self.sign[2]
            ).z - self.orig.z)
            * self.inv_dir.z;

        if (ray_min > z_max) || (z_min > ray_max) {
            return false;
        }

        if z_max < ray_max {
            ray_max = z_max;
        }

        ray_max > 0.0
    }
}
