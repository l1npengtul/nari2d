use crate::geometry::mesh::triangle_circumcenter;
use crate::{
    collections::{
        indexbimap::{IndexBiMap, Values},
        two_elem_move_once::TwoElemMoveOnceVec,
    },
    error::{mesh::MeshError, util::IndexOrValue, NResult},
    geometry::{
        mesh::{
            area, concave_hull, find_power_of_2_splitting, is_edge_encroached, is_subsegment, Edge,
            PointRef, Triangle, TriangleRef,
        },
        Angle, Point2d,
    },
};
use ahash::RandomState;
use rstar::RTree;
use staticvec::StaticVec;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

// This is a similar mesh implementation to Triangle's Triangle based tri-mesh.
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq)]
pub struct NariMesh {
    points: IndexBiMap<PointRef, Point2d>,
    triangles: IndexBiMap<TriangleRef, Triangle>,
    edges: HashMap<Edge, bool, RandomState>,
    point_relations: HashMap<PointRef, HashSet<TriangleRef, RandomState>, RandomState>,
    unit: f32,
}

impl NariMesh {
    pub fn new(points: Vec<Point2d>) -> NResult<Self> {
        let mut points = points;
        // remove duplicates so we dont explode later
        points.sort();
        points.dedup();

        let mut input_edges = TwoElemMoveOnceVec::from(concave_hull(&points, 3)?)
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

            for subedge in triangle.edges() {
                let sub_point_edge = (
                    points[subedge.start() as usize],
                    points[subedge.end() as usize],
                );
                for superedge in input_edges.iter() {
                    let super_point_edge = (
                        points[superedge.start() as usize],
                        points[superedge.end() as usize],
                    );
                    edges.insert(subedge, is_subsegment(&super_point_edge, &sub_point_edge));
                }
            }
        }

        let points: IndexBiMap<PointRef, Point2d> = IndexBiMap::with_data(points);

        Ok(NariMesh {
            points,
            triangles,
            point_relations,
            edges,
            unit,
        })
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

    pub fn insert_triangle_raw(&mut self, triangle: Triangle) {
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

        let mut point_rtree = RTree::bulk_load(self.points.values().map(|pt| *pt).collect());

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

                    for nearest_pt in point_rtree.nearest_neighbor_iter(&midpoint) {
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
                .split_encroached_subsegments(
                    &mut point_rtree,
                    &mut encroached_edges,
                    min_angle,
                    max_area,
                )
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
            }
            // TODO: Continue implement
        }

        //
        Ok(())
    }

    fn split_encroached_subsegments(
        &mut self,
        point_trees: &mut RTree<Point2d>,
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

                match self.points.get_by_index(&new_point_index) {
                    Some(p) => point_trees.insert(*p),
                    None => unreachable!(),
                }

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
                            let mut nearest_points = point_trees.nearest_neighbor_iter(&mid_point);
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