use crate::geometry::{Angle, Scale2d};
use rstar::{RTreeObject, AABB};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Point2d {
    x: f32,
    y: f32,
}

impl Point2d {
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Point2d { x, y }
    }

    #[inline]
    #[must_use]
    pub fn splat(v: f32) -> Self {
        Point2d { x: v, y: v }
    }

    #[inline]
    #[must_use]
    pub fn zero() -> Self {
        Point2d::default()
    }

    #[inline]
    #[must_use]
    pub fn origin() -> Self {
        Point2d::default()
    }

    #[inline]
    #[must_use]
    pub fn float_min() -> Self {
        Point2d {
            x: f32::MIN,
            y: f32::MIN,
        }
    }

    #[inline]
    #[must_use]
    pub fn float_max() -> Self {
        Point2d {
            x: f32::MAX,
            y: f32::MAX,
        }
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
        Point2d {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Point2d {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    #[inline]
    #[must_use]
    pub fn ceiling(self) -> Self {
        Point2d {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    #[inline]
    #[must_use]
    pub fn truncate(self) -> Self {
        Point2d {
            x: self.x.trunc(),
            y: self.y.trunc(),
        }
    }

    #[inline]
    #[must_use]
    pub fn fractional(self) -> Self {
        Point2d {
            x: self.x.fract(),
            y: self.y.fract(),
        }
    }

    #[inline]
    #[must_use]
    pub fn absoulte_value(self) -> Self {
        Point2d {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    #[inline]
    #[must_use]
    pub fn linear_interpolate(self, end: Point2d, along: f32) -> Self {
        let along = along.clamp(0_f32, 1_f32);
        Point2d {
            x: self.x + (end.x - self.x) * along,
            y: self.y + (end.y - self.y) * along,
        }
    }

    #[inline]
    #[must_use]
    pub fn max(self, other: Point2d) -> Self {
        Point2d {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn min(self, other: Point2d) -> Self {
        Point2d {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, start: Point2d, end: Point2d) -> Self {
        self.max(start).min(end)
    }

    #[inline]
    #[must_use]
    pub fn re_center(self, old_center: Point2d, new_center: Point2d) -> Self {
        let difference = new_center - old_center;
        self + difference
    }

    #[inline]
    #[must_use]
    pub fn scale(self, scale: Scale2d) -> Self {
        Point2d::new(self.x * scale.x(), self.y * scale.y())
    }

    #[inline]
    #[must_use]
    pub fn distance_to(&self, other: Point2d) -> f32 {
        f32::hypot(self.x - other.x, self.y - other.y)
    }

    #[inline]
    pub fn rotate(&self, angle: Angle, center: Point2d) -> Point2d {
        let temp_translated = self.re_center(center, Point2d::zero());
        let sine = angle.sin().radians();
        let cosine = angle.cos().radians();

        let x_rot = (cosine * temp_translated.x()) - (sine * temp_translated.y());
        let y_rot = (sine * temp_translated.x()) + (cosine * temp_translated.y());

        Point2d::new(x_rot, y_rot) + center
    }

    #[inline]
    pub fn rotate_origin(&self, angle: Angle) -> Self {
        let sine = angle.sin().radians();
        let cosine = angle.cos().radians();

        let x_rot = (cosine * self.x()) - (sine * self.y());
        let y_rot = (sine * self.x()) + (cosine * self.y());

        Point2d::new(x_rot, y_rot)
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        if self.x.is_nan() {
            return true;
        }
        if self.y.is_nan() {
            return true;
        }
        false
    }

    #[inline]
    pub fn is_infinite(&self) -> bool {
        if self.x.is_infinite() {
            return true;
        }
        if self.y.is_infinite() {
            return true;
        }
        false
    }
}

impl Add for Point2d {
    type Output = Point2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl Sub for Point2d {
    type Output = Point2d;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

impl Add<f32> for Point2d {
    type Output = Point2d;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for Point2d {
    type Output = Point2d;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f32> for Point2d {
    type Output = Point2d;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Point2d {
    type Output = Point2d;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Rem<f32> for Point2d {
    type Output = Point2d;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl AddAssign for Point2d {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.set_x(self.x() + rhs.x());
        self.set_y(self.y() + rhs.y());
    }
}

impl SubAssign for Point2d {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.set_x(self.x() - rhs.x());
        self.set_y(self.y() - rhs.y());
    }
}

impl AddAssign<f32> for Point2d {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.set_x(self.x() + rhs);
        self.set_y(self.y() + rhs);
    }
}

impl SubAssign<f32> for Point2d {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.set_x(self.x() - rhs);
        self.set_y(self.y() - rhs);
    }
}

impl MulAssign<f32> for Point2d {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.set_x(self.x() * rhs);
        self.set_y(self.y() * rhs);
    }
}

impl DivAssign<f32> for Point2d {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.set_x(self.x() / rhs);
        self.set_y(self.y() / rhs);
    }
}

impl RemAssign<f32> for Point2d {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.set_x(self.x() / rhs);
        self.set_y(self.y() / rhs);
    }
}

impl Add for &Point2d {
    type Output = Point2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl Sub for &Point2d {
    type Output = Point2d;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

impl Add<f32> for &Point2d {
    type Output = Point2d;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for &Point2d {
    type Output = Point2d;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<f32> for &Point2d {
    type Output = Point2d;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for &Point2d {
    type Output = Point2d;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Rem<f32> for &Point2d {
    type Output = Point2d;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Point2d {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl From<[f32; 2]> for Point2d {
    fn from(from: [f32; 2]) -> Self {
        Point2d::new(from[0], from[1])
    }
}

impl From<(f32, f32)> for Point2d {
    fn from(from: (f32, f32)) -> Self {
        Point2d::new(from.0, from.1)
    }
}

impl From<Scale2d> for Point2d {
    fn from(from: Scale2d) -> Self {
        Point2d::new(from.x(), from.y())
    }
}

impl AsRef<[f32; 2]> for Point2d {
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const crate::geometry::point2d::Point2d).cast::<[f32; 2]>() }
    }
}

impl AsMut<[f32; 2]> for Point2d {
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut crate::geometry::point2d::Point2d).cast::<[f32; 2]>() }
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point2d {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.is_infinite() == other.is_infinite() {
            return true;
        }
        if self.is_nan() == other.is_nan() {
            return true;
        }

        let epsilon_x = (2_f32 * f32::abs(self.x - other.x)) / (self.x.abs() + other.x.abs());
        let epsilon_y = (2_f32 * f32::abs(self.y - other.y)) / (self.y.abs() + other.y.abs());

        if (self.x == other.x || (f32::abs(self.x - other.x) <= epsilon_x))
            && (self.y == other.y || (f32::abs(self.y - other.y) <= epsilon_y))
        {
            return true;
        }

        false
    }
}

impl Hash for Point2d {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let data = if self.is_nan() {
            f32::NAN.to_bits() as u64
        } else if self.is_infinite() {
            f32::INFINITY.to_bits() as u64
        } else {
            let a: [u8; 4] = self.x.to_bits().to_ne_bytes();
            let b: [u8; 4] = self.y.to_bits().to_ne_bytes();
            let c = [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]];
            u64::from_ne_bytes(c)
        };
        data.hash(state)
    }
}

impl Eq for Point2d {}

// See f32::total_cmp().
pub(crate) fn float_cmp(left: &f32, right: &f32) -> Ordering {
    let mut left = left.to_bits() as i32;
    let mut right = right.to_bits() as i32;

    left ^= (((left >> 31) as u32) >> 1) as i32;
    right ^= (((right >> 31) as u32) >> 1) as i32;

    left.cmp(&right)
}

impl Ord for Point2d {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match float_cmp(&self.x, &other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => float_cmp(&self.y, &other.y),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

// In order of X coords, then Y Coords
impl PartialOrd for Point2d {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RTreeObject for Point2d {
    type Envelope = AABB<[f32; 2]>;

    #[inline]
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y])
    }
}
