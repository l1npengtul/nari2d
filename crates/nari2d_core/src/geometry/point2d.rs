use crate::collections::two_elem_move_once::TwoElemMoveOnceVec;
use crate::geometry::{
    float_cmp, lattice::StrengthPoint, nearly_equal_f32, Angle, IndexedPoint2d, Orientation,
    PointSlice, PointVec, PreCalcConstsSlice, PreCalcMultiplesSlice, Scale2d,
};
use robust::Coord;
use rstar::{Envelope, Point, RTreeObject, AABB};
use staticvec::staticvec;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};
use wide::f32x4;

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Point2d {
    x: f32,
    y: f32,
}

impl Point2d {
    pub const ZERO: Point2d = Point2d { x: 0.0, y: 0.0 };

    pub const INF: Point2d = Point2d {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };

    pub const NAN: Point2d = Point2d {
        x: f32::NAN,
        y: f32::NAN,
    };
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
        Point2d::ZERO
    }

    #[inline]
    #[must_use]
    pub fn origin() -> Self {
        Point2d::ZERO
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
    pub fn linear_interpolate(&self, end: &Self, along: f32) -> Self {
        let along = along.clamp(0_f32, 1_f32);
        Point2d {
            x: self.x + (end.x - self.x) * along,
            y: self.y + (end.y - self.y) * along,
        }
    }

    #[inline]
    #[must_use]
    pub fn mid_point(&self, other: &Self) -> Self {
        Point2d {
            x: (self.x + other.x) / 2_f32,
            y: (self.y + other.y) / 2_f32,
        }
    }

    #[inline]
    #[must_use]
    pub fn slope(&self, other: &Self) -> f32 {
        (self.x - other.x) / (self.y - other.y)
    }

    #[inline]
    #[must_use]
    pub fn max(&self, other: &Self) -> Self {
        Point2d {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn min(&self, other: &Self) -> Self {
        Point2d {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp(&self, start: &Self, end: &Self) -> Self {
        *self.max(start).min(end)
    }

    #[inline]
    #[must_use]
    pub fn re_center(&self, old_center: &Self, new_center: &Self) -> Self {
        let difference = new_center - old_center;
        self + difference
    }

    #[inline]
    #[must_use]
    pub fn scale(&self, scale: &Scale2d) -> Self {
        Point2d::new(self.x * scale.x(), self.y * scale.y())
    }

    #[inline]
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f32 {
        f32::hypot(self.x - other.x, self.y - other.y)
    }

    #[inline]
    pub fn rotate(&self, angle: Angle, center: &Self) -> Self {
        let temp_translated = self.re_center(center, &Point2d::zero());
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
    #[must_use]
    pub fn arc_tan2(&self, point: &Self) -> Angle {
        let homed = point - self;
        Angle::from_radians(homed.y().atan2(homed.x()))
    }

    #[inline]
    #[must_use]
    pub fn angle_of_3(&self, point1: &Self, point2: &Self) -> Angle {
        let homed_1 = point1 - self;
        let homed_2 = point2 - self;
        Angle::from_radians(homed_1.y().atan2(homed_1.x()))
            - Angle::from_radians(homed_2.y().atan2(homed_2.x()))
    }

    #[inline]
    #[must_use]
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
    #[must_use]
    pub fn is_infinite(&self) -> bool {
        if self.x.is_infinite() {
            return true;
        }
        if self.y.is_infinite() {
            return true;
        }
        false
    }

    #[inline]
    #[must_use]
    pub fn orientation(&self, b: &Point2d, c: &Point2d) -> Orientation {
        let slope_difference = (b.y - self.y) * (c.x - b.x) - (b.x - self.x) * (c.y - b.y);

        return if nearly_equal_f32(slope_difference, 0_f32) {
            Orientation::CoLinear
        } else if nearly_equal_f32(slope_difference, 1_f32) {
            Orientation::ClockWise
        } else {
            Orientation::CounterClockWise
        };
    }

    #[inline]
    #[must_use]
    pub fn point_on_segment(&self, p1: &Point2d, p2: &Point2d) -> bool {
        if self.x <= f32::max(p1.x, p2.x)
            && self.x >= f32::min(p1.x, p2.x)
            && self.y <= f32::max(p1.y, p2.y)
            && self.y >= f32::min(p1.y, p2.y)
        {
            return true;
        }
        false
    }

    // from http://alienryderflex.com/polygon/
    #[inline]
    #[must_use]
    pub fn point_in_polygon(&self, polygon: &[Point2d]) -> bool {
        let mut odd_nodes = false;
        for poly_points in TwoElemMoveOnceVec::from(polygon.into_iter()) {
            if (poly_points.1.y < self.y && poly_points.0.y >= self.y
                || poly_point.0.y < self.y && poly_points.1.y >= self.y)
                && (poly_points.1.x <= self.x || poly_points.0.y <= self.x)
            {
                odd_nodes ^= (poly_points.1.x
                    + (self.y - poly_points.1.y) / (poly_points.0.y - poly_points.1.y)
                        * (poly_points.0.x - poly_points.1.x)
                    < self.x)
            }
        }
        odd_nodes
    }

    // from http://alienryderflex.com/polygon/
    #[inline]
    #[must_use]
    pub fn point_in_polygon_with_precalc(
        &self,
        polygon: &[Point2d],
        constants: PreCalcConstsSlice<'_>,
        multiples: PreCalcMultiplesSlice<'_>,
    ) -> bool {
        let mut odd_nodes = false;

        for (idx, poly_points) in TwoElemMoveOnceVec::from(polygon.into_iter()).enumerate() {
            if (poly_points.1.y < self.y && poly_points.0.y >= self.y)
                || (poly_points.0.y < self.y && poly_points.1.y >= self.y)
            {
                odd_nodes ^= (self.y * multiples[idx] + constants[idx] < self.x);
            }
        }

        odd_nodes
    }

    #[inline]
    #[must_use]
    pub fn point_in_circle(&self, center: &Point2d, radius: f32, include_on_circle: bool) -> bool {
        let dx = f32::abs(self.x - center.x);
        let dy = f32::abs(self.y - center.y);

        if dx > radius || dy > radius {
            return false;
        }

        if (dx + dy) >= radius {
            return true;
        }

        return if include_on_circle {
            dx.powi(2) + dy.powi(2) <= radius.powi(2)
        } else {
            dx.powi(2) + dy.powi(2) < radius.powi(2)
        };
    }

    #[inline]
    #[must_use]
    pub fn point_in_circumcircle(&self, p1: &Point2d, p2: &Point2d, p3: &Point2d) -> bool {
        let mut points = staticvec![p1, p2, p3];
        points.sort_by(|a, b| match a.y().total_cmp(&b.y()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.x().total_cmp(&b.x()),
            Ordering::Less => Ordering::Less,
        });
        let a = unsafe { points.pop_unchecked() };

        let b = if points[0].x < points[1].x {
            unsafe { points.pop_unchecked() }
        } else {
            points[1]
        };

        let c = unsafe { points.pop_unchecked() };

        let ax = a.x - self.x;
        let ay = a.y - self.y;
        let bx = b.x - self.x;
        let by = b.y - self.y;
        let cx = c.x - self.x;
        let cy = c.y - self.y;

        return ((ax.powi(2) + ay.powi(2)) * (bx * cy - cx * by)
            - (bx.powi(2) + by.powi(2)) * (ax * cy - cx * ay)
            + (cx.powi(2) + cy.powi(2)) * (ax * by - bx * ay))
            > 0_f32;
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

impl From<IndexedPoint2d> for Point2d {
    fn from(ipt: IndexedPoint2d) -> Self {
        ipt.point
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
            unsafe { std::mem::transmute::<Point2d, u64>(Point2d::NAN) }
        } else if self.is_infinite() {
            unsafe { std::mem::transmute::<Point2d, u64>(Point2d::INF) }
        } else {
            // SAFETY: This operation is safe due to Point2d being repr(C), garunteeing a stable memory layout at runtime.
            unsafe { std::mem::transmute::<Point2d, u64>(*self) }
        };
        data.hash(state)
    }
}

impl Eq for Point2d {}

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

impl Into<Coord<f32>> for Point2d {
    fn into(self) -> Coord<f32> {
        Coord {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Coord<f32>> for &Point2d {
    fn into(self) -> Coord<f32> {
        Coord {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<StrengthPoint> for Point2d {
    fn from(s_pt: StrengthPoint) -> Self {
        s_pt.point()
    }
}

impl TwoDimensionalPoint for Point2d {
    type SCALAR = f32;

    fn new_point(x: Self::SCALAR, y: Self::SCALAR) -> Self {
        Point2d::new(x, y)
    }

    fn x(&self) -> Self::SCALAR {
        self.x
    }

    fn set_x(&mut self, new_x: Self::SCALAR) {
        self.x = new_x;
    }

    fn y(&self) -> Self::SCALAR {
        self.y
    }

    fn set_y(&mut self, new_y: Self::SCALAR) {
        self.y = new_y
    }
}

impl Point for Point2d {
    type Scalar = f32;
    const DIMENSIONS: usize = 2;

    fn generate(generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Point2d::new(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => {
                unreachable!()
            }
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}

impl RTreeObject for Point2d {
    type Envelope = AABB<Point2d>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(*self)
    }
}
