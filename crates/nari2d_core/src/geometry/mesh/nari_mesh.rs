use crate::{
    collections::{
        indexbimap::{IndexBiMap, Values},
        point_store::PointStore,
        two_elem_move_once::TwoElemMoveOnceVec,
    },
    error::{mesh::MeshError, util::IndexOrValue, NResult},
    geometry::{
        mesh::{
            area, concave_hull, find_power_of_2_splitting, is_edge_encroached, is_subsegment,
            line_intersect_circle, segment_intersects, triangle_centroid, triangle_circumcenter,
            Edge, EdgeRef, PointRef, Triangle, TriangleRef,
        },
        Angle, Point2d,
    },
};
use ahash::RandomState;
use itertools::Itertools;
use rstar::RTree;
use staticvec::{staticvec, StaticVec};
use std::{
    collections::{hash_set::IntoIter, BTreeMap, BTreeSet, HashMap, HashSet},
    ops::Index,
    rc::Rc,
};

// This is a similar mesh implementation to Triangle's Triangle based tri-mesh.
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq)]
pub struct NariMesh {
    points: PointStore<PointRef, Point2d>,
    triangles: IndexBiMap<TriangleRef, Triangle>,
    edges: HashMap<Edge, bool, RandomState>,
    point_relations: HashMap<PointRef, HashSet<TriangleRef, RandomState>, RandomState>,
    unit: f32,
}

impl NariMesh {
    pub fn new(points: Vec<Point2d>) -> NResult<Self> {
        let mut points = points;
        // remove duplicates so we don't explode later
        points.sort();
        points.dedup();

        let mut mesh = NariMesh {
            points: points.into(),
            triangles: Default::default(),
            edges: Default::default(),
            point_relations: Default::default(),
            unit: 0.0,
        };

        mesh.triangulate()?;

        Ok(mesh)
    }

    pub fn points_iter<'a>(&self) -> Values<'a, PointRef, Point2d> {
        self.points.values()
    }

    pub fn points_len(&self) -> usize {
        self.points.len()
    }

    pub fn triangle_iter<'a>(&self) -> Values<'a, TriangleRef, Triangle> {
        self.triangles.values()
    }

    pub fn triangles_len(&self) -> usize {
        self.triangles.len()
    }

    pub fn mesh_area(&self) -> Option<f64> {
        let mut total_area = 0_f64;
        for triangle in self.triangle_iter() {
            let point1 = self.points.get_by_index(&triangle.p1())?;
            let point2 = self.points.get_by_index(&triangle.p2())?;
            let point3 = self.points.get_by_index(&triangle.p3())?;
            total_area += area(point1, point2, point3);
        }
        Some(total_area)
    }

    pub fn triangulate(&mut self) -> NResult<()> {
        let points = self.points.values().map(|x| *x).collect::<Vec<_>>();
        let concave_hull = concave_hull(&points, 3)?;

        let mut input_edges = TwoElemMoveOnceVec::from(concave_hull.into_iter())
            .map(|(start, end)| (*start, *end))
            .collect::<Vec<(usize, usize)>>();

        // set the unit for the mesh that we will later use to figure out the concentric circles
        // determined by 0.04 * shortest_edge_length. Otherwise, it is set to 0.04
        let unit = input_edges
            .iter()
            .map(|e| {
                let start = points[e.0];
                let end = points[e.1];
                0.5 * start.distance_to(&end)
            })
            .min()
            .unwrap_or(0.5);

        let triangulation = match cdt::triangulate_with_edges(
            &points
                .iter()
                .map(|pt| (pt.x() as f64, pt.y() as f64))
                .collect::<Vec<(f64, f64)>>(),
            input_edges.iter(),
        ) {
            Ok(t) => t,
            Err(why) => {
                return Err(MeshError::Triangulation {
                    why: why.to_string(),
                }
                .into())
            }
        };

        let mut triangles: IndexBiMap<TriangleRef, Triangle> =
            IndexBiMap::with_capacity(triangulation.len() as u32);
        let mut point_relations =
            HashMap::with_capacity_and_hasher(triangulation.len() * 3, RandomState::new());
        let mut edges =
            HashMap::with_capacity_and_hasher(triangulation.len() * 3, RandomState::new());

        for tri in triangulation {
            let triangle = Triangle::new(tri.0 as u32, tri.1 as u32, tri.2 as u32);
            let triangle_idx = triangles.insert(triangle).0;

            for p_ref in triangle.points() {
                match point_relations.get_mut(&p_ref) {
                    Some(tris) => tris.insert(triangle_idx as u32),
                    None => {
                        let mut tris = HashSet::with_capacity_and_hasher(3, RandomState::new());
                        tris.insert(triangle_idx as u32);
                        point_relations.insert(p_ref, tris);
                    }
                }
            }

            for sub_edge in triangle.edges() {
                let sub_point_edge = (
                    points[sub_edge.start() as usize],
                    points[sub_edge.end() as usize],
                );
                for super_edge in input_edges.iter() {
                    let super_point_edge = (
                        points[super_edge.start() as usize],
                        points[super_edge.end() as usize],
                    );
                    edges.insert(sub_edge, is_subsegment(&super_point_edge, &sub_point_edge));
                }
            }
        }

        let points = points.into();

        self.unit = unit;
        self.triangles = triangles;
        self.points = points;
        self.point_relations = point_relations;
        self.edges = edges;

        Ok(())
    }

