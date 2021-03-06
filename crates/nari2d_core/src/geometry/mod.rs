use core::ops::{Deref, DerefMut};
use rstar::{Envelope, Point, PointDistance, RTreeObject, AABB};
use std::cmp::Ordering;

pub use angle::Angle;
pub use point2d::Point2d;
pub use scale2d::Scale2d;

mod angle;
mod bounds;
pub mod lattice;
pub mod mesh;
mod point2d;
mod scale2d;

pub type TMesh = Mesh<Point2d>;
pub type TMeshBuilder = MeshBuilder<f32>;

/// ```.ignore
///       C
///      / \
///     /   \
///  b /     \ a
///   /       \
///  A_________B
///       c
/// ```
///
#[inline]
#[must_use]
pub fn angles_of_triangle(p1_a: &Point2d, p2_b: &Point2d, p3_c: &Point2d) -> (Angle, Angle, Angle) {
    let a = Point2d::distance_to(p3_c, p2_b);
    let b = Point2d::distance_to(p1_a, p3_c);
    let c = Point2d::distance_to(p2_b, p1_a);
    let a_2 = a.powi(2);
    let b_2 = b.powi(2);
    let c_2 = c.powi(2);
    let angle_a = Angle::from_radians(f32::acos((b_2 + c_2 - a_2) / (2_f32 * b * c))).positive();
    let angle_b = Angle::from_radians(f32::acos((a_2 + c_2 - b_2) / (2_f32 * a * c))).positive();
    let angle_c = Angle::from_radians(f32::acos((a_2 + b_2 - c_2) / (2_f32 * a * b))).positive();
    (angle_a, angle_b, angle_c)
}

#[must_use]
pub fn is_triangle_bad(threshold: Angle, p1_a: Point2d, p2_b: Point2d, p3_c: Point2d) -> bool {
    let (a, b, c) = angles_of_triangle(p1_a, p2_b, p3_c);
    if a <= threshold && b <= threshold && c <= threshold {
        return true;
    }
    false
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct IndexedPoint2d {
    pub(crate) index: usize,
    pub(crate) point: Point2d,
}

impl Deref for IndexedPoint2d {
    type Target = Point2d;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl DerefMut for IndexedPoint2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.point
    }
}

impl RTreeObject for IndexedPoint2d {
    type Envelope = AABB<[f32; 2]>;

    #[inline]
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x(), self.y()])
    }
}

impl PointDistance for IndexedPoint2d {
    fn distance_2(
        &self,
        point: &<Self::Envelope as Envelope>::Point,
    ) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
        f32::pow((self.x() - point[0]), 2) + f32::pow((self.y() - point[1]), 2)
    }
}

#[inline]
fn nearly_equal_f32(n: f32, m: f32) -> bool {
    let epsilon = (2_f32 * f32::abs(n - m)) / (n.abs() + m.abs());

    if n == m || f32::abs(n - m) <= epsilon {
        return true;
    }
    false
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Orientation {
    CoLinear = 0,
    ClockWise = 1,
    CounterClockWise = 2,
}

// Marker type to tell the user that this was the result of a precalculation
pub type PreCalcMultiples = Vec<f32>;
pub type PreCalcConsts = Vec<f32>;
pub type PreCalcMultiplesSlice<'a> = &'a [f32];
pub type PreCalcConstsSlice<'a> = &'a [f32];

// from http://alienryderflex.com/polygon/
#[inline]
#[must_use]
pub fn pre_calculate_polygon_values(polygon: &[Point2d]) -> (PreCalcMultiples, PreCalcConsts) {
    let mut const_precalc = vec![0_f32; polygon.len()];
    let mut multi_precalc = vec![0_f32; polygon.len()];
    for (idx, poly_points) in PointSlice::from(polygon).enumerate() {
        let i = poly_points[1];
        let j = poly_points[0];

        if i.y() == j.y() {
            const_precalc[idx] = i.x();
            multi_precalc[idx] = 0_f32;
        } else {
            const_precalc[idx] =
                i.x() - (i.y() * j.x()) / (j.y() - i.y()) + (i.y() * i.x()) / (j.y() - i.y());
            multi_precalc[idx] = (j.x() - i.x()) / (j.y() - i.y());
        }
    }
    (multi_precalc, const_precalc)
}

// See f32::total_cmp().
#[inline]
pub(crate) fn float_cmp(left: &f32, right: &f32) -> Ordering {
    if left.is_nan() && right.is_nan() {
        return Ordering::Equal;
    } else if left.is_infinite() && right.is_infinite() {
        return Ordering::Equal;
    } else if left.is_nan() && !right.is_nan() {
        return Ordering::Greater;
    } else if !left.is_nan() && right.is_nan() {
        return Ordering::Less;
    } else if !left.is_infinite() && right.is_infinite() {
        return Ordering::Less;
    } else if left.is_infinite() && !right.is_infinite() {
        return Ordering::Greater;
    }

    let mut left = left.to_bits() as i32;
    let mut right = right.to_bits() as i32;

    left ^= (((left >> 31) as u32) >> 1) as i32;
    right ^= (((right >> 31) as u32) >> 1) as i32;

    left.cmp(&right)
}
