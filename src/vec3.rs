use std::ops;

use crate::{rand, rand_range};

type Elem = [f64; 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    elem: Elem,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { elem: [x, y, z] }
    }

    pub fn zero() -> Self {
        Self {
            elem: [0.0, 0.0, 0.0],
        }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.elem.iter().map(|x| x.powi(2)).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            elem: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        Self::new(rand(), rand(), rand())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_range(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                break v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let v = Self::random_unit_vector();
        if v.dot(normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            elem: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            elem: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            elem: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            elem: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

macro_rules! impl_numeric_operations_for {
    ($typ: ty) => {
        impl ops::Add<$typ> for Vec3 {
            type Output = Vec3;
            fn add(self, rhs: $typ) -> Self::Output {
                Self {
                    elem: [
                        self[0] + rhs as f64,
                        self[1] + rhs as f64,
                        self[2] + rhs as f64,
                    ],
                }
            }
        }
        impl ops::Add<Vec3> for $typ {
            type Output = Vec3;
            fn add(self, rhs: Vec3) -> Self::Output {
                Self::Output {
                    elem: [
                        self as f64 + rhs[0],
                        self as f64 + rhs[1],
                        self as f64 + rhs[2],
                    ],
                }
            }
        }
        impl ops::AddAssign<$typ> for Vec3 {
            fn add_assign(&mut self, rhs: $typ) {
                self[0] += rhs;
                self[1] += rhs;
                self[2] += rhs;
            }
        }
        impl ops::Sub<$typ> for Vec3 {
            type Output = Vec3;
            fn sub(self, rhs: $typ) -> Self::Output {
                Self {
                    elem: [
                        self[0] - rhs as f64,
                        self[1] - rhs as f64,
                        self[2] - rhs as f64,
                    ],
                }
            }
        }
        impl ops::Sub<Vec3> for $typ {
            type Output = Vec3;
            fn sub(self, rhs: Vec3) -> Self::Output {
                Self::Output {
                    elem: [
                        self as f64 - rhs[0],
                        self as f64 - rhs[1],
                        self as f64 - rhs[2],
                    ],
                }
            }
        }
        impl ops::SubAssign<$typ> for Vec3 {
            fn sub_assign(&mut self, rhs: $typ) {
                self[0] -= rhs;
                self[1] -= rhs;
                self[2] -= rhs;
            }
        }
        impl ops::Mul<$typ> for Vec3 {
            type Output = Self;
            fn mul(self, rhs: $typ) -> Self::Output {
                Self {
                    elem: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
                }
            }
        }
        impl ops::Mul<Vec3> for $typ {
            type Output = Vec3;
            fn mul(self, rhs: Vec3) -> Self::Output {
                Self::Output {
                    elem: [
                        self as f64 * rhs[0],
                        self as f64 * rhs[1],
                        self as f64 * rhs[2],
                    ],
                }
            }
        }
        impl ops::MulAssign<$typ> for Vec3 {
            fn mul_assign(&mut self, rhs: $typ) {
                self[0] *= rhs;
                self[1] *= rhs;
                self[2] *= rhs;
            }
        }
        impl ops::Div<$typ> for Vec3 {
            type Output = Self;
            fn div(self, rhs: $typ) -> Self::Output {
                self * (1.0 / rhs)
            }
        }
        impl ops::DivAssign<$typ> for Vec3 {
            fn div_assign(&mut self, rhs: $typ) {
                *self *= 1.0 / rhs;
            }
        }
    };
}

impl_numeric_operations_for!(f64);

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elem[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elem[index]
    }
}

pub type Point3 = Vec3;

mod tests {
    #[allow(unused_imports)]
    use super::Vec3;
    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 + v2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn add_number() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 + 2.0, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(2.0 + v1, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn add_aasign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 += v2;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 5.0);
        assert_eq!(v1 - v2, Vec3::new(-1.0, -1.0, -2.0));
    }

    #[test]
    fn sub_number() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 - 2.0, Vec3::new(-1.0, 0.0, 1.0));
        assert_eq!(2.0 - v1, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn sub_aasign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 5.0);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(-2.0, 0.0, -2.0));
    }

    #[test]
    fn neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v1, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn index() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1[0], 1.0);
        assert_eq!(v1[1], 2.0);
        assert_eq!(v1[2], 3.0);
    }

    #[test]
    fn index_mut() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1[0] = 2.0;
        v1[1] = 3.0;
        v1[2] = 4.0;
        assert_eq!(v1[0], 2.0);
        assert_eq!(v1[1], 3.0);
        assert_eq!(v1[2], 4.0);
    }

    #[test]
    fn length_squared() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }
    #[test]
    fn length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 3.0);
        assert_eq!(v1.dot(&v2), 16.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 3.0);
        assert_eq!(v1.cross(&v2), Vec3::new(0.0, 6.0, -4.0));
    }

    #[test]
    fn unit_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            v1.unit_vector(),
            Vec3::new(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );
    }
}
