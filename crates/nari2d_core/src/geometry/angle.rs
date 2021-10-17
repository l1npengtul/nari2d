use crate::geometry::Point2d;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

const TWO_PI: f32 = std::f32::consts::PI * 2_f32;
const PI: f32 = std::f32::consts::PI;

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    #[inline]
    #[must_use]
    pub fn new(radians: f32) -> Angle {
        Angle { radians }
    }

    #[inline]
    #[must_use]
    pub fn from_radians(radians: f32) -> Angle {
        Angle::new(radians)
    }

    #[inline]
    #[must_use]
    pub fn from_degrees(degrees: f32) -> Angle {
        let radians = degrees.to_degrees();
        Angle::new(radians)
    }

    #[inline]
    #[must_use]
    pub fn radians(self) -> f32 {
        self.radians
    }

    #[inline]
    pub fn set_radians(&mut self, new_radians: f32) {
        let mut angle = new_radians % TWO_PI;
        if angle < 0_f32 {
            angle += TWO_PI;
        }

        self.radians = angle;
    }

    #[inline]
    pub fn set_from_angle(&mut self, angle: &Angle) {
        self.radians = angle.radians;
    }

    #[inline]
    #[must_use]
    pub fn degrees(self) -> f32 {
        self.radians.to_degrees()
    }

    #[inline]
    #[must_use]
    pub fn sin(self) -> f32 {
        self.radians.sin()
    }

    #[inline]
    #[must_use]
    pub fn cos(self) -> f32 {
        self.radians.cos()
    }

    #[inline]
    #[must_use]
    pub fn tan(self) -> f32 {
        self.radians.tan()
    }

    #[inline]
    #[must_use]
    pub fn positive(self) -> Self {
        let mut angle = self.radians % TWO_PI;
        if angle < 0_f32 {
            angle += TWO_PI;
        }
        Angle::new(angle)
    }
}

impl Add for Angle {
    type Output = Angle;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians + rhs.radians).positive()
    }
}

impl Sub for Angle {
    type Output = Angle;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians - rhs.radians).positive()
    }
}

impl Mul for Angle {
    type Output = Angle;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians * rhs.radians).positive()
    }
}

impl Div for Angle {
    type Output = Angle;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians / rhs.radians).positive()
    }
}

impl Rem for Angle {
    type Output = Angle;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians % rhs.radians).positive()
    }
}

impl Add for &Angle {
    type Output = Angle;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians + rhs.radians).positive()
    }
}

impl Sub for &Angle {
    type Output = Angle;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians - rhs.radians).positive()
    }
}

impl Mul for &Angle {
    type Output = Angle;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians * rhs.radians).positive()
    }
}

impl Div for &Angle {
    type Output = Angle;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians / rhs.radians).positive()
    }
}

impl Rem for &Angle {
    type Output = Angle;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Angle::new(self.radians % rhs.radians).positive()
    }
}

impl Add<f32> for Angle {
    type Output = Angle;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians + rhs).positive()
    }
}

impl Sub<f32> for Angle {
    type Output = Angle;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians - rhs).positive()
    }
}

impl Mul<f32> for Angle {
    type Output = Angle;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians * rhs).positive()
    }
}

impl Div<f32> for Angle {
    type Output = Angle;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians / rhs).positive()
    }
}

impl Rem<f32> for Angle {
    type Output = Angle;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians % rhs).positive()
    }
}

impl Add<f32> for &Angle {
    type Output = Angle;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians + rhs).positive()
    }
}

impl Sub<f32> for &Angle {
    type Output = Angle;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians - rhs).positive()
    }
}

impl Mul<f32> for &Angle {
    type Output = Angle;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians * rhs).positive()
    }
}

impl Div<f32> for &Angle {
    type Output = Angle;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians / rhs).positive()
    }
}

impl Rem<f32> for &Angle {
    type Output = Angle;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Angle::new(self.radians % rhs).positive()
    }
}

impl AddAssign for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.set_radians(self.radians + rhs.radians);
    }
}

impl SubAssign for Angle {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.set_radians(self.radians - rhs.radians);
    }
}

impl MulAssign for Angle {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.set_radians(self.radians * rhs.radians);
    }
}

impl DivAssign for Angle {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.set_radians(self.radians / rhs.radians);
    }
}

impl RemAssign for Angle {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.set_radians(self.radians % rhs.radians);
    }
}

impl AddAssign<f32> for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.set_radians(self.radians + rhs);
    }
}

impl SubAssign<f32> for Angle {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.set_radians(self.radians - rhs);
    }
}

impl MulAssign<f32> for Angle {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.set_radians(self.radians * rhs);
    }
}

impl DivAssign<f32> for Angle {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.set_radians(self.radians / rhs);
    }
}

impl RemAssign<f32> for Angle {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.set_radians(self.radians % rhs);
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} RAD", self.positive())
    }
}