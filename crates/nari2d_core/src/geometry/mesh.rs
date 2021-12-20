// Modified from delaunator crate to support f32 types.

use crate::{
    error::{NResult, Nari2DError},
    geometry::TMesh,
    geometry::{angles_of_triangle, Angle, IndexedPoint2d, Orientation, Point2d},
};
use cdt::Error;
use delaunator::{triangulate, Point};
use rstar::RTree;
use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};
use crate::geometry::PointVec;

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq)]
pub struct NariMesh {
    internal: TMesh,
}

impl NariMesh {
    pub fn new(points: Vec<Point2d>) -> NResult<Self> {
        let mut points = points;
        // remove duplicates so we dont explode later
        points.sort();
        points.dedup();

        let edges = PointVec::from(concave_hull(&points, 3)?);

        let triangulation = cdt::triangulate_with_edges()
    }
}

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

pub fn segment_intersects(segment_1: [Point2d; 2], segment_2: [Point2d; 2]) -> bool {
    let ori_1 = Point2d::orientation(&segment_1[0], &segment_1[1], &segment_2[0]);
    let ori_2 = Point2d::orientation(&segment_1[0], &segment_1[1], &segment_2[1]);
    let ori_3 = Point2d::orientation(&segment_2[0], &segment_2[1], &segment_1[0]);
    let ori_4 = Point2d::orientation(&segment_2[0], &segment_2[1], &segment_1[1]);

    if ori_1 != ori_2 && ori_3 != ori_4 {
        return true;
    }

    if ori_3 == Orientation::Colinear && segment_1[0].point_on_segment(&segment_2[0], &segment_2[1])
    {
        return true;
    }
    if ori_4 == Orientation::Colinear && segment_1[1].point_on_segment(&segment_2[0], &segment_2[1])
    {
        return true;
    }
    if ori_1 == Orientation::Colinear && segment_2[0].point_on_segment(&segment_1[0], &segment_1[1])
    {
        return true;
    }
    if ori_2 == Orientation::Colinear && segment_2[1].point_on_segment(&segment_1[0], &segment_1[1])
    {
        return true;
    }

    false
}