    pub fn hull_edges(&self) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|(e, s)| return if s { Some(*e) } else { None })
            .collect()
    }

    pub fn hull_points(&self) -> Vec<Point2d> {
        self.hull_edges()
            .into_iter()
            .map(|e| (e.start(), e.end()))
            .filter_map(|(start, end)| {
                let a = match self.points.get_by_index(&start) {
                    Some(p) => p,
                    None => return None,
                };
                let b = match self.points.get_by_index(&end) {
                    Some(p) => p,
                    None => return None,
                };
                Some([a, b])
            })
            .flat_map(|x| *x)
            .unique()
            .collect()
    }

    // All these use a modified version of Bowyer Watson. https://en.wikipedia.org/wiki/Bowyer%E2%80%93Watson_algorithm
    // wattson in boiler in de_inferno what will she do
    pub fn insert_point(&mut self, point: Point2d) -> NResult<()> {
        // check if outside or inside hull
        let hull = self.hull_points();
        if point.point_in_polygon(&hull) {}
        Ok(())
    }

    pub fn insert_points(&mut self, point: impl IntoIterator<Item = Point2d>) -> NResult<()> {}

    // ONLY VALID IF INSIDE POLYGON!
    fn ipt_boiler_wattson(&mut self, point: Point2d) -> NResult<()> {
        // boiler wattson go!
        let mut bad_triangles = Vec::with_capacity(6);

        // use triangle connectivity to find all the triangles that might contain this point
        // 6 should be enough (?)
        self.points
            .nearest_neighbor_iter(&point)
            .take(6)
            .filter_map(|point| self.points.get_by_value(point))
            .filter_map(|p_ref| self.point_relations.get(p_ref))
            .flatten()
            .unique()
            .filter_map(|tri_ref| Some((*tri_ref, *(self.triangles.get_by_index(tri_ref)?))))
            .filter_map(|(index, triangle)| {
                let mut points: StaticVec<Point2d, 3> = StaticVec::new();
                for pt in triangle.points() {
                    match self.points.get_by_index(&pt) {
                        Some(p) => {
                            points.push(*p);
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some((index, (points[0], points[1], points[2])))
            })
            .for_each(|(i, (p1, p2, p3))| {
                if point.point_in_circumcircle(&p1, &p2, &p3) {
                    bad_triangles.push(i);
                }
            });

        let mut polygon = Vec::with_capacity(10);

        for tri_ref in bad_triangles {
            if let Some(triangle) = self.triangles.get_by_index(&tri_ref) {
                for edge in triangle.edges() {
                    if let Some(relations) = self.edge_refs(&edge) {}
                }
            }
        }

        Ok(())
    }

    fn ipt_outside(&mut self, point: Point2d) {}

    pub fn remove_point(&mut self, point: Point2d) -> NResult<Point2d> {}

    pub fn remove_points(&mut self, point: impl IntoIterator<Item = Point2d>) -> NResult<()> {}

    pub fn edge_refs(&self, edge: &Edge) -> Option<StaticVec<TriangleRef, 2>> {
        let start_point_tri_ref = self.point_relations.get(&edge.start())?;
        let end_point_tri_ref = self.point_relations.get(&edge.end())?;
        let mut intersection = start_point_tri_ref.intersection(end_point_tri_ref);
        let mut tri_refs: StaticVec<TriangleRef, 2> = StaticVec::new();

        match intersection.nth(0) {
            Some(i) => tri_refs.push(*i),
            None => {}
        }
        match intersection.nth(0) {
            Some(i) => tri_refs.push(*i),
            None => {}
        }
        Some(tri_refs)
    }

    pub fn insert_triangle_raw(&mut self, triangle: Triangle) -> TriangleRef {
        let triangle_ref = self.triangles.insert(triangle).0 as TriangleRef;

        triangle.points().into_iter().for_each(|p_ref| {
            match self.point_relations.get_mut(&p_ref) {
                Some(tri_refs) => {
                    tri_refs.insert(triangle_ref);
                }
                None => {
                    let mut tri_refs = HashSet::with_capacity_and_hasher(4, RandomState::new());
                    tri_refs.insert(triangle_ref);
                    self.point_relations.insert(p_ref, tri_refs);
                }
            }
        });

        triangle.edges().into_iter().for_each(|edge| {
            if let None = self.edges.get(&edge) {
                self.edges.insert(edge, false);
            }
        });

        triangle_ref
    }

    pub fn split_edge_across(&mut self, edge: &Edge, along: f32) -> NResult<([Edge; 2], PointRef)> {
        let edge_start = match self.points.get_by_index(&edge.start()) {
            Some(p) => p,
            None => {
                return Err(MeshError::PointNotFound {
                    idx: IndexOrValue::Index(edge.start().into()),
                }
                .into())
            }
        };

        let edge_end = match self.points.get_by_index(&edge.end()) {
            Some(p) => p,
            None => {
                return Err(MeshError::PointNotFound {
                    idx: IndexOrValue::Index(edge.end().into()),
                }
                .into())
            }
        };

        // check if the edge was a input edge
        let is_input_edge = self.edges.remove(&edge).unwrap_or(false);

        // remove the edge
        let previous_triangles = self.edge_refs(edge).unwrap_or_default();

        let mut non_edge_points: StaticVec<u32, 2> = StaticVec::new();

        for t_ref in previous_triangles {
            if let Some((_, triangle)) = self.triangles.remove_by_index(t_ref) {
                let non_edge = triangle.non_edge_point(edge)?;
                non_edge_points.push(non_edge);
            }
        }

        let in_between_point = edge_start.linear_interpolate(edge_end, along);

        // insert point
        let new_point_idx = self.points.insert(in_between_point).0;

        // add new edges
        let start_np = Edge::new(edge.start(), new_point_idx as u32);
        let end_np = Edge::new(edge.end(), new_point_idx as u32);

        // if it is an input edge set these as input edges as well
        self.edges.insert(start_np, is_input_edge);
        self.edges.insert(end_np, is_input_edge);

        // make and insert triangle and edges
        for non_edge_point in non_edge_points {
            let new_triangle_start =
                Triangle::new(new_point_idx as u32, edge.start(), non_edge_point);

            let new_triangle_end = Triangle::new(new_point_idx as u32, edge.end(), non_edge_point);

            let new_triangle_start_idx = self.triangles.insert(new_triangle_start).0 as TriangleRef;
            let new_triangle_end_idx = self.triangles.insert(new_triangle_end).0 as TriangleRef;

            new_triangle_start.points().into_iter().for_each(|pt| {
                match self.point_relations.get_mut(&pt) {
                    Some(tri_refs) => {
                        tri_refs.insert(new_triangle_start_idx);
                    }
                    None => {
                        let mut tri_refs = HashSet::with_capacity_and_hasher(4, RandomState::new());
                        tri_refs.insert(new_triangle_start_idx);
                        self.point_relations.insert(pt, tri_refs);
                    }
                }
            });

            new_triangle_end.points().into_iter().for_each(|pt| {
                match self.point_relations.get_mut(&pt) {
                    Some(tri_refs) => {
                        tri_refs.insert(new_triangle_end_idx);
                    }
                    None => {
                        let mut tri_refs = HashSet::with_capacity_and_hasher(4, RandomState::new());
                        tri_refs.insert(new_triangle_end_idx);
                        self.point_relations.insert(pt, tri_refs);
                    }
                }
            });
        }

        Ok(([start_np, end_np], new_point_idx))
    }

    // Terminator implementation of https://www.sciencedirect.com/science/article/pii/S0925772101000475?via%3Dihub
    pub fn refine_mesh(&mut self, min_angle: Angle) -> NResult<()> {
        let max_area = match self.mesh_area() {
            Some(a) => (a / (self.triangles_len() as f64)) as f32,
            None => f64::INFINITY,
        };

        let mut encroached_edges = Vec::with_capacity(self.edges.len() / 10);
        let mut encroached_triangles = Vec::with_capacity(self.triangles.len() / 10);

        encroached_edges.append(
            &mut self
                .edges
                .iter()
                .filter(|(_, sub)| **!sub)
                .map(|edge| {
                    // Since we are only checking if a point is encroached, all we need to do is get the nearest points
                    let p1 = match self.points.get_by_index(&edge.start()) {
                        Some(p) => p,
                        None => return None,
                    };
                    let p2 = match self.points.get_by_index(&edge.end()) {
                        Some(p) => p,
                        None => return None,
                    };
                    let midpoint = Point2d::mid_point(p1, p2);
                    let max_distance = p1.distance_to(&midpoint);

                    for nearest_pt in self.points.nearest_neighbor_iter(&midpoint) {
                        if nearest_pt == p1 || nearest_pt == p2 {
                            continue;
                        } else {
                            let distance = nearest_pt.distance_to(&midpoint);
                            return if distance <= max_distance {
                                Some(edge)
                            } else {
                                None
                            };
                        }
                    }
                    None
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<Edge>>(),
        );

        encroached_triangles.append(
            &mut self
                .split_encroached_subsegments(&mut encroached_edges, min_angle, max_area)
                .into_iter()
                .map(|tri_ref| self.triangles.get_by_index(&tri_ref))
                .filter(Option::is_some)
                .map(|x| *(x.unwrap()))
                .collect::<Vec<Triangle>>(),
        );

        for triangle in self.triangles.values() {
            let non_input_edges = triangle
                .edges()
                .into_iter()
                .filter(|e| *(self.edges.get(e).unwrap_or(&false)))
                .collect::<Vec<Edge>>();

            if non_input_edges.len() >= 2 {
                let edges_pair_iter = TwoElemMoveOnceVec::from(non_input_edges.into_iter())
                    .map(|(e1, e2)| {
                        let e1_p1 = match self.points.get_by_index(&e1.start()) {
                            Some(p) => p,
                            None => return Angle::new(0_f32),
                        };
                        let e1_p2 = match self.points.get_by_index(&e1.end()) {
                            Some(p) => p,
                            None => return Angle::new(0_f32),
                        };
                        let e2_p1 = match self.points.get_by_index(&e2.start()) {
                            Some(p) => p,
                            None => return Angle::new(0_f32),
                        };
                        let e2_p2 = match self.points.get_by_index(&e2.end()) {
                            Some(p) => p,
                            None => return Angle::new(0_f32),
                        };

                        // find center
                        return if e1_p1 == e2_p1 {
                            e1_p1.angle_of_3(e1_p2, e2_p2)
                        } else if e1_p1 == e2_p2 {
                            e1_p1.angle_of_3(e1_p2, e2_p1)
                        } else if e1_p2 == e2_p1 {
                            e1_p2.angle_of_3(e1_p1, e2_p2)
                        } else if e1_p2 == e2_p2 {
                            e1_p2.angle_of_3(e1_p1, e2_p1)
                        } else {
                            Angle::new(0_f32)
                        };
                    })
                    .filter(|a| a < &min_angle);

                if edges_pair_iter.count() != 0 {
                    encroached_triangles.push(*triangle)
                }
            } else {
                let p1 = match self.points.get_by_index(&triangle.p1()) {
                    Some(p) => p,
                    None => continue,
                };
                let p2 = match self.points.get_by_index(&triangle.p2()) {
                    Some(p) => p,
                    None => continue,
                };
                let p3 = match self.points.get_by_index(&triangle.p3()) {
                    Some(p) => p,
                    None => continue,
                };

                if area(p1, p2, p3) > max_size as f64 {
                    encroached_triangles.push(*triangle);
                }
            }
        }

        encroached_triangles.sort();
        encroached_triangles.dedup();

        while !encroached_triangles.is_empty() {
            let bad_triangle = unsafe { encroached_triangles.pop().unwrap_unchecked() };

            if let Some(_) = self.triangles.get_by_value(&bad_triangle) {
                let p1 = match self.points.get_by_index(&bad_triangle.p1()) {
                    Some(p) => p,
                    None => continue,
                };
                let p2 = match self.points.get_by_index(&bad_triangle.p2()) {
                    Some(p) => p,
                    None => continue,
                };
                let p3 = match self.points.get_by_index(&bad_triangle.p3()) {
                    Some(p) => p,
                    None => continue,
                };

                let circumcenter = triangle_circumcenter(p1, p2, p3);
                let radius = circumcenter.distance_to(p1);
                let e_edges = {
                    // we could probably do this better by using rtree, closest, etc etc
                    let mut ee = BTreeSet::new();

                    for edge in self.edges.keys() {
                        let p1 = match self.points.get_by_index(&edge.start()) {
                            Some(p) => p,
                            None => continue,
                        };
                        let p2 = match self.points.get_by_index(&edge.end()) {
                            Some(p) => p,
                            None => continue,
                        };

                        if is_edge_encroached(p1, p2, &circumcenter) {
                            ee.insert(*edge);
                        }
                    }
                    ee
                };

                if e_edges.len() == 0 {
                    let new_point = self.insert_circumcenter_with_radius(circumcenter, radius).1;
                    let (mut bad_edges, bad_t_refs) =
                        self.new_vertex(new_point, min_angle, max_area);
                    for t_ref in bad_t_refs {
                        match self.triangles.get_by_index(&t_ref) {
                            Some(t) => {
                                encroached_triangles.push(*t);
                            }
                            None => continue,
                        }
                    }
                    encroached_edges.append(&mut bad_edges);
                } else {
                    let shortest_edge = {
                        let mut distances =
                            staticvec![p1.distance_to(p2), p2.distance_to(p3), p1.distance_to(p3)];
                        distances.sort();
                        distances[0]
                    };

                    for edge in e_edges {
                        if area(p1, p2, p3) > max_area as f64
                            || self.split_permitted(&edge, shortest_edge)
                        {
                            encroached_edges.push(edge)
                        }
                    }

                    if !encroached_edges.is_empty() {
                        encroached_triangles.push(bad_triangle);
                        self.split_encroached_subsegments(
                            &mut encroached_edges,
                            min_angle,
                            max_area,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn split_permitted(&mut self, edge: &Edge, length: f32) -> bool {
        // what the FUCK is a subsegment cluster
        // <insert dog wheelchair image>

        // since the power of 2^i formula is self.area * 2^i = edge_length,
        // i = log_2(((edge_length / self.area) / 2))
        let nearest_power = (2_f32 * length / self.unit).log2();
        let nearest_power_pos_diff = nearest_power - ((nearest_power as u32) as f32);
        let nearest_power_neg_diff = (((nearest_power as u32) as f32) - nearest_power).abs();
        let tolerance = 0.2_f32;
        if *(self.edges.get(edge).unwrap_or(&false))
            || (nearest_power_pos_diff <= tolerance && nearest_power_neg_diff >= tolerance)
        {
            return true;
        }

        // go down each edge of the edge to find its point and see if its a cluster.
        // We obtain that through getting the triangles with that point then getting all the
        // edges. We then orient then clockwise around the center point +Y up, then we measure all the
        // angles. For each angle >= 60, we add to subsegment. if < 60, we add a new cell. the no. of cells
        // is how many subsegments this is a part of

        let mut subsegment_clusters: Vec<HashSet<(Point2d, Point2d)>> = Vec::new();
        let point_ref_start = edge.start();
        let point_start = match self.points.get_by_index(&point_ref_start) {
            Some(pt) => *pt,
            None => return false,
        };
        let point_ref_end = edge.end();
        let point_end = match self.points.get_by_index(&point_ref_end) {
            Some(pt) => *pt,
            None => return false,
        };

        // go down p1
        match self.point_relations.get(&edge.start()) {
            Some(tris) => {
                let mut points_adjacent = tris
                    .iter()
                    .filter_map(|tri| {
                        if let Some(triangle) = self.triangles.get_by_index(tri) {
                            Some(
                                triangle
                                    .edges()
                                    .into_iter()
                                    .filter_map(|edge| {
                                        if edge.start() == point_ref_start
                                            || edge.end() == point_ref_start
                                        {
                                            Some(edge)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<Edge>>(),
                            )
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .filter_map(|edge| {
                        if edge.start() != point_ref_start {
                            Some(match self.points.get_by_index(&edge.start()) {
                                Some(pt) => *pt,
                                None => return None,
                            })
                        } else if edge.end() != point_ref_start {
                            Some(match self.points.get_by_index(&edge.end()) {
                                Some(pt) => *pt,
                                None => return None,
                            })
                        } else {
                            None
                        }
                    })
                    .map(|pt| (pt, point_start.angle_of_3(&point_end, &pt)))
                    .collect::<Vec<(Point2d, Angle)>>();

                points_adjacent.sort_by(|prev, next| prev.1.cmp(&next.1));

                let sixty_degrees = Angle::from_degrees(60_f32);
                let mut prev_angle = Angle::new(0_f32);
                let mut holding_set = HashSet::new();

                for (point, angle) in points_adjacent.into_iter() {
                    if (angle - prev_angle) > sixty_degrees {
                        subsegment_clusters.push(holding_set.clone());
                        holding_set.clear();
                    }
                    holding_set.insert((point, point_start));
                    prev_angle = angle;
                }
                if !holding_set.is_empty() {
                    subsegment_clusters.push(holding_set);
                }
            }
            None => {}
        }

        // go down p2
        match self.point_relations.get(&edge.end()) {
            Some(tris) => {
                let mut points_adjacent = tris
                    .iter()
                    .filter_map(|tri| {
                        if let Some(triangle) = self.triangles.get_by_index(tri) {
                            Some(
                                triangle
                                    .edges()
                                    .into_iter()
                                    .filter_map(|edge| {
                                        if edge.start() == point_ref_start
                                            || edge.end() == point_ref_start
                                        {
                                            Some(edge)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<Edge>>(),
                            )
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .filter_map(|edge| {
                        if edge.start() != point_ref_start {
                            Some(match self.points.get_by_index(&edge.start()) {
                                Some(pt) => *pt,
                                None => return None,
                            })
                        } else if edge.end() != point_ref_start {
                            Some(match self.points.get_by_index(&edge.end()) {
                                Some(pt) => *pt,
                                None => return None,
                            })
                        } else {
                            None
                        }
                    })
                    .map(|pt| (pt, point_end.angle_of_3(&point_start, &pt)))
                    .collect::<Vec<(Point2d, Angle)>>();

                points_adjacent.sort_by(|prev, next| prev.1.cmp(&next.1));

                let sixty_degrees = Angle::from_degrees(60_f32);
                let mut prev_angle = Angle::new(0_f32);
                let mut holding_set = HashSet::new();

                for (point, angle) in points_adjacent.into_iter() {
                    if (angle - prev_angle) > sixty_degrees {
                        subsegment_clusters.push(holding_set.clone());
                        holding_set.clear();
                    }
                    holding_set.insert((point, point_end));
                    prev_angle = angle;
                }
                if !holding_set.is_empty() {
                    subsegment_clusters.push(holding_set);
                }
            }
            None => {}
        }

        if subsegment_clusters.len() >= 2 {
            return true;
        }

        let cluster = match subsegment_clusters.get(0) {
            Some(c) => {
                let c = c.clone();
                TwoElemMoveOnceVec::from(c.into_iter())
            }
            None => {
                return false;
            }
        };

        let this_edge_distance = point_start.distance_to(&point_end);
        let mut separation_angles = Vec::with_capacity(cluster.len());

        for (p1, p2) in cluster.iter() {
            let edge_dist = p1.distance_to(p2);
            if edge_dist < this_edge_distance {
                return true;
            }
        }

        for (e1, e2) in cluster {
            separation_angles.push(e1.1.angle_of_3(&e1.0, &e2.0))
        }

        separation_angles.sort();

        let r_min = this_edge_distance.abs()
            * (separation_angles.get(0).unwrap_or(&Angle::new(0 as f32)) / 2_f32).sin();

        return r_min >= length;
    }

    fn insert_circumcenter_with_radius(
        &mut self,
        circumcenter: Point2d,
        radius: f32,
    ) -> (Vec<TriangleRef>, PointRef) {
        let mut new_triangles = Vec::with_capacity(4);
        let splitting_point_ref = self.points.insert(circumcenter).0;
        self.point_relations.insert(
            splitting_point_ref,
            HashSet::with_capacity_and_hasher(4, RandomState::new()),
        );

        // get points inside or on circumcircle
        let mut triangle_in_circle: HashMap<TriangleRef, Triangle, RandomState> =
            HashMap::with_capacity_and_hasher(16, RandomState::new());
        self.points
            .locate_within_distance(circumcenter, radius.powi(2))
            .map(|x| self.points.get_by_value(x).unwrap())
            .map(|i| self.point_relations.get(i).unwrap_or(&HashSet::default()))
            .fold(&mut triangle_in_circle, |acc, next| {
                for t_ref in next {
                    let triangle = match self.triangles.get_by_index(t_ref) {
                        Some(t) => t,
                        None => continue,
                    };

                    let p1 = match self.points.get_by_index(&triangle.p1()) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.points.get_by_index(&triangle.p2()) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p3 = match self.points.get_by_index(&triangle.p3()) {
                        Some(p) => p,
                        None => continue,
                    };

                    // ONLY re-introduce the circle if its centroid is within  the radius
                    let centroid = triangle_centroid(p1, p2, p3);

                    if centroid.distance_to(&circumcenter) < radius {
                        acc.insert(*t_ref, *triangle);
                    }
                }
                acc
            });

        let mut no_remove_point = HashSet::with_capacity_and_hasher(4, RandomState::new());
        // We will check if any of the edges inside are subsegments, and if our point lies on the other side. If so,
        // the splitting point will be the midpoint of the subsegment to preserve the subsegment.
        // TODO
        for tri in triangle_in_circle.values() {
            for edge in tri.edges() {
                if self.edges.get(&edge).unwrap_or(&false) {
                    no_remove_point.insert(edge.start());
                    no_remove_point.insert(edge.end());
                }
            }
        }

        let mut points_to_connect = TwoElemMoveOnceVec::new();

        // delete points, triangle, edges inside the circle.
        for (tri_ref, triangle) in triangle_in_circle {
            // remove triangle from points
            // check if point is eligible for removal
            let p1 = match self.points.get_by_index(&triangle.p1()) {
                Some(p) => {
                    if !no_remove_point.contains(&triangle.p1())
                        || !p.point_in_circle(&circumcenter, radius, false)
                    {
                        if let Some(t_set) = self.point_relations.get_mut(&tri_ref) {
                            t_set.remove(&triangle.p1());
                        }
                        self.points.remove(p)
                    } else {
                        points_to_connect.push(triangle.p1());
                    }
                    p
                }
                None => continue,
            };
            let p2 = match self.points.get_by_index(&triangle.p2()) {
                Some(p) => {
                    if !no_remove_point.contains(&triangle.p2())
                        || !p.point_in_circle(&circumcenter, radius, false)
                    {
                        if let Some(t_set) = self.point_relations.get_mut(&tri_ref) {
                            t_set.remove(&triangle.p2());
                        }
                        self.points.remove(p)
                    } else {
                        points_to_connect.push(triangle.p2());
                    }
                    p
                }
                None => continue,
            };
            let p3 = match self.points.get_by_index(&triangle.p3()) {
                Some(p) => {
                    if !no_remove_point.contains(&triangle.p3())
                        || !p.point_in_circle(&circumcenter, radius, false)
                    {
                        if let Some(t_set) = self.point_relations.get_mut(&tri_ref) {
                            t_set.remove(&triangle.p3());
                        }
                        self.points.remove(p)
                    } else {
                        points_to_connect.push(triangle.p3());
                    }
                    p
                }
                None => continue,
            };

            // check edges
            if line_intersect_circle(&p1, &p2, &circumcenter, radius) {
                self.edges.remove(&Edge::new(triangle.p1(), triangle.p2()));
            }
            if line_intersect_circle(&p2, &p3, &circumcenter, radius) {
                self.edges.remove(&Edge::new(triangle.p2(), triangle.p3()));
            }
            if line_intersect_circle(&p1, &p3, &circumcenter, radius) {
                self.edges.remove(&Edge::new(triangle.p1(), triangle.p3()));
            }
        }

        for (sp, ep) in points_to_connect {
            new_triangles.push(self.insert_triangle_raw(Triangle::new(
                *sp,
                *ep,
                splitting_point_ref,
            )));
        }

        (new_triangles, splitting_point_ref)
    }

    fn split_encroached_subsegments(
        &mut self,
        encroached: &mut Vec<Edge>,
        min_angle: Angle,
        max_area: f32,
    ) -> Vec<TriangleRef> {
        let bad_triangles = Vec::with_capacity(encroached.len() / 3);
        while !encroached.is_empty() {
            // SAFETY: We just did a bounds check (`!encroached.is_empty()`), so this unwrap is safe.
            let edge = unsafe { encroached.pop().unwrap_unchecked() };
            // choose from concentric shells
            // i fucking hate life, like there is so much more fun i could be having if i was fucking normal and played games in my winter break or some shit
            // but fuck no i had to be this weird ass fuck who does this BS in her spare time
            // and i cant even get good scores on tests
            // fml, wish there was an easy way to die
            if let Some(_) = self.edges.get(&edge) {
                let start_point = match self.points.get_by_index(&edge.start()) {
                    Some(p) => p,
                    None => continue,
                };
                let end_point = match self.points.get_by_index(&edge.end()) {
                    Some(p) => p,
                    None => continue,
                };

                let splitting_vertex_ratio =
                    find_power_of_2_splitting(start_point, end_point, self.unit);
                // split the vertex
                let (new_edges, new_point_index) =
                    self.split_edge_across(&edge, splitting_vertex_ratio)?;

                let (mut bad_edges, mut bad_triangles) =
                    self.new_vertex(new_point_index, min_angle, max_area);
                encroached.append(&mut bad_edges);
                bad_triangles.append(&mut bad_triangles);

                new_edges.into_iter().for_each(|e| {
                    match (
                        self.points.get_by_index(&e.start()),
                        self.points.get_by_index(&e.end()),
                    ) {
                        (Some(edge_start), Some(edge_end)) => {
                            let mid_point = edge_start.mid_point(edge_end);
                            let mut nearest_points = self.points.nearest_neighbor_iter(&mid_point);
                            let close_p1 = nearest_points.next();
                            let close_p2 = nearest_points.next();
                            let close_p3 = nearest_points.next();
                            if let Some(cp3) = close_p3 {
                                if edge_start != cp3
                                    && edge_end != cp3
                                    && is_edge_encroached(edge_start, edge_end, cp3)
                                {
                                    encroached.push(e);
                                }
                            }
                            if let Some(cp2) = close_p2 {
                                if edge_start != cp3
                                    && edge_end != cp3
                                    && is_edge_encroached(edge_start, edge_end, cp2)
                                {
                                    encroached.push(e);
                                }
                            }
                            if let Some(cp1) = close_p1 {
                                if edge_start != cp1
                                    && edge_end != cp1
                                    && is_edge_encroached(edge_start, edge_end, cp1)
                                {
                                    encroached.push(e);
                                }
                            }
                        }
                        (_, _) => {}
                    }
                });
            } else {
                continue;
            }
        }

        bad_triangles
    }

    fn new_vertex(
        &mut self,
        point_ref: PointRef,
        min_angle: Angle,
        max_size: f32,
    ) -> (Vec<Edge>, Vec<TriangleRef>) {
        let mut bad_edges = Vec::with_capacity(4);
        let mut bad_triangles = Vec::with_capacity(2);

        for tri_ref in self
            .point_relations
            .get(&point_ref)
            .unwrap_or(&HashSet::default())
        {
            if let Some(triangle) = self.triangles.get_by_index(tri_ref) {
                let opposite_edge = triangle.opposite_edge(point_ref)?;
                let oe_pt_start = match self.points.get_by_index(&opposite_edge.start()) {
                    Some(p) => p,
                    None => continue,
                };
                let oe_pt_end = match self.points.get_by_index(&opposite_edge.end()) {
                    Some(p) => p,
                    None => continue,
                };
                let point = match self.points.get_by_index(&point_ref) {
                    Some(p) => p,
                    None => continue,
                };

                if self.edges.get(&opposite_edge).unwrap_or(&false)
                    && &is_edge_encroached(oe_pt_start, oe_pt_end, point)
                {
                    bad_edges.push(opposite_edge);
                    continue;
                }

                let non_input_edges = triangle
                    .edges()
                    .into_iter()
                    .filter(|e| *(self.edges.get(e).unwrap_or(&false)))
                    .collect::<Vec<Edge>>();

                if non_input_edges.len() >= 2 {
                    let edges_pair_iter = TwoElemMoveOnceVec::from(non_input_edges.into_iter())
                        .map(|(e1, e2)| {
                            let e1_p1 = match self.points.get_by_index(&e1.start()) {
                                Some(p) => p,
                                None => return Angle::new(0_f32),
                            };
                            let e1_p2 = match self.points.get_by_index(&e1.end()) {
                                Some(p) => p,
                                None => return Angle::new(0_f32),
                            };
                            let e2_p1 = match self.points.get_by_index(&e2.start()) {
                                Some(p) => p,
                                None => return Angle::new(0_f32),
                            };
                            let e2_p2 = match self.points.get_by_index(&e2.end()) {
                                Some(p) => p,
                                None => return Angle::new(0_f32),
                            };

                            // find center
                            return if e1_p1 == e2_p1 {
                                e1_p1.angle_of_3(e1_p2, e2_p2)
                            } else if e1_p1 == e2_p2 {
                                e1_p1.angle_of_3(e1_p2, e2_p1)
                            } else if e1_p2 == e2_p1 {
                                e1_p2.angle_of_3(e1_p1, e2_p2)
                            } else if e1_p2 == e2_p2 {
                                e1_p2.angle_of_3(e1_p1, e2_p1)
                            } else {
                                Angle::new(0_f32)
                            };
                        })
                        .filter(|a| a < &min_angle);

                    if edges_pair_iter.count() != 0 {
                        bad_triangles.push(*tri_ref)
                    }
                } else {
                    let p1 = match self.points.get_by_index(&triangle.p1()) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.points.get_by_index(&triangle.p2()) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p3 = match self.points.get_by_index(&triangle.p3()) {
                        Some(p) => p,
                        None => continue,
                    };

                    if area(p1, p2, p3) > max_size as f64 {
                        bad_triangles.push(*tri_ref);
                    }
                }
            }
        }
        (bad_edges, bad_triangles)
    }
}
