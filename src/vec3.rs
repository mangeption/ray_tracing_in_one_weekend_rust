use std::ops;
use crate::utils::nums::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

const EPS: f64 = 1e-8;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn unit_vector(&self) -> Self {
        let l = self.length();
        *self / l
    }

    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * Self::dot(self, n) * *n
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let neg = -*self;
        let x = Vec3::dot(&neg, n);
        let cos_theta = if x < 1.0 { x } else { 1.0 };
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }

    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Self {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3 {
            e: [self * t.e[0], self * t.e[1], self * t.e[2]],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Self {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i64) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = 256.0 * clamp((self.x() * scale).sqrt(), 0.0, 0.999);
        let g = 256.0 * clamp((self.y() * scale).sqrt(), 0.0, 0.999);
        let b = 256.0 * clamp((self.z() * scale).sqrt(), 0.0, 0.999);

        println!("{:?} {:?} {:?}", r as i64, g as i64, b as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec3() {
        let v = Vec3::new(1.1, 0.0, 3.456);
        assert_eq!(v.x(), 1.1);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 3.456);
        assert_approx_eq!(v.length_squared(), 13.153936, EPS);
        assert_approx_eq!(v.length(), 3.6269, EPS);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.1, 0.0, 3.456);
        let v2 = Vec3::new(4.1, -1.0, 0.0);

        v1 += v2;
        assert_approx_eq!(v1.x(), 5.1, EPS);
        assert_approx_eq!(v1.y(), -1.0, EPS);
        assert_approx_eq!(v1.z(), 3.456, EPS);
    }

    #[test]
    fn test_neg() {
        let v1 = -Vec3::new(1.1, 0.0, 3.456);

        assert_approx_eq!(v1.x(), -1.1, EPS);
        assert_approx_eq!(v1.y(), 0.0, EPS);
        assert_approx_eq!(v1.z(), -3.456, EPS);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.1, 0.0, -3.456);

        v1 *= 5.0;
        assert_approx_eq!(v1.x(), 5.5, EPS);
        assert_approx_eq!(v1.y(), 0.0, EPS);
        assert_approx_eq!(v1.z(), -17.28, EPS);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(1.1, 0.0, -3.456);

        v1 /= 5.0;
        assert_approx_eq!(v1.x(), 0.22, EPS);
        assert_approx_eq!(v1.y(), 0.0, EPS);
        assert_approx_eq!(v1.z(), -0.6912, EPS);
    }
}
