use crate::geometry::Point2d;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct Scale2d {
    x: f32,
    y: f32,
}

impl Scale2d {
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Scale2d { x, y }
    }

    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Scale2d { x: v, y: v }
    }

    #[inline]
    #[must_use]
    pub fn zero() -> Self {
        Scale2d::default()
    }

    #[inline]
    #[must_use]
    pub fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use]
    pub fn to_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_vec(self) -> Vec<f32> {
        vec![self.x, self.y]
    }

    #[inline]
    #[must_use]
    pub fn x(self) -> f32 {
        self.x
    }

    #[inline]
    pub fn set_x(&mut self, new_x: f32) {
        self.x = new_x;
    }

    #[inline]
    #[must_use]
    pub fn y(self) -> f32 {
        self.y
    }

    #[inline]
    pub fn set_y(&mut self, new_y: f32) {
        self.y = new_y;
    }

    #[inline]
    pub fn swap(&mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }

    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        Scale2d {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Scale2d {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    #[inline]
    #[must_use]
    pub fn ceiling(self) -> Self {
        Scale2d {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    #[inline]
    #[must_use]
    pub fn truncate(self) -> Self {
        Scale2d {
            x: self.x.trunc(),
            y: self.y.trunc(),
        }
    }

    #[inline]
    #[must_use]
    pub fn fractional(self) -> Self {
        Scale2d {
            x: self.x.fract(),
            y: self.y.fract(),
        }
    }

    #[inline]
    #[must_use]
    pub fn absoulte_value(self) -> Self {
        Scale2d {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    #[inline]
    #[must_use]
    pub fn linear_interpolate(self, end: Scale2d, along: f32) -> Self {
        let along = along.clamp(0_f32, 1_f32);
        Scale2d {
            x: self.x + (end.x - self.x) * along,
            y: self.y + (end.y - self.y) * along,
        }
    }

    #[inline]
    #[must_use]
    pub fn max(self, other: Scale2d) -> Self {
        Scale2d {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn min(self, other: Scale2d) -> Self {
        Scale2d {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, start: Scale2d, end: Scale2d) -> Self {
        self.max(start).min(end)
    }

    #[inline]
    #[must_use]
    pub fn re_center(self, old_center: Scale2d, new_center: Scale2d) -> Self {
        let difference = new_center - old_center;
        self + difference
    }
}

impl Add for Scale2d {
    type Output = Scale2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Scale2d {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl Sub for Scale2d {
    type Output = Scale2d;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Scale2d {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

impl Add<f32> for Scale2d {
    type Output = Scale2d;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for Scale2d {
    type Output = Scale2d;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f32> for Scale2d {
    type Output = Scale2d;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Scale2d {
    type Output = Scale2d;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl AddAssign for Scale2d {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.set_x(self.x() + rhs.x());
        self.set_y(self.y() + rhs.y());
    }
}

impl SubAssign for Scale2d {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.set_x(self.x() - rhs.x());
        self.set_y(self.y() - rhs.y());
    }
}

impl AddAssign<f32> for Scale2d {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.set_x(self.x() + rhs);
        self.set_y(self.y() + rhs);
    }
}

impl SubAssign<f32> for Scale2d {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.set_x(self.x() - rhs);
        self.set_y(self.y() - rhs);
    }
}

impl MulAssign<f32> for Scale2d {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.set_x(self.x() * rhs);
        self.set_y(self.y() * rhs);
    }
}

impl DivAssign<f32> for Scale2d {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.set_x(self.x() / rhs);
        self.set_y(self.y() / rhs);
    }
}

impl Add for &Scale2d {
    type Output = Scale2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Scale2d {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl Sub for &Scale2d {
    type Output = Scale2d;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Scale2d {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

impl Add<f32> for &Scale2d {
    type Output = Scale2d;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for &Scale2d {
    type Output = Scale2d;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f32> for &Scale2d {
    type Output = Scale2d;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for &Scale2d {
    type Output = Scale2d;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Scale2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<[f32; 2]> for Scale2d {
    fn from(from: [f32; 2]) -> Self {
        Scale2d::new(from[0], from[1])
    }
}

impl From<(f32, f32)> for Scale2d {
    fn from(from: (f32, f32)) -> Self {
        Scale2d::new(from.0, from.1)
    }
}

impl From<Point2d> for Scale2d {
    fn from(from: Point2d) -> Self {
        Scale2d::new(from.x(), from.y())
    }
}

impl Display for Scale2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}
