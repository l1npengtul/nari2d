pub use angle::Angle;
pub use point2d::Point2d;
pub use scale2d::Scale2d;

mod angle;
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
