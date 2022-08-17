use crate::geometry::orientation::Orientation;
use crate::{
    error::{NCResult, Nari2DCoreError},
    geometry::{
        angle::Angle,
        mesh::{Edge, EdgeId, PointEdge, PointId, Triangle, TriangleEdge, TriangleId},
        point2d::Point2d,
    },
};
use cgmath::MetricSpace;
use itertools::Itertools;
use nanorand::{Rng, WyRand};
use rstar::{primitives::GeomWithData, DefaultParams, RTree};
use slotmap::{SecondaryMap, SlotMap};
use smallvec::smallvec;
use std::collections::VecDeque;

// based off of https://www.gradientspace.com/tutorials/dmesh3
#[derive(Clone, Debug, Default)]
pub struct EditMesh {
    points: SlotMap<PointId, Point2d>,
    triangles: SlotMap<TriangleId, Triangle>,
    edges: SlotMap<EdgeId, Edge>,
    point_edges: SecondaryMap<PointId, PointEdge>,
    triangle_edges: SecondaryMap<TriangleId, TriangleEdge>,
    boarder_edges: Vec<EdgeId>,
}

impl EditMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn point(&self, id: PointId) -> Option<&Point2d> {
        self.points.get(id)
    }

    pub fn triangle(&self, id: TriangleId) -> Option<&Triangle> {
        self.triangles.get(id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn point_edge(&self, id: PointId) -> Option<&PointEdge> {
        self.point_edges.get(id)
    }

    pub fn triangle_edge(&self, id: TriangleId) -> Option<&TriangleEdge> {
        self.triangle_edges.get(id)
    }

    pub fn edge_from_points(&self, a: PointId, b: PointId) -> Option<&EdgeId> {
        let a_edges = self.point_edges.get(a)?;
        let b_edges = self.point_edges.get(b)?;

        for aege in a_edges.edges() {
            for bege in b_edges.edges() {
                if aege == bege {
                    return Some(aege);
                }
            }
        }

        None
    }

    pub fn insert_point(&mut self, point: Point2d) -> PointId {
        self.points.insert(point)
    }

    pub fn remove_point(&mut self, id: PointId) -> Option<Point2d> {
        self.points.remove(id)
    }

    pub fn insert_point_with_connectivity(&mut self, point: Point2d) -> Option<PointId> {
        // first, we need to determine if the point lies outside or inside.
        match self.point_inside_polygon(&point) {
            true => {}
            false => {}
        }
    }

    fn insert_point_outside(&mut self, point: Point2d) -> Option<PointId> {
        // get nearest 2 points
        let nearest_2 = self.closest_n(&point, 2)?;
        let pt_a = *nearest_2.get(0)?;
        let pt_b = *nearest_2.get(1)?;
        let a_b_edge = *self.edge_from_points(pt_a, pt_b)?; // this is the edge we will delete from the hull
        let new_pt = self.insert_point(point);
        // find the ccw triangle for this
        // make sure they are not colinear
        let real_pt_a = self.points.get(pt_a)?;
        let real_pt_b = self.points.get(pt_b)?;
        let real_pt_c = self.points.get(new_pt)?;
        let sorted_triangle: Triangle = match Point2d::orientation(real_pt_a, real_pt_b, real_pt_c)
        {
            Orientation::CounterClockWise => [pt_a, pt_b, new_pt].into(),
            Orientation::ClockWise => [pt_a, new_pt, pt_b].into(),
            Orientation::Colinear => {
                self.remove_point(new_pt);
                return None;
            }
        };

        let triangleid = self.insert_triangle(sorted_triangle);
        // insert the 2 new edges as well
        let edge_new_a = Edge {
            point0: pt_a,
            point1: new_pt,
            triangle0: Some(triangleid),
            triangle1: None,
        };
        let edge_new_b = Edge {
            point0: pt_b,
            point1: new_pt,
            triangle0: Some(triangleid),
            triangle1: None,
        };
        let edge_npt_a_id = self.insert_edge(edge_new_a);
        let edge_npt_b_id = self.insert_edge(edge_new_b);
        // update the existing edge
        self.edges
            .get_mut(a_b_edge)?
            .add_triangle_unoccupied(triangleid)?;
        // update point edges and triangle edges
        self.triangle_edges.insert(
            triangleid,
            TriangleEdge {
                edges: [edge_npt_a_id, edge_npt_b_id, a_b_edge],
            },
        );
        // self.point_edges
    }

    pub fn insert_edge(&mut self, edge: Edge) -> EdgeId {
        self.edges.insert(edge)
    }

    pub fn insert_triangle(&mut self, triangle: Triangle) -> TriangleId {
        self.triangles.insert(triangle)
    }

    // https://repositorium.sdum.uminho.pt/bitstream/1822/6429/1/ConcaveHull_ACM_MYS.pdf
    pub fn calculate_concave_hull(&self, nearest: i32) -> NCResult<Vec<EdgeId>> {
        let how_many_nearest = nearest as usize;

        if self.points.len() < how_many_nearest {
            return Err(Nari2DCoreError::TooFewPoints(self.points.len() as u8));
        }

        let data = self
            .points
            .iter()
            .map(|(data, pt)| GeomWithData::new(*pt, data))
            .collect_vec();
        let mut tree: RTree<GeomWithData<Point2d, PointId>, DefaultParams> =
            RTree::bulk_load_with_params(data); // TODO: maybe try contributing a const rtree with borrowed data?
        let mut hull = vec![];
        let first_point = tree
            .nearest_neighbor(&Point2d::new(0_f32, f32::NEG_INFINITY))
            .ok_or(Nari2DCoreError::ThisIsABug(
                "No neighbour - this is a bug! please report it!".into(),
            ))?;
        hull.push(first_point.data);

        let mut current_point = first_point;

        tree.remove_at_point(first_point.geom());

        let mut previous_angle = Angle::new(0_f32);
        let mut step = 2;

        while (current_point != first_point || step == 2) && hull.len() < self.points.len() {
            if step == 5 {
                tree.insert(*first_point);
            }
            let nearest_points_by_angle = tree
                .nearest_neighbor_iter(current_point.geom())
                .take(how_many_nearest)
                .sorted_by(|a, b| {
                    // FIXME: I dont know what "right hand turn" is
                    let mut subpt = *current_point.geom();
                    subpt.y -= 1_f32;
                    let a_angle = Angle::from_3_points(&subpt, current_point.geom(), a.geom());
                    let b_angle = Angle::from_3_points(&subpt, current_point.geom(), b.geom());
                    a_angle.cmp(&b_angle)
                })
                .collect_vec();

            let mut its = true;
            let mut index = 0;

            while its && index < nearest_points_by_angle.len() {
                index += 1;
                let lastpoint = if nearest_points_by_angle.get(index) == Some(&first_point) {
                    1
                } else {
                    0
                };

                let mut jdx_mutator = 2;
                its = false;
                while !its && jdx_mutator < (hull.len() - lastpoint) {
                    let a = some_or_continue!(hull
                        .get(step - 1)
                        .map(|x| self.points.get(*x))
                        .flatten());
                    let b = some_or_continue!(nearest_points_by_angle.get(index)).geom();
                    let c = some_or_continue!(hull
                        .get(step - 1 - jdx_mutator)
                        .map(|x| self.points.get(*x))
                        .flatten());
                    let d = some_or_continue!(hull
                        .get(step - jdx_mutator)
                        .map(|x| self.points.get(*x))
                        .flatten());
                    its = Point2d::line_intersect((a, b), (c, d));
                    jdx_mutator += 1;
                }
            }

            if its {
                return Err(Nari2DCoreError::HullCalculation("k too small!".into()));
            }

            current_point = some_or_continue!(nearest_points_by_angle.get(index));
            hull.push(current_point.data);

            let h_step = some_or_continue!(hull.get(step).map(|x| self.points.get(*x)).flatten());
            let h_step_n1 =
                some_or_continue!(hull.get(step - 1).map(|x| self.points.get(*x)).flatten());
            previous_angle = Angle::from_2_points(h_step, h_step_n1);
            tree.remove(current_point);
            step += 1;
        }
        let polygon = hull
            .iter()
            .map(|x| self.points.get(*x).map(|x| *x))
            .collect::<Option<Vec<Point2d>>>()
            .ok_or(Nari2DCoreError::ThisIsABug("This should not error!".into()))?;

        for point in tree.iter() {
            // check points
            if !point.geom().is_inside(&polygon) {
                return Err(Nari2DCoreError::HullCalculation("k too small!".into()));
            }
        }

        let mut previous = *hull.last().ok_or(Nari2DCoreError::ThisIsABug(
            "Hull should not be empty".into(),
        ))?;
        let mut hull_edges = vec![];

        for pid in hull {
            match self.edge_from_points(pid, previous) {
                Some(eid) => hull_edges.push(*eid),
                None => {
                    return Err(Nari2DCoreError::DoesNotExist(
                        "Edge for points does not exist, is the indexed mesh broken?".into(),
                    ))
                }
            }
        }

        Ok(hull_edges)
    }

    pub fn recalculate_hull(&mut self, smoothness: i32) -> NCResult<()> {
        let hull = self.calculate_concave_hull(smoothness)?;
        self.boarder_edges = hull;
        Ok(())
    }

    // http://paper.academicpub.org/Paper?id=15630
    pub fn retriangulate_mesh(&mut self) -> NCResult<()> {
        const POINT_SHIFT_CONST: f32 = 0.000_001_f32;

        if self.points.len() > 3 {
            return Err(Nari2DCoreError::TooFewPoints(self.points.len() as u8));
        }

        let mut rand = WyRand::new();
        let mut shifted_amts = SecondaryMap::with_capacity(self.points.len());

        self.points.iter_mut().for_each(|(id, pt)| {
            let random_deviation = (rand.generate_range(-9..9) as f32) * POINT_SHIFT_CONST; // garuntee no points are colinear
            pt.x += random_deviation;
            shifted_amts.insert(id, random_deviation);
        });

        let mut convex_hull = Vec::with_capacity(3);

        // clear edges and relations
        self.edges.clear();
        self.triangles.clear();
        self.point_edges.clear();
        self.boarder_edges.clear();
        self.triangle_edges.clear();

        // seed triangle
        let mut pts = self
            .points
            .iter()
            .sorted()
            .collect::<VecDeque<(PointId, &Point2d)>>();
        let seed_tri_p0 = pts
            .pop_front()
            .ok_or(Nari2DCoreError::ThisIsABug("This should exist!".into()))?;
        let seed_tri_p1 = pts
            .pop_front()
            .ok_or(Nari2DCoreError::ThisIsABug("This should exist!".into()))?;
        let seed_tri_p2 = pts
            .pop_front()
            .ok_or(Nari2DCoreError::ThisIsABug("This should exist!".into()))?;
        if Point2d::orientation(seed_tri_p0.1, seed_tri_p2.1, seed_tri_p1.1).is_counter_clock_wise()
        {
            //
        } else {
            //
        }
    }

    // https://github.com/mourner/delaunator-rs/blob/master/src/lib.rs
    pub fn closest(&self, point: &Point2d) -> Option<PointId> {
        if self.points.len() == 0 {
            return None;
        }
        let mut distance_min = f32::INFINITY;
        let mut point_id = PointId::default();

        for (id, pt) in self.points.iter() {
            let dist = pt.distance2(point);
            if dist > 0_f32 && dist < distance_min {
                point_id = id;
                distance_min = dist;
            }
        }

        if distance_min == f32::INFINITY {
            None
        } else {
            Some(point_id)
        }
    }

    // https://github.com/mourner/delaunator-rs/blob/master/src/lib.rs
    pub fn closest_n(&self, point: &Point2d, n: u32) -> Option<Vec<PointId>> {
        if self.points.len() == 0 {
            return None;
        }
        let distanced = self
            .points
            .iter()
            .sorted_by(|(_, a), (_, b)| {
                let dist_a = point.distance2(a);
                let dist_b = point.distance2(b);

                f32::total_cmp(&dist_a, &dist_b)
            })
            .map(|(id, _)| id)
            .take(n as usize)
            .collect_vec();

        Some(distanced)
    }

    // https://github.com/mourner/delaunator-rs/blob/master/src/lib.rs
    pub fn bbox_center(&self) -> Point2d {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::INFINITY;

        for point in self.points.values() {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.min(point.x);
            max_y = max_y.min(point.y);
        }

        Point2d::new((min_x + max_x) / 2_f32, (min_y + max_y) / 2_f32)
    }

    pub fn seed_triangle(&self) -> Option<[PointId; 3]> {
        let center = self.bbox_center();
        let idx_a = self.closest(&center)?;
        let a_point = self.points.get(idx_a)?;

        let idx_b = self.closest(a_point)?;
        let b_point = self.points.get(idx_b)?;

        let mut radius_min = f32::INFINITY;
        let mut idx_c = PointId::default();

        for (idx, pt) in self.points.iter() {
            if idx == idx_a || idx == idx_b {
                continue;
            }

            let circumradius = Point2d::circumradius2(a_point, b_point, pt);
            if circumradius < radius_min {
                idx_c = idx;
                radius_min = circumradius;
            }
        }

        if radius_min == f32::INFINITY {
            None
        } else {
            let c_point = self.points.get(idx_c)?;
            let oriented_pts = if Point2d::orientation(a_point, b_point, c_point).is_clockwise() {
                [idx_a, idx_c, idx_b]
            } else {
                [idx_a, idx_b, idx_c]
            };
            Some(oriented_pts)
        }
    }

    fn visible_edge(&self, edges: &[EdgeId], point: &Point2d) -> Option<EdgeId> {
        todo!()
    }

    fn point_inside_polygon(&self, point: &Point2d) -> bool {
        // turn the edges into points
        let point_a_gon = match self
            .boarder_edges
            .iter()
            .map(|x| {
                self.edges
                    .get(*x)
                    .map(|edge| self.points.get(edge.point0).map(|pt| *pt))
                    .flatten()
            })
            .collect::<Option<Vec<Point2d>>>()
        {
            None => return false,
            Some(v) => v,
        };

        point.is_inside(&point_a_gon)
    }
}
