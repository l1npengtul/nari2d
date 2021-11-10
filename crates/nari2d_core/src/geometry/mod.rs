use rstar::{Envelope, Point, PointDistance, RTreeObject, AABB};
use std::ops::{Deref, DerefMut};

pub use angle::Angle;
pub use point2d::Point2d;
pub use scale2d::Scale2d;

mod angle;
mod bounds;
mod mesh;
mod point2d;
mod scale2d;

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
pub fn angles_of_triangle(p1_a: Point2d, p2_b: Point2d, p3_c: Point2d) -> (Angle, Angle, Angle) {
    let a = Point2d::distance_to(&p3_c, p2_b);
    let b = Point2d::distance_to(&p1_a, p3_c);
    let c = Point2d::distance_to(&p2_b, p1_a);
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

#[derive(Clone, Debug, Default)]
pub struct PointVec {
    int: Vec<Point2d>,
    idx: usize,
}

impl PointVec {
    pub fn new() -> Self {
        PointVec {
            int: vec![],
            idx: 0,
        }
    }
}

impl From<Vec<Point2d>> for PointVec {
    fn from(from: Vec<Point2d>) -> Self {
        PointVec { int: from, idx: 0 }
    }
}

impl Iterator for PointVec {
    type Item = (Point2d, Point2d);

    fn next(&mut self) -> Option<Self::Item> {
        let current = match self.int.get(self.idx) {
            Some(pt) => *pt,
            None => return None,
        };
        let next = match self.int.get(self.idx + 1) {
            Some(pt) => *pt,
            None => current,
        };

        self.idx += 1;
        Some((current, next))
    }
}

impl Deref for PointVec {
    type Target = Vec<Point2d>;

    fn deref(&self) -> &Self::Target {
        &self.int
    }
}

impl DerefMut for PointVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.int
    }
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
    Colinear,
    ClockWise,
    CounterClockWise,
}
