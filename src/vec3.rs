use core::{fmt, panic};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use rand::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn modsq(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn modulus(&self) -> f64 {
        self.modsq().sqrt()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.modulus()
    }

    pub fn clamp(&mut self) {
        self.e[0] = self.e[0].max(0.0);
        self.e[0] = self.e[0].min(0.999);
        self.e[1] = self.e[1].max(0.0);
        self.e[1] = self.e[1].min(0.999);
        self.e[2] = self.e[2].max(0.0);
        self.e[2] = self.e[2].min(0.999);
    }

    pub fn sqrt(&mut self) {
        self.e[0] = self.e[0].sqrt();
        self.e[1] = self.e[1].sqrt();
        self.e[2] = self.e[2].sqrt();
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        Self {
            e: [
                rng.gen_range(0f64..1f64),
                rng.gen_range(0f64..1f64),
                rng.gen_range(0f64..1f64),
            ],
        }
    }
    pub fn randomr(rng: &mut ThreadRng, r: Range<f64>) -> Self {
        Self {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r),
            ],
        }
    }
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        ],
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        if rhs == 0.0 {
            panic!("divided by zero");
        }
        Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.x() * 256.0) as u8,
            (self.y() * 256.0) as u8,
            (self.z() * 256.0) as u8
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = 1f64.min(-dot(uv, n));
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_para = *n * -(1f64 - r_out_perp.modsq()).abs().sqrt();
    r_out_para + r_out_perp
}

pub fn random_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let phi = rng.gen_range(0.0..std::f64::consts::PI);
    let varphi = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    Vec3::new(
        phi.sin() * varphi.cos(),
        phi.sin() * varphi.sin(),
        phi.cos(),
    )
}

pub fn random_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let varphi = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    Vec3 {
        e: [varphi.cos(), varphi.sin(), 0f64],
    }
}
