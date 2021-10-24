pub use angle::Angle;
pub use point2d::Point2d;
pub use scale2d::Scale2d;
use std::ops::{Deref, DerefMut};

mod angle;
mod bounds;
pub mod mesh;
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
