use std::{
    f64::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use image::Rgb;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, Default)]
pub struct V(pub f64, pub f64, pub f64);

impl Add<V> for V {
    type Output = V;
    fn add(self, rhs: V) -> V {
        let V(x1, y1, z1) = self;
        let V(x2, y2, z2) = rhs;
        V(x1 + x2, y1 + y2, z1 + z2)
    }
}
impl AddAssign for V {
    fn add_assign(&mut self, rhs: Self) {
        let V(x1, y1, z1) = *self;
        let V(x2, y2, z2) = rhs;
        *self = V(x1 + x2, y1 + y2, z1 + z2);
    }
}

impl Sub for V {
    type Output = V;
    fn sub(self, rhs: V) -> V {
        let V(x1, y1, z1) = self;
        let V(x2, y2, z2) = rhs;
        V(x1 - x2, y1 - y2, z1 - z2)
    }
}
impl SubAssign for V {
    fn sub_assign(&mut self, rhs: Self) {
        let V(x1, y1, z1) = *self;
        let V(x2, y2, z2) = rhs;
        *self = V(x1 - x2, y1 - y2, z1 - z2);
    }
}

impl Mul<f64> for V {
    type Output = V;
    fn mul(self, rhs: f64) -> V {
        let V(x, y, z) = self;
        V(x * rhs, y * rhs, z * rhs)
    }
}
impl MulAssign<f64> for V {
    fn mul_assign(&mut self, rhs: f64) {
        let V(x, y, z) = *self;
        *self = V(x * rhs, y * rhs, z * rhs);
    }
}
impl Mul<V> for f64 {
    type Output = V;
    fn mul(self, rhs: V) -> V {
        let V(x, y, z) = rhs;
        V(self * x, self * y, self * z)
    }
}

impl Div<f64> for V {
    type Output = V;
    fn div(self, rhs: f64) -> V {
        let V(x, y, z) = self;
        V(x / rhs, y / rhs, z / rhs)
    }
}
impl DivAssign<f64> for V {
    fn div_assign(&mut self, rhs: f64) {
        let V(x, y, z) = *self;
        *self = V(x / rhs, y / rhs, z / rhs);
    }
}
impl Neg for V {
    type Output = V;
    fn neg(self) -> V {
        let V(x, y, z) = self;
        V(-x, -y, -z)
    }
}

impl V {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn dot(&self, rhs: V) -> f64 {
        let V(x1, y1, z1) = self;
        let V(x2, y2, z2) = rhs;
        x1 * x2 + y1 * y2 + z1 * z2
    }
    pub fn cross(&self, rhs: V) -> V {
        let V(x1, y1, z1) = self;
        let V(x2, y2, z2) = rhs;
        V(y1 * z2 - z1 * y2, -x1 * z2 + z1 * x2, x1 * y2 - y1 * x2)
    }
    pub fn norm(&self) -> f64 {
        let V(x, y, z) = self;
        (x * x + y * y + z * z).sqrt()
    }
    pub fn norm_squared(&self) -> f64 {
        let V(x, y, z) = self;
        x * x + y * y + z * z
    }
    pub fn unit(&self) -> V {
        *self / self.norm()
    }
    pub fn random_unit() -> V {
        let mut rng = thread_rng();
        let theta = rng.gen_range(0.0..=PI);
        let phi = rng.gen_range(0.0..2.0 * PI);
        V(
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos(),
        )
    }
}

impl Into<Rgb<u8>> for V {
    fn into(self) -> Rgb<u8> {
        let V(r, g, b) = self;
        if r < 0.0 || g < 0.0 || b < 0.0 || r > 1.0 || g > 1.0 || b > 1.0 {
            panic!()
        }
        Rgb([
            (r * 255.999) as u8,
            (g * 255.999) as u8,
            (b * 255.999) as u8,
        ])
    }
}
