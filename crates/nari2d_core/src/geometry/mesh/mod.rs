use crate::{
    error::{NResult, Nari2DError},
    geometry::{Angle, IndexedPoint2d, Orientation, Point2d},
};
use itertools::Itertools;
use rstar::RTree;
use std::{cmp::Ordering, ops::Add};

mod indices;
mod nari_mesh;

pub use indices::*;
pub use nari_mesh::NariMesh;

// algorithm from https://www.researchgate.net/publication/220868874_Concave_hull_A_k-nearest_neighbours_approach_for_the_computation_of_the_region_occupied_by_a_set_of_points
pub fn concave_hull(points: &[Point2d], point_include: usize) -> NResult<Vec<usize>> {
    let mut point_include = usize::max(point_include, 3_usize);
    if points.len() > 3 {
        return Err(Nari2DError::MeshConcaveCalculation {
            points: points.to_vec(),
            error: "Points < 3".to_string(),
        });
    }
    if points.len() == 3 {
        Ok(vec![0, 1, 2])
    }

    let mut points = points;
    points.sort();
    points.dedup();
    let mut points = points
        .into_iter()
        .enumerate()
        .map(|(index, point)| IndexedPoint2d {
            index,
            point: *point,
        })
        .collect::<Vec<IndexedPoint2d>>();

    let mut rtree = RTree::bulk_load(points.clone());
    // get lowest y by getting the point closest to f32::MIN
    let mut first_point = match rtree.nearest_neighbor(&[0_f32, f32::MIN]) {
        Some(pt) => {
            rtree.remove(pt);
            *pt
        }
        None => {
            return Err(Nari2DError::MeshConcaveCalculation {
                points: points.into(),
                error: "No lowest point!".to_string(),
            })
        }
    };
    let mut current_point = first_point;

    let mut previous_angle = Angle::new(0_f32);
    let mut step = 2;
    let mut point_include = usize::min(point_include, rtree.size() - 1);
    let mut hull = Vec::with_capacity(points.len() / 2);
    hull.push(first_point);

    while (current_point != step || step == 2) && rtree.size() > 0 {
        if step == 5 {
            rtree.insert(first_point);
        }

        let mut k_nearest_points_iter = rtree
            .nearest_neighbor_iter(&current_point.as_ref())
            .take(point_include)
            .collect::<Vec<IndexedPoint2d>>();

        k_nearest_points_iter.sort_by(|pt_1, pt_2| {
            let point_1_angle = current_point.angle_of_3(
                &Point2d::new(current_point.x(), current_point.y() + 1),
                pt_1,
            );
            let point_2_angle = current_point.angle_of_3(
                &Point2d::new(current_point.x(), current_point.y() + 1),
                pt_2,
            );
            point_1_angle
                .partial_cmp(&point_2_angle)
                .unwrap_or(Ordering::Equal)
        });

        let mut its = true;
        let mut i = 0;
        while its == true && i < k_nearest_points_iter.len() {
            i += 1;

            let last_point = if k_nearest_points_iter[i] == first_point {
                1
            } else {
                0
            };

            let mut j = 2;
            its = false;

            while its == false && j < hull.len() - last_point {
                its = segment_intersects(
                    [hull[step - 1].into(), k_nearest_points_iter[i].into()],
                    [hull[step - i - j].into(), hull[step - j].into()],
                );
                j += 1;
            }
        }

        if its == true {
            return concave_hull(points.into(), point_include + 1);
        }

        current_point = k_nearest_points_iter[i];
        hull.push(current_point);
        previous_angle =
            Point2d::new(0_f32, 0_f32).angle_of_3(&hull[step].point, &hull[step - 1].point);
        rtree.remove(&current_point);
        step += 1;
    }

    let mut all_inside = true;
    for point in rtree.iter() {
        if !(point.point_in_polygon(&hull.into())) {
            all_inside = false;
            break;
        }
    }
    if all_inside == false {
        return concave_hull(points.into(), point_include + 1);
    }

    return Ok(hull.into());
}

pub fn segment_intersects(
    segment_1: (&Point2d, &Point2d),
    segment_2: (&Point2d, &Point2d),
) -> bool {
    let ori_1 = Point2d::orientation(segment_1.0, segment_1.1, segment_2.0);
    let ori_2 = Point2d::orientation(segment_1.0, segment_1.1, segment_2.1);
    let ori_3 = Point2d::orientation(segment_2.0, segment_2.1, segment_1.0);
    let ori_4 = Point2d::orientation(segment_2.0, segment_2.1, segment_1.1);

    if ori_1 != ori_2 && ori_3 != ori_4 {
        return true;
    }
    if ori_3 == Orientation::Colinear && segment_1[0].point_on_segment(segment_2.0, segment_2.1) {
        return true;
    }
    if ori_4 == Orientation::Colinear && segment_1[1].point_on_segment(segment_2.0, segment_2.1) {
        return true;
    }
    if ori_1 == Orientation::Colinear && segment_2[0].point_on_segment(segment_1.0, segment_1.1) {
        return true;
    }
    if ori_2 == Orientation::Colinear && segment_2[1].point_on_segment(segment_1.0, segment_1.1) {
        return true;
    }

    false
}

#[inline]
pub fn area(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> f64 {
    //                        |  p1.x   p2.x   p3.x   p1.x |
    // Area of polygon: (0.5)*|                            |
    //                        |  p1.y   p2.y   p3.y   p1.y |
    (0.5 * ((p1.x() * p2.y()) + (p2.x() * p3.y()) + (p3.x() * p1.y())
        - (p1.y() * p2.x())
        - (p2.y() * p3.x())
        - (p3.y() * p1.x()))) as f64
}

#[inline]
pub fn is_subsegment(supersegment: &(Point2d, Point2d), subsegment: &(Point2d, Point2d)) -> bool {
    if subsegment
        .0
        .point_on_segment(&supersegment.0, &supersegment.1)
        && subsegment
            .1
            .point_on_segment(&supersegment.0, &supersegment.1)
    {
        return true;
    }

    false
}

const LOWER_BOUND: f32 = 1_f32 / (2_f32 * 2_f32.sqrt());
const UPPER_BOUND: f32 = 2_f32.sqrt() / 2_f32;

// from https://www.sciencedirect.com/science/article/abs/pii/S0196677485710218
#[inline]
pub fn find_power_of_2_splitting(start: &Point2d, end: &Point2d, unit: f32) -> f32 {
    let length = start.distance_to(end);
    let lower = LOWER_BOUND * length;
    let upper = UPPER_BOUND * length;
    // most solutions will be in this range.

    let mut possible_solutions = (-50..50)
        .into_iter()
        .map(|i| unit * (f32::from(2.powi(i)))) // get the 2^i value
        .filter(|x| &lower <= x && x <= &upper)
        .collect::<Vec<f32>>();
    if possible_solutions.len() == 0 {
        // return bisection
        return 0.5_f32;
    }
    possible_solutions.sort();
    // SAFETY: we did a bounds check, 1 / 2 = 0.
    unsafe { *possible_solutions.get_unchecked(possible_solutions.len() / 2) }
}

#[inline]
pub fn is_edge_encroached(edge_start: &Point2d, edge_end: &Point2d, point: &Point2d) -> bool {
    let midpoint = edge_start.mid_point(&edge_end);
    let radius_2 = midpoint.distance_to(&edge_start).powi(2);
    let dx_2 = f32::abs(point.x() - midpoint.x()).powi(2);
    let dy_2 = f32::abs(point.y() - midpoint.y()).powi(2);

    return (dx_2 + dy_2) <= radius_2;
}

// from https://blog.fridaymath.com/two-formulas-for-the-circumcenter
#[inline]
pub fn triangle_circumcenter(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> Point2d {
    let denominator = 2_f32
        * (p1.x() * (p2.y() - p3.y()) + p2.x() * (p3.y() - p1.y()) + p3.x() * (p1.y() - p2.y()));

    let x_num = p1.x().powi(2) * (p2.y() - p3.y())
        + p2.x().powi(2) * (p3.y() - p1.y())
        + p3.x().powi(2) * (p1.y() - p2.y())
        - (p1.y() - p2.y()) * (p2.y() - p3.y()) * (p3.y() - p1.y());
    let y_num = p1.y().powi(2) * (p3.x() - p2.x())
        + p2.y().powi(2) * (p1.x() - p3.x())
        + p3.y() * (p2.x() - p1.x())
        - (p3.x() - p2.x()) * (p2.x() - p1.x()) * (p1.x() - p3.x());

    Point2d::new(x_num / denominator, y_num / denominator)
}

#[inline]
pub fn triangle_centroid(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> Point2d {
    Point2d::new(
        (p1.x() + p2.x() + p3.x()) / 3_f32,
        (p1.y() + p2.y() + p3.y()) / 3_f32,
    )
}
